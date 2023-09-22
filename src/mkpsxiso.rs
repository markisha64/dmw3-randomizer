use std::process::Command;

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
