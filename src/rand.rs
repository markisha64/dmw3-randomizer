use anyhow::Context;
use async_std::fs;
use async_std::fs::File;
use async_std::prelude::*;
use binread::BinRead;
use binwrite::BinWrite;
use dmw3_model::Header;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;

use crate::json::Preset;
use crate::lang::Language;
use crate::mkpsxiso;
use crate::mkpsxiso::xml_file;
use crate::pack::Packed;
pub use dmw3_structs;

mod encounters;
mod fixes;
mod maps;
mod models;
mod parties;
mod scaling;
mod shops;
use dmw3_structs::{
    DigivolutionConditions, DigivolutionData, EncounterData, EnemyStats, EntityData, EntityLogic,
    Environmental, ItemShopData, MapColor, MoveData, PartyData, Pointer, Shop, StageLoadData,
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
    fn write_buf(&self, source_buf: &mut Vec<u8>) -> anyhow::Result<()>;
}

impl<T: BinWrite> WriteObjects for Object<T> {
    fn write_buf(&self, write_buf: &mut Vec<u8>) -> anyhow::Result<()> {
        let mut buf = vec![];

        self.modified.write(&mut buf)?;
        write_buf[self.index..(self.index + self.slen)].copy_from_slice(&mut buf);

        Ok(())
    }
}

