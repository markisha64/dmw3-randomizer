use std::{fs, io::Write};

use anyhow::Context;
use async_std::fs::{create_dir_all, File};
use async_std::prelude::*;
use binwrite::BinWrite;
use dmw3_structs::{Pointer, StageEncounter, StageEncounterArea};
use tar::{Builder, Header};

use crate::rand::{read_objects, Objects};

static DEFAULT_AREA: StageEncounterArea = StageEncounterArea {
    steps_inddex: 0,
    teams: [
        Pointer { value: 0 },
        Pointer { value: 0 },
        Pointer { value: 0 },
        Pointer { value: 0 },
        Pointer { value: 0 },
        Pointer { value: 0 },
        Pointer { value: 0 },
        Pointer { value: 0 },
    ],
};

static DEFAULT_ENCOUNTERS: [StageEncounter; 8] = [
    StageEncounter {
        team_id: 0,
        stage: 0,
        music: 0,
    },
    StageEncounter {
        team_id: 0,
        stage: 0,
        music: 0,
    },
    StageEncounter {
        team_id: 0,
        stage: 0,
        music: 0,
    },
    StageEncounter {
        team_id: 0,
        stage: 0,
        music: 0,
    },
    StageEncounter {
        team_id: 0,
        stage: 0,
        music: 0,
    },
    StageEncounter {
        team_id: 0,
        stage: 0,
        music: 0,
    },
    StageEncounter {
        team_id: 0,
        stage: 0,
        music: 0,
    },
    StageEncounter {
        team_id: 0,
        stage: 0,
        music: 0,
    },
];

pub async fn dump(path: &std::path::PathBuf) -> anyhow::Result<()> {
    let objects = read_objects(path).await?;

    let rom_name = path
        .file_name()
        .context("Failed to get file name")?
        .to_str()
        .context("Failed to convert to str")?;

    fs::create_dir_all(format!("dump/{rom_name}"))?;

    let mut enemy_stats_bytes = Vec::new();
    let mut encounter_bytes = Vec::new();
    let mut enemy_party_bytes = Vec::new();
    let mut party_exp_bits_bytes = Vec::new();
    let mut digivolution_bytes = Vec::new();
    let mut rookie_bytes = Vec::new();
    let mut item_shop_bytes = Vec::new();
    let mut digivolution_condition_bytes = Vec::new();
    let mut move_data_bytes = Vec::new();

    let mut shop_bytes = Vec::new();
    let mut shop_item_bytes = Vec::new();

    let mut screen_name_mapping_bytes = Vec::new();

    let _ = &objects.enemy_stats.original.write(&mut enemy_stats_bytes)?;

    let _ = &objects.encounters.original.write(&mut encounter_bytes)?;

    let _ = &objects
        .enemy_parties
        .original
        .write(&mut enemy_party_bytes)?;

    let _ = &objects
        .digivolution_data
        .original
        .write(&mut digivolution_bytes)?;

    let _ = &objects.rookie_data.original.write(&mut rookie_bytes)?;

    let _ = &objects
        .item_shop_data
        .original
        .write(&mut item_shop_bytes)?;

    let _ = &objects
        .dv_cond
        .original
        .write(&mut digivolution_condition_bytes)?;

    let _ = &objects.move_data.original.write(&mut move_data_bytes)?;

    let _ = &objects.shops.original.write(&mut shop_bytes)?;

    let _ = &objects.shop_items.original.write(&mut shop_item_bytes)?;

    let _ = &objects
        .party_exp_bits
        .original
        .write(&mut party_exp_bits_bytes)?;

    let _ = &objects
        .screen_name_mapping
        .write(&mut screen_name_mapping_bytes);

    fs::write(format!("dump/{rom_name}/enemy_stats"), enemy_stats_bytes)?;

    fs::write(format!("dump/{rom_name}/encounters"), encounter_bytes)?;

    fs::write(format!("dump/{rom_name}/enemy_parties"), enemy_party_bytes)?;

    fs::write(
        format!("dump/{rom_name}/party_exp_bits"),
        party_exp_bits_bytes,
    )?;

    fs::write(format!("dump/{rom_name}/digivolutions"), digivolution_bytes)?;

    fs::write(format!("dump/{rom_name}/rookies"), rookie_bytes)?;

    fs::write(format!("dump/{rom_name}/item_shops"), item_shop_bytes)?;

    fs::write(
        format!("dump/{rom_name}/digivolution_conditions"),
        digivolution_condition_bytes,
    )?;

    fs::write(format!("dump/{rom_name}/move_data"), move_data_bytes)?;

    fs::write(format!("dump/{rom_name}/shops"), shop_bytes)?;

    fs::write(format!("dump/{rom_name}/shop_items"), shop_item_bytes)?;

    fs::write(
        format!("dump/{rom_name}/screen_name_mapping"),
        screen_name_mapping_bytes,
    )?;

    for map_obj in &objects.map_objects {
        let mut areas = Vec::new();
        let mut encounters = Vec::new();
        let mut stage_id = Vec::new();

        map_obj.stage_id.write(&mut stage_id)?;

        fs::create_dir_all(format!("dump/{}/maps/{}", rom_name, &map_obj.file_name))?;

        for stage_encounters_obj in &map_obj.stage_encounters {
            for area in &stage_encounters_obj.stage_encounter_areas {
                let warea = match area {
                    Some(a) => &a.modified,
                    None => &DEFAULT_AREA.clone(),
                };

                warea.write(&mut areas)?;
            }

            for encounter in &stage_encounters_obj.stage_encounters {
                let wencounter = match encounter {
                    Some(a) => &a.modified,
                    None => &Vec::from(DEFAULT_ENCOUNTERS.clone()),
                };

                wencounter.write(&mut encounters)?;
            }
        }

        fs::write(
            format!(
                "dump/{}/maps/{}/stage_encounter_areas",
                rom_name, &map_obj.file_name
            ),
            areas,
        )?;
        fs::write(
            format!(
                "dump/{}/maps/{}/stage_encounters",
                rom_name, &map_obj.file_name
            ),
            encounters,
        )?;
        fs::write(
            format!("dump/{}/maps/{}/stage_id", rom_name, &map_obj.file_name),
            stage_id,
        )?;
    }

    Ok(())
}

