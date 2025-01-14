use crate::json::Scaling;

use anyhow::Context;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::rand::{dmw3_structs::EncounterData, Objects};

pub fn patch(
    preset: &Scaling,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
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
        let min_lv_index: usize = modified_encounters
            .iter()
            .enumerate()
            .filter(|(_, x)| x.digimon_id == enemy_stats.digimon_id as u32)
            .min_by(|(_, x), (_, y)| x.lv.cmp(&y.lv))
            .context("empty encounters")?
            .0;

        let min_lv: &mut EncounterData = &mut modified_encounters[min_lv_index];

        let modifier = match preset.scaling_offset {
            0 => 0,
            offset => {
                let rand = rng.next_u64();
                let modulo = (offset as u64) * 2 + 1;

                -(offset as i32) + (rand % modulo) as i32
            }
        };

        let target_stats = preset.base_stats + min_lv.lv as i32 * preset.stat_modifier + modifier;
        let target_res = preset.base_res + min_lv.lv as i32 * preset.res_modifier + modifier;

        let mut target_stats_normalized = target_stats;

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

        let mut base_multiplier: u16 = 16;
        if enemy_stats.attack > 0 {
            let move_data = &mut objects.move_data.modified[enemy_stats.attack as usize - 1];

            let mut target_power = 40 + min_lv.lv * 10;

            if move_data.hit_effect == dmw3_consts::MULTI_HIT && move_data.freq > 1 {
                target_power /= move_data.freq as u16;
            }

            let current_power = move_data.power;

            if preset.natural_scaling {
                // equivalent to ceil() division without converting to floats
                let before_division = 16 * target_power;
                base_multiplier = match before_division % current_power {
                    0 => before_division / current_power,
                    _ => (before_division / current_power) + 1,
                };

                // base multiplier is rounded up so need
                // to adjust power accordingly
                // target power = move power * multiplier
                // => move power = target power / multiplier
                move_data.power = (target_power * 16) / base_multiplier;

                // need to do same thing on stats
                target_stats_normalized = (target_stats_normalized * 16) / (base_multiplier as i32);
            } else {
                move_data.power = target_power;
            }
        }

        // base stats
        enemy_stats.str = (enemy_stats.str as i32 * target_stats_normalized / current_stats) as i16;
        enemy_stats.def = (enemy_stats.def as i32 * target_stats_normalized / current_stats) as i16;
        enemy_stats.wis = (enemy_stats.wis as i32 * target_stats_normalized / current_stats) as i16;
        enemy_stats.spt = (enemy_stats.spt as i32 * target_stats_normalized / current_stats) as i16;
        enemy_stats.spd = (enemy_stats.spd as i32 * target_stats_normalized / current_stats) as i16;

        // resistances
        enemy_stats.fir_res = (enemy_stats.fir_res as i32 * target_res / current_res) as i16;
        enemy_stats.wtr_res = (enemy_stats.wtr_res as i32 * target_res / current_res) as i16;
        enemy_stats.ice_res = (enemy_stats.ice_res as i32 * target_res / current_res) as i16;
        enemy_stats.wnd_res = (enemy_stats.wnd_res as i32 * target_res / current_res) as i16;
        enemy_stats.thd_res = (enemy_stats.thd_res as i32 * target_res / current_res) as i16;
        enemy_stats.mch_res = (enemy_stats.mch_res as i32 * target_res / current_res) as i16;
        enemy_stats.drk_res = (enemy_stats.drk_res as i32 * target_res / current_res) as i16;

        // modify multipliers
        min_lv.multiplier = base_multiplier;

        let min_lv: EncounterData = modified_encounters[min_lv_index];

        let encounters: Vec<&mut EncounterData> = modified_encounters
            .iter_mut()
            .enumerate()
            .filter(|(index, x)| {
                x.digimon_id == enemy_stats.digimon_id as u32 && *index != min_lv_index
            })
            .map(|(_, x)| x)
            .collect();

        for encounter in encounters {
            encounter.multiplier = (((preset.base_stats
                + preset.stat_modifier * encounter.lv as i32)
                * base_multiplier as i32)
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
        .position(|x| x.digimon_id as u16 == dmw3_consts::GALACTICMON_1ST_PHASE)
        .context("failed to find galacticmon phase 1 encounter")?;

    let phase_3 = phase_1 + 2;

    let target_multiplier = modified_encounters[phase_3].multiplier;
    let current_multiplier = modified_encounters[phase_1].multiplier;

    let phase_1_digimon_id = modified_encounters[phase_1].digimon_id;
    let phase_3_digimon_id = modified_encounters[phase_3].digimon_id;

    if target_multiplier != current_multiplier && phase_1_digimon_id != phase_3_digimon_id {
        let stats = modified_enemy_stats
            .iter_mut()
            .find(|&&mut x| x.digimon_id == phase_1_digimon_id as u16)
            .context("failed to find phase 1 stats")?;

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

    Ok(())
}
