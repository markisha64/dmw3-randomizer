use chrono::Utc;
use clap::Parser;

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

    println!("{pa}", pa = args.seed);
    println!("{pa}", pa = args.path.to_str().unwrap());
}
