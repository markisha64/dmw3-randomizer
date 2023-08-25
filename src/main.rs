use clap::Parser;

mod cli;
mod consts;
mod json;
mod mkpsxiso;

mod rand;
use rand::patch;

fn main() {
    let args = cli::Arguments::parse();

    let preset = json::load_preset(&args.preset);

    match mkpsxiso::extract(&args) {
        Err(_) => panic!("Error extracting"),
        _ => {}
    }

    patch(&preset);

    let bin = format!("dmw3-{x}.bin", x = preset.seed);
    let cue = format!("dmw3-{x}.cue", x = preset.seed);

    match mkpsxiso::build(&bin, &cue) {
        Err(_) => panic!("Error repacking"),
        _ => {}
    }

    println!("randomized into {bin}");
}
