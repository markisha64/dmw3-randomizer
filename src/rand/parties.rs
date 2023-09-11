use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::Randomizer;
use crate::rand::Objects;

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
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
