use crate::json::Scaling;

use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::consts;
use crate::rand::{structs::EncounterData, Objects};

pub fn patch(preset: &Scaling, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let len = objects.encounters.original.len();
    let modified_encounters = &mut objects.encounters.modified;
    let modified_enemy_stats = &mut objects.enemy_stats.modified;
    let encounters = &objects.encounters.original;

    for i in 0..len {
        let old_encounter = &encounters[i];
        let new_encounter = &mut modified_encounters[i];

        new_encounter.max_hp = old_encounter.max_hp;
        new_encounter.lv = old_encounter.lv;
    }

    for encounter in modified_encounters.iter_mut() {
        encounter.max_hp = ((encounter.max_hp as f64) * preset.hp_modifier) as u16;
    }

    for enemy_stats in &mut *modified_enemy_stats {
        let min_lv: &mut EncounterData = modified_encounters
            .iter_mut()
            .filter(|&&mut x| x.digimon_id == enemy_stats.digimon_id as u32)
            .min_by(|&&mut x, &&mut y| x.lv.cmp(&y.lv))
            .unwrap();

        let modifier = match preset.scaling_offset {
            0 => 0,
            offset => {
                let rand = rng.next_u64();
                let modulo = (offset as u64) * 2 + 1;

                (-1 * offset as i32) + (rand % modulo) as i32
            }
        };

        let expected_stats = preset.base_stats + min_lv.lv as i32 * preset.stat_modifier + modifier;
        let expect_res = preset.base_res + min_lv.lv as i32 * preset.res_modifier + modifier;

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
            objects.move_data.modified[enemy_stats.attack as usize].power = 40 + min_lv.lv * 10;
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
            encounter.multiplier =
                (((preset.base_stats + preset.stat_modifier * encounter.lv as i32) * 16)
                    / (preset.base_stats + preset.stat_modifier * min_lv.lv as i32))
                    as u16;
        }
    }

    // this is a fix for possible galacticmon phase 3
    // this equalizes phase 1 and phase 3 multipliers
    // without changing numbers (reciprocals)
    // 125% * 80% = 100%

    let phase_1 = objects
        .encounters
        .original
        .iter()
        .position(|x| x.digimon_id as u16 == consts::GALACTICMON_1ST_PHASE)
        .unwrap();

    let phase_3 = phase_1 + 2;

    let target_multiplier = modified_encounters[phase_3].multiplier;
    let current_multiplier = modified_encounters[phase_1].multiplier;

    let phase_1_digimon_id = modified_encounters[phase_1].digimon_id;
    let phase_3_digimon_id = modified_encounters[phase_3].digimon_id;

    if target_multiplier != current_multiplier && phase_1_digimon_id != phase_3_digimon_id {
        let stats = modified_enemy_stats
            .iter_mut()
            .find(|&&mut x| x.digimon_id == phase_1_digimon_id as u16)
            .unwrap();

        stats.str = ((stats.str as u16 * current_multiplier) / target_multiplier) as i16;
        stats.def = ((stats.def as u16 * current_multiplier) / target_multiplier) as i16;
        stats.spt = ((stats.spt as u16 * current_multiplier) / target_multiplier) as i16;
        stats.wis = ((stats.wis as u16 * current_multiplier) / target_multiplier) as i16;
        stats.spd = ((stats.spd as u16 * current_multiplier) / target_multiplier) as i16;

        for encounter in modified_encounters.iter_mut() {
            if encounter.digimon_id == phase_1_digimon_id {
                encounter.multiplier =
                    (encounter.multiplier * target_multiplier) / current_multiplier;
            }
        }

        let attack = &mut objects.move_data.modified[stats.attack as usize];

        attack.power = (attack.power * current_multiplier) / target_multiplier;
    }
}