impl<T: BinWrite> WriteObjects for ObjectArray<T> {
    fn write_buf(&self, write_buf: &mut Vec<u8>) -> anyhow::Result<()> {
        let mut buf = vec![];

        self.modified.write(&mut buf)?;
        write_buf[self.index..(self.index + self.slen * self.original.len())]
            .copy_from_slice(&mut buf);

        Ok(())
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

pub struct ModelObject {
    packed: dmw3_pack::Packed,
    og_len: usize,
    file_name: String,
    header: Header,
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
    pub enemy_parties: ObjectArray<PartyData>,
    pub rookie_data: ObjectArray<DigivolutionData>,
    pub digivolution_data: ObjectArray<DigivolutionData>,
    pub shops: ObjectArray<Shop>,
    pub shop_items: ObjectArray<u16>,
    pub item_shop_data: ObjectArray<ItemShopData>,
    pub move_data: ObjectArray<MoveData>,
    pub dv_cond: ObjectArray<DigivolutionConditions>,
    pub stage_load_data: Vec<StageLoadData>,
    pub map_objects: Vec<MapObject>,
    pub model_objects: Vec<ModelObject>,

    pub text_files: BTreeMap<String, TextFileGroup>,
    pub items: TextFileGroup,
}

enum Executable {
    PAL,
    USA,
    JAP,
}

impl Executable {
    fn to_model_path(&self) -> &'static str {
        match self {
            Executable::USA => "AAA/DAT/MODEL",
            _ => "AAA/DAT/FIGHT/MODEL",
        }
    }

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
                dmw3_consts::ITEM_NAMES,
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
                dmw3_consts::ITEM_NAMES,
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

async fn read_map_objects(
    path: &PathBuf,
    stage_load_data: &Vec<StageLoadData>,
    stage: &Pointer,
    file_map: &Vec<mkpsxiso::File>,
    sector_offsets: &Vec<u32>,
    executable: &Executable,
) -> anyhow::Result<Vec<MapObject>> {
    let rom_name = path
        .file_name()
        .context("Failed to get file name")?
        .to_str()
        .context("Failed to convert to str")?;

    let mut pro_folder = fs::read_dir(format!("extract/{}/AAA/PRO", rom_name)).await?;

    let mut result: Vec<MapObject> = Vec::new();

    while let Some(file_res) = pro_folder.next().await {
        let file = file_res?;

        if let Ok(file_type) = file.file_type().await {
            if file_type.is_dir() {
                continue;
            }
        }

        let file_name = file.file_name().into_string().unwrap();
        if !file_name.starts_with("WSTAG") || file_name.starts_with("WSTAG9") {
            continue;
        }

        let buf = fs::read(format!("extract/{}/AAA/PRO/{}", rom_name, file_name)).await?;

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

        let idx = init_stage_pointers.to_index_overlay(stage.value) as usize;

        let jr_ra_instruction_index = buf[idx..]
            .windows(4)
            .position(|x| x == dmw3_consts::JR_RA_INSTRUCTION);

        if jr_ra_instruction_index.is_none() {
            continue;
        }

        let jump_return = Pointer {
            value: init_stage_pointers.value + jr_ra_instruction_index.unwrap() as u32,
        };

        let initsp_index = init_stage_pointers.to_index_overlay(stage.value) as usize;
        let initp_end_index = jump_return.to_index_overlay(stage.value) as usize;

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
            .position(|x| x == dmw3_consts::LI_INSTRUCTION);

        if res.is_none() {
            continue;
        }

        let li_instruction = bg_set_idx - res.unwrap() - 2;

        let sector_offset = file_map
            .iter()
            .find(|file| file.name == file_name)
            .context("Failed to find sector offset")?
            .offs;

        let sector_offsets_index = sector_offsets
            .iter()
            .position(|off| *off == sector_offset)
            .context("Failed to get idx")? as u32;

        let stage_load_data_row = stage_load_data
            .iter()
            .find(|sldr| sldr.file_index == sector_offsets_index)
            .context("Failed to get stage load data")?;

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

        if let Some(map_color_offset) = buf[initsp_index..initp_end_index].chunks(4).position(|x| {
            x[0] == dmw3_consts::STAGE_COLOR_INSTRUCTION_HALF[0]
                && x[1] == dmw3_consts::STAGE_COLOR_INSTRUCTION_HALF[1]
                && x[2] != dmw3_consts::LI_INSTRUCTION[0]
                && x[3] != dmw3_consts::LI_INSTRUCTION[1]
        }) {
            if let Some(map_color_addiu) = buf[initsp_index..initsp_index + map_color_offset * 4]
                .windows(4)
                .rev()
                .position(|x| x[3] == dmw3_consts::ADDIU)
            {
                let aidx = initsp_index + map_color_offset * 4 - map_color_addiu - 4;
                let addiu_first_half = i16::from_le_bytes([buf[aidx], buf[aidx + 1]]);

                let address = (0x800a0000 + addiu_first_half as i64) as u32;

                let map_color_address = Pointer { value: address };

                let idx = map_color_address.to_index_overlay(stage.value) as usize;

                let mut map_color_reader = Cursor::new(&buf[idx..idx + 4]);
                let color = MapColor::read(&mut map_color_reader)?;

                map_color = Some(Object {
                    original: color.clone(),
                    modified: color.clone(),
                    index: idx,
                    slen: 0x4,
                });
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
            dmw3_consts::ENVIRONMENTAL_INSTRUCTION[0],
            dmw3_consts::ENVIRONMENTAL_INSTRUCTION[1],
            sw[0],
            sw[1],
        ];

        let entities_instruction = [
            dmw3_consts::ENTITIES_INSTRUCTION[0],
            dmw3_consts::ENTITIES_INSTRUCTION[1],
            sw[0],
            sw[1],
        ];

        let talk_file_instruction = [
            dmw3_consts::TALK_FILE_INSTRUCTION[0],
            dmw3_consts::TALK_FILE_INSTRUCTION[1],
            sw[0],
            sw[1],
        ];

        if let Some(talk_file_set) = buf[initsp_index..initp_end_index]
            .windows(4)
            .position(|x| x == talk_file_instruction)
        {
            let instr = match executable {
                Executable::PAL => dmw3_consts::TALK_FILE_ADDIU,
                _ => dmw3_consts::LI_INSTRUCTION,
            };

            let res = buf[initsp_index..initsp_index + talk_file_set]
                .windows(2)
                .rev()
                .position(|x| x == instr)
                .context("Failed to find instruction")?;

            let instruction = initsp_index + talk_file_set - res - 2;

            talk_file = Some(u16::from_le_bytes([
                buf[instruction - 2],
                buf[instruction - 1],
            ]));
        }

        if let Some(environmental_set) = buf[initsp_index..initp_end_index]
            .chunks(4)
            .position(|x| x == environmental_instruction)
        {
            let environmental_address =
                Pointer::from_instruction(&buf[initsp_index..initsp_index + environmental_set * 4]);

            let env_index = environmental_address.to_index_overlay(stage.value);
            environmentals_index = Some(env_index);

            let mut environmentals_reader = Cursor::new(&buf[(env_index as usize)..]);

            loop {
                let environmental = Environmental::read(&mut environmentals_reader)?;

                if environmental.conditions[0] == 0x0000ffff
                    && environmental.conditions[1] == 0x0000ffff
                    && environmental.next_stage_id == 0
                {
                    break;
                }

                environmentals.push(environmental);
            }
        }

        if let Some(entities_set) = buf[initsp_index..initp_end_index]
            .chunks(4)
            .position(|x| x == entities_instruction)
        {
            let entities_address =
                Pointer::from_instruction(&buf[initsp_index..initsp_index + entities_set * 4]);

            if entities_address.is_valid() {
                let ent_index = entities_address.to_index_overlay(stage.value) as usize;

                let mut i = 0;
                loop {
                    let ptr = Pointer::from(&buf[ent_index + i * 4..ent_index + (i + 1) * 4]);

                    if ptr.null() {
                        break;
                    }

                    i += 1;
                }

                let real_pointer = Pointer::from(&buf[ent_index..ent_index + 4]);

                let real_idx = real_pointer.to_index_overlay(stage.value);
                entities_index = Some(real_idx);

                let mut entities_reader =
                    Cursor::new(&buf[(real_idx as usize)..(real_idx as usize) + 0x14 * i]);

                for _ in 0..i {
                    let entity = EntityData::read(&mut entities_reader)?;

                    if !entity.logic.null() {
                        let logic_idx = entity.logic.to_index_overlay(stage.value);

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
                                let script_idx = logic.script.to_index_overlay(stage.value);

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

        let environmental_object = environmentals_index.map(|idx| ObjectArray {
            original: environmentals.clone(),
            modified: environmentals.clone(),
            index: idx as usize,
            slen: 0x18,
        });

        let entities_object = entities_index.map(|idx| ObjectArray {
            original: entities.clone(),
            modified: entities.clone(),
            index: idx as usize,
            slen: 0x14,
        });

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

    Ok(result)
}

async fn write_model_objects(
    path: &PathBuf,
    objects: &Vec<ModelObject>,
    executable: &Executable,
) -> anyhow::Result<()> {
    let rom_name = path
        .file_name()
        .context("Failed to get file name")?
        .to_str()
        .context("Failed to convert to str")?;

    for model in objects {
        if model.packed.buffer.len()
            > ((model.og_len / 2048) + (model.og_len % 2048 != 0) as usize) * 2048
        {
            continue;
        }

        let mut new_model = File::create(format!(
            "extract/{}/{}/{}",
            rom_name,
            executable.to_model_path(),
            model.file_name,
        ))
        .await?;

        new_model.write_all(&model.packed.buffer).await?;
    }

    Ok(())
}

async fn read_model_objects(
    path: &PathBuf,
    executable: &Executable,
) -> anyhow::Result<Vec<ModelObject>> {
    let rom_name = path
        .file_name()
        .context("Failed to get file name")?
        .to_str()
        .context("Failed to convert to str")?;

    let mut model_itr = fs::read_dir(format!(
        "extract/{}/{}/",
        rom_name,
        executable.to_model_path()
    ))
    .await?;

    let mut r = Vec::new();

    while let Some(modelr) = model_itr.next().await {
        let model = modelr?;

        let model_buf = fs::read(model.path()).await?;

        let packed = dmw3_pack::Packed::try_from(model_buf)?;

        let header_buf = match packed.iter().rev().find(|idx| {
            if packed.assumed_length[*idx] < 8 {
                return false;
            }

            let s = match packed.get_file(*idx) {
                Ok(f) => f,
                Err(_) => return false,
            };

            let len = u32::from_le_bytes([s[4], s[5], s[6], s[7]]) as usize;

            if (8 + len * 12) != packed.assumed_length[*idx] {
                return false;
            }

            let header = match Header::read(&mut Cursor::new(s)) {
                Ok(s) => s,
                Err(_) => return false,
            };

            match header
                .parts
                .iter()
                .find(|x| x.parent_index >= header._part_count)
            {
                Some(_) => false,
                None => true,
            }
        }) {
            Some(h) => h,
            _ => continue,
        };

        let file_name = String::from(
            model
                .path()
                .file_name()
                .context("Failed to get file name")?
                .to_str()
                .context("Failed to convert to str")?,
        );

        let header = Header::read(&mut Cursor::new(&packed.get_file(header_buf)?))?;

        r.push(ModelObject {
            og_len: packed.buffer.len(),
            packed,
            file_name,
            header,
        })
    }

    Ok(r)
}

pub async fn read_objects(path: &PathBuf) -> anyhow::Result<Objects> {
    let rom_name = path
        .file_name()
        .context("Failed file name get")?
        .to_str()
        .context("Failed to_str conversion")?;
    let iso_project = xml_file().await?;

    let mut itr = fs::read_dir(format!("extract/{}/", rom_name)).await?;

    let mut executable_opt = None;
    while let Some(x) = itr.next().await {
        let dir_entry = x?;

        match Executable::from_str(dir_entry.file_name().into_string().unwrap().as_str()) {
            Some(exec) => {
                executable_opt = Some(exec);
                break;
            }
            None => {}
        }
    }

    let executable = executable_opt.context("Can't find extracted executable")?;

    let stats_buf = fs::read(format!("extract/{}/{}", rom_name, dmw3_consts::STATS_FILE)).await?;

    let encounter_buf = fs::read(format!(
        "extract/{}/{}",
        rom_name,
        dmw3_consts::ENCOUNTERS_FILE
    ))
    .await?;

    let main_buf = fs::read(format!("extract/{}/{}", rom_name, executable.as_str())).await?;
    let shops_buf = fs::read(format!("extract/{}/{}", rom_name, dmw3_consts::SHOPS_FILE)).await?;
    let exp_buf = fs::read(format!("extract/{}/{}", rom_name, dmw3_consts::EXP_FILE)).await?;
    let map_buf = fs::read(format!("extract/{}/{}", rom_name, dmw3_consts::MAP_FILE)).await?;
    let pack_select_buf = fs::read(format!(
        "extract/{}/{}",
        rom_name,
        dmw3_consts::PACK_SELECT_FILE
    ))
    .await?;

    let overlay_address = Pointer {
        value: dmw3_consts::OVERLAY_ADDRESS,
    };

    let overlay = Pointer::from(
        &main_buf[overlay_address.to_index() as usize..overlay_address.to_index() as usize + 4],
    );

    let stage_address = Pointer {
        value: dmw3_consts::STAGE_ADDRESS,
    };

    let stage = Pointer::from(
        &main_buf[stage_address.to_index() as usize..stage_address.to_index() as usize + 4],
    );

    let file_map = iso_project.flatten()?;

    let mut sector_offsets: Vec<u32> = Vec::new();
    sector_offsets.reserve(file_map.len());

    let sector_offsets_index = executable.to_sector_offsets_address().to_index() as usize;

    for i in 0..file_map.len() {
        sector_offsets.push(u32::from_le_bytes(
            main_buf[sector_offsets_index + i * 4..sector_offsets_index + i * 4 + 4].try_into()?,
        ));
    }

    let mut enemy_stats_reader = Cursor::new(&stats_buf);

    let mut enemy_stats_arr: Vec<dmw3_structs::EnemyStats> = Vec::new();

    loop {
        let stats = EnemyStats::read(&mut enemy_stats_reader)?;

        if stats.digimon_id == 0 {
            break;
        }

        enemy_stats_arr.push(stats);
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
        let encounter = EncounterData::read(&mut encounter_data_reader)?;

        // this check works because after Encounter array
        // goes Team array which starts with a non null ptr
        if encounter.digimon_id > 500 {
            break;
        }

        encounter_data_arr.push(encounter);
    }

    let mut enemy_party_data_arr = Vec::new();
    enemy_party_data_arr.reserve(335);

    let enemy_party_data_index = encounter_data_index + 0xc * encounter_data_arr.len();

    let mut enemy_party_reader = Cursor::new(&encounter_buf[enemy_party_data_index..]);

    for _ in 0..335 {
        enemy_party_data_arr.push(PartyData::read(&mut enemy_party_reader)?);
    }

    let shops_index = shops_buf
        .windows(8)
        .position(|window| window == b"\x00\x00\x00\x00\x0b\x00\x00\x00")
        .context("Can't find shops beginning")?
        + 4;

    let mut shops_reader = Cursor::new(&shops_buf[shops_index..]);

    let mut shops_arr: Vec<Shop> = Vec::new();
    shops_arr.reserve(dmw3_consts::SHOPS_LEN);

    for _ in 0..dmw3_consts::SHOPS_LEN {
        let shop = Shop::read(&mut shops_reader)?;

        shops_arr.push(shop);
    }

    let front_index = shops_arr
        .first()
        .context("No shops found")?
        .items
        .to_index_overlay(overlay.value as u32) as usize;

    let back_shop = shops_arr.last().context("No shops found")?;

    let back_index =
        (back_shop.item_count + back_shop.items.to_index_overlay(overlay.value as u32)) as usize;

    let shop_items_arr: Vec<u16> = shops_buf[front_index..back_index]
        .to_vec()
        .chunks_exact(2)
        .map(|a| u16::from_ne_bytes([a[0], a[1]]))
        .collect();

    let item_shop_data_index = main_buf
        .windows(12)
        .position(|window| -> bool {
            window == b"\x64\x01\x65\x01\x66\x01\x67\x01\x00\x00\x00\x00"
        })
        .context("Can't find item shop data beginning")?;

    let mut item_shop_data_reader = Cursor::new(&main_buf[item_shop_data_index..]);

    let mut item_shop_data_arr: Vec<ItemShopData> = Vec::new();
    item_shop_data_arr.reserve(403);

    for _ in 0..403 {
        let item_shop_data = ItemShopData::read(&mut item_shop_data_reader)?;

        item_shop_data_arr.push(item_shop_data);
    }

    let digivolution_data_index = main_buf
        .windows(16)
        .position(|window| -> bool {
            window == b"\x7f\x01\x30\x00\x2c\x00\x29\x00\x22\x00\x21\x00\x01\x00\x55\x00"
        })
        .context("Can't find digivolution data beginning")?;

    let mut digivolution_data_reader = Cursor::new(&main_buf[digivolution_data_index..]);

    let mut rookie_data_arr: Vec<DigivolutionData> = Vec::new();
    rookie_data_arr.reserve(dmw3_consts::ROOKIE_COUNT);

    for _ in 0..dmw3_consts::ROOKIE_COUNT {
        let rookie_data = DigivolutionData::read(&mut digivolution_data_reader)?;

        rookie_data_arr.push(rookie_data);
    }

    let mut digivolution_data_arr: Vec<DigivolutionData> = Vec::new();
    digivolution_data_arr.reserve(dmw3_consts::DIGIVOLUTION_COUNT);

    for _ in 0..dmw3_consts::DIGIVOLUTION_COUNT {
        let digivolution_data = DigivolutionData::read(&mut digivolution_data_reader)?;

        digivolution_data_arr.push(digivolution_data);
    }

    let mut move_data_arr: Vec<MoveData> = Vec::new();
    move_data_arr.reserve(444);

    let move_data_index = main_buf
        .windows(18)
        .position(|window| -> bool {
            window == b"\x00\x00\x3c\x00\x02\x02\x6e\x01\x01\x01\x01\x01\x01\x00\x02\x39\x05\x01"
        })
        .context("Can't find move data beginning")?;

    let mut move_data_reader = Cursor::new(&main_buf[move_data_index..]);

    // hardcoding this for now
    for _ in 0..443 {
        let move_data = MoveData::read(&mut move_data_reader)?;

        move_data_arr.push(move_data);
    }

    let parties_index = main_buf
        .windows(9)
        .position(|window| window == dmw3_consts::PACKS)
        .context("Can't find parties")?;

    let default_packs: Vec<u32> = dmw3_consts::PACKS.iter().map(|x| *x as u32).collect();
    let mut default_pack_preview: Vec<u8> = Vec::new();
    for mon in dmw3_consts::PACKS {
        // yes this is ugly, no I don't care
        default_pack_preview.push(*mon);
        default_pack_preview.extend(b"\x00\x00\x00");
    }

    let pack_select_preview_index = pack_select_buf
        .windows(36)
        .position(|window| window == default_pack_preview)
        .context("Can't find parties preview")?;

    let mut dv_cond_arr: Vec<DigivolutionConditions> = Vec::new();
    dv_cond_arr.reserve(dmw3_consts::ROOKIE_COUNT);

    let dv_cond_index = exp_buf
        .windows(8)
        .position(|x| x == b"\x09\x00\x00\x00\x00\x00\x01\x00")
        .context("Can't find DV conditions beginning")?;

    let mut dv_cond_reader = Cursor::new(&exp_buf[dv_cond_index..]);

    for _ in 0..dmw3_consts::ROOKIE_COUNT {
        let dv_cond = DigivolutionConditions::read(&mut dv_cond_reader)?;

        dv_cond_arr.push(dv_cond);
    }

    let stage_load_data_index = executable
        .to_stage_load_data_address()
        .to_index_overlay(overlay.value as u32) as usize;

    let mut stage_load_data_arr: Vec<StageLoadData> = Vec::new();
    stage_load_data_arr.reserve(dmw3_consts::STAGE_LOAD_DATA_LENGTH);

    let mut stage_load_data_reader = Cursor::new(&map_buf[stage_load_data_index..]);

    for _ in 0..dmw3_consts::STAGE_LOAD_DATA_LENGTH {
        let stage_load_data = StageLoadData::read(&mut stage_load_data_reader)?;

        stage_load_data_arr.push(stage_load_data);
    }

    let mut item_files: HashMap<Language, TextFile> = HashMap::new();

    for lang in executable.languages() {
        let fsname = lang.to_file_name(dmw3_consts::ITEM_NAMES);

        let file = fs::read(format!(
            "extract/{}/{}",
            rom_name,
            lang.to_path(dmw3_consts::ITEM_NAMES)
        ))
        .await?;

        let packed = Packed::from_text(file);

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

            let file = fs::read(format!("extract/{}/{}", rom_name, lang.to_path(sname))).await?;

            let packed = Packed::from_text(file);

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
    let enemy_party_data_arr_copy = enemy_party_data_arr.clone();
    let rookie_data_copy = rookie_data_arr.clone();
    let digivolution_data_copy = digivolution_data_arr.clone();
    let dv_cond_copy = dv_cond_arr.clone();

    let enemy_stats_object = ObjectArray {
        original: enemy_stats_arr,
        modified: enemy_stats_arr_copy,
        index: 0,
        slen: 0x46,
    };

    let encounters_object = ObjectArray {
        original: encounter_data_arr,
        modified: encounter_data_arr_copy,
        index: encounter_data_index,
        slen: 0xc,
    };

    let enemy_parties_object = ObjectArray {
        original: enemy_party_data_arr,
        modified: enemy_party_data_arr_copy,
        index: enemy_party_data_index,
        slen: 0x1c,
    };

    let parties_object: ObjectArray<u8> = ObjectArray {
        original: dmw3_consts::PACKS.into(),
        modified: dmw3_consts::PACKS.into(),
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
    )
    .await?;

    let model_objects = read_model_objects(path, &executable).await?;

    Ok(Objects {
        executable,
        file_map,
        stage,
        model_objects,
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
        enemy_parties: enemy_parties_object,

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
    })
}

async fn write_map_objects(path: &PathBuf, objects: &mut Vec<MapObject>) -> anyhow::Result<()> {
    let rom_name = path
        .file_name()
        .context("Failed to get file name")?
        .to_str()
        .context("Failed to convert to str")?;

    for object in objects {
        let buf = &mut object.buf;

        if let Some(environmentals) = &mut object.environmentals {
            environmentals.write_buf(buf)?;
        }

        if let Some(entities) = &mut object.entities {
            entities.write_buf(buf)?;
        }

        for logic in &mut object.entity_logics {
            logic.write_buf(buf)?;
        }

        for script in &mut object.scripts {
            script.write_buf(buf)?;
        }

        if let Some(map_color) = &mut object.map_color {
            map_color.write_buf(buf)?;
        }

        object.background_file_index.write_buf(buf)?;

        // write file
        let mut new_file =
            File::create(format!("extract/{}/AAA/PRO/{}", rom_name, object.file_name)).await?;

        new_file.write_all(buf).await?;
    }

    Ok(())
}

async fn write_objects(path: &PathBuf, objects: &mut Objects) -> anyhow::Result<()> {
    objects.enemy_stats.write_buf(&mut objects.bufs.stats_buf)?;
    objects
        .encounters
        .write_buf(&mut objects.bufs.encounter_buf)?;
    objects.parties.write_buf(&mut objects.bufs.main_buf)?;
    objects.rookie_data.write_buf(&mut objects.bufs.main_buf)?;
    objects
        .digivolution_data
        .write_buf(&mut objects.bufs.main_buf)?;
    objects.shop_items.write_buf(&mut objects.bufs.shops_buf)?;
    objects.shops.write_buf(&mut objects.bufs.shops_buf)?;
    objects
        .item_shop_data
        .write_buf(&mut objects.bufs.main_buf)?;
    objects.move_data.write_buf(&mut objects.bufs.main_buf)?;
    objects.dv_cond.write_buf(&mut objects.bufs.exp_buf)?;
    objects
        .pack_previews
        .write_buf(&mut objects.bufs.pack_select_buf)?;

    let rom_name = path
        .file_name()
        .context("Failed to get file name")?
        .to_str()
        .context("Failed to convert to str")?;

    let mut new_main_executable = File::create(format!(
        "extract/{}/{}",
        rom_name,
        objects.executable.as_str()
    ))
    .await?;

    let mut new_stats =
        File::create(format!("extract/{}/{}", rom_name, dmw3_consts::STATS_FILE)).await?;

    let mut new_encounters = File::create(format!(
        "extract/{}/{}",
        rom_name,
        dmw3_consts::ENCOUNTERS_FILE
    ))
    .await?;

    let mut new_shops =
        File::create(format!("extract/{}/{}", rom_name, dmw3_consts::SHOPS_FILE)).await?;

    let mut new_exp =
        File::create(format!("extract/{}/{}", rom_name, dmw3_consts::EXP_FILE)).await?;

    let mut new_pack_select = File::create(format!(
        "extract/{}/{}",
        rom_name,
        dmw3_consts::PACK_SELECT_FILE
    ))
    .await?;

    for sname in objects.executable.text_files().iter() {
        for lang in objects.executable.languages() {
            let text_file = objects
                .text_files
                .get(*sname)
                .context("Failed to get text file")?
                .files
                .get(lang)
                .context("Failed to get language")?;

            let mut new_file =
                File::create(format!("extract/{}/{}", rom_name, lang.to_path(sname))).await?;

            let bytes: Vec<u8> = text_file.file.clone().into();

            new_file.write_all(&bytes).await?
        }
    }

    new_main_executable
        .write_all(&objects.bufs.main_buf)
        .await?;

    new_stats.write_all(&objects.bufs.stats_buf).await?;

    new_encounters
        .write_all(&objects.bufs.encounter_buf)
        .await?;

    new_shops.write_all(&objects.bufs.shops_buf).await?;

    new_exp.write_all(&objects.bufs.exp_buf).await?;

    new_pack_select
        .write_all(&objects.bufs.pack_select_buf)
        .await?;

    write_map_objects(path, &mut objects.map_objects).await?;

    write_model_objects(path, &objects.model_objects, &objects.executable).await?;

    Ok(())
}

pub async fn patch(path: &PathBuf, preset: &Preset) -> anyhow::Result<Objects> {
    let mut objects = read_objects(path).await.unwrap();

    let mut rng = Xoshiro256StarStar::seed_from_u64(preset.randomizer.seed);

    if preset.randomizer.encounters.enabled {
        encounters::patch(&preset.randomizer, &mut objects, &mut rng)?;
    }

    if preset.randomizer.parties.enabled {
        parties::patch(&preset.randomizer, &mut objects, &mut rng)?;
    }

    if preset.scaling.enabled {
        scaling::patch(&preset.scaling, &mut objects, &mut rng)?;
    }

    if preset.fixes.scaling {
        fixes::scaling(&mut objects);
    }

    if preset.randomizer.shops.enabled {
        shops::patch(&preset.randomizer.shops, &mut objects, &mut rng)?;
    }

    if preset.randomizer.maps.enabled {
        maps::patch(&preset.randomizer, &mut objects, &mut rng)?;
    }

    if preset.randomizer.models.enabled {
        models::patch(&preset.randomizer, &mut objects, &mut rng)?;
    }

    write_objects(path, &mut objects).await?;

    Ok(objects)
}
