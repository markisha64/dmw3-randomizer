use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::Randomizer;
use crate::rand::Objects;

use super::structs::DigivolutionData;

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
    fn update(&self, dscaling: &mut DigivolutionData, amount: u16) {
        let ptr = match self {
            Stat::Str => &mut dscaling.str,
            Stat::Def => &mut dscaling.def,
            Stat::Spt => &mut dscaling.spt,
            Stat::Wis => &mut dscaling.wis,
            Stat::Spd => &mut dscaling.spd,
            // Stat::Chr => &mut dscaling.startChr,
            Stat::FirRes => &mut dscaling.fir_res,
            Stat::WtrRes => &mut dscaling.wtr_res,
            Stat::IceRes => &mut dscaling.ice_res,
            Stat::WndRes => &mut dscaling.wnd_res,
            Stat::ThdRes => &mut dscaling.thd_res,
            Stat::MchRes => &mut dscaling.mch_res,
            Stat::DrkRes => &mut dscaling.drk_res,
        };

        (*ptr) += amount;
    }

    fn set(&self, dscaling: &mut DigivolutionData, amount: u16) {
        let ptr = match self {
            Stat::Str => &mut dscaling.str,
            Stat::Def => &mut dscaling.def,
            Stat::Spt => &mut dscaling.spt,
            Stat::Wis => &mut dscaling.wis,
            Stat::Spd => &mut dscaling.spd,
            // Stat::Chr => &mut dscaling.startChr,
            Stat::FirRes => &mut dscaling.fir_res,
            Stat::WtrRes => &mut dscaling.wtr_res,
            Stat::IceRes => &mut dscaling.ice_res,
            Stat::WndRes => &mut dscaling.wnd_res,
            Stat::ThdRes => &mut dscaling.thd_res,
            Stat::MchRes => &mut dscaling.mch_res,
            Stat::DrkRes => &mut dscaling.drk_res,
        };

        (*ptr) = amount;
    }
}

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    if preset.parties.random_parties {
        let parties = &mut objects.parties.modified;
        let mut all_digimon: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        let rindex = (rng.next_u64() % 6) as usize;
        for i in 0..3 {
            // Fisher-Yates shuffles
            for _ in 0..preset.shuffles {
                for j in 0..6 {
                    let uniform = rng.next_u64() as usize;
                    let k = j + uniform % (7 - j);

                    all_digimon.swap(j, k);
                }
            }

            for j in 0..3 {
                parties[i * 3 + j] = all_digimon[rindex + j];
            }
        }
    }

    if preset.parties.random_stat_distribution {
        let mut stats: Vec<Stat> = vec![Stat::Str, Stat::Def, Stat::Spt, Stat::Wis, Stat::Spd];

        let min_sum = preset.parties.min_starting_stat * 5;
        for scaling in &mut objects.scaling.modified {
            let mut leftover = preset.parties.total_starting_stats - min_sum;

            for _ in 0..preset.shuffles {
                for i in 0..3 {
                    let uniform = rng.next_u64() as usize;
                    let j = i + uniform % (4 - i);

                    stats.swap(i, j);
                }
            }

            for stat in &stats {
                stat.set(scaling, preset.parties.min_starting_stat);
            }

            for stat in &stats {
                let new_add = ((rng.next_u64()) % ((leftover + 1) as u64)) as u16;
                leftover -= new_add;

                stat.update(scaling, new_add);
            }

            if leftover > 0 {
                stats.last().unwrap().update(scaling, leftover);
            }
        }
    }

    if preset.parties.random_res_distribution {
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
        for scaling in &mut objects.scaling.modified {
            let mut leftover = preset.parties.total_starting_res - min_sum;

            for _ in 0..preset.shuffles {
                for i in 0..5 {
                    let uniform = rng.next_u64() as usize;
                    let j = i + uniform % (6 - i);

                    resistances.swap(i, j);
                }
            }

            for res in &resistances {
                res.set(scaling, preset.parties.min_starting_res);
            }

            for res in &resistances {
                let new_add = ((rng.next_u64()) % ((leftover + 1) as u64)) as u16;
                leftover -= new_add;

                res.update(scaling, new_add);
            }

            if leftover > 0 {
                resistances.last().unwrap().update(scaling, leftover);
            }
        }
    }

    if preset.parties.stat_affinities {
        for scaling in &mut objects.scaling.modified {
            for stat in &mut scaling.stat_offsets {
                (*stat) = 1 + (rng.next_u64() % 5) as u8;
            }
        }
    }

    if preset.parties.res_affinities {
        for scaling in &mut objects.scaling.modified {
            for res in &mut scaling.res_offsets {
                (*res) = 1 + (rng.next_u64() % 5) as u8;
            }
        }
    }
}