fn append_file<W: Write>(
    tar_builder: &mut Builder<W>,
    file_name: &str,
    buf: &Vec<u8>,
) -> anyhow::Result<()> {
    let mut header = Header::new_gnu();

    header.set_size(buf.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();

    tar_builder.append_data(&mut header, file_name, &buf[..])?;

    Ok(())
}

pub async fn create_spoiler(
    objects: &Objects,
    path: &std::path::PathBuf,
    file_name: &str,
) -> anyhow::Result<()> {
    let mut enemy_stats_bytes = Vec::new();
    let mut encounter_bytes = Vec::new();
    let mut enemy_party_bytes = Vec::new();
    let mut party_exp_bits_bytes = Vec::new();
    let mut digivolution_bytes = Vec::new();
    let mut rookie_bytes = Vec::new();
    let mut item_shop_bytes = Vec::new();
    let mut digivolution_condition_bytes = Vec::new();
    let mut move_data_bytes = Vec::new();

    let mut shop_bytes = Vec::new();
    let mut shop_item_bytes = Vec::new();

    let mut screen_name_mapping_bytes = Vec::new();

    let _ = &objects.enemy_stats.modified.write(&mut enemy_stats_bytes)?;

    let _ = &objects.encounters.modified.write(&mut encounter_bytes)?;

    let _ = &objects
        .enemy_parties
        .modified
        .write(&mut enemy_party_bytes)?;

    let _ = &objects
        .digivolution_data
        .modified
        .write(&mut digivolution_bytes)?;

    let _ = &objects.rookie_data.modified.write(&mut rookie_bytes)?;

    let _ = &objects
        .item_shop_data
        .modified
        .write(&mut item_shop_bytes)?;

    let _ = &objects
        .dv_cond
        .modified
        .write(&mut digivolution_condition_bytes)?;

    let _ = &objects.move_data.modified.write(&mut move_data_bytes)?;

    let _ = &objects.shops.modified.write(&mut shop_bytes)?;

    let _ = &objects.shop_items.modified.write(&mut shop_item_bytes)?;

    let _ = &objects
        .party_exp_bits
        .modified
        .write(&mut party_exp_bits_bytes)?;

    let _ = &objects
        .screen_name_mapping
        .write(&mut screen_name_mapping_bytes);

    let mut buffer = Vec::new();
    let mut tar_builder = Builder::new(&mut buffer);

    append_file(&mut tar_builder, "enemy_stats", &enemy_stats_bytes)?;
    append_file(&mut tar_builder, "encounters", &encounter_bytes)?;
    append_file(&mut tar_builder, "enemy_parties", &enemy_party_bytes)?;
    append_file(&mut tar_builder, "digivolutions", &digivolution_bytes)?;
    append_file(&mut tar_builder, "rookies", &rookie_bytes)?;
    append_file(&mut tar_builder, "item_shops", &item_shop_bytes)?;
    append_file(
        &mut tar_builder,
        "digivolution_conditions",
        &digivolution_condition_bytes,
    )?;
    append_file(&mut tar_builder, "move_data", &move_data_bytes)?;

    append_file(&mut tar_builder, "shops", &shop_bytes)?;
    append_file(&mut tar_builder, "shop_items", &shop_item_bytes)?;
    append_file(&mut tar_builder, "party_exp_bits", &party_exp_bits_bytes)?;

    append_file(
        &mut tar_builder,
        "screen_name_mapping",
        &screen_name_mapping_bytes,
    )?;

    for map_obj in &objects.map_objects {
        let mut areas = Vec::new();
        let mut encounters = Vec::new();
        let mut stage_id = Vec::new();

        map_obj.stage_id.write(&mut stage_id)?;

        for stage_encounters_obj in &map_obj.stage_encounters {
            for area in &stage_encounters_obj.stage_encounter_areas {
                let warea = match area {
                    Some(a) => &a.modified,
                    None => &DEFAULT_AREA.clone(),
                };

                warea.write(&mut areas)?;
            }

            for encounter in &stage_encounters_obj.stage_encounters {
                let wencounter = match encounter {
                    Some(a) => &a.modified,
                    None => &Vec::from(DEFAULT_ENCOUNTERS.clone()),
                };

                wencounter.write(&mut encounters)?;
            }
        }

        append_file(
            &mut tar_builder,
            format!("maps/{}/stage_encounter_areas", &map_obj.file_name).as_str(),
            &areas,
        )?;
        append_file(
            &mut tar_builder,
            format!("maps/{}/stage_encounters", &map_obj.file_name).as_str(),
            &encounters,
        )?;
        append_file(
            &mut tar_builder,
            format!("maps/{}/stage_id", &map_obj.file_name).as_str(),
            &stage_id,
        )?;
    }

    tar_builder.finish()?;

    drop(tar_builder);

    let rom_name = path
        .file_name()
        .context("Failed file name get")?
        .to_str()
        .context("Failed to_str conversion")?;

    create_dir_all(format!("randomized/{}/{}", rom_name, file_name)).await?;

    let mut spoiler =
        File::create(format!("randomized/{}/{}/spoiler.tar", rom_name, file_name)).await?;

    spoiler.write_all(&buffer).await?;

    Ok(())
}
