use std::collections::BTreeSet;

use crate::{rand::Objects, util};
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::Randomizer;

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let maps = &preset.maps;

    if maps.color {
        color(objects, rng);
    }

    if maps.backgrounds {
        backgrounds(preset, objects, rng);
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

fn backgrounds(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let possible_indices: BTreeSet<u16> = BTreeSet::from_iter(
        objects
            .map_objects
            .iter()
            .filter(|x| x.background_file_index.is_some())
            .map(|x| (x.background_file_index).as_ref().unwrap().original),
    );

    let maps_with_bgs = objects
        .map_objects
        .iter()
        .filter(|x| x.background_file_index.is_some())
        .count();

    let possible_arr = Vec::from_iter(possible_indices.into_iter());
    let mut shuffled_bgs =
        util::uniform_random_vector(&possible_arr, maps_with_bgs, preset.shuffles, rng);

    for map in &mut objects.map_objects {
        if let Some(bg_file_index_object) = &mut map.background_file_index {
            bg_file_index_object.modified = shuffled_bgs.pop().unwrap();
        }
    }
}
