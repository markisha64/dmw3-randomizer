use std::collections::{BTreeSet, HashMap, HashSet};

use crate::{
    json::{GroupStrategy, MusicPool},
    rand::{shops::shoppable, Objects},
    util::{self, shuffle, uniform_random_vector},
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

    if maps.mobius_desert {
        random_mobius_desert(objects, preset, rng)?;
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

struct Gates {
    north: Vec<(i16, i16)>,
    east: Vec<(i16, i16)>,
    south: Vec<(i16, i16)>,
    west: Vec<(i16, i16)>,
}

impl Gates {
    fn arr_from_direction(&mut self, direction: u16) -> &mut Vec<(i16, i16)> {
        match direction {
            5 => &mut self.east,
            7 => &mut self.south,
            1 => &mut self.west,
            3 => &mut self.north,
            _ => &mut self.north,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Gate {
    Reserved,
    Empty,
    To((i16, i16)),
}

#[derive(Clone, Copy, Debug)]
enum MobiusType {
    Mobius1,
    Mobius2,
}

#[derive(Clone, Debug)]
struct Node {
    id: (i16, i16),
    mobius_type: MobiusType,
    north: Gate,
    east: Gate,
    south: Gate,
    west: Gate,
}

impl Node {
    fn arr_from_direction(&mut self, direction: u16) -> &mut Gate {
        match direction {
            5 => &mut self.east,
            7 => &mut self.south,
            1 => &mut self.west,
            3 => &mut self.north,
            _ => &mut self.north,
        }
    }

    fn get_empty(&self) -> Vec<u16> {
        let mut rv = Vec::new();

        if self.north == Gate::Empty {
            rv.push(3);
        }

        if self.east == Gate::Empty {
            rv.push(5);
        }

        if self.south == Gate::Empty {
            rv.push(7);
        }

        if self.west == Gate::Empty {
            rv.push(1);
        }

        rv
    }
}

fn recursive(
    mut node: Node,
    mobius_1_gates: &mut Gates,
    mobius_2_gates: &mut Gates,
    mobius_1_nodes: &mut Vec<Node>,
    mobius_2_nodes: &mut Vec<Node>,
    mobius_1_visited: &mut HashSet<(i16, i16)>,
    mobius_2_visited: &mut HashSet<(i16, i16)>,
    rng: &mut Xoshiro256StarStar,
    shuffles: u8,
) -> anyhow::Result<Node> {
    match node.mobius_type {
        MobiusType::Mobius1 => mobius_1_visited.insert(node.id),
        MobiusType::Mobius2 => mobius_2_visited.insert(node.id),
    };

    let mut empty_directions = node.get_empty();
    shuffle(&mut empty_directions, 5, rng);

    for direction in empty_directions {
        let opposite = (direction + 4) % 8;

        let (gate_id, random) = match node.mobius_type {
            MobiusType::Mobius1 => mobius_2_gates
                .arr_from_direction(opposite)
                .pop()
                .map(|x| (x, false))
                .unwrap_or((
                    mobius_2_nodes[(rng.next_u32() as usize) % mobius_2_nodes.len()].id,
                    true,
                )),
            MobiusType::Mobius2 => mobius_1_gates
                .arr_from_direction(opposite)
                .pop()
                .map(|x| (x, false))
                .unwrap_or((
                    mobius_1_nodes[(rng.next_u32() as usize) % mobius_1_nodes.len()].id,
                    true,
                )),
        };

        *node.arr_from_direction(direction) = Gate::To(gate_id);

        if match node.mobius_type {
            MobiusType::Mobius1 => !mobius_2_visited.contains(&gate_id),
            MobiusType::Mobius2 => !mobius_1_visited.contains(&gate_id),
        } {
            if !random {
                let to_update = match node.mobius_type {
                    MobiusType::Mobius1 => mobius_2_nodes
                        .iter_mut()
                        .find(|x| x.id == gate_id)
                        .context("missing gate 1"),
                    MobiusType::Mobius2 => mobius_1_nodes
                        .iter_mut()
                        .find(|x| x.id == gate_id)
                        .context("missing gate 2"),
                }?;

                *to_update.arr_from_direction(opposite) = Gate::To(node.id);
            }

            let to_update = match node.mobius_type {
                MobiusType::Mobius1 => mobius_2_nodes
                    .iter()
                    .find(|x| x.id == gate_id)
                    .context("missing gate 3"),
                MobiusType::Mobius2 => mobius_1_nodes
                    .iter()
                    .find(|x| x.id == gate_id)
                    .context("missing gate 4"),
            }?
            .clone();

            let updated = recursive(
                to_update,
                mobius_1_gates,
                mobius_2_gates,
                mobius_1_nodes,
                mobius_2_nodes,
                mobius_1_visited,
                mobius_2_visited,
                rng,
                shuffles,
            )?;

            let to_update = match node.mobius_type {
                MobiusType::Mobius1 => mobius_2_nodes
                    .iter_mut()
                    .find(|x| x.id == gate_id)
                    .context("missing gate 5"),
                MobiusType::Mobius2 => mobius_1_nodes
                    .iter_mut()
                    .find(|x| x.id == gate_id)
                    .context("missing gate 6"),
            }?;

            *to_update = updated;
        }
    }

    Ok(node)
}

fn random_mobius_desert_helper(
    objects: &mut Objects,
    preset: &Randomizer,
    rng: &mut Xoshiro256StarStar,
    mobius_1_file_name: &str,
    mobius_2_file_name: &str,
    mirage_id: u16,
    s_noise_id: u16,
) -> anyhow::Result<()> {
    let mobius_desert_1 = objects
        .map_objects
        .iter()
        .find(|x| x.file_name.starts_with(mobius_1_file_name))
        .context("failed to find mobius asuka 1")?
        .stage_overrides
        .as_ref()
        .context("no stage overrides")?;
    let mobius_desert_2 = objects
        .map_objects
        .iter()
        .find(|x| x.file_name.starts_with(mobius_2_file_name))
        .context("failed to find mobius asuka 2")?
        .stage_overrides
        .as_ref()
        .context("no stage overrides")?;

    let mut mobius_1_gates = Gates {
        north: Vec::new(),
        east: Vec::new(),
        south: Vec::new(),
        west: Vec::new(),
    };
    let mut mobius_1_nodes = Vec::new();

    for i in 0..mobius_desert_1.stage_overrides.len() {
        let stage_override = &mobius_desert_1.stage_overrides[i];
        let var1 = stage_override.original.var1;
        let var2 = stage_override.original.var2;

        if var1 == 0 && var2 == 0 {
            continue;
        }

        let mut node = Node {
            north: Gate::Empty,
            east: Gate::Empty,
            south: Gate::Empty,
            west: Gate::Empty,
            id: (var1, var2),
            mobius_type: MobiusType::Mobius1,
        };

        for env_override in &mobius_desert_1.environmental_overrides[i] {
            if env_override.original.var1 == 0 && env_override.original.var2 == 0 {
                continue;
            }

            let direction = env_override.original.next_stage_direction;

            if env_override.original.next_stage_id == mirage_id
                || env_override.original.next_stage_id == s_noise_id
            {
                *node.arr_from_direction(direction) = Gate::Reserved;
            }

            mobius_1_gates
                .arr_from_direction(direction)
                .push((env_override.original.var1, env_override.original.var2));
        }

        mobius_1_nodes.push(node);
    }

    let mut mobius_2_gates = Gates {
        north: Vec::new(),
        east: Vec::new(),
        south: Vec::new(),
        west: Vec::new(),
    };
    let mut mobius_2_nodes = Vec::new();

    for i in 0..mobius_desert_2.stage_overrides.len() {
        let stage_override = &mobius_desert_2.stage_overrides[i];
        let var1 = stage_override.original.var1;
        let var2 = stage_override.original.var2;

        if var1 == 0 && var2 == 0 {
            continue;
        }

        let mut node = Node {
            north: Gate::Empty,
            east: Gate::Empty,
            south: Gate::Empty,
            west: Gate::Empty,
            id: (var1, var2),
            mobius_type: MobiusType::Mobius2,
        };

        for env_override in &mobius_desert_2.environmental_overrides[i] {
            if env_override.original.var1 == 0 && env_override.original.var2 == 0 {
                continue;
            }

            let direction = env_override.original.next_stage_direction;

            if env_override.original.next_stage_id == mirage_id
                || env_override.original.next_stage_id == s_noise_id
            {
                *node.arr_from_direction(direction) = Gate::Reserved;
            }

            mobius_2_gates
                .arr_from_direction(direction)
                .push((env_override.original.var1, env_override.original.var2));
        }

        mobius_2_nodes.push(node);
    }

    let mut mobius_1_visited = HashSet::new();
    let mut mobius_2_visited = HashSet::new();

    shuffle(&mut mobius_1_gates.north, preset.shuffles, rng);
    shuffle(&mut mobius_1_gates.east, preset.shuffles, rng);
    shuffle(&mut mobius_1_gates.south, preset.shuffles, rng);
    shuffle(&mut mobius_1_gates.west, preset.shuffles, rng);

    shuffle(&mut mobius_2_gates.north, preset.shuffles, rng);
    shuffle(&mut mobius_2_gates.east, preset.shuffles, rng);
    shuffle(&mut mobius_2_gates.south, preset.shuffles, rng);
    shuffle(&mut mobius_2_gates.west, preset.shuffles, rng);

    let mobius_enrance = mobius_2_nodes
        .iter()
        .find(|x| x.id == (1, 1))
        .context("missing mobius entrance")?
        .clone();

    let updated_entrance = recursive(
        mobius_enrance,
        &mut mobius_1_gates,
        &mut mobius_2_gates,
        &mut mobius_1_nodes,
        &mut mobius_2_nodes,
        &mut mobius_1_visited,
        &mut mobius_2_visited,
        rng,
        preset.shuffles,
    )?;

    let to_update = mobius_1_nodes
        .iter_mut()
        .find(|x| x.id == (1, 1))
        .context("missing mobius entrance")?;

    *to_update = updated_entrance;

    let mobius_desert_1 = objects
        .map_objects
        .iter_mut()
        .find(|x| x.file_name.starts_with(mobius_1_file_name))
        .context("failed to find mobius asuka 1")?
        .stage_overrides
        .as_mut()
        .context("no stage overrides")?;

    for i in 0..mobius_desert_1.stage_overrides.len() {
        let stage_override = &mobius_desert_1.stage_overrides[i];
        let var1 = stage_override.original.var1;
        let var2 = stage_override.original.var2;

        if var1 == 0 && var2 == 0 {
            continue;
        }

        let new_node = mobius_1_nodes
            .iter_mut()
            .find(|x| x.id == (var1, var2))
            .context("missing node")?;

        for env_override in &mut mobius_desert_1.environmental_overrides[i] {
            if let Gate::To((var1, var2)) =
                new_node.arr_from_direction(env_override.original.next_stage_direction)
            {
                env_override.modified.var1 = *var1;
                env_override.modified.var2 = *var2;
            }
        }
    }

    let mobius_desert_2 = objects
        .map_objects
        .iter_mut()
        .find(|x| x.file_name.starts_with(mobius_2_file_name))
        .context("failed to find mobius asuka 1")?
        .stage_overrides
        .as_mut()
        .context("no stage overrides")?;

    for i in 0..mobius_desert_2.stage_overrides.len() {
        let stage_override = &mobius_desert_2.stage_overrides[i];
        let var1 = stage_override.original.var1;
        let var2 = stage_override.original.var2;

        if var1 == 0 && var2 == 0 {
            continue;
        }

        let new_node = mobius_1_nodes
            .iter_mut()
            .find(|x| x.id == (var1, var2))
            .context("missing node")?;

        for env_override in &mut mobius_desert_2.environmental_overrides[i] {
            if let Gate::To((var1, var2)) =
                new_node.arr_from_direction(env_override.original.next_stage_direction)
            {
                env_override.modified.var1 = *var1;
                env_override.modified.var2 = *var2;
            }
        }
    }

    Ok(())
}

fn random_mobius_desert(
    objects: &mut Objects,
    preset: &Randomizer,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    random_mobius_desert_helper(objects, preset, rng, "WSTAG635", "WSTAG640", 599, 602)?;
    random_mobius_desert_helper(objects, preset, rng, "WSTAG636", "WSTAG641", 707, 704)?;

    Ok(())
}
