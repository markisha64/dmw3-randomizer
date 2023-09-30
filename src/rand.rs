use binread::BinRead;
use binwrite::BinWrite;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::PathBuf;

use crate::consts;
use crate::json::Preset;

mod encounters;
mod fixes;
mod parties;
mod scaling;
mod shops;
pub mod structs;
use structs::{DigivolutionData, EncounterData, EnemyStats, MoveData, Pointer, Shop};

use self::structs::ItemShopData;

pub struct Object<T> {
    pub original: Vec<T>,
    pub modified: Vec<T>,
    index: usize,
    slen: usize,
}

trait WriteObjects {
    fn write_buf(&self, source_buf: &mut Vec<u8>);
}

impl<T: BinWrite> WriteObjects for Object<T> {
    fn write_buf(&self, write_buf: &mut Vec<u8>) {
        let mut buf = vec![];

        self.modified.write(&mut buf).unwrap();
        write_buf[self.index..(self.index + self.slen * self.original.len())]
            .copy_from_slice(&mut buf);
    }
}

struct Bufs {
    stats_buf: Vec<u8>,
    encounter_buf: Vec<u8>,
    main_buf: Vec<u8>,
    shops_buf: Vec<u8>,
}

pub struct Objects {
    bufs: Bufs,
    executable: Executable,
    // overlay_address_pointer: Pointer,
    pub enemy_stats: Object<EnemyStats>,
    pub encounters: Object<EncounterData>,
    pub parties: Object<u8>,
    pub digivolution_data: Object<DigivolutionData>,
    pub shops: Object<Shop>,
    pub shop_items: Object<u16>,
    pub item_shop_data: Object<ItemShopData>,
    pub move_data: Object<MoveData>,
}

enum Executable {
    PAL,
    USA,
    JAP,
}

impl Executable {
    fn as_str(&self) -> &'static str {
        match self {
            Executable::PAL => "SLES_039.36",
            Executable::USA => "SLUS_014.36",
            Executable::JAP => "SLPS_034.46",
        }
    }

    fn from_str(string: &str) -> Option<Executable> {
        match string {
            "SLES_039.36" => Some(Executable::PAL),
            "SLUS_014.36" => Some(Executable::USA),
            "SLPS_034.46" => Some(Executable::JAP),
            _ => None,
        }
    }
}

