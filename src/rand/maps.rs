use std::collections::{BTreeSet, HashMap};

use crate::{json::GroupStrategy, rand::Objects, util};
use anyhow::Context;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::{Maps, Randomizer, ShopItems};

use super::shops::item_in_ironmon;

pub fn type_script_add_item(value: u16) -> bool {
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
        ShopItems::Ironmon => {
            for i in 1..len {
                if objects.item_shop_data.original[i].sell_price > 0 && item_in_ironmon(i) {
                    shoppable.insert(i as u32);
                }
            }
        }
    }

    Vec::from_iter(shoppable)
}

pub fn patch(
    preset: &Randomizer,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    let maps = &preset.maps;

    if maps.color {
        color(objects, rng);
    }

    if maps.backgrounds {
        backgrounds(preset, objects, rng)?;
    }

    if maps.item_boxes {
        item_boxes(preset, objects, rng)?;
    }

    if maps.fight_backgrounds {
        random_fight_backgrounds(preset, objects, rng);
    }

    if maps.ironmon_charisma {
        ironmon_charisma(objects);
    }

    Ok(())
}

fn random_fight_backgrounds_ungrouped(objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    for map in &mut objects.map_objects {
        for se_obj in &mut map.stage_encounters {
            for opt in &mut se_obj.stage_encounters {
                if let Some(encounters_obj) = opt {
                    for encounter in &mut encounters_obj.modified {
                        encounter.stage = rng.next_u32() % 0x39;
                    }
                }
            }
        }
    }
}

fn random_fight_backgrounds_grouped(
    preset: &Randomizer,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) {
    let mut generated = HashMap::new();

    for map in &mut objects.map_objects {
        for se_obj in &mut map.stage_encounters {
            for opt in &mut se_obj.stage_encounters {
                if let Some(encounters_obj) = opt {
                    for encounter in &mut encounters_obj.modified {
                        encounter.stage = match generated.get(&encounter.team_id) {
                            Some(x) => *x,
                            None => {
                                let nv = rng.next_u32() % 0x39;

                                generated.insert(encounter.team_id, nv);

                                nv
                            }
                        };
                    }
                }
            }
        }

        if preset.maps.group_strategy == GroupStrategy::Map {
            generated.clear();
        }
    }
}

fn random_fight_backgrounds(
    preset: &Randomizer,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) {
    if preset.maps.group_strategy == GroupStrategy::None {
        random_fight_backgrounds_ungrouped(objects, rng);
    } else {
        random_fight_backgrounds_grouped(preset, objects, rng);
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

fn backgrounds(
    preset: &Randomizer,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
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
        map.background_file_index.modified = shuffled_bgs.pop().context("no bgs left")?;
    }

    Ok(())
}

fn item_boxes(
    preset: &Randomizer,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    let pool = shoppable(objects, &preset.maps);

    for map in &mut objects.map_objects {
        if let Some(entities) = &mut map.entities {
            // println!("name {}", map.file_name);
            let logic_min = entities
                .entities
                .modified
                .iter()
                .find(|x| !x.logic.null())
                .map(|x| x.logic);

            let scripts = entities
                .entity_logics
                .modified
                .iter()
                .filter(|x| !x.script.null())
                .map(|x| x.script);

            let conditions = entities
                .entity_logics
                .modified
                .iter()
                .filter(|x| !x.conditions.null())
                .map(|x| x.conditions);

            let mut script_cond = Vec::from_iter(scripts);
            script_cond.extend(conditions);

            let script_cond_min = script_cond.iter().min_by(|a, b| a.value.cmp(&b.value));

            let minn = logic_min.zip(script_cond_min);

            if let Some((min_logic, min_script_cond)) = minn {
                for entity in &mut entities.entities.modified {
                    if !dmw3_consts::ITEM_BOX_SPRITES.contains(&entity.sprite)
                        || entity.logic.null()
                    {
                        continue;
                    }

                    let logic_idx = ((entity.logic.value - min_logic.value) / 0xc) as usize;

                    for logic in &mut entities.entity_logics.modified[logic_idx..] {
                        if logic.text_index == 0 {
                            break;
                        }

                        if logic.script.null() {
                            continue;
                        }

                        let script_idx =
                            ((logic.script.value - min_script_cond.value) / 0x4) as usize;

                        for script in &mut entities.scripts_conditions.modified[script_idx..] {
                            if script.is_last_step() {
                                break;
                            }

                            let t = (script.bitfield & 0xfffe) >> 8;

                            if !type_script_add_item(t) {
                                continue;
                            }

                            let nv = pool[(rng.next_u64() % pool.len() as u64) as usize] as u16;

                            script.bitfield = nv | ((script.bitfield >> 9) << 9);

                            let real_file = objects
                                .file_map
                                .iter()
                                .find(|x| x.offs == objects.sector_offsets[map.talk_file as usize])
                                .context("failed to find real file")?;

                            let sname = &real_file.name[1..];

                            let group = objects
                                .text_files
                                .get_mut(sname)
                                .context("failed to get mut")?;

                            for (_lang, text_file) in &mut group.files {
                                text_file.file.files[map.talk_file as usize] = vec![0, 0, 0, 0];
                            }

                            if let Some(idx) = group.mapped_items.get(&nv) {
                                logic.text_index = (*idx) as u32;

                                break;
                            }

                            let doesnt_fit = group
                                .files
                                .iter()
                                .find(|(lang, text_file)| {
                                    objects
                                        .items
                                        .files
                                        .get(lang)
                                        .map(|l| {
                                            let item_name = l.file.files[nv as usize].clone();

                                            let csize = text_file.file.file_size_text();

                                            let received_item_text =
                                                lang.to_received_item(item_name);

                                            csize + 4 + received_item_text.len()
                                                > ((csize / 2048) + (csize % 2048 != 0) as usize)
                                                    * 2048
                                        })
                                        .unwrap_or(false)
                                })
                                .is_some();

                            if doesnt_fit {
                                if let Some(idx) = group.generic_item {
                                    logic.text_index = idx as u32;
                                }

                                break;
                            }

                            let mut idx = 0;
                            for (lang, talk_file) in &mut group.files {
                                let item_name = objects
                                    .items
                                    .files
                                    .get(lang)
                                    .context("failed to get by lang")?
                                    .file
                                    .files[nv as usize]
                                    .clone();

                                let received_item_text = lang.to_received_item(item_name);
                                idx = talk_file.file.files.len();

                                logic.text_index = idx as u32;

                                talk_file.file.files.push(received_item_text);
                            }

                            group.mapped_items.insert(nv, idx as u16);

                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn ironmon_charisma(objects: &mut Objects) {
    // objects.charisma_reqs.modified = vec![
    // 1, 150, 210, 285, 378, 492, 630, 795, 990, 1218, 1482, 1785, 2049, 2277, 2472,
    // ];
    objects.charisma_reqs.modified = vec![
        1, 1, 1, 40, 80, 120, 160, 200, 240, 280, 320, 360, 400, 440, 480,
    ];
}
