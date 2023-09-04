use clap::Parser;

/// Randomize dmw3
#[derive(Parser, Debug, Default)]
pub struct Arguments {
    /// bin path
    pub path: Option<std::path::PathBuf>,
    /// randomizer preset json
    #[clap(long)]
    pub preset: Option<std::path::PathBuf>,
}
