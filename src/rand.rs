use binread::BinRead;
use binwrite::BinWrite;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::PathBuf;

use crate::consts;
use crate::json::Preset;
use crate::lang::Language;
use crate::mkpsxiso;
use crate::mkpsxiso::xml_file;
use crate::pack::Packed;

mod encounters;
mod fixes;
mod maps;
mod parties;
mod scaling;
mod shops;
pub mod structs;
use structs::{
    DigivolutionConditions, DigivolutionData, EncounterData, EnemyStats, EntityData, EntityLogic,
    Environmental, ItemShopData, MapColor, MoveData, Pointer, Shop, StageLoadData,
};

pub struct Object<T> {
    pub original: T,
    pub modified: T,
    index: usize,
    slen: usize,
}

pub struct TextFile {
    file: Packed,
    _file_name: String,
}

pub struct TextFileGroup {
    files: HashMap<Language, TextFile>,
    mapped_items: HashMap<u32, u16>,
    generic_item: Option<u16>,
}

pub struct ObjectArray<T> {
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
        write_buf[self.index..(self.index + self.slen)].copy_from_slice(&mut buf);
    }
}

impl<T: BinWrite> WriteObjects for ObjectArray<T> {
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
    exp_buf: Vec<u8>,
    pack_select_buf: Vec<u8>,
    _map_buf: Vec<u8>,
}

pub struct MapObject {
    file_name: String,
    buf: Vec<u8>,
    environmentals: Option<ObjectArray<Environmental>>,
    map_color: Option<Object<MapColor>>,
    background_file_index: Object<u16>,
    entities: Option<ObjectArray<EntityData>>,
    entity_logics: Vec<Object<EntityLogic>>,
    scripts: Vec<ObjectArray<u32>>,
    talk_file: Option<u16>,
    _stage_id: u16,
}

pub struct Objects {
    bufs: Bufs,
    executable: Executable,
    stage: Pointer,

    // hard coded data
    pub file_map: Vec<mkpsxiso::File>,
    pub sector_offsets: Vec<u32>,

    pub parties: ObjectArray<u8>,
    pub pack_previews: ObjectArray<u32>,

    pub enemy_stats: ObjectArray<EnemyStats>,
    pub encounters: ObjectArray<EncounterData>,
    pub rookie_data: ObjectArray<DigivolutionData>,
    pub digivolution_data: ObjectArray<DigivolutionData>,
    pub shops: ObjectArray<Shop>,
    pub shop_items: ObjectArray<u16>,
    pub item_shop_data: ObjectArray<ItemShopData>,
    pub move_data: ObjectArray<MoveData>,
    pub dv_cond: ObjectArray<DigivolutionConditions>,
    pub stage_load_data: Vec<StageLoadData>,
    pub map_objects: Vec<MapObject>,

    pub text_files: BTreeMap<String, TextFileGroup>,
    pub items: TextFileGroup,
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

    fn languages(&self) -> &[Language] {
        match self {
            Executable::PAL => &[
                Language::English,
                Language::French,
                Language::Italian,
                Language::German,
                Language::Spanish,
            ],
            Executable::USA => &[Language::US],
            Executable::JAP => &[Language::Japanese],
        }
    }

    fn text_files(&self) -> &[&str] {
        match self {
            Executable::USA => &[
                consts::ITEM_NAMES,
                "STALK00.BIN",
                "STALK01.BIN",
                "STALK02.BIN",
                "STALK03.BIN",
                "STALK04.BIN",
                "STALK05.BIN",
                "STALK06.BIN",
                "STALK07.BIN",
            ],
            _ => &[
                consts::ITEM_NAMES,
                "STALK00.BIN",
                "STALK01.BIN",
                "STALK02.BIN",
                "STALK03.BIN",
                "STALK04.BIN",
                "STALK05.BIN",
                "STALK06.BIN",
                "STALK07.BIN",
                "STALK08.BIN",
                "STALK09.BIN",
            ],
        }
    }

    fn to_stage_load_data_address(&self) -> Pointer {
        match self {
            Executable::PAL => Pointer { value: 0x8009a884 },
            Executable::USA => Pointer { value: 0x800998e4 },
            Executable::JAP => Pointer { value: 0x8009a598 },
        }
    }

