use std::collections::BTreeSet;

use crate::{rand::Objects, util};
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::{Maps, Randomizer, ShopItems};

use super::dmw3_structs::Pointer;

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

    Vec::from_iter(shoppable)
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

    let possible_arr = Vec::from_iter(possible_indices);
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
                if !dmw3_consts::ITEM_BOX_SPRITES.contains(&entity.sprite) || entity.logic.null() {
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

                    script.modified[i] = nv | ((script.original[i] >> 9) << 9);

                    if map.talk_file.is_none() {
                        break;
                    }

                    let talk_file_index = map.talk_file.unwrap();

                    let real_file = objects
                        .file_map
                        .iter()
                        .find(|x| x.offs == objects.sector_offsets[talk_file_index as usize])
                        .unwrap();

                    let sname = &real_file.name[1..];

                    let group = objects.text_files.get_mut(sname).unwrap();

                    for (_lang, text_file) in &mut group.files {
                        text_file.file.files[talk_file_index as usize] = vec![0, 0, 0, 0];
                    }

                    if let Some(idx) = group.mapped_items.get(&nv) {
                        logic.modified.text_index = *idx;

                        break;
                    }

                    let doesnt_fit = group
                        .files
                        .iter()
                        .find(|(lang, text_file)| {
                            let item_name = objects.items.files.get(lang).unwrap().file.files
                                [nv as usize]
                                .clone();

                            let csize = text_file.file.file_size();

                            let received_item_text = lang.to_received_item(item_name);

                            csize + 4 + received_item_text.len()
                                > ((csize / 2048) + (csize % 2048 != 0) as usize) * 2048
                        })
                        .is_some();

                    if doesnt_fit {
                        if let Some(idx) = group.generic_item {
                            logic.modified.text_index = idx;
                        }

                        break;
                    }

                    let mut idx = 0;
                    for (lang, talk_file) in &mut group.files {
                        let item_name =
                            objects.items.files.get(lang).unwrap().file.files[nv as usize].clone();

                        let received_item_text = lang.to_received_item(item_name);
                        idx = talk_file.file.files.len() as u16;

                        logic.modified.text_index = idx;

                        talk_file.file.files.push(received_item_text);
                    }

                    group.mapped_items.insert(nv, idx);

                    break;
                }
            }
        }
    }
}
