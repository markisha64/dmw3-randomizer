use std::collections::HashMap;

use anyhow::Context;
use async_std::{fs::File, io::WriteExt};
use serde::Serialize;

use crate::rand::{maps::type_script_add_item, Objects};

#[derive(Debug, Serialize)]
struct Requirements {
    version: String,
}

impl Default for Requirements {
    fn default() -> Self {
        Self {
            version: "0.6.3".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct ProgressionBalancing {
    random: i32,
    #[serde(rename = "random-low")]
    random_low: i32,
    #[serde(rename = "random-high")]
    random_high: i32,
    disabled: i32,
    normal: i32,
    extreme: i32,
}

impl Default for ProgressionBalancing {
    fn default() -> Self {
        Self {
            random: 0,
            random_low: 0,
            random_high: 0,
            disabled: 0,
            normal: 50,
            extreme: 0,
        }
    }
}

#[derive(Debug, Serialize)]
struct Accessibility {
    full: i32,
    minimal: i32,
}

impl Default for Accessibility {
    fn default() -> Self {
        Self {
            full: 0,
            minimal: 0,
        }
    }
}

#[derive(Debug, Serialize)]
struct DigimonWorld2003Options {
    #[serde(rename = "progression_balancing")]
    progression_balancing: ProgressionBalancing,
    accessibility: Accessibility,
    plando_items: Vec<String>,
    shops: Vec<Vec<u16>>,
    item_boxes: HashMap<String, Vec<u16>>,

    local_items: Vec<String>,
    non_local_items: Vec<String>,
    start_inventory: HashMap<String, i32>,
    start_hints: Vec<String>,
    start_location_hints: Vec<String>,
    exclude_locations: Vec<String>,
    priority_locations: Vec<String>,
    item_links: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Options {
    name: String,
    description: String,
    game: String,
    #[serde(rename = "Digimon World 2003")]
    options: DigimonWorld2003Options,
}

fn generate_archipelago_yaml(objects: &Objects) -> anyhow::Result<Options> {
    let ptr = objects.shops.modified.first().context("empty shops")?.items;

    let shops = objects
        .shops
        .modified
        .iter()
        .map(|shop| {
            // divide by 2 because u16 (2 bytes)
            let i = ((shop.items.value - ptr.value) / 2) as usize;

            Vec::from(&objects.shop_items.modified[i..i + shop.item_count as usize])
        })
        .collect::<Vec<_>>();

    let mut item_boxes = HashMap::new();

    for map in &objects.map_objects {
        let mut boxes = Vec::new();

        if let Some(entities) = &map.entities {
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
                for entity in &entities.entities.modified {
                    if !dmw3_consts::ITEM_BOX_SPRITES.contains(&entity.sprite)
                        || entity.logic.null()
                    {
                        continue;
                    }

                    let logic_idx = ((entity.logic.value - min_logic.value) / 0xc) as usize;

                    for logic in &entities.entity_logics.modified[logic_idx..] {
                        if logic.text_index == 0 {
                            break;
                        }

                        if logic.script.null() {
                            continue;
                        }

                        let script_idx =
                            ((logic.script.value - min_script_cond.value) / 0x4) as usize;

                        for script in &entities.scripts_conditions.modified[script_idx..] {
                            if script.is_last_step() {
                                break;
                            }

                            let t = (script.bitfield & 0xfffe) >> 8;

                            if !type_script_add_item(t) {
                                continue;
                            }

                            boxes.push(script.bitfield & 0x1ff);
                        }
                    }
                }
            }
        }

        if !boxes.is_empty() {
            item_boxes.insert(map.file_name.clone(), boxes);
        }
    }

    Ok(Options {
        name: "Player".to_string(),
        description: "Archipelago YAML created by Digimon World 3 Randomizer".to_string(),
        game: "Digimon World 2003".to_string(),
        options: DigimonWorld2003Options {
            progression_balancing: ProgressionBalancing::default(),
            accessibility: Accessibility::default(),
            plando_items: Vec::new(),
            shops,
            item_boxes,
            local_items: Vec::new(),
            non_local_items: Vec::new(),
            start_inventory: HashMap::new(),
            start_hints: Vec::new(),
            start_location_hints: Vec::new(),
            exclude_locations: Vec::new(),
            priority_locations: Vec::new(),
            item_links: Vec::new(),
        },
    })
}

pub async fn create_archipelago_yaml(
    objects: &Objects,
    path: &std::path::PathBuf,
    file_name: &str,
) -> anyhow::Result<()> {
    let rom_name = path
        .file_name()
        .context("Failed file name get")?
        .to_str()
        .context("Failed to_str conversion")?;

    let mut yaml_file =
        File::create(format!("randomized/{}/{}/game.yaml", rom_name, file_name)).await?;

    let options = generate_archipelago_yaml(objects)?;

    let yaml_data = serde_yaml::to_string(&options)?;

    yaml_file.write(&yaml_data.into_bytes()[..]).await?;

    Ok(())
}
