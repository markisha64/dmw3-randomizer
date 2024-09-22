use async_std::fs;
use quick_xml::de::from_str;
use serde::Deserialize;
use tokio::process::Command;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct IsoProject {
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct Track {
    #[serde(rename = "@type")]
    r#type: String,
    directory_tree: DirectoryTree,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum DirEntry {
    Dir(Dir),
    File(File),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct DirectoryTree {
    #[serde(rename = "$value")]
    field: Vec<DirEntry>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct Dir {
    #[serde(rename = "$value")]
    field: Vec<DirEntry>,
    #[serde(rename = "@name")]
    _name: String,
    #[serde(rename = "@source")]
    _source: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct File {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@source")]
    _source: String,
    #[serde(rename = "@offs")]
    pub offs: u32,
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

pub async fn extract(path: &std::path::PathBuf) -> anyhow::Result<bool> {
    let bin = find_bin("dumpsxiso").await?;

    Ok(Command::new(bin)
        .arg("-x")
        .arg(format!(
            "extract/{}/",
            path.file_name().unwrap().to_str().unwrap()
        ))
        .arg("-s")
        .arg("extract/out.xml")
        .arg("-pt")
        .arg(path)
        .output()
        .await?
        .status
        .success())
}

pub async fn xml_file() -> anyhow::Result<IsoProject> {
    let xml = fs::read_to_string("extract/out.xml").await?;

    Ok(from_str(&xml)?)
}

pub async fn build(rom_name: &str, file_name: &str) -> anyhow::Result<bool> {
    let binf = find_bin("mkpsxiso").await?;

    let bin = format!("randomized/{}/{}/new.bin", rom_name, file_name);
    let cue = format!("randomized/{}/{}/new.cue", rom_name, file_name);

    Ok(Command::new(binf)
        .arg("-o")
        .arg(&bin)
        .arg("-c")
        .arg(&cue)
        .arg("extract/out.xml")
        .arg("-y")
        .output()
        .await?
        .status
        .success())
}
