use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use super::super::consts;
use super::super::json::{Encounters, Randomizer, TNTStrategy};
use super::structs::EncounterData;
use super::Objects;

fn skip(digimon_id: u16, preset: &Encounters) -> bool {
    return (preset.cardmon
        && (consts::CARDMON_MIN <= digimon_id && digimon_id <= consts::CARDMON_MAX))
        || (preset.bosses && consts::BOSSES.contains(&digimon_id))
        || (preset.strategy == TNTStrategy::Keep && digimon_id == consts::TRICERAMON_ID);
}

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let len = objects.encounters.original.len();
    let modified_encounters = &mut objects.encounters.modified;
    let encounters = &objects.encounters.original;

    // Fisher-Yates shuffles
    for _ in 0..preset.shuffles {
        for i in 0..(len - 2) {
            let uniform: usize = rng.next_u64() as usize;
            let j = i + uniform % (len - i - 1);

            let digimon_id_1 = modified_encounters[i].digimon_id as u16;
            let digimon_id_2 = modified_encounters[j].digimon_id as u16;

            if skip(digimon_id_1 as u16, &preset.encounters)
                || skip(digimon_id_2 as u16, &preset.encounters)
            {
                continue;
            }

            modified_encounters.swap(i, j);
        }
    }

    for i in 0..len {
        let old_encounter = &encounters[i];
        let new_encounter = &mut modified_encounters[i];

        let digimon_id_1 = old_encounter.digimon_id as u16;

        if skip(digimon_id_1 as u16, &preset.encounters) {
            continue;
        }

        // hp and mp
        new_encounter.max_hp = (new_encounter.max_hp as u32 * old_encounter.lv as u32
            / new_encounter.lv as u32) as u16;

        new_encounter.lv = old_encounter.lv;
    }

    let modified_enemy_stats = &mut objects.enemy_stats.modified;

    for enemy_stats in &mut *modified_enemy_stats {
        let encounters: Vec<&EncounterData> = modified_encounters
            .iter()
            .filter(|&x| x.digimon_id == enemy_stats.digimon_id as u32)
            .collect();

        let min_lv = encounters.iter().min_by(|&x, &y| x.lv.cmp(&y.lv)).unwrap();

        let expect_avg_stats = 36 + min_lv.lv * 10;
        let expect_avg_res = 87 + min_lv.lv * 2;

        let avg_stats: i32 = (enemy_stats.str as i32
            + enemy_stats.def as i32
            + enemy_stats.wis as i32
            + enemy_stats.spt as i32
            + enemy_stats.spd as i32)
            / 5
            + 1;

        let avg_res: i32 = (enemy_stats.fir_res as i32
            + enemy_stats.wtr_res as i32
            + enemy_stats.ice_res as i32
            + enemy_stats.wnd_res as i32
            + enemy_stats.thd_res as i32
            + enemy_stats.mch_res as i32
            + enemy_stats.drk_res as i32)
            / 7
            + 1;

        // base stats
        enemy_stats.str = (enemy_stats.str as i32 * expect_avg_stats as i32 / avg_stats) as i16;
        enemy_stats.def = (enemy_stats.def as i32 * expect_avg_stats as i32 / avg_stats) as i16;
        enemy_stats.wis = (enemy_stats.wis as i32 * expect_avg_stats as i32 / avg_stats) as i16;
        enemy_stats.spt = (enemy_stats.spt as i32 * expect_avg_stats as i32 / avg_stats) as i16;
        enemy_stats.spd = (enemy_stats.spd as i32 * expect_avg_stats as i32 / avg_stats) as i16;

        // resistances
        enemy_stats.fir_res = (enemy_stats.fir_res as i32 * expect_avg_res as i32 / avg_res) as i16;
        enemy_stats.wtr_res = (enemy_stats.wtr_res as i32 * expect_avg_res as i32 / avg_res) as i16;
        enemy_stats.wnd_res = (enemy_stats.wnd_res as i32 * expect_avg_res as i32 / avg_res) as i16;
        enemy_stats.thd_res = (enemy_stats.thd_res as i32 * expect_avg_res as i32 / avg_res) as i16;
        enemy_stats.mch_res = (enemy_stats.mch_res as i32 * expect_avg_res as i32 / avg_res) as i16;
        enemy_stats.drk_res = (enemy_stats.drk_res as i32 * expect_avg_res as i32 / avg_res) as i16;
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
            .position(|&x| x.digimon_id as u16 == consts::TRICERAMON_ID && x.lv == 6 && x.x == 16)
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
