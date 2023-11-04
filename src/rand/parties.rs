use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::consts;
use crate::json::Randomizer;
use crate::rand::Objects;
use std::collections::BTreeSet;

use super::structs::DigivolutionData;
use crate::util;

#[derive(Clone, Copy)]
enum Stat {
    Str,
    Def,
    Spt,
    Wis,
    Spd,
    // Chr,
    FirRes,
    WtrRes,
    IceRes,
    WndRes,
    ThdRes,
    MchRes,
    DrkRes,
}

impl Stat {
    fn set_affinity(&self, ddata: &mut DigivolutionData, amount: u8) {
        let ptr = match self {
            Stat::Str => &mut ddata.stat_offsets[0],
            Stat::Def => &mut ddata.stat_offsets[1],
            Stat::Spt => &mut ddata.stat_offsets[2],
            Stat::Wis => &mut ddata.stat_offsets[3],
            Stat::Spd => &mut ddata.stat_offsets[4],
            // Stat::Chr => &mut ddata.stat_offsets[5]?,
            Stat::FirRes => &mut ddata.res_offsets[0],
            Stat::WtrRes => &mut ddata.res_offsets[1],
            Stat::IceRes => &mut ddata.res_offsets[2],
            Stat::WndRes => &mut ddata.res_offsets[3],
            Stat::ThdRes => &mut ddata.res_offsets[4],
            Stat::MchRes => &mut ddata.res_offsets[5],
            Stat::DrkRes => &mut ddata.res_offsets[6],
        };

        (*ptr) = amount;
    }

    fn set(&self, ddata: &mut DigivolutionData, amount: u16) {
        let ptr = match self {
            Stat::Str => &mut ddata.str,
            Stat::Def => &mut ddata.def,
            Stat::Spt => &mut ddata.spt,
            Stat::Wis => &mut ddata.wis,
            Stat::Spd => &mut ddata.spd,
            // Stat::Chr => &mut ddata.startChr,
            Stat::FirRes => &mut ddata.fir_res,
            Stat::WtrRes => &mut ddata.wtr_res,
            Stat::IceRes => &mut ddata.ice_res,
            Stat::WndRes => &mut ddata.wnd_res,
            Stat::ThdRes => &mut ddata.thd_res,
            Stat::MchRes => &mut ddata.mch_res,
            Stat::DrkRes => &mut ddata.drk_res,
        };

        (*ptr) = amount;
    }
}

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    if preset.parties.parties {
        let parties = &mut objects.parties.modified;
        let mut all_digimon: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let rindex = (rng.next_u64() % (consts::ROOKIE_COUNT - 2) as u64) as usize;
        for i in 0..3 {
            util::shuffle(&mut all_digimon, preset.shuffles, rng);

            for j in 0..3 {
                parties[i * 3 + j] = all_digimon[rindex + j];
            }
        }
    }

    if preset.parties.stat_distribution {
        let stats: Vec<Stat> = vec![Stat::Str, Stat::Def, Stat::Spt, Stat::Wis, Stat::Spd];

        let min_sum = preset.parties.min_starting_stat * 5;
        let before_addition = (preset.parties.total_starting_stats - min_sum) as u64;

        let affinity_range = consts::MAX_STAT_AFFINITY - consts::MIN_STAT_AFFINITY;
        let total_affinity = (affinity_range * stats.len() as u8) as u64;

        for rookie_data in &mut objects.rookie_data.modified {
            let before_normalization: Vec<u64> =
                stats.iter().map(|_| rng.next_u32() as u64).collect();

            let sum: u64 = before_normalization.iter().fold(0, |a, b| a + *b);

            for i in 0..stats.len() {
                stats[i].set(
                    rookie_data,
                    ((before_normalization[i] * before_addition) / sum) as u16
                        + preset.parties.min_starting_stat,
                );

                stats[i].set_affinity(
                    rookie_data,
                    ((before_normalization[i] * total_affinity) / (sum * 2)) as u8
                        + consts::MIN_STAT_AFFINITY,
                );
            }
        }
    }

    if preset.parties.res_distribution {
        let resistances: Vec<Stat> = vec![
            Stat::FirRes,
            Stat::WtrRes,
            Stat::IceRes,
            Stat::WndRes,
            Stat::ThdRes,
            Stat::MchRes,
            Stat::DrkRes,
        ];

        let min_sum = preset.parties.min_starting_res * 5;
        let before_addition = (preset.parties.total_starting_res - min_sum) as u64;

        let affinity_range = consts::MAX_STAT_AFFINITY - consts::MIN_STAT_AFFINITY;
        let total_affinity = (affinity_range * resistances.len() as u8) as u64;

        for rookie_data in &mut objects.rookie_data.modified {
            let before_normalization: Vec<u64> =
                resistances.iter().map(|_| rng.next_u32() as u64).collect();

            let sum: u64 = before_normalization.iter().fold(0, |a, b| a + *b);

            for i in 0..resistances.len() {
                resistances[i].set(
                    rookie_data,
                    ((before_normalization[i] * before_addition) / sum) as u16
                        + preset.parties.min_starting_res,
                );

                resistances[i].set_affinity(
                    rookie_data,
                    ((before_normalization[i] * total_affinity) / (sum * 2)) as u8
                        + consts::MIN_STAT_AFFINITY,
                );
            }
        }
    }

    if preset.parties.learned_tech {
        learned_moves(objects, rng);
    }

    if preset.parties.signatures {
        signatues(objects, rng, preset);
    }

    if preset.parties.digivolutions {
        if preset.parties.keep_stages {
            dv_cond_limited(preset, objects, rng);
        } else {
            dv_cond_unlimited(preset, objects, rng);
        }

        blasts(objects);
    }

    if preset.parties.exp_modifier {
        let min = preset.parties.min_exp_modifier;
        let range = (preset.parties.max_exp_modifier - min + 1) as u64;

        for rookie in &mut objects.rookie_data.modified {
            rookie.exp_modifier = min + (rng.next_u64() % range) as u8;
        }
    }

    if preset.parties.starting_hp_mp {
        match preset.parties.balance_hp_mp {
            true => hp_mp_balanced(objects, rng, preset),
            false => hp_mp_unbalanced(objects, rng, preset),
        }
    }
}

