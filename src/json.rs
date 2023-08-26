use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Preset {
    #[serde(default = "default_randomizer")]
    pub randomizer: Randomizer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Randomizer {
    #[serde(default = "default_seed")]
    pub seed: u64,
    #[serde(default = "default_shuffles")]
    pub shuffles: u8,
    #[serde(default = "default_encounters")]
    pub encounters: Encounters,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Encounters {
    #[serde(default = "default_bool_true")]
    pub cardmon: bool,
    #[serde(default = "default_bool_true")]
    pub bosses: bool,
    #[serde(default = "default_bool_true")]
    pub randomize_parties: bool,
    #[serde(default = "TNTStrategy::default")]
    pub strategy: TNTStrategy,
}

fn default_randomizer() -> Randomizer {
    serde_json::from_str("{}").unwrap()
}

fn default_encounters() -> Encounters {
    serde_json::from_str("{}").unwrap()
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