fn read_objects(path: &PathBuf) -> Objects {
    let rom_name = path.file_name().unwrap().to_str().unwrap();
    let itr = fs::read_dir(format!("extract/{}/", rom_name)).unwrap();

    let executable_opt = itr
        .map(|x| -> Option<Executable> {
            let dir_entry = x.unwrap();

            Executable::from_str(dir_entry.file_name().into_string().unwrap().as_str())
        })
        .find(|x| match x {
            Some(_) => true,
            None => false,
        });

    let executable = match executable_opt {
        Some(x) => match x {
            Some(y) => y,
            None => panic!("can't find extracted executable"),
        },
        None => panic!("can't find extracted executable"),
    };

    let stats_buf = fs::read(format!("extract/{}/{}", rom_name, consts::STATS_FILE)).unwrap();
    let encounter_buf =
        fs::read(format!("extract/{}/{}", rom_name, consts::ENCOUNTERS_FILE)).unwrap();
    let main_buf = fs::read(format!("extract/{}/{}", rom_name, executable.as_str())).unwrap();
    let shops_buf = fs::read(format!("extract/{}/{}", rom_name, consts::SHOPS_FILE)).unwrap();

    let overlay_address = Pointer {
        value: consts::OVERLAYADDRESS,
    };

    let overlay = Pointer::from(
        &main_buf[overlay_address.to_index() as usize..overlay_address.to_index() as usize + 4],
    );

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

    let shops_index = shops_buf
        .windows(8)
        .position(|window| window == b"\x00\x00\x00\x00\x0b\x00\x00\x00")
        .unwrap()
        + 4;

    let mut shops_reader = Cursor::new(&shops_buf[shops_index..]);

    let mut shops_arr: Vec<Shop> = Vec::new();
    shops_arr.reserve(consts::SHOPS_LEN);

    for _ in 0..consts::SHOPS_LEN {
        let shop = Shop::read(&mut shops_reader);

        match shop {
            Ok(s) => shops_arr.push(s),
            Err(_) => panic!("Binread error"),
        }
    }

    let front_index = shops_arr
        .first()
        .unwrap()
        .items
        .to_index_overlay(overlay.value as u32) as usize;

    let back_shop = shops_arr.last().unwrap();

    let back_index =
        (back_shop.item_count + back_shop.items.to_index_overlay(overlay.value as u32)) as usize;

    let shop_items_arr: Vec<u16> = shops_buf[front_index..back_index]
        .to_vec()
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_ne_bytes([a[0], a[1]]))
        .collect();

    let item_shop_data_index = main_buf
        .windows(12)
        .position(|window| -> bool {
            window == b"\x64\x01\x65\x01\x66\x01\x67\x01\x00\x00\x00\x00"
        })
        .unwrap();

    let mut item_shop_data_reader = Cursor::new(&main_buf[item_shop_data_index..]);

    let mut item_shop_data_arr: Vec<ItemShopData> = Vec::new();
    item_shop_data_arr.reserve(403);

    for _ in 0..403 {
        let item_shop_data = ItemShopData::read(&mut item_shop_data_reader);

        match item_shop_data {
            Ok(data) => item_shop_data_arr.push(data),
            Err(_) => panic!("Binread error"),
        }
    }

    let digivolution_data_index = main_buf
        .windows(16)
        .position(|window| -> bool {
            window == b"\x7f\x01\x30\x00\x2c\x00\x29\x00\x22\x00\x21\x00\x01\x00\x55\x00"
        })
        .unwrap();

    let mut digivolution_data_reader = Cursor::new(&main_buf[digivolution_data_index..]);

    let mut digivolution_data_arr: Vec<DigivolutionData> = Vec::new();
    digivolution_data_arr.reserve(9);

    for _ in 0..9 {
        let digivolution_data = DigivolutionData::read(&mut digivolution_data_reader);

        match digivolution_data {
            Ok(data) => digivolution_data_arr.push(data),
            Err(_) => panic!("Binread error"),
        }
    }

    let mut move_data_arr: Vec<MoveData> = Vec::new();
    move_data_arr.reserve(444);

    let move_data_index = main_buf
        .windows(18)
        .position(|window| -> bool {
            window == b"\x00\x00\x3c\x00\x02\x02\x6e\x01\x01\x01\x01\x01\x01\x00\x02\x39\x05\x01"
        })
        .unwrap();

    let mut move_data_reader = Cursor::new(&main_buf[move_data_index..]);

    // hardcoding this for now
    for _ in 0..443 {
        let move_data = MoveData::read(&mut move_data_reader);

        match move_data {
            Ok(move_data) => move_data_arr.push(move_data),
            Err(_) => panic!("Binread error"),
        }
    }

    let parties_index = main_buf
        .windows(9)
        .position(|window| -> bool { window == b"\x00\x06\x07\x02\x03\x06\x01\x05\x07" })
        .unwrap();

    let enemy_stats_arr_copy = enemy_stats_arr.clone();
    let encounter_data_arr_copy = encounter_data_arr.clone();
    let digivolution_data_copy = digivolution_data_arr.clone();

    let enemy_stats_object = Object {
        original: enemy_stats_arr,
        modified: enemy_stats_arr_copy,
        index: enemy_stats_index,
        slen: 0x46,
    };

    let encounters_object = Object {
        original: encounter_data_arr,
        modified: encounter_data_arr_copy,
        index: encounter_data_index,
        slen: 0xc,
    };

    let parties_object: Object<u8> = Object {
        original: main_buf[parties_index..parties_index + 9].to_vec(),
        modified: main_buf[parties_index..parties_index + 9].to_vec(),
        index: parties_index,
        slen: 0x1,
    };

    let digivolution_data_object: Object<DigivolutionData> = Object {
        original: digivolution_data_arr,
        modified: digivolution_data_copy,
        index: digivolution_data_index,
        slen: 0x58,
    };

    let shops_object: Object<Shop> = Object {
        original: shops_arr.clone(),
        modified: shops_arr.clone(),
        index: shops_index,
        slen: 0x8,
    };

    let shop_items_object: Object<u16> = Object {
        original: shop_items_arr.clone(),
        modified: shop_items_arr.clone(),
        index: front_index,
        slen: 0x2,
    };

    let item_shop_data_object: Object<ItemShopData> = Object {
        original: item_shop_data_arr.clone(),
        modified: item_shop_data_arr.clone(),
        index: item_shop_data_index,
        slen: 0xc,
    };

    let move_data_object: Object<MoveData> = Object {
        original: move_data_arr.clone(),
        modified: move_data_arr.clone(),
        index: move_data_index,
        slen: 0x12,
    };

    Objects {
        executable,
        // overlay_address_pointer: overlay,
        bufs: Bufs {
            encounter_buf,
            stats_buf,
            main_buf,
            shops_buf,
        },
        enemy_stats: enemy_stats_object,
        encounters: encounters_object,
        parties: parties_object,
        digivolution_data: digivolution_data_object,
        shops: shops_object,
        shop_items: shop_items_object,
        item_shop_data: item_shop_data_object,
        move_data: move_data_object,
    }
}

