use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use super::super::json::Randomizer;
use super::Objects;

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let parties = &mut objects.parties.modified;
    let mut all_digimon: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let rindex = (rng.next_u64() % 7) as usize;
    for i in 0..3 {
        // Fisher-Yates shuffles
        for _ in 0..preset.shuffles {
            for j in 0..7 {
                let uniform = rng.next_u64() as usize;
                let k = j + uniform % (8 - j);

                all_digimon.swap(j, k);
            }
        }

        for j in 0..3 {
            parties[i * 3 + j] = all_digimon[rindex + j];
        }
    }
}
