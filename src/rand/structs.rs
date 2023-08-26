use binread::BinRead;
use binwrite::BinWrite;
use std::fmt::Debug;

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct EnemyStats {
    pub digimon_id: u16,

    pub droppable_item: u16,

    pub drop_rate: u16,

    some_index: u16,

    unk3: u16,

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

    pub x: u16,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
struct Moveset {
    action: u8,
    comparator: u8,
    value: u16,
}
