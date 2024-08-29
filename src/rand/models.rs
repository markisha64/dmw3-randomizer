use rand_xoshiro::Xoshiro256StarStar;

use crate::{json::Randomizer, rand::Objects};

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    for _ in 0..preset.shuffles {
        for model in &mut objects.model_objects {}
    }
}
