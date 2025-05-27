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
    #[serde(default = "default_party_exp_bits")]
    pub party_exp_bits: PartyExpBits,
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
    #[serde(default = "default_card_game")]
    pub card_game: CardGame,
    #[serde(default = "default_map")]
    pub maps: Maps,
    #[serde(default = "default_models")]
    pub models: Models,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartyExpBits {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
    #[serde(default = "default_f64")]
    pub dv_exp_modifier: f64,
    #[serde(default = "default_f64")]
    pub exp_modifier: f64,
    #[serde(default = "default_f64")]
    pub bits_modifier: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Models {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
    #[serde(default = "default_bool_false")]
    pub hue_enabled: bool,
    #[serde(default = "default_bool_false")]
    pub stage_hue_enabled: bool,
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
    #[serde(default = "default_bool_true")]
    pub keep_galacticmon: bool,
    #[serde(default = "TNTStrategy::default")]
    pub strategy: TNTStrategy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Maps {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
    #[serde(default = "default_bool_false")]
    pub color: bool,
    #[serde(default = "default_bool_false")]
    pub backgrounds: bool,
    #[serde(default = "default_bool_true")]
    pub item_boxes: bool,
    #[serde(default = "ShopItems::default")]
    pub item_boxes_items_only: ShopItems,
    #[serde(default = "default_bool_true")]
    pub fight_backgrounds: bool,
    #[serde(default = "GroupStrategy::default")]
    pub group_strategy: GroupStrategy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parties {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
    #[serde(default = "default_bool_true")]
    pub parties: bool,
    #[serde(default = "default_bool_true")]
    pub stat_distribution: bool,
    #[serde(default = "default_bool_true")]
    pub res_distribution: bool,
    #[serde(default = "default_total_starting_stats")]
    pub total_starting_stats: u16,
    #[serde(default = "default_min_starting_stat")]
    pub min_starting_stat: u16,
    #[serde(default = "default_total_starting_ress")]
    pub total_starting_res: u16,
    #[serde(default = "default_min_starting_res")]
    pub min_starting_res: u16,
    #[serde(default = "default_bool_true")]
    pub learned_tech: bool,
    #[serde(default = "default_bool_true")]
    pub signatures: bool,
    #[serde(default = "default_bool_true")]
    pub digivolutions: bool,
    #[serde(default = "default_bool_true")]
    pub keep_stages: bool,
    #[serde(default = "default_bool_true")]
    pub exp_modifier: bool,
    #[serde(default = "default_min_exp_modifier")]
    pub min_exp_modifier: u8,
    #[serde(default = "default_max_exp_modifier")]
    pub max_exp_modifier: u8,
    #[serde(default = "default_min_hp_modifier")]
    pub min_hp_modifier: u8,
    #[serde(default = "default_max_hp_modifier")]
    pub max_hp_modifier: u8,
    #[serde(default = "default_min_mp_modifier")]
    pub min_mp_modifier: u8,
    #[serde(default = "default_max_mp_modifier")]
    pub max_mp_modifier: u8,
    #[serde(default = "default_bool_true")]
    pub starting_hp_mp: bool,
    #[serde(default = "default_bool_true")]
    pub balance_hp_mp: bool,
    #[serde(default = "default_min_starting_hp")]
    pub min_starting_hp: u8,
    #[serde(default = "default_max_starting_hp")]
    pub max_starting_hp: u8,
    #[serde(default = "default_min_starting_mp")]
    pub min_starting_mp: u8,
    #[serde(default = "default_max_starting_mp")]
    pub max_starting_mp: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Shops {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
    #[serde(default = "default_bool_true")]
    pub limit_shop_items_enabled: bool,
    #[serde(default = "default_shop_limit")]
    pub limit_shop_items: u8,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CardGame {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
    #[serde(default = "default_bool_true")]
    pub shop_items: bool,
    #[serde(default = "default_bool_true")]
    pub buy_price: bool,
    #[serde(default = "default_min_card_buy_price")]
    pub min_card_buy_price: i64,
    #[serde(default = "default_max_card_buy_price")]
    pub max_card_buy_price: i64,
    #[serde(default = "default_bool_true")]
    pub boosters: bool,
    #[serde(default = "default_bool_true")]
    pub starting_folder: bool,
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
    #[serde(default = "default_f64")]
    pub hp_modifier: f64,
    #[serde(default = "default_bool_true")]
    pub natural_scaling: bool,
}

fn default_shop_limit() -> u8 {
    8
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

fn default_card_game() -> CardGame {
    serde_json::from_str("{}").unwrap()
}

fn default_map() -> Maps {
    serde_json::from_str("{}").unwrap()
}

fn default_models() -> Models {
    serde_json::from_str("{}").unwrap()
}

fn default_party_exp_bits() -> PartyExpBits {
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

fn default_f64() -> f64 {
    1.0
}

fn default_min_sell_price() -> i64 {
    6
}

fn default_max_sell_price() -> i64 {
    6000
}

fn default_min_card_buy_price() -> i64 {
    500
}

fn default_max_card_buy_price() -> i64 {
    11000
}

fn default_total_starting_stats() -> u16 {
    200
}

fn default_min_starting_stat() -> u16 {
    30
}

fn default_total_starting_ress() -> u16 {
    680
}

fn default_min_starting_res() -> u16 {
    60
}

fn default_min_exp_modifier() -> u8 {
    6
}

fn default_max_exp_modifier() -> u8 {
    10
}

fn default_min_hp_modifier() -> u8 {
    60
}

fn default_max_hp_modifier() -> u8 {
    100
}

fn default_min_mp_modifier() -> u8 {
    30
}

fn default_max_mp_modifier() -> u8 {
    75
}

fn default_min_starting_hp() -> u8 {
    130
}

fn default_max_starting_hp() -> u8 {
    180
}

fn default_min_starting_mp() -> u8 {
    10
}

fn default_max_starting_mp() -> u8 {
    200
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum TNTStrategy {
    Shuffle,
    Keep,
    #[default]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum GroupStrategy {
    None,
    Map,
    #[default]
    Party,
}

impl From<u8> for GroupStrategy {
    fn from(value: u8) -> Self {
        match value {
            0 => GroupStrategy::None,
            1 => GroupStrategy::Map,
            _ => GroupStrategy::Party,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ShopItems {
    Buyable,
    #[default]
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
