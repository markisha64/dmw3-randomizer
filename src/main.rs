use anyhow::Context;
use clap::Parser;

mod cli;
mod json;
mod mkpsxiso;

mod db;
mod dump;
mod lang;
mod rand;
mod util;
use rand::patch;
use tokio::runtime::Runtime;

use crate::dump::create_spoiler;

mod gui;

fn main() -> anyhow::Result<()> {
    db::init()?;

    let args = cli::Arguments::parse();

    let rt = Runtime::new()?;

    rt.block_on(async {
        if let Some(path) = &args.path {
            if args.dump {
                mkpsxiso::extract(path).await?;

                dump::dump(path).await?;
            }
        }

        Ok::<(), anyhow::Error>(())
    })?;

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
