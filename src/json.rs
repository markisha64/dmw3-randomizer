use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Preset {
    #[serde(default = "default_seed")]
    pub seed: u64,
    #[serde(default = "default_bool_true")]
    pub cardmon: bool,
    #[serde(default = "default_bool_true")]
    pub bosses: bool,
    #[serde(default = "default_shuffles")]
    pub shuffles: u8,
    #[serde(default = "default_bool_true")]
    pub rp: bool,
    #[serde(default = "TNTStrategy::default")]
    pub strategy: TNTStrategy,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TNTStrategy {
    Shuffle,
    Keep,
    Swap,
}

impl TNTStrategy {
    fn default() -> Self {
        TNTStrategy::Swap
    }
}

pub fn load_preset(path: &Option<std::path::PathBuf>) -> Box<Preset> {
    match path {
        Some(path) => {
            let json_str = fs::read_to_string(path).unwrap();

            let preset: Preset = serde_json::from_str(json_str.as_str()).unwrap();

            Box::new(preset)
        }
        None => Box::new(serde_json::from_str("{}").unwrap()),
    }
}
