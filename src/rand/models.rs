use rand_xoshiro::Xoshiro256StarStar;
use rlen::rlen_decode;
use tim::Tim;

use crate::{json::Randomizer, pack::Packed, rand::Objects};

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    for _ in 0..preset.shuffles {
        for model in &mut objects.model_objects {
            let texture_packed =
                Packed::from(model.packed.files[model.header.texture_offset as usize].clone());

            let texture_raw = match rlen_decode(&texture_packed.files[0].clone()[..]) {
                Ok(file) => file,
                Err(_) => texture_packed.files[0].clone(),
            };

            let mut texture_tim = Tim::from(texture_raw);

            for i in 0..16 {
                for j in 0..4 {}
            }
        }
    }
}