fn write_objects(path: &PathBuf, objects: &mut Objects) -> Result<(), ()> {
    objects.enemy_stats.write_buf(&mut objects.bufs.stats_buf);
    objects
        .encounters
        .write_buf(&mut objects.bufs.encounter_buf);
    objects.parties.write_buf(&mut objects.bufs.main_buf);
    objects
        .digivolution_data
        .write_buf(&mut objects.bufs.main_buf);
    objects.shop_items.write_buf(&mut objects.bufs.shops_buf);
    objects.shops.write_buf(&mut objects.bufs.shops_buf);
    objects.item_shop_data.write_buf(&mut objects.bufs.main_buf);
    objects.move_data.write_buf(&mut objects.bufs.main_buf);

    let rom_name = path.file_name().unwrap().to_str().unwrap();

    let mut new_main_executable = File::create(format!(
        "extract/{}/{}",
        rom_name,
        objects.executable.as_str()
    ))
    .unwrap();
    let mut new_stats =
        File::create(format!("extract/{}/{}", rom_name, consts::STATS_FILE)).unwrap();
    let mut new_encounters =
        File::create(format!("extract/{}/{}", rom_name, consts::ENCOUNTERS_FILE)).unwrap();
    let mut new_shops =
        File::create(format!("extract/{}/{}", rom_name, consts::SHOPS_FILE)).unwrap();

    match new_main_executable.write_all(&objects.bufs.main_buf) {
        Err(_) => return Err(()),
        _ => {}
    }

    match new_stats.write_all(&objects.bufs.stats_buf) {
        Err(_) => return Err(()),
        _ => {}
    }

    match new_encounters.write_all(&objects.bufs.encounter_buf) {
        Err(_) => return Err(()),
        _ => {}
    }

    match new_shops.write_all(&objects.bufs.shops_buf) {
        Err(_) => return Err(()),
        _ => {}
    }

    Ok(())
}

pub fn patch(path: &PathBuf, preset: &Preset) {
    let mut objects = read_objects(path);

    let mut rng = Xoshiro256StarStar::seed_from_u64(preset.randomizer.seed);

    if preset.randomizer.encounters.enabled {
        encounters::patch(&preset.randomizer, &mut objects, &mut rng);
    }

    if preset.randomizer.parties.enabled {
        parties::patch(&preset.randomizer, &mut objects, &mut rng);
    }

    if preset.scaling.enabled {
        scaling::patch(&preset.scaling, &mut objects, &mut rng);
    }

    if preset.fixes.scaling {
        fixes::scaling(&mut objects);
    }

    if preset.randomizer.shops.enabled {
        shops::patch(&preset.randomizer.shops, &mut objects, &mut rng);
    }

    match write_objects(path, &mut objects) {
        Err(_) => panic!("Error writing objects"),
        _ => (),
    }
}
