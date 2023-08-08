use binread::BinRead;
use binwrite::BinWrite;
use chrono::Utc;
use clap::Parser;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;

/// Randomize dmw3
#[derive(Parser)]
struct Arguments {
    /// iso path
    path: std::path::PathBuf,
    /// randomization seed
    #[clap(long, default_value_t = Utc::now().timestamp() as u64)]
    seed: u64,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
struct EnemyStats {
    digimon_id: u16,

    droppable_item: u16,

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

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
struct EncounterData {
    digimon_id: u32,

    lv: u16,

    max_hp: u16,

    max_mp: u16,

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

    let mut enemy_stats_reader = Cursor::new(&file_buffer[enemy_stats_index..]);

    let mut enemy_stats_arr: Vec<EnemyStats> = Vec::new();

    loop {
        let stats = EnemyStats::read(&mut enemy_stats_reader);
        let unwrapped: EnemyStats;

        match stats {
            Ok(stat) => unwrapped = stat,
            Err(_) => panic!("Binread error"),
        }

        if unwrapped.digimon_id == 0 {
            break;
        }

        enemy_stats_arr.push(unwrapped);
    }

    let encounter_data_index = file_buffer
        .windows(16)
        .position(|window| {
            window == b"\x66\x01\x00\x00\x0c\x00\x30\x03\x0f\x27\x10\x00\x7c\x00\x00\x00"
        })
        .unwrap();

    let mut encounter_data_reader = Cursor::new(&file_buffer[encounter_data_index..]);

    let mut encounter_data_arr: Vec<EncounterData> = Vec::new();

    loop {
        let encounter = EncounterData::read(&mut encounter_data_reader);
        let unwrapped;

        match encounter {
            Ok(enc) => unwrapped = enc,
            Err(_) => panic!("Binread error"),
        }

        if unwrapped.digimon_id == 0 {
            break;
        }

        encounter_data_arr.push(unwrapped);
    }

    let mut enemy_stats_arr_copy = enemy_stats_arr.clone();
    let mut encounter_data_arr_copy = encounter_data_arr.clone();

    let mut rng = Xoshiro256StarStar::seed_from_u64(args.seed);

    // Fisher-Yates shuffles
    let len = encounter_data_arr.len();
    for i in 0..(len - 2) {
        let uniform: usize = rng.next_u64() as usize;
        let j = i + uniform % (len - i - 1);

        encounter_data_arr_copy.swap(i, j);
    }

    for i in 0..len {
        let old_encounter = encounter_data_arr[i];
        let mut new_encounter = encounter_data_arr_copy[i];

        let enemy_stats_options = enemy_stats_arr_copy
            .iter_mut()
            .find(|&&mut estat| estat.digimon_id as u32 == new_encounter.digimon_id);

        let enemy_stats: &mut EnemyStats;

        match enemy_stats_options {
            None => continue,
            Some(stats) => enemy_stats = stats,
        }

        // base stats
        enemy_stats.str =
            (enemy_stats.str as i32 * old_encounter.lv as i32 / new_encounter.lv as i32) as i16;
        enemy_stats.def =
            (enemy_stats.def as i32 * old_encounter.lv as i32 / new_encounter.lv as i32) as i16;
        enemy_stats.wis =
            (enemy_stats.wis as i32 * old_encounter.lv as i32 / new_encounter.lv as i32) as i16;
        enemy_stats.spt =
            (enemy_stats.spt as i32 * old_encounter.lv as i32 / new_encounter.lv as i32) as i16;
        enemy_stats.spd =
            (enemy_stats.spd as i32 * old_encounter.lv as i32 / new_encounter.lv as i32) as i16;

        // resistances
        enemy_stats.fir_res =
            (enemy_stats.fir_res as i32 * old_encounter.lv as i32 / new_encounter.lv as i32) as i16;
        enemy_stats.wtr_res =
            (enemy_stats.wtr_res as i32 * old_encounter.lv as i32 / new_encounter.lv as i32) as i16;
        enemy_stats.wnd_res =
            (enemy_stats.wnd_res as i32 * old_encounter.lv as i32 / new_encounter.lv as i32) as i16;
        enemy_stats.thd_res =
            (enemy_stats.thd_res as i32 * old_encounter.lv as i32 / new_encounter.lv as i32) as i16;
        enemy_stats.mch_res =
            (enemy_stats.mch_res as i32 * old_encounter.lv as i32 / new_encounter.lv as i32) as i16;
        enemy_stats.drk_res =
            (enemy_stats.drk_res as i32 * old_encounter.lv as i32 / new_encounter.lv as i32) as i16;

        // hp and mp
        new_encounter.max_hp = (new_encounter.max_hp as u32 * old_encounter.lv as u32
            / new_encounter.lv as u32) as u16;
        new_encounter.max_mp = (new_encounter.max_mp as u32 * old_encounter.lv as u32
            / new_encounter.lv as u32) as u16;
    }

    let mut write_buf = file_buffer.clone();

    let mut enemy_stats_buf = vec![];
    let mut encounter_data_buf = vec![];

    enemy_stats_arr_copy.write(&mut enemy_stats_buf).unwrap();
    encounter_data_arr_copy
        .write(&mut encounter_data_buf)
        .unwrap();

    write_buf[enemy_stats_index..(enemy_stats_index + enemy_stats_arr.len() * 0x46)]
        .copy_from_slice(&mut enemy_stats_buf);

    write_buf[encounter_data_index..(encounter_data_index + encounter_data_arr.len() * 0xc)]
        .copy_from_slice(&mut encounter_data_buf);

    let filename = format!("dmw3-{x}.iso", x = args.seed);

    println!("randomizing into {filename}");

    let mut file = File::create(filename).unwrap();

    let _ = file.write_all(&mut write_buf);
}
