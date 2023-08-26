use binread::BinRead;
use binwrite::BinWrite;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;

use super::consts;
use super::json::Preset;

mod encounters;
pub mod structs;
use structs::{EncounterData, EnemyStats};

pub struct Object<T> {
    pub buf: Vec<u8>,
    pub original: Vec<T>,
    pub modified: Vec<T>,
    index: usize,
    slen: usize,
}

trait WriteObjects {
    fn write_buf(&self) -> Vec<u8>;
}

impl<T: BinWrite> WriteObjects for Object<T> {
    fn write_buf(&self) -> Vec<u8> {
        let mut write_buf = self.buf.clone();
        let mut buf = vec![];

        self.modified.write(&mut buf).unwrap();
        write_buf[self.index..(self.index + self.slen * self.original.len())]
            .copy_from_slice(&mut buf);

        write_buf
    }
}

pub struct Objects {
    pub enemy_stats: Object<EnemyStats>,
    pub encounters: Object<EncounterData>,
    pub parties: Object<u8>,
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
        slen: 0x46,
    };

    let encounters_object = Object {
        buf: encounter_buf,
        original: encounter_data_arr,
        modified: encounter_data_arr_copy,
        index: encounter_data_index,
        slen: 0xc,
    };

    let parties_object: Object<u8> = Object {
        buf: main_buf.clone(),
        original: main_buf[parties_index..parties_index + 9].to_vec(),
        modified: main_buf[parties_index..parties_index + 9].to_vec(),
        index: parties_index,
        slen: 0x1,
    };

    Objects {
        enemy_stats: enemy_stats_object,
        encounters: encounters_object,
        parties: parties_object,
    }
}

fn write_objects(objects: &Objects) -> Result<(), ()> {
    let write_stats_buf = objects.enemy_stats.write_buf();
    let write_encounters_buf = objects.encounters.write_buf();
    let write_main_buf = objects.parties.write_buf();

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

    encounters::patch(preset, &mut objects, &mut rng);

    match write_objects(&objects) {
        Err(_) => panic!("Error writing objects"),
        _ => (),
    }
}
