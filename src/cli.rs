use chrono::Utc;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

/// Randomize dmw3
#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[command(flatten)]
    pub args: Arguments,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Mod tools for extracting and rebuilding the ROM
    Mod {
        #[command(subcommand)]
        action: ModAction,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ModAction {
    /// Extract ROM contents to the specified path
    Extract {
        /// path to extract to
        path: std::path::PathBuf,
    },
    /// Rebuild ROM from the specified path
    Rebuild {
        /// path to rebuild from
        path: std::path::PathBuf,
    },
}

/// Randomize dmw3
#[derive(Parser, Debug, Clone, Deserialize, Serialize)]
pub struct Arguments {
    /// bin path
    pub path: Option<std::path::PathBuf>,
    /// randomizer preset json
    #[clap(long)]
    pub preset: Option<std::path::PathBuf>,
    /// randomizer seed (overrides preset)
    #[clap(long)]
    pub seed: Option<u64>,
    /// output file name
    #[clap(short, long)]
    pub output: Option<String>,
    #[clap(short, long)]
    #[arg(default_value_t = false)]
    pub dump: bool,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            path: None,
            preset: None,
            seed: Some(Utc::now().timestamp() as u64),
            output: None,
            dump: false,
        }
    }
}
