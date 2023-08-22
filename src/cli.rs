use clap::Parser;

/// Randomize dmw3
#[derive(Parser, Debug)]
pub struct Arguments {
    /// bin path
    pub path: std::path::PathBuf,
    /// randomizer preset json
    #[clap(long)]
    pub preset: Option<std::path::PathBuf>,
}
