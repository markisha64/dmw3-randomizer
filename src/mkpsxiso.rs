use std::process::Command;
use std::process::Output;

use super::cli::Arguments;

pub fn extract(args: &Arguments) -> std::io::Result<Output> {
    Command::new("dumpsxiso")
        .arg("-x")
        .arg("extract/")
        .arg("-s")
        .arg("out.xml")
        .arg("-pt")
        .arg(&args.path)
        .output()
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
