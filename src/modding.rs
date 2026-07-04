use std::path::PathBuf;

use anyhow::Context as _;
use async_std::{
    fs::{self, create_dir_all, File},
    io::WriteExt,
};

use crate::{
    cli::ModAction,
    mkpsxiso,
    objects::{
        fix_lba, read_bufs, read_cargo_tower_text, read_executable, read_iso_project, read_items,
        read_model_objects, read_objects, read_sector_offsets, read_text_files, write_objects,
        Objects,
    },
};

async fn extract(path: &PathBuf) -> anyhow::Result<()> {
    mkpsxiso::extract(path).await?;

    let objects = read_objects(path).await?;

    let serialized = serde_json::to_string_pretty(&objects)?;

    let rom_name = path
        .file_name()
        .context("Failed to get file name")?
        .to_str()
        .context("Failed to convert to str")?;

    let mut json_file = File::create(format!("extract/{}/objects.json", rom_name)).await?;

    json_file.write_all(serialized.as_bytes()).await?;

    Ok(())
}

async fn rebuild(path: &PathBuf) -> anyhow::Result<()> {
    let rom_name = path
        .file_name()
        .context("Failed to get file name")?
        .to_str()
        .context("Failed to convert to str")?;

    let json = fs::read_to_string(format!("extract/{}/objects.json", rom_name)).await?;

    let mut objects: Objects = serde_json::from_str(&json)?;

    objects.executable = read_executable(rom_name).await?;
    objects.bufs = read_bufs(rom_name, &objects.executable).await?;
    (objects.sector_offsets, objects.file_sizes, _) =
        read_sector_offsets(&objects.bufs, &objects.executable)?;
    (objects.iso_project, objects.file_map) = read_iso_project(path).await?;
    objects.cargo_tower_text = read_cargo_tower_text(rom_name, &objects.executable).await?;
    objects.text_files = read_text_files(rom_name, &objects.executable).await?;
    objects.items = read_items(rom_name, &objects.executable).await?;

    objects.model_objects =
        read_model_objects(path, objects.executable.to_model_path(), "M").await?;
    objects.stage_model_objects =
        read_model_objects(path, objects.executable.to_stage_model_path(), "MEFT1").await?;

    for map_object in &mut objects.map_objects {
        map_object.buf = fs::read(format!(
            "extract/{}/AAA/PRO/{}",
            rom_name, map_object.file_name
        ))
        .await?;
    }

    write_objects(path, &mut objects).await?;

    fix_lba(path, &mut objects).await?;

    create_dir_all(format!("randomized/{}/{}", rom_name, "rebuilt")).await?;

    mkpsxiso::build(rom_name, "rebuilt").await?;

    Ok(())
}

pub async fn handle_mod(action: &ModAction) -> anyhow::Result<()> {
    match action {
        ModAction::Extract { path } => extract(path).await,
        ModAction::Rebuild { path } => rebuild(path).await,
    }
}
