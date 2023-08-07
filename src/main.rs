use binread::{io::Cursor, BinRead};
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

    unk1: u16,

    unk2: u16,

    unk3: u16,

    unk4: u16,

    unk5: u16,

    str: i16,

    def: i16,

    spt: i16,

    wis: i16,

    spd: i16,

    fir_res: i16,

    wtr_res: i16,

    ice_res: i16,

    wnd_res: i16,

    thd_res: i16,

    mch_res: i16,

    drk_res: i16,

    psn_rate: u16,

    par_rate: u16,

    cnf_rate: u16,

    slp_rate: u16,

    ko_rate: u16,

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

#[derive(BinRead, Debug)]
struct encounterData {
    digimonId: u32,

    lv: u16,

    maxHp: u16,

    maxMp: u16,

    x: u16,
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

    let mut reader = Cursor::new(&file_buffer[enemy_stats_index..]);

    let mut enemyStatsArr: Vec<enemyStats> = Vec::new();

    loop {
        let stats = enemyStats::read(&mut reader);
        let unwrapped: enemyStats;

        match stats {
            Ok(stat) => unwrapped = stat,
            Err(_) => panic!("Binread error"),
        }

        if unwrapped.digimonId == 0 {
            break;
        }

        enemyStatsArr.push(unwrapped);
    }

    println!("{enemy_stats_index}");
}
