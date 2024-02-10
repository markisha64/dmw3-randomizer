use clap::Parser;

mod cli;
mod consts;
mod json;
mod mkpsxiso;

mod lang;
mod pack;
mod rand;
mod util;
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
                None => String::from(format!("{}", preset.randomizer.seed)),
            };

            if !mkpsxiso::extract(&path) {
                panic!("Error extracting");
            }

            patch(&path, &preset);

            if !mkpsxiso::build(&file_name) {
                panic!("Error repacking")
            }

            println!("randomized into {file_name}");
        }
        None => {
            gui::launch();
        }
    }
}
