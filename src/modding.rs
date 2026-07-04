use std::path::PathBuf;

use anyhow::Context as _;
use async_std::{fs::File, io::WriteExt};

use crate::{cli::ModAction, mkpsxiso, objects::read_objects};

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
    Ok(())
}

pub async fn handle_mod(action: &ModAction) -> anyhow::Result<()> {
    match action {
        ModAction::Extract { path } => extract(path).await,
        ModAction::Rebuild { path } => rebuild(path).await,
    }
}
