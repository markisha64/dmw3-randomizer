use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::Randomizer;
use crate::rand::Objects;

use super::structs::Scaling;

#[derive(Clone, Copy)]
enum Stat {
    Str,
    Def,
    Spt,
    Wis,
    Spd,
    Chr,
    FirRes,
    WtrRes,
    IceRes,
    WndRes,
    ThdRes,
    MchRes,
    DrkRes,
}

impl Stat {
    fn update(&self, dscaling: &mut Scaling, amount: u16) {
        let ptr = match self {
            Stat::Str => &mut dscaling.startStr,
            Stat::Def => &mut dscaling.startDef,
            Stat::Spt => &mut dscaling.startSpt,
            Stat::Wis => &mut dscaling.startWis,
            Stat::Spd => &mut dscaling.startSpd,
            Stat::Chr => &mut dscaling.startChr,
            Stat::FirRes => &mut dscaling.startFirRes,
            Stat::WtrRes => &mut dscaling.startWtrRes,
            Stat::IceRes => &mut dscaling.startIceRes,
            Stat::WndRes => &mut dscaling.startWndRes,
            Stat::ThdRes => &mut dscaling.startThdRes,
            Stat::MchRes => &mut dscaling.startMchRes,
            Stat::DrkRes => &mut dscaling.startDrkRes,
        };

        (*ptr) += amount;
    }

    fn set(&self, dscaling: &mut Scaling, amount: u16) {
        let ptr = match self {
            Stat::Str => &mut dscaling.startStr,
            Stat::Def => &mut dscaling.startDef,
            Stat::Spt => &mut dscaling.startSpt,
            Stat::Wis => &mut dscaling.startWis,
            Stat::Spd => &mut dscaling.startSpd,
            Stat::Chr => &mut dscaling.startChr,
            Stat::FirRes => &mut dscaling.startFirRes,
            Stat::WtrRes => &mut dscaling.startWtrRes,
            Stat::IceRes => &mut dscaling.startIceRes,
            Stat::WndRes => &mut dscaling.startWndRes,
            Stat::ThdRes => &mut dscaling.startThdRes,
            Stat::MchRes => &mut dscaling.startMchRes,
            Stat::DrkRes => &mut dscaling.startDrkRes,
        };

        (*ptr) = amount;
    }
}

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    if preset.party.random_parties {
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

    let mut stats: Vec<Stat> = vec![
        Stat::Str,
        Stat::Def,
        Stat::Spt,
        Stat::Wis,
        Stat::Spd,
        Stat::Chr,
    ];
}
