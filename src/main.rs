use binread::BinRead;
use binwrite::BinWrite;
use clap::Parser;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;
use std::process::Command;
use std::str;
mod json;

/// Randomize dmw3
#[derive(Parser, Debug)]
struct Arguments {
    /// bin path
    path: std::path::PathBuf,
    /// randomizer preset json
    #[clap(long)]
    preset: Option<std::path::PathBuf>,
}

const CARDMON_MIN: u16 = 0x1c9;
const CARDMON_MAX: u16 = 0x1d0;
const BOSSES: [u16; 26] = [
    0x46, 0x7c, 0x8d, 0xb2, 0x151, 0x164, 0x166, 0x1b4, 0x1ba, 0x1bb, 0x1bc, 0x1bd, 0x1be, 0x1bf,
    0x1c0, 0x1c1, 0x1c2, 0x1c3, 0x1c4, 0x1c5, 0x1c6, 0x1c7, 0x1c8, 0x1d1, 0x1d2, 0x1d3,
];
const TRICERAMON_ID: u16 = 0xcb;

const MAIN_EXECUTABLE: &str = "./extract/SLES_039.36";
const STATS_FILE: &str = "./extract/AAA/PRO/SDIGIEDT.PRO";
const ENCOUNTERS_FILE: &str = "./extract/AAA/PRO/FIELDSTG.PRO";

