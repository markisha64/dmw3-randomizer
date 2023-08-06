use binread::BinRead;
use chrono::Utc;
use clap::Parser;
use std::fmt::Debug;
use std::fs;

/// Randomize dmw3
#[derive(Parser)]
struct Arguments {
    /// iso path
    path: std::path::PathBuf,
    /// randomization seed
    #[clap(long, default_value_t = Utc::now().timestamp().try_into().unwrap())]
    seed: u64,
}

#[derive(BinRead, Debug)]
struct enemyStats {
    digimonId: u16,

    droppableItem: u16,

    #[br(offset = 0xe)]
    str: i16,

    #[br(offset = 0x10)]
    def: i16,

    #[br(offset = 0x12)]
    spt: i16,

    #[br(offset = 0x14)]
    wis: i16,

    #[br(offset = 0x16)]
    spd: i16,

    #[br(offset = 0x18)]
    fir: i16,

    #[br(offset = 0x1a)]
    fir_res: i16,

    #[br(offset = 0x1c)]
    wtr_res: i16,

    #[br(offset = 0x1e)]
    ice_res: i16,

    #[br(offset = 0x20)]
    wnd_res: i16,

    #[br(offset = 0x22)]
    thd_res: i16,

    #[br(offset = 0x24)]
    mch_res: i16,

    #[br(offset = 0x26)]
    drk_res: i16,
}

fn main() {
    let args = Arguments::parse();

    let file_buffer = fs::read(args.path).unwrap();

    let enemy_stats_index = file_buffer
        .windows(16)
        .position(|window| {
            window == b"\x20\x00\x00\x00\x02\x00\x3a\x00\xDC\x00\x00\x00\x00\x00\x32\x00"
        })
        .unwrap();

    println!("{enemy_stats_index}");
}