fn hp_mp_unbalanced(objects: &mut Objects, rng: &mut Xoshiro256StarStar, preset: &Randomizer) {
    let min_hp = preset.parties.min_starting_hp;
    let min_mp = preset.parties.min_starting_mp;
    let hp_range = (preset.parties.max_starting_hp - min_hp + 1) as u64;
    let mp_range = (preset.parties.max_starting_mp - min_mp + 1) as u64;

    let min_hp_modifier = preset.parties.min_hp_modifier;
    let min_mp_modifier = preset.parties.min_mp_modifier;
    let hp_modifier_range = (preset.parties.max_hp_modifier - min_hp_modifier + 1) as u64;
    let mp_modifier_range = (preset.parties.max_mp_modifier - min_mp_modifier + 1) as u64;

    for rookie in &mut objects.rookie_data.modified {
        let hp_distribution = rng.next_u64() % (hp_range + 1);
        let mp_distribution = rng.next_u64() % (mp_range + 1);

        rookie.starting_hp = min_hp + hp_distribution as u8;
        rookie.starting_mp = min_mp + mp_distribution as u8;

        rookie.hp_modifier =
            min_hp_modifier + ((hp_distribution * hp_modifier_range as u64) / hp_range) as u8;
        rookie.mp_modifier =
            min_mp_modifier + ((mp_distribution * mp_modifier_range as u64) / mp_range) as u8;
    }
}

fn hp_mp_balanced(objects: &mut Objects, rng: &mut Xoshiro256StarStar, preset: &Randomizer) {
    let min_hp = preset.parties.min_starting_hp;
    let min_mp = preset.parties.min_starting_mp;
    let hp_range = (preset.parties.max_starting_hp - min_hp + 1) as u64;
    let mp_range = (preset.parties.max_starting_mp - min_mp + 1) as u64;

    let min_hp_modifier = preset.parties.min_hp_modifier;
    let min_mp_modifier = preset.parties.min_mp_modifier;
    let hp_modifier_range = (preset.parties.max_hp_modifier - min_hp_modifier + 1) as u64;
    let mp_modifier_range = (preset.parties.max_mp_modifier - min_mp_modifier + 1) as u64;

    for rookie in &mut objects.rookie_data.modified {
        let hp_distribution = rng.next_u32() as u64;
        let mp_distribution = rng.next_u32() as u64;

        let total = hp_distribution + mp_distribution;

        rookie.starting_hp = min_hp + ((hp_distribution * hp_range as u64) / total) as u8;
        rookie.starting_mp = min_mp + ((mp_distribution * mp_range as u64) / total) as u8;

        rookie.hp_modifier =
            min_hp_modifier + ((hp_distribution * hp_modifier_range as u64) / total) as u8;
        rookie.mp_modifier =
            min_mp_modifier + ((mp_distribution * mp_modifier_range as u64) / total) as u8;
    }
}

fn learned_moves(objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let mut learnable: BTreeSet<u16> = BTreeSet::new();

    for digivolution in &objects.digivolution_data.original {
        for tech in digivolution.tech.iter() {
            let deref = *tech;

            if deref != 0 {
                learnable.insert(deref);
            }
        }
    }

    for digivolution in &mut objects.digivolution_data.modified {
        let mut learnable_arr = Vec::from_iter(learnable.clone().into_iter());

        for tech in &mut digivolution.tech {
            if *tech == 0 {
                continue;
            }

            let mv = (rng.next_u64() % learnable_arr.len() as u64) as usize;

            *tech = learnable_arr[mv];
            learnable_arr.remove(mv);
        }
    }
}

