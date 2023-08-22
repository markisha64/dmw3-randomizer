use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    #[serde(default = "default_seed")]
    seed: u64,
    #[serde(default = "default_bool_true")]
    cardmon: bool,
    #[serde(default = "default_bool_true")]
    bosses: bool,
    #[serde(default = "default_shuffles")]
    shuffles: u8,
    #[serde(default = "default_bool_true")]
    rp: bool,
    #[serde(default = "TNTStrategy::default")]
    strategy: TNTStrategy,
}

fn default_seed() -> u64 {
    Utc::now().timestamp() as u64
}

fn default_bool_true() -> bool {
    true
}

fn default_shuffles() -> u8 {
    5
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum TNTStrategy {
    Shuffle,
    Keep,
    Swap,
}

impl TNTStrategy {
    fn default() -> Self {
        TNTStrategy::Swap
    }
}
