use std::path::Path;
use std::process::Command;
use std::process::Output;

pub fn extract(path: &std::path::PathBuf) -> Result<(), ()> {
    let res = Command::new("dumpsxiso")
        .arg("-x")
        .arg("extract/")
        .arg("-s")
        .arg("out.xml")
        .arg("-pt")
        .arg(&path)
        .output();

    match res {
        Ok(_) => {
            if !Path::new("./out.xml").exists() {
                return Err(());
            }

            Ok(())
        }
        Err(_) => Err(()),
    }
}

pub fn build(bin: &str, cue: &str) -> std::io::Result<Output> {
    Command::new("mkpsxiso")
        .arg("-o")
        .arg(&bin)
        .arg("-c")
        .arg(&cue)
        .arg("./out.xml")
        .arg("-y")
        .output()
}
