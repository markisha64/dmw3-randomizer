use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Preset {
    #[serde(default = "default_randomizer")]
    pub randomizer: Randomizer,
    #[serde(default = "default_fixes")]
    pub fixes: Fixes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Randomizer {
    #[serde(default = "default_seed")]
    pub seed: u64,
    #[serde(default = "default_shuffles")]
    pub shuffles: u8,
    #[serde(default = "default_encounters")]
    pub encounters: Encounters,
    #[serde(default = "default_parties")]
    pub parties: Parties,
    #[serde(default = "default_shops")]
    pub shops: Shops,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Encounters {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
    #[serde(default = "default_bool_true")]
    pub cardmon: bool,
    #[serde(default = "default_bool_true")]
    pub bosses: bool,
    #[serde(default = "TNTStrategy::default")]
    pub strategy: TNTStrategy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parties {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Shops {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
    #[serde(default = "default_shop_limit")]
    pub limit_shop_items: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Fixes {
    #[serde(default = "default_bool_true")]
    pub scaling: bool,
}

fn default_shop_limit() -> Option<u8> {
    Some(8)
}

fn default_randomizer() -> Randomizer {
    serde_json::from_str("{}").unwrap()
}

fn default_fixes() -> Fixes {
    serde_json::from_str("{}").unwrap()
}

fn default_encounters() -> Encounters {
    serde_json::from_str("{}").unwrap()
}

fn default_parties() -> Parties {
    serde_json::from_str("{}").unwrap()
}

fn default_shops() -> Shops {
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TNTStrategy {
    Shuffle,
    Keep,
    Swap,
}

impl From<u8> for TNTStrategy {
    fn from(value: u8) -> Self {
        match value {
            0 => TNTStrategy::Shuffle,
            1 => TNTStrategy::Keep,
            _ => TNTStrategy::Swap,
        }
    }
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
