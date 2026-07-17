use anyhow::Context;
use clap::Parser;

mod cli;
mod json;
mod mkpsxiso;

mod db;
mod dump;
mod lang;
mod objects;
mod rand;
mod util;
use rand::patch;
use tokio::runtime::Runtime;

use crate::{dump::create_spoiler, modding::handle_mod};

mod gui;
mod modding;

fn main() -> anyhow::Result<()> {
    db::init()?;

    let cli = cli::Cli::parse();

    let rt = Runtime::new()?;

    if let Some(cli::Command::Mod { action }) = &cli.command {
        return rt.block_on(handle_mod(action));
    }

    let args = cli.args;

    if let Some(path) = &args.path {
        rt.block_on(async {
            if args.dump {
                mkpsxiso::extract(path).await?;

                dump::dump(path).await?;
            }

            Ok::<(), anyhow::Error>(())
        })?;
    }

    match &args.path {
        Some(path) => rt.block_on(async {
            let mut preset = json::load_preset(&args.preset);

            preset.randomizer.seed = match &args.seed {
                Some(seed) => *seed,
                None => preset.randomizer.seed,
            };

            let file_name = match args.output {
                Some(name) => name,
                None => format!("{}", preset.randomizer.seed),
            };

            mkpsxiso::extract(path).await?;

            let rom_name = path
                .file_name()
                .context("Failed file name get")?
                .to_str()
                .context("Failed to_str conversion")?;

            let objects = patch(path, &preset).await?;

            create_spoiler(&objects, path, file_name.as_str()).await?;

            mkpsxiso::build(rom_name, &file_name).await?;

            println!("randomized into {file_name}");

            Ok(())
        }),
        None => {
            gui::launch_app();

            Ok(())
        }
    }
}
