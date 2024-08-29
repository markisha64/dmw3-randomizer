use std::collections::{BTreeMap, BTreeSet, HashSet};

use rand_xoshiro::Xoshiro256StarStar;

use crate::json::{Encounters, Randomizer, TNTStrategy};
use crate::rand::{dmw3_structs::EncounterData, Objects};
use crate::util::{self, uniform_random_vector};

fn skip(encounter: &EncounterData, preset: &Encounters) -> bool {
    (!preset.cardmon
        && (dmw3_consts::CARDMON_MIN <= encounter.digimon_id as u16
            && encounter.digimon_id as u16 <= dmw3_consts::CARDMON_MAX))
        || (!preset.bosses && dmw3_consts::BOSSES.contains(&(encounter.digimon_id as u16)))
        || (preset.strategy == TNTStrategy::Keep
            && encounter.digimon_id as u16 == dmw3_consts::TRICERAMON_ID
            && encounter.multiplier == 16)
        || (preset.keep_zanbamon
            && encounter.digimon_id as u16 == dmw3_consts::ZANBAMON_ID
            && encounter.multiplier == 16)
        || (preset.keep_galacticmon
            && dmw3_consts::GALACTICMON_IDS.contains(&(encounter.digimon_id as u16)))
}

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let len = objects.encounters.original.len();
    let modified_encounters = &mut objects.encounters.modified;
    let modified_enemy_stats = &mut objects.enemy_stats.modified;
    let encounters = &objects.encounters.original;

    let possible_ids: BTreeSet<u32> = BTreeSet::from_iter(
        encounters
            .iter()
            .filter(|x| !skip(x, &preset.encounters))
            .map(|x| x.digimon_id),
    );

    let skipped_count = encounters
        .iter()
        .filter(|x| skip(x, &preset.encounters))
        .count();

    let mut shuffled_encounters_digimon: BTreeMap<u32, Vec<EncounterData>> = BTreeMap::new();

    let possible_arr = Vec::from_iter(possible_ids);
    let mut shuffled_ids =
        util::uniform_random_vector(&possible_arr, len - skipped_count, preset.shuffles, rng);

    for digimon_id in possible_arr {
        let possible_encounters: Vec<&EncounterData> = encounters
            .iter()
            .filter(|x| x.digimon_id == digimon_id)
            .collect();

        let possible_encounters: HashSet<EncounterData> =
            HashSet::from_iter(possible_encounters.iter().map(|x| **x));

        let possible_encounters_arr = Vec::from_iter(possible_encounters.into_iter());

        let count = shuffled_ids.iter().filter(|x| **x == digimon_id).count();

        shuffled_encounters_digimon.insert(
            digimon_id,
            uniform_random_vector(&possible_encounters_arr, count, preset.shuffles, rng),
        );
    }

    for encounter in modified_encounters.iter_mut() {
        if skip(encounter, &preset.encounters) {
            continue;
        };

        let new_encounter_id = shuffled_ids.pop().unwrap();

        *encounter = shuffled_encounters_digimon
            .get_mut(&new_encounter_id)
            .unwrap()
            .pop()
            .unwrap();
    }

    if preset.encounters.strategy == TNTStrategy::Swap {
        let tric = modified_enemy_stats
            .iter()
            .find(|&x| x.digimon_id == dmw3_consts::TRICERAMON_ID)
            .unwrap();

        let mut titem = tric.droppable_item;
        let mut tdrop = tric.drop_rate;

        let tric_index = encounters
            .iter()
            .position(|&x| {
                x.digimon_id as u16 == dmw3_consts::TRICERAMON_ID && x.lv == 6 && x.multiplier == 16
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
            .find(|&&mut x| x.digimon_id == dmw3_consts::TRICERAMON_ID)
            .unwrap();

        std::mem::swap(&mut titem, &mut tricm.droppable_item);
        std::mem::swap(&mut tdrop, &mut tricm.drop_rate);
    }
}
