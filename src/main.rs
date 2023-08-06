use chrono::Utc;
use clap::Parser;
use std::fs;

/// Randomize dmw3
#[derive(Parser)]
struct Arguments {
    /// iso path
    path: std::path::PathBuf,
    /// randomization seed
    #[clap(long, default_value_t = Utc::now().timestamp().try_into().unwrap())]
    seed: u64,
}

fn main() {
    let args = Arguments::parse();

    let file_buffer = fs::read(args.path).unwrap();

    let enemy_stats_index = file_buffer
        .windows(16)
        .position(|window| {
            window == b"\x20\x00\x00\x00\x02\x00\x3a\x00\xDC\x00\x00\x00\x00\x00\x32\x00"
        })
        .unwrap();

    println!("{enemy_stats_index}");
}