    fn to_sector_offsets_address(&self) -> Pointer {
        match self {
            Executable::PAL => Pointer { value: 0x80044f6c },
            Executable::USA => Pointer { value: 0x80044b78 },
            Executable::JAP => Pointer { value: 0x80044d98 },
        }
    }
}

fn read_map_objects(
    path: &PathBuf,
    stage_load_data: &Vec<StageLoadData>,
    stage: &Pointer,
    file_map: &Vec<mkpsxiso::File>,
    sector_offsets: &Vec<u32>,
    executable: &Executable,
) -> Vec<MapObject> {
    let rom_name = path.file_name().unwrap().to_str().unwrap();
    let pro_folder = fs::read_dir(format!("extract/{}/AAA/PRO", rom_name)).unwrap();

    let mut result: Vec<MapObject> = Vec::new();

    for file_res in pro_folder {
        let file = file_res.unwrap();

        if let Ok(file_type) = file.file_type() {
            if file_type.is_dir() {
                continue;
            }
        }

        let file_name = file.file_name().into_string().unwrap();
        if !file_name.starts_with("WSTAG") || file_name.starts_with("WSTAG9") {
            continue;
        }

        let buf = fs::read(format!("extract/{}/AAA/PRO/{}", rom_name, file_name)).unwrap();

        let mut offset: usize = 0x4;
        let mut idx = buf.len() - offset;
        let mut sstart_idx = buf.len() - offset - 0x10;
        let mut sstart = buf[sstart_idx..sstart_idx + 8]
            .iter()
            .fold(0, |a, b| a + *b as u32);
        let mut init_stage_pointers = Pointer::from(&buf[idx..idx + 4]);

        while !(init_stage_pointers.is_valid() && sstart == 0) && (offset + 0x24) <= buf.len() {
            offset += 0x14;

            idx = buf.len() - offset;
            sstart_idx = buf.len() - offset - 0x10;

            sstart = buf[sstart_idx..sstart_idx + 8]
                .iter()
                .fold(0, |a, b| a + *b as u32);

            init_stage_pointers = Pointer::from(&buf[idx..idx + 4]);
        }

        if init_stage_pointers.is_valid() {
            let mut pptr = Pointer::from(&buf[idx - 4..idx]);
            while pptr.is_valid() {
                idx -= 4;

                init_stage_pointers = Pointer::from(&buf[idx..idx + 4]);
                pptr = Pointer::from(&buf[idx - 4..idx]);
            }
        }

        if !init_stage_pointers.is_valid() {
            continue;
        }

        let idx = init_stage_pointers.to_index_overlay(stage.value as u32) as usize;

        let jr_ra_instruction_index = buf[idx..]
            .windows(4)
            .position(|x| x == consts::JR_RA_INSTRUCTION);

        if jr_ra_instruction_index.is_none() {
            continue;
        }

        let jump_return = Pointer {
            value: init_stage_pointers.value + jr_ra_instruction_index.unwrap() as u32,
        };

        let initsp_index = init_stage_pointers.to_index_overlay(stage.value as u32) as usize;
        let initp_end_index = jump_return.to_index_overlay(stage.value as u32) as usize;

        let bg_set_offset_result = buf[initsp_index..initp_end_index]
            .windows(2)
            .position(|x| x == b"\x08\x00");

        if bg_set_offset_result.is_none() {
            continue;
        }

        let bg_set_offset = bg_set_offset_result.unwrap();
        let bg_set_idx = initsp_index + bg_set_offset;

        let sw = &buf[bg_set_idx + 2..bg_set_idx + 4];

        let res = buf[initsp_index..bg_set_idx]
            .windows(2)
            .rev()
            .position(|x| x == consts::LI_INSTRUCTION);

        if res.is_none() {
            continue;
        }

        let li_instruction = bg_set_idx - res.unwrap() - 2;

        let sector_offset = file_map
            .iter()
            .find(|file| file.name == file_name)
            .unwrap()
            .offs;

        let sector_offsets_index = sector_offsets
            .iter()
            .position(|off| *off == sector_offset)
            .unwrap() as u32;

        let stage_load_data_row = stage_load_data
            .iter()
            .find(|sldr| sldr.file_index == sector_offsets_index)
            .unwrap();

        let bg_file_index = u16::from_le_bytes([buf[li_instruction - 2], buf[li_instruction - 1]]);

        let background_file_index_index = li_instruction - 2;
        let background_file_index = bg_file_index;

        let stage_id = stage_load_data_row.stage_id as u16;

        let background_object = Object {
            original: background_file_index,
            modified: background_file_index,
            slen: 0x2,
            index: background_file_index_index as usize,
        };

        let mut map_color: Option<Object<MapColor>> = None;

        if let Some(map_color_offset) = buf[initsp_index..initp_end_index]
            .windows(2)
            .position(|x| x == consts::STAGE_COLOR_INSTRUCTION_HALF)
        {
            if let Some(map_color_addiu) = buf[initsp_index..initsp_index + map_color_offset]
                .windows(4)
                .rev()
                .position(|x| x[3] == consts::ADDIU)
            {
                let aidx = initsp_index + map_color_offset - map_color_addiu - 4;
                let addiu_first_half = i16::from_le_bytes([buf[aidx], buf[aidx + 1]]);

                let address = (0x800a0000 + addiu_first_half as i64) as u32;

                let map_color_address = Pointer { value: address };

                let idx = map_color_address.to_index_overlay(stage.value as u32) as usize;

                let mut map_color_reader = Cursor::new(&buf[idx..idx + 4]);
                let res = MapColor::read(&mut map_color_reader);

                map_color = match res {
                    Ok(color) => Some(Object {
                        original: color.clone(),
                        modified: color.clone(),
                        index: idx,
                        slen: 0x4,
                    }),
                    Err(_) => panic!("binread error"),
                };
            }
        }

        let mut environmentals: Vec<Environmental> = Vec::new();
        let mut environmentals_index: Option<u32> = None;

        let mut entities: Vec<EntityData> = Vec::new();
        let mut entities_index: Option<u32> = None;

        let mut entity_logics = Vec::new();
        let mut scripts = Vec::new();

        let mut talk_file = None;

        // we need to assemble full sw instruction
        let environmental_instruction = [
            consts::ENVIRONMENTAL_INSTRUCTION[0],
            consts::ENVIRONMENTAL_INSTRUCTION[1],
            sw[0],
            sw[1],
        ];

        let entities_instruction = [
            consts::ENTITIES_INSTRUCTION[0],
            consts::ENTITIES_INSTRUCTION[1],
            sw[0],
            sw[1],
        ];

        let talk_file_instruction = [
            consts::TALK_FILE_INSTRUCTION[0],
            consts::TALK_FILE_INSTRUCTION[1],
            sw[0],
            sw[1],
        ];

        if let Some(talk_file_set) = buf[initsp_index..initp_end_index]
            .windows(4)
            .position(|x| x == talk_file_instruction)
        {
            let instr = match executable {
                Executable::PAL => consts::TALK_FILE_ADDIU,
                _ => consts::LI_INSTRUCTION,
            };

            let res = buf[initsp_index..initsp_index + talk_file_set]
                .windows(2)
                .rev()
                .position(|x| x == instr)
                .unwrap();

            let instruction = initsp_index + talk_file_set - res - 2;

            talk_file = Some(u16::from_le_bytes([
                buf[instruction - 2],
                buf[instruction - 1],
            ]));
        }

        if let Some(environmental_set) = buf[initsp_index..initp_end_index]
            .windows(4)
            .position(|x| x == environmental_instruction)
        {
            let environmental_address =
                Pointer::from_instruction(&buf, initsp_index + environmental_set);

            let env_index = environmental_address.to_index_overlay(stage.value as u32);
            environmentals_index = Some(env_index);

            let mut environmentals_reader = Cursor::new(&buf[(env_index as usize)..]);

            loop {
                let environmental = Environmental::read(&mut environmentals_reader);
                let unwrapped;

                match environmental {
                    Ok(env) => unwrapped = env,
                    Err(_) => panic!("binread error"),
                }

                if unwrapped.conditions[0] == 0x0000ffff
                    && unwrapped.conditions[1] == 0x0000ffff
                    && unwrapped.next_stage_id == 0
                {
                    break;
                }

                environmentals.push(unwrapped);
            }
        }

        if let Some(entities_set) = buf[initsp_index..initp_end_index]
            .windows(4)
            .position(|x| x == entities_instruction)
        {
            let entities_address = Pointer::from_instruction(&buf, initsp_index + entities_set);

            if entities_address.is_valid() {
                let ent_index = entities_address.to_index_overlay(stage.value as u32) as usize;

                let mut i = 0;
                loop {
                    let ptr = Pointer::from(&buf[ent_index + i * 4..ent_index + (i + 1) * 4]);

                    if ptr.null() {
                        break;
                    }

                    i += 1;
                }

                let real_pointer = Pointer::from(&buf[ent_index..ent_index + 4]);

                let real_idx = real_pointer.to_index_overlay(stage.value as u32);
                entities_index = Some(real_idx);

                let mut entities_reader =
                    Cursor::new(&buf[(real_idx as usize)..(real_idx as usize) + 0x14 * i]);

                for _ in 0..i {
                    let entity_result = EntityData::read(&mut entities_reader);

                    if entity_result.is_err() {
                        panic!("binread error");
                    }

                    let entity = entity_result.unwrap();

                    if !entity.logic.null() {
                        let logic_idx = entity.logic.to_index_overlay(stage.value as u32);

                        let mut logic_reader =
                            Cursor::new(&buf[logic_idx as usize..logic_idx as usize + 0xa]);

                        if let Ok(logic) = EntityLogic::read(&mut logic_reader) {
                            entity_logics.push(Object {
                                original: logic.clone(),
                                modified: logic.clone(),
                                slen: 0xa,
                                index: logic_idx as usize,
                            });

                            let mut full_script = Vec::new();
                            if !logic.script.null() {
                                let script_idx = logic.script.to_index_overlay(stage.value as u32);

                                let mut script_reader = Cursor::new(&buf[script_idx as usize..]);

                                loop {
                                    let script_result = u32::read(&mut script_reader);

                                    match script_result {
                                        Ok(script) => {
                                            if script == 0x0000ffff {
                                                break;
                                            }

                                            full_script.push(script);
                                        }
                                        Err(_) => panic!("binread error"),
                                    }
                                }

                                if !full_script.is_empty() {
                                    scripts.push(ObjectArray {
                                        original: full_script.clone(),
                                        modified: full_script.clone(),
                                        slen: 0x4,
                                        index: script_idx as usize,
                                    })
                                }
                            }
                        }
                    }

                    entities.push(entity);
                }
            }
        }

        let environmental_object = match environmentals_index {
            Some(idx) => Some(ObjectArray {
                original: environmentals.clone(),
                modified: environmentals.clone(),
                index: idx as usize,
                slen: 0x18,
            }),
            None => None,
        };

        let entities_object = match entities_index {
            Some(idx) => Some(ObjectArray {
                original: entities.clone(),
                modified: entities.clone(),
                index: idx as usize,
                slen: 0x14,
            }),
            None => None,
        };

        // TODO: instead of always checking first 2 instructions before
        // I need to find the first lui (which is considerably suckier)
        result.push(MapObject {
            file_name,
            buf: buf.clone(),
            environmentals: environmental_object,
            entities: entities_object,
            entity_logics,
            scripts,
            map_color,
            background_file_index: background_object,
            talk_file,
            _stage_id: stage_id,
        });
    }

    return result;
}

