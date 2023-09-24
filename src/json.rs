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
    #[serde(default = "default_scaling")]
    pub scaling: Scaling,
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
    #[serde(default = "default_bool_true")]
    pub keep_zanbamon: bool,
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
    #[serde(default = "ShopItems::default")]
    pub items_only: ShopItems,
    #[serde(default = "default_bool_true")]
    pub sell_price: bool,
    #[serde(default = "default_min_sell_price")]
    pub min_sell_price: i64,
    #[serde(default = "default_max_sell_price")]
    pub max_sell_price: i64,
    #[serde(default = "default_bool_true")]
    pub keep_tnt: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Fixes {
    #[serde(default = "default_bool_false")]
    pub scaling: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Scaling {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
    #[serde(default = "default_scaling_offset")]
    pub scaling_offset: i64,
    #[serde(default = "default_base_stats")]
    pub base_stats: i32,
    #[serde(default = "default_base_res")]
    pub base_res: i32,
    #[serde(default = "default_stat_modifier")]
    pub stat_modifier: i32,
    #[serde(default = "default_res_modifier")]
    pub res_modifier: i32,
    #[serde(default = "default_hp_modifier")]
    pub hp_modifier: f64,
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

fn default_scaling() -> Scaling {
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

fn default_bool_false() -> bool {
    false
}

fn default_shuffles() -> u8 {
    5
}

fn default_scaling_offset() -> i64 {
    50
}

fn default_base_stats() -> i32 {
    180
}

fn default_base_res() -> i32 {
    620
}

fn default_stat_modifier() -> i32 {
    50
}

fn default_res_modifier() -> i32 {
    15
}

fn default_hp_modifier() -> f64 {
    1.0
}

fn default_min_sell_price() -> i64 {
    6
}

fn default_max_sell_price() -> i64 {
    11000
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

impl Default for TNTStrategy {
    fn default() -> Self {
        TNTStrategy::Swap
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ShopItems {
    Buyable,
    Sellable,
}

impl From<u8> for ShopItems {
    fn from(value: u8) -> Self {
        match value {
            0 => ShopItems::Buyable,
            _ => ShopItems::Sellable,
        }
    }
}

impl Default for ShopItems {
    fn default() -> Self {
        ShopItems::Sellable
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
