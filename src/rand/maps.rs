use binread::BinRead;
use binwrite::BinWrite;
use std::collections::BTreeSet;
use std::io::Cursor;

use crate::consts;
use crate::{rand::Objects, util};
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::Randomizer;
use crate::rand::structs::EntityLogic;

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
            .map(|x| x.background_file_index.original),
    );

    let maps_with_bgs = objects.map_objects.len();

    let possible_arr = Vec::from_iter(possible_indices.into_iter());
    let mut shuffled_bgs =
        util::uniform_random_vector(&possible_arr, maps_with_bgs, preset.shuffles, rng);

    for map in &mut objects.map_objects {
        map.background_file_index.modified = shuffled_bgs.pop().unwrap();
    }
}

fn item_boxes(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    for map in &mut objects.map_objects {
        if let Some(entities) = &mut map.entities {
            for entity in &mut entities.modified {
                if (!consts::ITEM_BOX_SPRITES.contains(&entity.sprite) || entity.logic.null()) {
                    continue;
                }

                let logic_idx = entity.logic.to_index_overlay(objects.stage.value) as usize;

                let mut logic_reader = Cursor::new(&map.buf[logic_idx..]);

                if let Ok(logic) = EntityLogic::read(&mut logic_reader) {}
            }
        }
    }
}
