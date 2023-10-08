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
    fn update(&self, ddata: &mut DigivolutionData, amount: u16) {
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

        (*ptr) += amount;
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
        let mut all_digimon: [u8; consts::ROOKIE_COUNT] = [0, 1, 2, 3, 4, 5, 6, 7];
        let rindex = (rng.next_u64() % (consts::ROOKIE_COUNT - 2) as u64) as usize;
        for i in 0..3 {
            // Fisher-Yates shuffles
            for _ in 0..preset.shuffles {
                for j in 0..(consts::ROOKIE_COUNT - 2) {
                    let uniform = rng.next_u64() as usize;
                    let k = j + uniform % (consts::ROOKIE_COUNT - 1 - j);

                    all_digimon.swap(j, k);
                }
            }

            for j in 0..3 {
                parties[i * 3 + j] = all_digimon[rindex + j];
            }
        }
    }

    if preset.parties.stat_distribution {
        let mut stats: Vec<Stat> = vec![Stat::Str, Stat::Def, Stat::Spt, Stat::Wis, Stat::Spd];

        let min_sum = preset.parties.min_starting_stat * 5;
        for rookie_data in &mut objects.rookie_data.modified {
            let mut leftover = preset.parties.total_starting_stats - min_sum;

            for _ in 0..preset.shuffles {
                for i in 0..3 {
                    let uniform = rng.next_u64() as usize;
                    let j = i + uniform % (4 - i);

                    stats.swap(i, j);
                }
            }

            for stat in &stats {
                stat.set(rookie_data, preset.parties.min_starting_stat);
            }

            for stat in &stats {
                let new_add = ((rng.next_u64()) % ((leftover + 1) as u64)) as u16;
                leftover -= new_add;

                stat.update(rookie_data, new_add);
            }

            if leftover > 0 {
                stats.last().unwrap().update(rookie_data, leftover);
            }
        }
    }

    if preset.parties.res_distribution {
        let mut resistances: Vec<Stat> = vec![
            Stat::FirRes,
            Stat::WtrRes,
            Stat::IceRes,
            Stat::WndRes,
            Stat::ThdRes,
            Stat::MchRes,
            Stat::DrkRes,
        ];

        let min_sum = preset.parties.min_starting_res * 7;
        for rookie_data in &mut objects.rookie_data.modified {
            let mut leftover = preset.parties.total_starting_res - min_sum;

            for _ in 0..preset.shuffles {
                for i in 0..5 {
                    let uniform = rng.next_u64() as usize;
                    let j = i + uniform % (6 - i);

                    resistances.swap(i, j);
                }
            }

            for res in &resistances {
                res.set(rookie_data, preset.parties.min_starting_res);
            }

            for res in &resistances {
                let new_add = ((rng.next_u64()) % ((leftover + 1) as u64)) as u16;
                leftover -= new_add;

                res.update(rookie_data, new_add);
            }

            if leftover > 0 {
                resistances.last().unwrap().update(rookie_data, leftover);
            }
        }
    }

    if preset.parties.stat_affinities {
        for rookie_data in &mut objects.rookie_data.modified {
            for stat in &mut rookie_data.stat_offsets {
                (*stat) = 1 + (rng.next_u64() % 5) as u8;
            }
        }
    }

    if preset.parties.res_affinities {
        for rookie_data in &mut objects.rookie_data.modified {
            for res in &mut rookie_data.res_offsets {
                (*res) = 1 + (rng.next_u64() % 5) as u8;
            }
        }
    }

    if preset.parties.learned_tech {
        learned_moves(objects, rng, preset);
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
    }
}

fn learned_moves(objects: &mut Objects, rng: &mut Xoshiro256StarStar, preset: &Randomizer) {
    let mut learnable: BTreeSet<u16> = BTreeSet::new();
    let mut move_count: usize = 0;

    for digivolution in &objects.digivolution_data.original {
        for tech in digivolution.tech.iter() {
            let deref = *tech;

            if deref != 0 {
                learnable.insert(deref);
                move_count += 1;
            }
        }
    }

    let learnable_arr = Vec::from_iter(learnable.clone().into_iter());
    let mut shuffled_arr =
        util::uniform_random_vector(&learnable_arr, move_count, preset.shuffles, rng);

    for digivolution in &mut objects.digivolution_data.modified {
        let shuffled = BTreeSet::from_iter(shuffled_arr.clone().into_iter());

        let mut move_pool = Vec::from_iter(shuffled.into_iter());

        for tech in &mut digivolution.tech {
            if *tech == 0 {
                continue;
            }

            let mv = (rng.next_u64() % move_pool.len() as u64) as usize;

            *tech = move_pool[mv];
            shuffled_arr.remove(
                shuffled_arr
                    .iter()
                    .position(|x| *x == move_pool[mv])
                    .unwrap(),
            );
            move_pool.remove(mv);
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

    for _ in 0..preset.shuffles {
        for i in 0..(consts::ROOKIE_COUNT - 2) {
            let uniform = rng.next_u64() as usize;
            let j = i + uniform % (consts::ROOKIE_COUNT - 1 - i);

            learnable_rookie_arr.swap(i, j);
        }

        for i in 0..(consts::DIGIVOLUTION_COUNT - 2) {
            let uniform = rng.next_u64() as usize;
            let j = i + uniform % (consts::DIGIVOLUTION_COUNT - 1 - i);

            learnable_arr.swap(i, j);
        }
    }

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