fn signatues(objects: &mut Objects, rng: &mut Xoshiro256StarStar, preset: &Randomizer) {
    let mut learnable_rookie: BTreeSet<u16> = BTreeSet::new();
    let mut learnable: BTreeSet<u16> = BTreeSet::new();

    for rookie in &objects.rookie_data.original {
        learnable_rookie.insert(rookie.ori_tech);
    }

    for digivolution in &objects.digivolution_data.original {
        learnable.insert(digivolution.ori_tech);
    }

    let mut learnable_rookie_arr = Vec::from_iter(learnable_rookie.into_iter());
    let mut learnable_arr = Vec::from_iter(learnable.into_iter());

    util::shuffle(&mut learnable_rookie_arr, preset.shuffles, rng);
    util::shuffle(&mut learnable_arr, preset.shuffles, rng);

    for i in 0..consts::ROOKIE_COUNT {
        objects.rookie_data.modified[i].ori_tech = learnable_rookie_arr[i];
    }

    for i in 0..consts::DIGIVOLUTION_COUNT {
        objects.digivolution_data.modified[i].ori_tech = learnable_arr[i];
    }
}

fn dv_cond_unlimited(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    for dindex in 0..consts::ROOKIE_COUNT {
        let conds = &mut objects.dv_cond.modified[dindex];

        // swap ids
        for _ in 0..preset.shuffles {
            for i in 0..(consts::DIGIVOLUTION_COUNT - 2) {
                let uniform: usize = rng.next_u64() as usize;
                let j = i + uniform % (consts::DIGIVOLUTION_COUNT - 1 - i);

                let ind = conds.conditions[j].index;

                conds.conditions[j].index = conds.conditions[i].index;
                conds.conditions[i].index = ind;
            }
        }

        // we can clone because we're not touching index anymore
        let cloned = conds.conditions.clone();

        // swap dv reqs
        for cond in &mut conds.conditions {
            if cond.dv_index_1 > 0 {
                let cond_index = objects.dv_cond.original[dindex]
                    .conditions
                    .iter()
                    .position(|x| x.index == cond.dv_index_1 as u32)
                    .unwrap();

                cond.dv_index_1 = (cloned[cond_index].index) as u16;
            }

            if cond.dv_index_2 > 0 {
                let cond_index = objects.dv_cond.original[dindex]
                    .conditions
                    .iter()
                    .position(|x| x.index == cond.dv_index_2 as u32)
                    .unwrap();

                cond.dv_index_2 = (cloned[cond_index].index) as u16;
            }
        }
    }
}

fn dv_cond_limited(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    for dindex in 0..consts::ROOKIE_COUNT {
        let conds = &mut objects.dv_cond.modified[dindex];

        // swap ids
        let mut dv_limited = |ids: Vec<u16>| {
            let length = ids.len();

            let indices: Vec<usize> = ids
                .iter()
                .map(|x| -> usize {
                    objects.dv_cond.original[dindex]
                        .conditions
                        .iter()
                        .position(|y| {
                            objects.digivolution_data.original[y.index as usize - 9].digimon_id
                                == *x
                        })
                        .unwrap()
                })
                .collect();

            for _ in 0..preset.shuffles {
                for i in 0..length - 2 {
                    let uniform: usize = rng.next_u64() as usize;
                    let j = i + uniform % (length - 1 - i);

                    let ind = conds.conditions[indices[j]].index;

                    conds.conditions[indices[j]].index = conds.conditions[indices[i]].index;
                    conds.conditions[indices[i]].index = ind;
                }
            }
        };

        dv_limited(Vec::from(consts::CHAMPIONS));
        dv_limited(Vec::from(consts::ULTIMATES));
        dv_limited(Vec::from(consts::MEGAS));
        dv_limited(Vec::from(consts::MEGAPLUS));
        dv_limited(Vec::from(consts::ULTRAS));

        // we can clone because we're not touching index anymore
        let cloned = conds.conditions.clone();

        // swap dv reqs
        for cond in &mut conds.conditions {
            if cond.dv_index_1 > 0 {
                let cond_index = objects.dv_cond.original[dindex]
                    .conditions
                    .iter()
                    .position(|x| x.index == cond.dv_index_1 as u32)
                    .unwrap();

                cond.dv_index_1 = (cloned[cond_index].index) as u16;
            }

            if cond.dv_index_2 > 0 {
                let cond_index = objects.dv_cond.original[dindex]
                    .conditions
                    .iter()
                    .position(|x| x.index == cond.dv_index_2 as u32)
                    .unwrap();

                cond.dv_index_2 = (cloned[cond_index].index) as u16;
            }
        }
    }
}

fn blasts(objects: &mut Objects) {
    for r in 0..consts::ROOKIE_COUNT {
        let rookie = &mut objects.rookie_data.modified[r];
        for i in 0..5 {
            if rookie.blast_indices[i] == 0 {
                continue;
            }

            let index = objects.dv_cond.original[r]
                .conditions
                .iter()
                .position(|x| x.index == rookie.blast_indices[i] as u32)
                .unwrap();

            rookie.blast_indices[i] = objects.dv_cond.modified[r].conditions[index].index as u8;
        }
    }
}
