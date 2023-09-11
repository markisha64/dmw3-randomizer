use clap::Parser;

mod cli;
mod consts;
mod json;
mod mkpsxiso;

mod rand;
use rand::patch;

mod gui;

fn main() {
    let args = cli::Arguments::parse();

    match &args.path {
        Some(path) => {
            let mut preset = json::load_preset(&args.preset);

            preset.randomizer.seed = match &args.seed {
                Some(seed) => *seed,
                None => preset.randomizer.seed,
            };

            let file_name = match args.output {
                Some(name) => name,
                None => String::from("{preset.randomizer.seed}"),
            };

            match mkpsxiso::extract(&path) {
                Err(_) => panic!("Error extracting"),
                _ => {}
            }

            patch(&preset);

            match mkpsxiso::build(&file_name) {
                Err(_) => panic!("Error repacking"),
                _ => {}
            }

            println!("randomized into {file_name}");
        }
        None => {
            gui::launch();
        }
    }
}