fn skip(digimon_id: u16, preset: &json::Preset) -> bool {
    return (preset.cardmon && (CARDMON_MIN <= digimon_id && digimon_id <= CARDMON_MAX))
        || (preset.bosses && BOSSES.contains(&digimon_id))
        || (preset.strategy == json::TNTStrategy::Keep && digimon_id == TRICERAMON_ID);
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
struct EnemyStats {
    digimon_id: u16,

    droppable_item: u16,

    drop_rate: u16,

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

    let preset = json::load_preset(&args.preset);

    match Command::new("dumpsxiso")
        .arg("-x")
        .arg("extract/")
        .arg("-s")
        .arg("out.xml")
        .arg("-pt")
        .arg(&args.path)
        .output()
    {
        Err(_) => panic!("Error extracting"),
        _ => {}
    }

    if !Path::new("./out.xml").exists() {
        panic!("Error extracting");
    }

    let stats_buf = fs::read(STATS_FILE).unwrap();
    let encounter_buf = fs::read(ENCOUNTERS_FILE).unwrap();

    let enemy_stats_index = stats_buf
        .windows(16)
        .position(|window| {
            window == b"\x20\x00\x00\x00\x02\x00\x3a\x00\xDC\x00\x00\x00\x00\x00\x32\x00"
        })
        .unwrap();

    let mut enemy_stats_reader = Cursor::new(&stats_buf[enemy_stats_index..]);

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

    let encounter_data_index = encounter_buf
        .windows(16)
        .position(|window| {
            window == b"\x66\x01\x00\x00\x0c\x00\x30\x03\x0f\x27\x10\x00\x7c\x00\x00\x00"
        })
        .unwrap();

    let mut encounter_data_reader = Cursor::new(&encounter_buf[encounter_data_index..]);

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

    let main_buf = fs::read(MAIN_EXECUTABLE).unwrap();
    let parties_index = main_buf
        .windows(9)
        .position(|window| -> bool { window == b"\x00\x06\x07\x02\x03\x06\x01\x05\x07" })
        .unwrap();

    let mut enemy_stats_arr_copy = enemy_stats_arr.clone();
    let mut encounter_data_arr_copy = encounter_data_arr.clone();

    let mut rng = Xoshiro256StarStar::seed_from_u64(preset.seed);

    let len = encounter_data_arr.len();

    // Fisher-Yates shuffles
    for _ in 0..preset.shuffles {
        for i in 0..(len - 2) {
            let uniform: usize = rng.next_u64() as usize;
            let j = i + uniform % (len - i - 1);

            let digimon_id_1 = encounter_data_arr_copy[i].digimon_id as u16;
            let digimon_id_2 = encounter_data_arr_copy[j].digimon_id as u16;

            if skip(digimon_id_1 as u16, &preset) || skip(digimon_id_2 as u16, &preset) {
                continue;
            }

            encounter_data_arr_copy.swap(i, j);
        }
    }

    let mut parties: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut all_digimon: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let rindex = (rng.next_u64() % 7) as usize;
    if preset.rp {
        for i in 0..3 {
            for j in 0..7 {
                let uniform = rng.next_u64() as usize;
                let k = j + uniform % (8 - j);

                all_digimon.swap(j, k);
            }

            for j in 0..3 {
                parties[i * 3 + j] = all_digimon[rindex + j];
            }
        }
    }

    for i in 0..len {
        let old_encounter = &encounter_data_arr[i];
        let new_encounter = &mut encounter_data_arr_copy[i];

        let digimon_id_1 = old_encounter.digimon_id as u16;

        if skip(digimon_id_1 as u16, &preset) {
            continue;
        }

        // hp and mp
        new_encounter.max_hp = (new_encounter.max_hp as u32 * old_encounter.lv as u32
            / new_encounter.lv as u32) as u16;

        new_encounter.lv = old_encounter.lv;
    }

    for enemy_stats in &mut enemy_stats_arr_copy {
        let encounters: Vec<&EncounterData> = encounter_data_arr_copy
            .iter()
            .filter(|&x| x.digimon_id == enemy_stats.digimon_id as u32)
            .collect();

        let min_lv = encounters.iter().min_by(|&x, &y| x.lv.cmp(&y.lv)).unwrap();

        let expect_avg_stats = 36 + min_lv.lv * 10;
        let expect_avg_res = 87 + min_lv.lv * 2;

        let avg_stats: i32 = (enemy_stats.str as i32
            + enemy_stats.def as i32
            + enemy_stats.wis as i32
            + enemy_stats.spt as i32
            + enemy_stats.spd as i32)
            / 5
            + 1;

        let avg_res: i32 = (enemy_stats.fir_res as i32
            + enemy_stats.wtr_res as i32
            + enemy_stats.ice_res as i32
            + enemy_stats.wnd_res as i32
            + enemy_stats.thd_res as i32
            + enemy_stats.mch_res as i32
            + enemy_stats.drk_res as i32)
            / 7
            + 1;

        // base stats
        enemy_stats.str = (enemy_stats.str as i32 * expect_avg_stats as i32 / avg_stats) as i16;
        enemy_stats.def = (enemy_stats.def as i32 * expect_avg_stats as i32 / avg_stats) as i16;
        enemy_stats.wis = (enemy_stats.wis as i32 * expect_avg_stats as i32 / avg_stats) as i16;
        enemy_stats.spt = (enemy_stats.spt as i32 * expect_avg_stats as i32 / avg_stats) as i16;
        enemy_stats.spd = (enemy_stats.spd as i32 * expect_avg_stats as i32 / avg_stats) as i16;

        // resistances
        enemy_stats.fir_res = (enemy_stats.fir_res as i32 * expect_avg_res as i32 / avg_res) as i16;
        enemy_stats.wtr_res = (enemy_stats.wtr_res as i32 * expect_avg_res as i32 / avg_res) as i16;
        enemy_stats.wnd_res = (enemy_stats.wnd_res as i32 * expect_avg_res as i32 / avg_res) as i16;
        enemy_stats.thd_res = (enemy_stats.thd_res as i32 * expect_avg_res as i32 / avg_res) as i16;
        enemy_stats.mch_res = (enemy_stats.mch_res as i32 * expect_avg_res as i32 / avg_res) as i16;
        enemy_stats.drk_res = (enemy_stats.drk_res as i32 * expect_avg_res as i32 / avg_res) as i16;
    }

    if preset.strategy == json::TNTStrategy::Swap {
        let tric = enemy_stats_arr_copy
            .iter()
            .find(|&x| x.digimon_id == TRICERAMON_ID)
            .unwrap();

        let mut titem = tric.droppable_item;
        let mut tdrop = tric.drop_rate;

        let tric_index = encounter_data_arr
            .iter()
            .position(|&x| x.digimon_id as u16 == TRICERAMON_ID && x.lv == 6 && x.x == 16)
            .unwrap();

        let swapped = enemy_stats_arr_copy
            .iter_mut()
            .find(|&&mut x| x.digimon_id == encounter_data_arr_copy[tric_index].digimon_id as u16)
            .unwrap();

        std::mem::swap(&mut titem, &mut swapped.droppable_item);
        std::mem::swap(&mut tdrop, &mut swapped.drop_rate);

        let tricm = enemy_stats_arr_copy
            .iter_mut()
            .find(|&&mut x| x.digimon_id == TRICERAMON_ID)
            .unwrap();

        std::mem::swap(&mut titem, &mut tricm.droppable_item);
        std::mem::swap(&mut tdrop, &mut tricm.drop_rate);
    }

    let mut write_stats_buf = stats_buf.clone();
    let mut write_encounters_buf = encounter_buf.clone();
    let mut write_main_buf = main_buf.clone();

    let mut enemy_stats_buf = vec![];
    let mut encounter_data_buf = vec![];

    enemy_stats_arr_copy.write(&mut enemy_stats_buf).unwrap();
    encounter_data_arr_copy
        .write(&mut encounter_data_buf)
        .unwrap();

    write_main_buf[parties_index..(parties_index + 9)].copy_from_slice(&mut parties);

    write_stats_buf[enemy_stats_index..(enemy_stats_index + enemy_stats_arr.len() * 0x46)]
        .copy_from_slice(&mut enemy_stats_buf);

    write_encounters_buf
        [encounter_data_index..(encounter_data_index + encounter_data_arr.len() * 0xc)]
        .copy_from_slice(&mut encounter_data_buf);

    let bin = format!("dmw3-{x}.bin", x = preset.seed);
    let cue = format!("dmw3-{x}.cue", x = preset.seed);

    let mut new_main_executable = File::create(MAIN_EXECUTABLE).unwrap();
    let mut new_stats = File::create(STATS_FILE).unwrap();
    let mut new_encounters = File::create(ENCOUNTERS_FILE).unwrap();

    match new_main_executable.write_all(&write_main_buf) {
        Err(_) => panic!("Error writing main executable"),
        _ => {}
    }

    match new_stats.write_all(&write_stats_buf) {
        Err(_) => panic!("Error writing stats"),
        _ => {}
    }

    match new_encounters.write_all(&write_encounters_buf) {
        Err(_) => panic!("Error writing encounters"),
        _ => {}
    }

    match Command::new("mkpsxiso")
        .arg("-o")
        .arg(&bin)
        .arg("-c")
        .arg(&cue)
        .arg("./out.xml")
        .arg("-y")
        .output()
    {
        Err(_) => panic!("Error repacking"),
        _ => {}
    }

    println!("randomized into {bin}");
}
