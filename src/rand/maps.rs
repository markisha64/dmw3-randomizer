use std::collections::{BTreeSet, HashMap};

use crate::{
    json::{GroupStrategy, MusicPool},
    rand::{shops::shoppable, Objects},
    util::{self, uniform_random_vector},
};
use anyhow::Context;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::Randomizer;

pub fn type_script_add_item(value: u16) -> bool {
    (value >= 0x80) && (value - 0x80) < 0xf
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

    if maps.music {
        music(objects, preset, rng)?;
    }

    if maps.battle_music {
        battle_music(objects, preset, rng)?;
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
    let pool: Vec<_> = shoppable(objects, &preset.maps.item_boxes_items_only)
        .into_iter()
        .collect();
    let language = objects
        .executable
        .languages()
        .first()
        .context("executable with no languages")?;

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

                            let nv = pool[(rng.next_u64() % pool.len() as u64) as usize];

                            script.bitfield = nv | ((script.bitfield >> 9) << 9);

                            let real_file = objects
                                .file_map
                                .iter()
                                .find(|x| {
                                    x.offs
                                        == Some(
                                            objects.sector_offsets.original[map.talk_file as usize],
                                        )
                                })
                                .context("failed to find real file")?;

                            let sname = &real_file.name[1..];

                            let group = objects
                                .text_files
                                .get_mut(sname)
                                .context("failed to get mut")?;

                            // alrady exists (rare)
                            if let Some(idx) = group.mapped_items.get(&nv) {
                                logic.text_index = (*idx) as u32;

                                break;
                            }

                            if group.overwritten.contains(&logic.text_index) {
                                // index already overwritten
                                let idx = group
                                    .files
                                    .get(language)
                                    .context("missing lang")?
                                    .file
                                    .files
                                    .len();

                                for (lang, talk_file) in &mut group.files {
                                    let item_name = objects
                                        .items
                                        .files
                                        .get(lang)
                                        .context("failed to get by lang")?
                                        .file
                                        .files[nv as usize]
                                        .clone();

                                    talk_file.file.files.push(lang.to_received_item(item_name));
                                }

                                logic.text_index = idx as u32;

                                group.mapped_items.insert(nv, idx as u16);
                            } else {
                                // index is safe for overwrite
                                for (lang, talk_file) in &mut group.files {
                                    let item_name = objects
                                        .items
                                        .files
                                        .get(lang)
                                        .context("failed to get by lang")?
                                        .file
                                        .files[nv as usize]
                                        .clone();

                                    talk_file.file.files[logic.text_index as usize] =
                                        lang.to_received_item(item_name);
                                }

                                group.overwritten.insert(logic.text_index);
                                group.mapped_items.insert(nv, logic.text_index as u16);
                            }

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
    objects.charisma_reqs.modified = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
}

pub fn music_pool(objects: &mut Objects, music_pool: MusicPool) -> Vec<(u16, u16)> {
    let mut pool = BTreeSet::new();

    for map_object in &mut objects.map_objects {
        if music_pool != MusicPool::Battle {
            for music_set in &mut map_object.music.original {
                pool.insert((music_set.sep_track, music_set.sep_file));
            }
        }

        if music_pool != MusicPool::Overworld {
            for se_obj in &mut map_object.stage_encounters {
                for opt in &mut se_obj.stage_encounters {
                    if let Some(encounters_obj) = opt {
                        for encounter in &mut encounters_obj.original {
                            if encounter.team_id != 0 {
                                let sep_file = (encounter.music >> 16) as u16;
                                let sep_track = (encounter.music >> 18) as u16 & 0x7f;

                                pool.insert((sep_track, sep_file));
                            }
                        }
                    }
                }
            }
        }
    }

    Vec::from_iter(pool.into_iter())
}

fn music(
    objects: &mut Objects,
    preset: &Randomizer,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    let mut pool = music_pool(objects, preset.maps.music_pool);

    let pool_len = objects
        .map_objects
        .iter()
        .fold(0, |pv, cv| pv + cv.music.original.len());

    let mut randomized = uniform_random_vector(&mut pool, pool_len, preset.shuffles, rng);

    for map_object in &mut objects.map_objects {
        for music_set in &mut map_object.music.modified {
            let (sep_track, sep_file) = randomized.pop().context("missing seps")?;

            music_set.sep_track = sep_track;
            music_set.sep_file = sep_file;
        }
    }

    Ok(())
}

fn battle_music_ungrouped(
    preset: &Randomizer,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    let pool = music_pool(objects, preset.maps.battle_music_pool);

    let pool_len = objects.map_objects.iter().fold(0, |pv, cv| {
        pv + cv.stage_encounters.iter().fold(0, |pv_se, cv| {
            pv_se
                + cv.stage_encounters.iter().fold(0, |pv_e, cv| {
                    if cv.is_some() {
                        return pv_e + 1;
                    }

                    pv_e
                })
        })
    });

    let mut randomized = uniform_random_vector(&pool, pool_len, preset.shuffles, rng);

    for map in &mut objects.map_objects {
        for se_obj in &mut map.stage_encounters {
            for opt in &mut se_obj.stage_encounters {
                if let Some(encounters_obj) = opt {
                    for encounter in &mut encounters_obj.modified {
                        let (_, sep_file) = randomized.pop().context("missing music")?;

                        encounter.music = (sep_file as u32) << 16;
                    }
                }
            }
        }
    }

    Ok(())
}

fn battle_music_grouped(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let mut generated = HashMap::new();

    let pool = music_pool(objects, preset.maps.battle_music_pool);
    let pool_len = pool.len() as u32;

    for map in &mut objects.map_objects {
        for se_obj in &mut map.stage_encounters {
            for opt in &mut se_obj.stage_encounters {
                if let Some(encounters_obj) = opt {
                    for encounter in &mut encounters_obj.modified {
                        encounter.music = match generated.get(&encounter.team_id) {
                            Some(x) => *x,
                            None => {
                                let nv = rng.next_u32() % pool_len;
                                let (_, sep_file) = pool[nv as usize];

                                let music = (sep_file as u32) << 16;

                                generated.insert(encounter.team_id, music);

                                music
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

fn battle_music(
    objects: &mut Objects,
    preset: &Randomizer,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    if preset.maps.battle_music_group_strategy == GroupStrategy::None {
        battle_music_ungrouped(preset, objects, rng)?;
    } else {
        battle_music_grouped(preset, objects, rng);
    }

    Ok(())
}
