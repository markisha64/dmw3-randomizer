use binread::BinRead;
use binwrite::BinWrite;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;

use super::consts;
use super::json::{Preset, TNTStrategy};

pub mod structs;
use structs::{EncounterData, EnemyStats};

fn skip(digimon_id: u16, preset: &Preset) -> bool {
    return (preset.cardmon
        && (consts::CARDMON_MIN <= digimon_id && digimon_id <= consts::CARDMON_MAX))
        || (preset.bosses && consts::BOSSES.contains(&digimon_id))
        || (preset.strategy == TNTStrategy::Keep && digimon_id == consts::TRICERAMON_ID);
}

struct Object<T> {
    buf: Vec<u8>,
    original: Vec<T>,
    modified: Vec<T>,
    index: usize,
}

struct Objects {
    enemy_stats: Object<EnemyStats>,
    encounters: Object<EncounterData>,
    parties: Object<u8>,
}

fn read_objects() -> Objects {
    let stats_buf = fs::read(consts::STATS_FILE).unwrap();
    let encounter_buf = fs::read(consts::ENCOUNTERS_FILE).unwrap();
    let main_buf = fs::read(consts::MAIN_EXECUTABLE).unwrap();

    let enemy_stats_index = stats_buf
        .windows(16)
        .position(|window| {
            window == b"\x20\x00\x00\x00\x02\x00\x3a\x00\xDC\x00\x00\x00\x00\x00\x32\x00"
        })
        .unwrap();

    let mut enemy_stats_reader = Cursor::new(&stats_buf[enemy_stats_index..]);

    let mut enemy_stats_arr: Vec<structs::EnemyStats> = Vec::new();

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

    let parties_index = main_buf
        .windows(9)
        .position(|window| -> bool { window == b"\x00\x06\x07\x02\x03\x06\x01\x05\x07" })
        .unwrap();

    let enemy_stats_arr_copy = enemy_stats_arr.clone();
    let encounter_data_arr_copy = encounter_data_arr.clone();

    let enemy_stats_object = Object {
        buf: stats_buf,
        original: enemy_stats_arr,
        modified: enemy_stats_arr_copy,
        index: enemy_stats_index,
    };

    let encounters_object = Object {
        buf: encounter_buf,
        original: encounter_data_arr,
        modified: encounter_data_arr_copy,
        index: encounter_data_index,
    };

    let parties_object: Object<u8> = Object {
        buf: main_buf.clone(),
        original: main_buf[parties_index..parties_index + 9].to_vec(),
        modified: main_buf[parties_index..parties_index + 9].to_vec(),
        index: parties_index,
    };

    Objects {
        enemy_stats: enemy_stats_object,
        encounters: encounters_object,
        parties: parties_object,
    }
}

fn write_objects(objects: &Objects) -> Result<(), ()> {
    let mut write_stats_buf = objects.enemy_stats.buf.clone();
    let mut write_encounters_buf = objects.encounters.buf.clone();
    let mut write_main_buf = objects.parties.buf.clone();

    let mut enemy_stats_buf = vec![];
    let mut encounter_data_buf = vec![];

    let modified_enemy_stats = &objects.enemy_stats.modified;
    let modified_encounters = &objects.encounters.modified;
    let mut modified_parties = &objects.parties.modified;

    modified_enemy_stats.write(&mut enemy_stats_buf).unwrap();
    modified_encounters.write(&mut encounter_data_buf).unwrap();

    let parties_index = objects.parties.index;
    let enemy_stats_index = objects.enemy_stats.index;
    let encounter_data_index = objects.encounters.index;

    write_main_buf[parties_index..(parties_index + 9)].copy_from_slice(&mut modified_parties);

    write_stats_buf[enemy_stats_index..(enemy_stats_index + modified_enemy_stats.len() * 0x46)]
        .copy_from_slice(&mut enemy_stats_buf);

    write_encounters_buf
        [encounter_data_index..(encounter_data_index + modified_encounters.len() * 0xc)]
        .copy_from_slice(&mut encounter_data_buf);

    let mut new_main_executable = File::create(consts::MAIN_EXECUTABLE).unwrap();
    let mut new_stats = File::create(consts::STATS_FILE).unwrap();
    let mut new_encounters = File::create(consts::ENCOUNTERS_FILE).unwrap();

    match new_main_executable.write_all(&write_main_buf) {
        Err(_) => return Err(()),
        _ => {}
    }

    match new_stats.write_all(&write_stats_buf) {
        Err(_) => return Err(()),
        _ => {}
    }

    match new_encounters.write_all(&write_encounters_buf) {
        Err(_) => return Err(()),
        _ => {}
    }

    Ok(())
}

pub fn patch(preset: &Preset) {
    let mut objects = read_objects();

    let mut rng = Xoshiro256StarStar::seed_from_u64(preset.seed);

    let len = objects.encounters.original.len();
    let modified_encounters = &mut objects.encounters.modified;
    let encounters = &objects.encounters.original;

    // Fisher-Yates shuffles
    for _ in 0..preset.shuffles {
        for i in 0..(len - 2) {
            let uniform: usize = rng.next_u64() as usize;
            let j = i + uniform % (len - i - 1);

            let digimon_id_1 = modified_encounters[i].digimon_id as u16;
            let digimon_id_2 = modified_encounters[j].digimon_id as u16;

            if skip(digimon_id_1 as u16, &preset) || skip(digimon_id_2 as u16, &preset) {
                continue;
            }

            modified_encounters.swap(i, j);
        }
    }

    let parties = &mut objects.parties.modified;
    let mut all_digimon: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let rindex = (rng.next_u64() % 7) as usize;
    if preset.randomize_parties {
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
        let old_encounter = &encounters[i];
        let new_encounter = &mut modified_encounters[i];

        let digimon_id_1 = old_encounter.digimon_id as u16;

        if skip(digimon_id_1 as u16, &preset) {
            continue;
        }

        // hp and mp
        new_encounter.max_hp = (new_encounter.max_hp as u32 * old_encounter.lv as u32
            / new_encounter.lv as u32) as u16;

        new_encounter.lv = old_encounter.lv;
    }

    let modified_enemy_stats = &mut objects.enemy_stats.modified;

    for enemy_stats in &mut *modified_enemy_stats {
        let encounters: Vec<&EncounterData> = modified_encounters
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

    if preset.strategy == TNTStrategy::Swap {
        let tric = modified_enemy_stats
            .iter()
            .find(|&x| x.digimon_id == consts::TRICERAMON_ID)
            .unwrap();

        let mut titem = tric.droppable_item;
        let mut tdrop = tric.drop_rate;

        let tric_index = encounters
            .iter()
            .position(|&x| x.digimon_id as u16 == consts::TRICERAMON_ID && x.lv == 6 && x.x == 16)
            .unwrap();

        let swapped = modified_enemy_stats
            .iter_mut()
            .find(|&&mut x| x.digimon_id == modified_encounters[tric_index].digimon_id as u16)
            .unwrap();

        std::mem::swap(&mut titem, &mut swapped.droppable_item);
        std::mem::swap(&mut tdrop, &mut swapped.drop_rate);

        let tricm = modified_enemy_stats
            .iter_mut()
            .find(|&&mut x| x.digimon_id == consts::TRICERAMON_ID)
            .unwrap();

        std::mem::swap(&mut titem, &mut tricm.droppable_item);
        std::mem::swap(&mut tdrop, &mut tricm.drop_rate);

        match write_objects(&objects) {
            Err(_) => panic!("Error writing objects"),
            _ => (),
        }
    }
}
