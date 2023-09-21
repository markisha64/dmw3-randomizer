use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::consts;
use crate::json::{Encounters, Randomizer, TNTStrategy};
use crate::rand::{structs::EncounterData, Objects};

fn skip(encounter: &EncounterData, preset: &Encounters) -> bool {
    return (preset.cardmon
        && (consts::CARDMON_MIN <= encounter.digimon_id as u16
            && encounter.digimon_id as u16 <= consts::CARDMON_MAX))
        || (preset.bosses && consts::BOSSES.contains(&(encounter.digimon_id as u16)))
        || (preset.strategy == TNTStrategy::Keep
            && encounter.digimon_id as u16 == consts::TRICERAMON_ID
            && encounter.multiplier == 16)
        || (preset.keep_zanbamon
            && encounter.digimon_id as u16 == consts::ZANBAMON_ID
            && encounter.multiplier == 16);
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

            if skip(&modified_encounters[i], &preset.encounters)
                || skip(&modified_encounters[j], &preset.encounters)
            {
                continue;
            }

            modified_encounters.swap(i, j);
        }
    }

    for i in 0..len {
        let old_encounter = &encounters[i];
        let new_encounter = &mut modified_encounters[i];

        if skip(old_encounter, &preset.encounters) {
            continue;
        }

        if preset.encounters.scaling {
            // hp and mp
            new_encounter.max_hp = ((new_encounter.max_hp - 100) as u32 * old_encounter.lv as u32
                / new_encounter.lv as u32
                + 100) as u16;

            new_encounter.lv = old_encounter.lv;
        }
    }

    let modified_enemy_stats = &mut objects.enemy_stats.modified;

    if preset.encounters.scaling {
        for enemy_stats in &mut *modified_enemy_stats {
            let min_lv: &mut EncounterData = modified_encounters
                .iter_mut()
                .filter(|&&mut x| x.digimon_id == enemy_stats.digimon_id as u32)
                .min_by(|&&mut x, &&mut y| x.lv.cmp(&y.lv))
                .unwrap();

            let modifier = match preset.encounters.scaling_offset {
                0 => 0,
                offset => {
                    let rand = rng.next_u64();
                    let modulo = (offset as u64) * 2 + 1;

                    (-1 * offset as i32) + (rand % modulo) as i32
                }
            };

            let expected_stats = preset.encounters.base_stats
                + min_lv.lv as i32 * preset.encounters.stat_modifier
                + modifier;
            let expect_res = preset.encounters.base_res
                + min_lv.lv as i32 * preset.encounters.res_modifier
                + modifier;

            let current_stats: i32 = (enemy_stats.str
                + enemy_stats.def
                + enemy_stats.wis
                + enemy_stats.spt
                + enemy_stats.spd) as i32;

            let current_res: i32 = (enemy_stats.fir_res
                + enemy_stats.wtr_res
                + enemy_stats.ice_res
                + enemy_stats.wnd_res
                + enemy_stats.thd_res
                + enemy_stats.mch_res
                + enemy_stats.drk_res) as i32;

            // base stats
            enemy_stats.str = (enemy_stats.str as i32 * expected_stats / current_stats) as i16;
            enemy_stats.def = (enemy_stats.def as i32 * expected_stats / current_stats) as i16;
            enemy_stats.wis = (enemy_stats.wis as i32 * expected_stats / current_stats) as i16;
            enemy_stats.spt = (enemy_stats.spt as i32 * expected_stats / current_stats) as i16;
            enemy_stats.spd = (enemy_stats.spd as i32 * expected_stats / current_stats) as i16;

            // resistances
            enemy_stats.fir_res = (enemy_stats.fir_res as i32 * expect_res / current_res) as i16;
            enemy_stats.wtr_res = (enemy_stats.wtr_res as i32 * expect_res / current_res) as i16;
            enemy_stats.ice_res = (enemy_stats.ice_res as i32 * expect_res / current_res) as i16;
            enemy_stats.wnd_res = (enemy_stats.wnd_res as i32 * expect_res / current_res) as i16;
            enemy_stats.thd_res = (enemy_stats.thd_res as i32 * expect_res / current_res) as i16;
            enemy_stats.mch_res = (enemy_stats.mch_res as i32 * expect_res / current_res) as i16;
            enemy_stats.drk_res = (enemy_stats.drk_res as i32 * expect_res / current_res) as i16;

            if enemy_stats.attack > 0 {
                objects.move_data.modified[enemy_stats.attack as usize - 1].power =
                    40 + min_lv.lv * 10;
            }

            // modify multipliers
            min_lv.multiplier = 16;

            let min_lv: EncounterData = modified_encounters
                .iter()
                .filter(|&x| x.digimon_id == enemy_stats.digimon_id as u32)
                .min_by(|&x, &y| x.lv.cmp(&y.lv))
                .unwrap()
                .clone();

            let encounters: Vec<&mut EncounterData> = modified_encounters
                .iter_mut()
                .filter(|&&mut x| {
                    x.digimon_id == enemy_stats.digimon_id as u32
                        && !(x.lv == min_lv.lv && x.multiplier == 16)
                })
                .collect();

            for encounter in encounters {
                encounter.multiplier = (((preset.encounters.base_stats
                    + preset.encounters.stat_modifier * encounter.lv as i32)
                    * 16)
                    / (preset.encounters.base_stats
                        + preset.encounters.stat_modifier * min_lv.lv as i32))
                    as u16;
            }
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
