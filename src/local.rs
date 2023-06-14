use crate::constant::*;
use crate::util::{download_file, get_home, unzip_file};

use std::fs::{self, File};
use std::io::{Error, Read, Result, Write};
use std::path::{Path, PathBuf};
use std::time;

use colored::Colorize;

pub fn check_localtime() -> Result<u64> {
    let mut path = get_home()?;
    path.push(TLDR_HOME_DIR);
    path.push(TIME_FILE);

    let mut time_file = File::open(&path)?;
    let mut old_time = String::new();
    time_file.read_to_string(&mut old_time)?;
    let old_time = match old_time.parse::<u64>() {
        Ok(old_time) => old_time,
        Err(_) => return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid time")),
    };

    let cur_time = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let diff_time = cur_time - old_time;
    if diff_time > 60 * 60 * 24 * 7 * 2 {
        // older than 2 weeks
        println!(
            "{}",
            "Local data is older than two weeks, use --update to update it."
                .bold()
                .red()
        );
    }

    Ok(diff_time)
}

pub fn update_localtime() -> Result<()> {
    let mut path = get_home()?;
    path.push(TLDR_HOME_DIR);
    let dir = path.clone();
    path.push(TIME_FILE);

    let mut time_file: File;
    match File::create(&path) {
        Ok(file) => time_file = file,
        Err(_) => {
            fs::create_dir(dir)?;
            time_file = File::create(&path)?;
        }
    }

    let cur_time = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    time_file.write_all(cur_time.to_string().as_bytes())?;

    Ok(())
}

pub fn has_localdb() -> Result<bool> {
    let mut path = get_home()?;
    path.push(TLDR_HOME_DIR);
    Ok(path.exists())
}

pub fn update_localdb() -> Result<()> {
    let mut dir = get_home()?;
    dir.push(TLDR_HOME_DIR);
    match fs::create_dir(&dir) {
        Err(err) if err.kind() != std::io::ErrorKind::AlreadyExists => return Err(err),
        _ => (),
    };

    let mut tmp_path = PathBuf::from(TMP_DIR);
    let tmp_dir = tmp_path.clone();
    match fs::remove_dir_all(&tmp_dir) {
        Err(err) if err.kind() != std::io::ErrorKind::NotFound => return Err(err),
        _ => (),
    }
    fs::create_dir(&tmp_dir)?;

    tmp_path.push(TMP_FILE);

    println!("{}", "Downloading...".bold().yellow());
    download_file(ZIP_URL, &tmp_path)?;
    println!("{}", "Successfully downloaded.".bold().green());

    println!("{}", "Unzipping...".bold().yellow());
    fs::remove_dir_all(&dir)?;
    unzip_file(&tmp_path, &dir)?;
    println!("{}", "Done.".bold().green());

    let mut old_dir = dir.clone();
    old_dir.push(ZIP_EXTRACTED_DIR);
    dir.push(DB_DIR);
    fs::rename(old_dir, dir)?;

    fs::remove_dir_all(tmp_dir)?;
    update_localtime()?;

    Ok(())
}

pub fn clear_localdb() -> Result<()> {
    let mut path = get_home()?;
    path.push(TLDR_HOME_DIR);
    fs::remove_dir_all(path)?;
    println!("{}", "Local data cleared.".bold().green());

    Ok(())
}

pub fn get_file_content<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
