use binread::BinRead;
use binwrite::BinWrite;
use std::fmt::Debug;

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct EnemyStats {
    pub digimon_id: u16,

    pub droppable_item: u16,

    pub drop_rate: u16,

    some_index: u16,

    pub attack: u16,

    move_1: u16,

    move_2: u16,

    pub str: i16,

    pub def: i16,

    pub spt: i16,

    pub wis: i16,

    pub spd: i16,

    pub fir_res: i16,

    pub wtr_res: i16,

    pub ice_res: i16,

    pub wnd_res: i16,

    pub thd_res: i16,

    pub mch_res: i16,

    pub drk_res: i16,

    pub psn_rate: u16,

    pub par_rate: u16,

    pub cnf_rate: u16,

    pub slp_rate: u16,

    pub ko_rate: u16,

    digimon_type: u16,

    moveset_1: Moveset,

    moveset_2: Moveset,

    moveset_3: Moveset,

    moveset_4: Moveset,

    counter_moveset: Moveset,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct EncounterData {
    pub digimon_id: u32,

    pub lv: u16,

    pub max_hp: u16,

    pub max_mp: u16,

    pub multiplier: u16,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
struct Moveset {
    action: u8,
    comparator: u8,
    value: u16,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct DigivolutionData {
    pub digimon_id: u16,

    pub str: u16,

    pub def: u16,

    pub spt: u16,

    pub wis: u16,

    pub spd: u16,

    pub chr: u16,

    pub fir_res: u16,

    pub wtr_res: u16,

    pub ice_res: u16,

    pub wnd_res: u16,

    pub thd_res: u16,

    pub mch_res: u16,

    pub drk_res: u16,

    attack: u16,

    #[br(count = 5)]
    pub tech: Vec<u16>,

    pub ori_tech: u16,

    dv_tech: u16,

    psn_rate: u8,

    par_rate: u8,

    cnf_rate: u8,

    slp_rate: u8,

    ko_rate: u8,

    #[br(count = 5)]
    tech_learn_level: Vec<u8>,

    #[br(count = 5)]
    tech_load_level: Vec<u8>,

    unk0: u8,

    dv: u8,

    unk1: u8,

    exp_modifier: u8,

    unk: u16,

    hp_modifier: u8,

    mp_modifier: u8,

    #[br(count = 6)]
    pub stat_offsets: Vec<u8>,

    #[br(count = 7)]
    pub res_offsets: Vec<u8>,

    #[br(count = 8)]
    unk_arr_1: Vec<u8>,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct Shop {
    pub item_count: u32,
    pub items: Pointer,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct Pointer {
    pub value: u32,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct MoveData {
    mp: u16,
    pub power: u16,
    unk1: u16,
    accuracy: u8,
    unk2: u8,
    boost: u8,
    effective_against: u8,
    pub hit_effect: u8,
    effect_rate: u8,
    effect_value: u8,
    unk3: u8,
    unk4: u16,
    move_type: u8,
    pub freq: u8,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct ItemShopData {
    unk_ptr: Pointer,
    pub buy_price: u16,
    pub sell_price: u16,
    unk: u32,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct DigivolutionCondition {
    pub index: u32,      // +1
    pub dv_index_1: u16, // +1
    rq_level_1: u16,
    pub dv_index_2: u16, // +1
    rq_level_2: u16,
    rq_type: u16,
    rq: u16,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct DigivolutionConditions {
    #[br(count = 44)]
    pub conditions: Vec<DigivolutionCondition>,
}

impl Pointer {
    pub fn to_index(&self) -> u32 {
        self.value - 0x8000f800
    }

    pub fn to_index_overlay(&self, index: u32) -> u32 {
        self.value - index
    }

    // pub fn from_index(index: u32) -> u32 {
    //     index + 0x8000f800
    // }

    // pub fn from_index_overlay(index: u32, overlay: u32) -> u32 {
    //     index + overlay
    // }
}

impl From<&[u8]> for Pointer {
    fn from(buf: &[u8]) -> Self {
        Pointer {
            value: u32::from_ne_bytes([buf[0], buf[1], buf[2], buf[3]]),
        }
    }
}