fn read_objects(path: &PathBuf) -> Objects {
    let rom_name = path.file_name().unwrap().to_str().unwrap();
    let iso_project = xml_file();

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
    let exp_buf = fs::read(format!("extract/{}/{}", rom_name, consts::EXP_FILE)).unwrap();
    let map_buf = fs::read(format!("extract/{}/{}", rom_name, consts::MAP_FILE)).unwrap();
    let pack_select_buf =
        fs::read(format!("extract/{}/{}", rom_name, consts::PACK_SELECT_FILE)).unwrap();

    let overlay_address = Pointer {
        value: consts::OVERLAY_ADDRESS,
    };

    let overlay = Pointer::from(
        &main_buf[overlay_address.to_index() as usize..overlay_address.to_index() as usize + 4],
    );

    let stage_address = Pointer {
        value: consts::STAGE_ADDRESS,
    };

    let stage = Pointer::from(
        &main_buf[stage_address.to_index() as usize..stage_address.to_index() as usize + 4],
    );

    let file_map = iso_project.flatten();

    let mut sector_offsets: Vec<u32> = Vec::new();
    sector_offsets.reserve(file_map.len());

    let sector_offsets_index = executable.to_sector_offsets_address().to_index() as usize;

    for i in 0..file_map.len() {
        sector_offsets.push(u32::from_le_bytes(
            main_buf[sector_offsets_index + i * 4..sector_offsets_index + i * 4 + 4]
                .try_into()
                .unwrap(),
        ));
    }

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

        // this check works because after Encounter array
        // goes Team array which starts with a non null ptr
        if unwrapped.digimon_id > 500 {
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

    let mut rookie_data_arr: Vec<DigivolutionData> = Vec::new();
    rookie_data_arr.reserve(consts::ROOKIE_COUNT);

    for _ in 0..consts::ROOKIE_COUNT {
        let rookie_data = DigivolutionData::read(&mut digivolution_data_reader);

        match rookie_data {
            Ok(data) => rookie_data_arr.push(data),
            Err(_) => panic!("Binread error"),
        }
    }

    let mut digivolution_data_arr: Vec<DigivolutionData> = Vec::new();
    digivolution_data_arr.reserve(consts::DIGIVOLUTION_COUNT);

    for _ in 0..consts::DIGIVOLUTION_COUNT {
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
        .position(|window| window == consts::PACKS)
        .unwrap();

    let default_packs: Vec<u32> = consts::PACKS.iter().map(|x| *x as u32).collect();
    let mut default_pack_preview: Vec<u8> = Vec::new();
    for mon in consts::PACKS {
        // yes this is ugly, no I don't care
        default_pack_preview.push(*mon);
        default_pack_preview.extend(b"\x00\x00\x00");
    }

    let pack_select_preview_index = pack_select_buf
        .windows(36)
        .position(|window| window == default_pack_preview)
        .unwrap();

    let mut dv_cond_arr: Vec<DigivolutionConditions> = Vec::new();
    dv_cond_arr.reserve(consts::ROOKIE_COUNT);

    let dv_cond_index = exp_buf
        .windows(8)
        .position(|x| x == b"\x09\x00\x00\x00\x00\x00\x01\x00")
        .unwrap();

    let mut dv_cond_reader = Cursor::new(&exp_buf[dv_cond_index..]);

    for _ in 0..consts::ROOKIE_COUNT {
        let dv_cond = DigivolutionConditions::read(&mut dv_cond_reader);

        match dv_cond {
            Ok(cond) => dv_cond_arr.push(cond),
            Err(_) => panic!("Binread error"),
        }
    }

    let stage_load_data_index = executable
        .to_stage_load_data_address()
        .to_index_overlay(overlay.value as u32) as usize;

    let mut stage_load_data_arr: Vec<StageLoadData> = Vec::new();
    stage_load_data_arr.reserve(consts::STAGE_LOAD_DATA_LENGTH);

    let mut stage_load_data_reader = Cursor::new(&map_buf[stage_load_data_index..]);

    for _ in 0..consts::STAGE_LOAD_DATA_LENGTH {
        let res = StageLoadData::read(&mut stage_load_data_reader);

        match res {
            Ok(stage_load_data) => stage_load_data_arr.push(stage_load_data),
            Err(_) => panic!("Binread error"),
        }
    }

    let mut item_files: HashMap<Language, TextFile> = HashMap::new();

    for lang in executable.languages() {
        let fsname = lang.to_file_name(consts::ITEM_NAMES);

        let file = fs::read(format!(
            "extract/{}/{}",
            rom_name,
            lang.to_path(consts::ITEM_NAMES)
        ))
        .unwrap();

        let packed = Packed::from(file);

        item_files.insert(
            *lang,
            TextFile {
                file: packed,
                _file_name: fsname,
            },
        );
    }

    let items = TextFileGroup {
        files: item_files,
        mapped_items: HashMap::new(),
        generic_item: None,
    };

    let mut text_files: BTreeMap<String, TextFileGroup> = BTreeMap::new();
    for sname in executable.text_files() {
        let mut files: HashMap<Language, TextFile> = HashMap::new();

        let mut generic_item = None;
        for lang in executable.languages() {
            let fsname = lang.to_file_name(sname);

            let file = fs::read(format!("extract/{}/{}", rom_name, lang.to_path(sname))).unwrap();

            let packed = Packed::from(file);

            files.insert(
                *lang,
                TextFile {
                    file: packed,
                    _file_name: fsname,
                },
            );
        }

        let doesnt_fit = files.iter().find(|(lang, talk_file)| {
            let csize = talk_file.file.file_size();

            let generic_text = &lang.to_received_item_generic();

            csize + 4 + generic_text.len() > ((csize / 2048) + (csize % 2048 != 0) as usize) * 2048
        });

        if doesnt_fit.is_none() {
            for (lang, talk_file) in &mut files {
                let generic_text = lang.to_received_item_generic();
                generic_item = Some(talk_file.file.files.len() as u16);

                talk_file.file.files.push(generic_text);
            }
        }

        let group = TextFileGroup {
            files,
            mapped_items: HashMap::new(),
            generic_item,
        };

        text_files.insert(String::from(*sname), group);
    }

    let enemy_stats_arr_copy = enemy_stats_arr.clone();
    let encounter_data_arr_copy = encounter_data_arr.clone();
    let rookie_data_copy = rookie_data_arr.clone();
    let digivolution_data_copy = digivolution_data_arr.clone();
    let dv_cond_copy = dv_cond_arr.clone();

    let enemy_stats_object = ObjectArray {
        original: enemy_stats_arr,
        modified: enemy_stats_arr_copy,
        index: enemy_stats_index,
        slen: 0x46,
    };

    let encounters_object = ObjectArray {
        original: encounter_data_arr,
        modified: encounter_data_arr_copy,
        index: encounter_data_index,
        slen: 0xc,
    };

    let parties_object: ObjectArray<u8> = ObjectArray {
        original: consts::PACKS.into(),
        modified: consts::PACKS.into(),
        index: parties_index,
        slen: 0x1,
    };

    let party_previewes_object: ObjectArray<u32> = ObjectArray {
        original: default_packs.clone(),
        modified: default_packs.clone(),
        index: pack_select_preview_index,
        slen: 0x4,
    };

    let rookie_data_object: ObjectArray<DigivolutionData> = ObjectArray {
        original: rookie_data_arr,
        modified: rookie_data_copy,
        index: digivolution_data_index,
        slen: 0x58,
    };

    let digivolution_data_object: ObjectArray<DigivolutionData> = ObjectArray {
        original: digivolution_data_arr,
        modified: digivolution_data_copy,
        index: digivolution_data_index + 0x58 * 8,
        slen: 0x58,
    };

    let shops_object: ObjectArray<Shop> = ObjectArray {
        original: shops_arr.clone(),
        modified: shops_arr.clone(),
        index: shops_index,
        slen: 0x8,
    };

    let shop_items_object: ObjectArray<u16> = ObjectArray {
        original: shop_items_arr.clone(),
        modified: shop_items_arr.clone(),
        index: front_index,
        slen: 0x2,
    };

    let item_shop_data_object: ObjectArray<ItemShopData> = ObjectArray {
        original: item_shop_data_arr.clone(),
        modified: item_shop_data_arr.clone(),
        index: item_shop_data_index,
        slen: 0xc,
    };

    let move_data_object: ObjectArray<MoveData> = ObjectArray {
        original: move_data_arr.clone(),
        modified: move_data_arr.clone(),
        index: move_data_index,
        slen: 0x12,
    };

    let dv_cond_object: ObjectArray<DigivolutionConditions> = ObjectArray {
        original: dv_cond_arr,
        modified: dv_cond_copy,
        index: dv_cond_index,
        slen: 0x2c0,
    };

    let map_objects = read_map_objects(
        path,
        &stage_load_data_arr,
        &stage,
        &file_map,
        &sector_offsets,
        &executable,
    );

    Objects {
        executable,
        file_map,
        stage,
        sector_offsets,
        // overlay_address_pointer: overlay,
        bufs: Bufs {
            encounter_buf,
            stats_buf,
            main_buf,
            shops_buf,
            exp_buf,
            pack_select_buf,
            _map_buf: map_buf,
        },
        enemy_stats: enemy_stats_object,
        encounters: encounters_object,

        parties: parties_object,
        pack_previews: party_previewes_object,

        rookie_data: rookie_data_object,
        digivolution_data: digivolution_data_object,
        shops: shops_object,
        shop_items: shop_items_object,
        item_shop_data: item_shop_data_object,
        move_data: move_data_object,
        dv_cond: dv_cond_object,
        stage_load_data: stage_load_data_arr,
        map_objects,

        text_files,
        items,
    }
}

fn write_map_objects(path: &PathBuf, objects: &mut Vec<MapObject>) -> Result<(), ()> {
    let rom_name = path.file_name().unwrap().to_str().unwrap();

    for object in objects {
        let buf = &mut object.buf;

        if let Some(environmentals) = &mut object.environmentals {
            environmentals.write_buf(buf);
        }

        if let Some(entities) = &mut object.entities {
            entities.write_buf(buf);
        }

        for logic in &mut object.entity_logics {
            logic.write_buf(buf);
        }

        for script in &mut object.scripts {
            script.write_buf(buf);
        }

        if let Some(map_color) = &mut object.map_color {
            map_color.write_buf(buf);
        }

        object.background_file_index.write_buf(buf);

        // write file
        let mut new_file =
            File::create(format!("extract/{}/AAA/PRO/{}", rom_name, object.file_name)).unwrap();

        match new_file.write_all(buf) {
            Err(_) => return Err(()),
            _ => {}
        }
    }

    Ok(())
}

fn write_objects(path: &PathBuf, objects: &mut Objects) -> Result<(), ()> {
    objects.enemy_stats.write_buf(&mut objects.bufs.stats_buf);
    objects
        .encounters
        .write_buf(&mut objects.bufs.encounter_buf);
    objects.parties.write_buf(&mut objects.bufs.main_buf);
    objects.rookie_data.write_buf(&mut objects.bufs.main_buf);
    objects
        .digivolution_data
        .write_buf(&mut objects.bufs.main_buf);
    objects.shop_items.write_buf(&mut objects.bufs.shops_buf);
    objects.shops.write_buf(&mut objects.bufs.shops_buf);
    objects.item_shop_data.write_buf(&mut objects.bufs.main_buf);
    objects.move_data.write_buf(&mut objects.bufs.main_buf);
    objects.dv_cond.write_buf(&mut objects.bufs.exp_buf);
    objects
        .pack_previews
        .write_buf(&mut objects.bufs.pack_select_buf);

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
    let mut new_exp = File::create(format!("extract/{}/{}", rom_name, consts::EXP_FILE)).unwrap();
    let mut new_pack_select =
        File::create(format!("extract/{}/{}", rom_name, consts::PACK_SELECT_FILE)).unwrap();

    for sname in objects.executable.text_files().iter() {
        for lang in objects.executable.languages() {
            let text_file = objects
                .text_files
                .get(*sname)
                .unwrap()
                .files
                .get(lang)
                .unwrap();

            let mut new_file =
                File::create(format!("extract/{}/{}", rom_name, lang.to_path(sname))).unwrap();

            let bytes: Vec<u8> = text_file.file.clone().into();

            match new_file.write_all(&bytes) {
                Err(_) => return Err(()),
                _ => {}
            }
        }
    }

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

    match new_exp.write_all(&objects.bufs.exp_buf) {
        Err(_) => return Err(()),
        _ => {}
    }

    match new_pack_select.write_all(&objects.bufs.pack_select_buf) {
        Err(_) => return Err(()),
        _ => {}
    }

    match write_map_objects(path, &mut objects.map_objects) {
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

    if preset.randomizer.maps.enabled {
        maps::patch(&preset.randomizer, &mut objects, &mut rng);
    }

    match write_objects(path, &mut objects) {
        Err(_) => panic!("Error writing objects"),
        _ => (),
    }
}
