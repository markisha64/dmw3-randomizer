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

    match args.path {
        Some(path) => {
            let preset = json::load_preset(&args.preset);

            match mkpsxiso::extract(&path) {
                Err(_) => panic!("Error extracting"),
                _ => {}
            }

            patch(&preset);

            let bin = format!("dmw3-{x}.bin", x = preset.randomizer.seed);
            let cue = format!("dmw3-{x}.cue", x = preset.randomizer.seed);

            match mkpsxiso::build(&bin, &cue) {
                Err(_) => panic!("Error repacking"),
                _ => {}
            }

            println!("randomized into {bin}");
        }
        None => {
            gui::launch();
        }
    }
}
