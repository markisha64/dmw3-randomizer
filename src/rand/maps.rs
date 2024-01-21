use std::collections::BTreeSet;

use crate::consts;
use crate::{rand::Objects, util};
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::{Maps, Randomizer, ShopItems};
use crate::rand::Executable;

use super::structs::Pointer;

fn type_script_add_item(value: u32) -> bool {
    (value >= 0x80) && (value - 0x80) < 0xf
}

fn shoppable(objects: &mut Objects, preset: &Maps) -> Vec<u32> {
    let len = objects.item_shop_data.original.len();

    let mut shoppable: BTreeSet<u32> = BTreeSet::new();

    match preset.item_boxes_items_only {
        ShopItems::Buyable => {
            for i in 1..len {
                if objects.item_shop_data.original[i].buy_price > 0 {
                    shoppable.insert(i as u32);
                }
            }
        }
        ShopItems::Sellable => {
            for i in 1..len {
                if objects.item_shop_data.original[i].sell_price > 0 {
                    shoppable.insert(i as u32);
                }
            }
        }
    }

    Vec::from_iter(shoppable.into_iter())
}

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let maps = &preset.maps;

    if maps.color {
        color(objects, rng);
    }

    if maps.backgrounds {
        backgrounds(preset, objects, rng);
    }

    if maps.item_boxes {
        item_boxes(preset, objects, rng);
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
    let pool = shoppable(objects, &preset.maps);

    for map in &mut objects.map_objects {
        if let Some(entities) = &mut map.entities {
            for entity in &mut entities.modified {
                if !consts::ITEM_BOX_SPRITES.contains(&entity.sprite) || entity.logic.null() {
                    continue;
                }

                let logic = map
                    .entity_logics
                    .iter_mut()
                    .find(|x| {
                        Pointer::from_index_overlay(x.index as u32, objects.stage.value)
                            == entity.logic
                    })
                    .unwrap();

                if logic.original.script.null() {
                    continue;
                }

                let script = map
                    .scripts
                    .iter_mut()
                    .find(|x| {
                        Pointer::from_index_overlay(x.index as u32, objects.stage.value)
                            == logic.original.script
                    })
                    .unwrap();

                for i in 0..script.original.len() {
                    let t = (script.original[i] & 0xfffe) >> 8;

                    if !type_script_add_item(t) {
                        continue;
                    }

                    let nv = pool[(rng.next_u64() % pool.len() as u64) as usize];

                    script.modified[i] = nv | ((script.original[i] >> 8) << 8);

                    if map.talk_file.is_none() {
                        break;
                    }

                    let talk_file_index = map.talk_file.unwrap();

                    for lang in objects.executable.languages() {
                        let sector_offset = match objects.executable {
                            Executable::PAL => {
                                objects.sector_offsets[(talk_file_index + (*lang as u16)) as usize]
                            }
                            _ => objects.sector_offsets[talk_file_index as usize],
                        };

                        let real_file = objects
                            .file_map
                            .iter()
                            .find(|x| x.offs == sector_offset)
                            .unwrap();

                        let item_names = objects
                            .text_files
                            .get(lang.to_file_name(consts::ITEM_NAMES).as_str())
                            .unwrap();

                        let item_name = item_names.file.files[nv as usize].clone();

                        let talk_file = objects.text_files.get_mut(&real_file.name).unwrap();

                        // check if were going over file sector length
                        let csize = talk_file.file.file_size();
                        if csize + 4 + item_name.len()
                            > ((csize / 2048) + (csize % 2048 != 0) as usize) * 2048
                        {
                            break;
                        }

                        talk_file.file.files.push(lang.to_received_item(item_name));
                        logic.modified.text_index = talk_file.file.files.len() as u16 - 1;
                    }

                    break;
                }
            }
        }
    }
}
