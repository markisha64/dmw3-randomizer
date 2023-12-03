use crate::rand::Objects;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::Maps;

pub fn patch(preset: &Maps, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    if preset.color {
        color(objects, rng);
    }
}

fn color(objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    for map in &mut objects.map_objects {
        if let Some(color_object) = &mut map.map_color {
            color_object.modified.red = (rng.next_u64() % 256) as u8;
            color_object.modified.green = (rng.next_u64() % 256) as u8;
            color_object.modified.blue = (rng.next_u64() % 256) as u8;
        }
    }
}
