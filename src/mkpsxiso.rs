use quick_xml;
use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs;
use std::process::Command;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct IsoProject {
    track: Vec<Track>,
}

impl IsoProject {
    pub fn flatten(&self) -> Vec<File> {
        let mut result: Vec<File> = Vec::new();
        let data = self.track.iter().find(|t| t.r#type == "data").unwrap();

        for entry in data.directory_tree.field.iter() {
            match entry {
                DirEntry::File(file) => {
                    result.push(file.clone());
                }
                DirEntry::Dir(dir) => rflatten(dir, &mut result),
            }
        }

        return result;
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

fn exists(exec: &str) -> bool {
    let output_res = Command::new(exec).output();

    match output_res {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn find_bin(name: &str) -> Result<String, ()> {
    if exists(name) {
        return Ok(String::from(name));
    }

    let built = format!("mkpsxiso/build/{name}");
    if exists(&built) {
        return Ok(built);
    }

    Err(())
}

pub fn extract(path: &std::path::PathBuf) -> bool {
    let bin_res = find_bin("dumpsxiso");
    if bin_res.is_err() {
        return false;
    }

    Command::new(bin_res.unwrap())
        .arg("-x")
        .arg(format!(
            "extract/{}/",
            path.file_name().unwrap().to_str().unwrap()
        ))
        .arg("-s")
        .arg("extract/out.xml")
        .arg("-pt")
        .arg(&path)
        .output()
        .unwrap()
        .status
        .success()
}

pub fn xml_file() -> IsoProject {
    let xml = fs::read_to_string("extract/out.xml").unwrap();

    from_str(&xml).unwrap()
}

pub fn build(file_name: &str) -> bool {
    let bin_res = find_bin("mkpsxiso");
    if bin_res.is_err() {
        return false;
    }

    let bin = format!("{}.bin", file_name);
    let cue = format!("{}.cue", file_name);

    Command::new(bin_res.unwrap())
        .arg("-o")
        .arg(&bin)
        .arg("-c")
        .arg(&cue)
        .arg("extract/out.xml")
        .arg("-y")
        .output()
        .unwrap()
        .status
        .success()
}
