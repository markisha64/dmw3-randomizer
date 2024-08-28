use clap::Parser;

mod cli;
mod json;
mod mkpsxiso;

mod dump;
mod lang;
mod pack;
mod rand;
mod util;
use rand::patch;
use tokio::runtime::Runtime;

mod gui;

fn main() {
    let args = cli::Arguments::parse();

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        if let Some(path) = &args.path {
            if args.dump {
                if !mkpsxiso::extract(&path).await {
                    panic!("Error extracting");
                }

                dump::dump(&path).await;

                return ();
            }
        }
    });

    match &args.path {
        Some(path) => rt.block_on(async {
            let mut preset = json::load_preset(&args.preset);

            preset.randomizer.seed = match &args.seed {
                Some(seed) => *seed,
                None => preset.randomizer.seed,
            };

            let file_name = match args.output {
                Some(name) => name,
                None => String::from(format!("{}", preset.randomizer.seed)),
            };

            if !mkpsxiso::extract(&path).await {
                panic!("Error extracting");
            }

            patch(&path, &preset).await;

            if !mkpsxiso::build(&file_name).await {
                panic!("Error repacking")
            }

            println!("randomized into {file_name}");
        }),
        None => {
            gui::launch_app();
        }
    }
}
