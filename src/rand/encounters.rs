use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::consts;
use crate::json::{Encounters, Randomizer, TNTStrategy};
use crate::rand::{structs::EncounterData, Objects};

fn skip(encounter: &EncounterData, preset: &Encounters) -> bool {
    return (!preset.cardmon
        && (consts::CARDMON_MIN <= encounter.digimon_id as u16
            && encounter.digimon_id as u16 <= consts::CARDMON_MAX))
        || (!preset.bosses && consts::BOSSES.contains(&(encounter.digimon_id as u16)))
        || (preset.strategy == TNTStrategy::Keep
            && encounter.digimon_id as u16 == consts::TRICERAMON_ID
            && encounter.multiplier == 16)
        || (preset.keep_zanbamon
            && encounter.digimon_id as u16 == consts::ZANBAMON_ID
            && encounter.multiplier == 16)
        || (preset.keep_galacticmon
            && consts::GALACTICMON_IDS.contains(&(encounter.digimon_id as u16)));
}

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let len = objects.encounters.original.len();
    let modified_encounters = &mut objects.encounters.modified;
    let modified_enemy_stats = &mut objects.enemy_stats.modified;
    let encounters = &objects.encounters.original;

    // Fisher-Yates shuffles
    for _ in 0..preset.shuffles {
        for i in 0..(len - 2) {
            let uniform: usize = rng.next_u64() as usize;
            let j = i + uniform % (len - i - 1);

            if skip(&modified_encounters[i], &preset.encounters)
                || skip(&modified_encounters[j], &preset.encounters)
            {
                continue;
            }

            modified_encounters.swap(i, j);
        }
    }

    if preset.encounters.strategy == TNTStrategy::Swap {
        let tric = modified_enemy_stats
            .iter()
            .find(|&x| x.digimon_id == consts::TRICERAMON_ID)
            .unwrap();

        let mut titem = tric.droppable_item;
        let mut tdrop = tric.drop_rate;

        let tric_index = encounters
            .iter()
            .position(|&x| {
                x.digimon_id as u16 == consts::TRICERAMON_ID && x.lv == 6 && x.multiplier == 16
            })
            .unwrap();

        let swapped = modified_enemy_stats
            .iter_mut()
            .find(|&&mut x| x.digimon_id == modified_encounters[tric_index].digimon_id as u16)
            .unwrap();

        std::mem::swap(&mut titem, &mut swapped.droppable_item);
        std::mem::swap(&mut tdrop, &mut swapped.drop_rate);

        let tricm = modified_enemy_stats
            .iter_mut()
            .find(|&&mut x| x.digimon_id == consts::TRICERAMON_ID)
            .unwrap();

        std::mem::swap(&mut titem, &mut tricm.droppable_item);
        std::mem::swap(&mut tdrop, &mut tricm.drop_rate);
    }
}
