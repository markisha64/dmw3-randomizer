use binread::BinRead;
use binwrite::BinWrite;
use std::fmt::Debug;

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct EnemyStats {
    pub digimon_id: u16,

    pub droppable_item: u16,

    pub drop_rate: u16,

    unk2: u16,

    unk3: u16,

    unk4: u16,

    unk5: u16,

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

    unk11: u16,

    unk12: u16,

    unk13: u16,

    unk14: u16,

    unk15: u16,

    unk16: u16,

    unk17: u16,

    unk18: u16,

    unk19: u16,

    unk20: u16,

    unk21: u16,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct EncounterData {
    pub digimon_id: u32,

    pub lv: u16,

    pub max_hp: u16,

    pub max_mp: u16,

    pub x: u16,
}
