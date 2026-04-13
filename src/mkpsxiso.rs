use anyhow::{anyhow, Context};
use async_std::fs;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case", rename = "iso_project")]
pub struct IsoProject {
    #[serde(rename = "@image_name")]
    pub image_name: String,
    #[serde(rename = "@cue_sheet")]
    pub cue_sheet: String,
    track: Vec<Track>,
}

impl IsoProject {
    pub fn flatten(&self) -> anyhow::Result<Vec<File>> {
        let mut result: Vec<File> = Vec::new();
        let data = self
            .track
            .iter()
            .find(|t| t.r#type == "data")
            .ok_or(anyhow::anyhow!("Missing data track"))?;

        for entry in data.directory_tree.field.iter() {
            match entry {
                DirEntry::File(file) => {
                    result.push(file.clone());
                }
                DirEntry::Dir(dir) => rflatten(dir, &mut result),
            }
        }

        Ok(result)
    }

    pub fn remove_offsets(&mut self) -> anyhow::Result<()> {
        let data = self
            .track
            .iter_mut()
            .find(|t| t.r#type == "data")
            .ok_or(anyhow::anyhow!("Missing data track"))?;

        for entry in data.directory_tree.field.iter_mut() {
            match entry {
                DirEntry::File(file) => {
                    file.offs = None;
                }
                DirEntry::Dir(dir) => rremove_offsets(dir),
            }
        }

        Ok(())
    }
}

fn rflatten(dir: &Dir, result: &mut Vec<File>) {
    for entry in dir.field.iter() {
        match entry {
            DirEntry::File(file) => {
                result.push(file.clone());
            }
            DirEntry::Dir(dir) => rflatten(dir, result),
        }
    }
}

fn rremove_offsets(dir: &mut Dir) {
    for entry in dir.field.iter_mut() {
        match entry {
            DirEntry::File(file) => {
                file.offs = None;
            }
            DirEntry::Dir(dir) => rremove_offsets(dir),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
struct Track {
    #[serde(rename = "@type")]
    r#type: String,
    directory_tree: DirectoryTree,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifiers: Option<Identifiers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<License>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_attributes: Option<DefaultAttributes>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
enum DirEntry {
    Dir(Dir),
    File(File),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
struct DirectoryTree {
    #[serde(rename = "$value")]
    field: Vec<DirEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Identifiers {
    #[serde(rename = "@system")]
    pub system: String,

    #[serde(rename = "@application")]
    pub application: String,

    #[serde(rename = "@volume")]
    pub volume: String,

    #[serde(rename = "@publisher")]
    pub publisher: String,

    #[serde(rename = "@data_preparer")]
    pub data_preparer: String,

    #[serde(rename = "@copyright")]
    pub copyright: String,

    #[serde(rename = "@creation_date")]
    pub creation_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct License {
    #[serde(rename = "@file")]
    pub file: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefaultAttributes {
    #[serde(rename = "@gmt_offs")]
    pub gmt_offs: i32,

    #[serde(rename = "@xa_attrib")]
    pub xa_attrib: i32,

    #[serde(rename = "@xa_perm")]
    pub xa_perm: i32,

    #[serde(rename = "@xa_gid")]
    pub xa_gid: i32,

    #[serde(rename = "@xa_uid")]
    pub xa_uid: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
struct Dir {
    #[serde(rename = "$value")]
    field: Vec<DirEntry>,
    #[serde(rename = "@name")]
    _name: String,
    #[serde(rename = "@source")]
    _source: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct File {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@offs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offs: Option<u32>,
    #[serde(rename = "@type")]
    _type: String,
}

async fn exists(exec: &str) -> anyhow::Result<bool> {
    Ok(Command::new(exec).output().await?.status.success())
}

async fn find_bin(name: &str) -> anyhow::Result<String> {
    if exists(name).await? {
        return Ok(String::from(name));
    }

    let built = format!("mkpsxiso/build/{name}");
    if exists(&built).await? {
        return Ok(built);
    }

    Err(anyhow::anyhow!("Can't find bin"))
}

pub async fn extract(path: &std::path::PathBuf) -> anyhow::Result<()> {
    let bin = find_bin("dumpsxiso").await?;

    let file_name = path
        .file_name()
        .context("failed to get file_name")?
        .to_str()
        .context("failed to convert file name to string")?;

    let output = Command::new(bin)
        .arg("-x")
        .arg(format!("extract/{}/", file_name))
        .arg("-s")
        .arg("extract/out.xml")
        .arg("-pt")
        .arg(path)
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!(String::from_utf8_lossy(&output.stdout).to_string()));
    }

    Ok(())
}

pub enum Entry {
    File {
        name: String,
        length: u32,
        lba: u32,
        _timecode: String,
        _bytes: u64,
        _source: String,
    },
    Dir {
        _name: String,
        _lba: u32,
        _timecode: String,
    },
    Xa {
        name: String,
        length: u32,
        lba: u32,
        _timecode: String,
        _bytes: u64,
        _source: String,
    },
    DirEnd(String),
}

pub struct LbaLog {
    pub bin_file: String,
    pub cue_file: String,
    pub entries: Vec<Entry>,
}

async fn parse_lba_log() -> anyhow::Result<LbaLog> {
    let content = fs::read_to_string("extract/lba.txt").await?;
    let mut log = LbaLog {
        bin_file: String::new(),
        cue_file: String::new(),
        entries: Vec::new(),
    };

    for line in content.lines() {
        let trimmed = line.trim();

        // Parse header metadata
        if let Some(val) = trimmed.strip_prefix("Image bin file:") {
            log.bin_file = val.trim().to_string();
            continue;
        }
        if let Some(val) = trimmed.strip_prefix("Image cue file:") {
            log.cue_file = val.trim().to_string();
            continue;
        }

        let mut cols = trimmed.split_whitespace();
        let entry_type = match cols.next() {
            Some(t) => t,
            None => continue,
        };

        match entry_type {
            "File" | "XA" => {
                // Columns: Type  Name  Length  LBA  Timecode  Bytes  SourceFile
                let name = match cols.next() {
                    Some(v) => v.to_string(),
                    None => continue,
                };
                let length: u32 = match cols.next().and_then(|v| v.parse().ok()) {
                    Some(v) => v,
                    None => continue,
                };
                let lba: u32 = match cols.next().and_then(|v| v.parse().ok()) {
                    Some(v) => v,
                    None => continue,
                };
                let timecode = match cols.next() {
                    Some(v) => v.to_string(),
                    None => continue,
                };
                let bytes: u64 = match cols.next().and_then(|v| v.parse().ok()) {
                    Some(v) => v,
                    None => continue,
                };
                let source = cols.next().unwrap_or("").to_string();

                if entry_type == "XA" {
                    log.entries.push(Entry::Xa {
                        name,
                        length,
                        lba,
                        _timecode: timecode,
                        _bytes: bytes,
                        _source: source,
                    });
                } else {
                    log.entries.push(Entry::File {
                        name,
                        length,
                        lba,
                        _timecode: timecode,
                        _bytes: bytes,
                        _source: source,
                    });
                }
            }
            "Dir" => {
                // Columns: Type  Name  (optional: LBA  Timecode)
                let name = match cols.next() {
                    Some(v) => v.to_string(),
                    None => continue,
                };
                let lba: u32 = cols.next().and_then(|v| v.parse().ok()).unwrap_or(0);
                let timecode = cols.next().unwrap_or("").to_string();
                log.entries.push(Entry::Dir {
                    _name: name,
                    _lba: lba,
                    _timecode: timecode,
                });
            }
            "End" => {
                let name = cols.next().unwrap_or("").to_string();
                log.entries.push(Entry::DirEnd(name));
            }
            _ => {}
        }
    }

    Ok(log)
}

pub async fn xml_file() -> anyhow::Result<IsoProject> {
    let xml = fs::read_to_string("extract/out.xml").await?;

    Ok(from_str(&xml)?)
}

pub async fn get_lba() -> anyhow::Result<LbaLog> {
    let binf = find_bin("mkpsxiso").await?;

    let output = Command::new(binf)
        .arg("extract/new.xml")
        .arg("-y")
        .arg("-lba")
        .arg("extract/lba.txt")
        .arg("-noisogen")
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!(String::from_utf8_lossy(&output.stdout).to_string()));
    }

    parse_lba_log().await
}

pub async fn build(rom_name: &str, file_name: &str) -> anyhow::Result<()> {
    let binf = find_bin("mkpsxiso").await?;

    let bin = format!("randomized/{}/{}/new.bin", rom_name, file_name);
    let cue = format!("randomized/{}/{}/new.cue", rom_name, file_name);

    let output = Command::new(binf)
        .arg("-o")
        .arg(&bin)
        .arg("-c")
        .arg(&cue)
        .arg("extract/new.xml")
        .arg("-y")
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!(String::from_utf8_lossy(&output.stdout).to_string()));
    }

    Ok(())
}
