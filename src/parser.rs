use crate::constant::*;
use crate::local::get_file_content;
use crate::util::{get_home, get_platform};

use std::io::{Error, Result};
use std::path::{Path, PathBuf};

use colored::Colorize;

enum Color {
    Red,
    Blue,
}

impl Color {
    fn change_self(&mut self) {
        match self {
            Color::Red => *self = Color::Blue,
            Color::Blue => *self = Color::Red,
        }
    }
}

// pub fn construct_url(item: &str, platform: &str) -> String {
//     format!("{BASE_URL}/{platform}/{item}.md")
// }

pub fn construct_path(item: Option<&str>, platform: &str) -> Result<PathBuf> {
    let mut path = get_home()?;
    path.push(TLDR_HOME_DIR);
    path.push(DB_DIR);
    path.push(PAGES_DIR_ENG);
    path.push(platform);
    if let Some(item) = item {
        path.push(format!("{item}.md"));
    }
    Ok(path)
}

pub fn parse_tldrpage(input: &str) -> Result<()> {
    println!();

    for line in input.split('\n').collect::<Vec<_>>() {
        match line.bytes().next() {
            Some(b'#') => println!("{}", line[2..].bold()),
            Some(b'>') => println!("{}", &line[2..]),
            Some(b'-') => print!("{}", line.green()),
            Some(b'`') => {
                let line = line
                    .strip_prefix('`')
                    .unwrap()
                    .strip_suffix('`')
                    .ok_or(Error::new(std::io::ErrorKind::InvalidData, "Invalid line"))?;

                let line_buf: Vec<_> = line.split("{{").collect();
                let mut line = Vec::with_capacity(2 * line_buf.len());
                for substr in line_buf {
                    let substr_buf: Vec<_> = substr.split("}}").collect();
                    for substr in substr_buf {
                        line.push(substr);
                    }
                }

                let mut color = Color::Red;
                print!("    ");
                for substr in line {
                    match color {
                        Color::Red => print!("{}", substr.red()),
                        Color::Blue => print!("{}", substr.blue()),
                    }
                    color.change_self();
                }
                println!();
            }
            _ => println!(),
        }
    }

    Ok(())
}

pub fn print_tldrpage(input: &str, mut platform: Option<&str>) -> Result<()> {
    if platform.is_none() {
        platform = Some(get_platform());
    }

    let platform = platform.unwrap();

    if platform != "common" {
        let path = construct_path(Some(input), platform)?;
        if let Ok(output) = get_file_content(path) {
            parse_tldrpage(&output)?;
            return Ok(());
        }
    }

    let path = construct_path(Some(input), "common")?;
    if let Ok(output) = get_file_content(path) {
        parse_tldrpage(&output)?;
        return Ok(());
    }

    Err(Error::new(std::io::ErrorKind::NotFound, "Item not found"))
}

pub fn parse_tldrlist<P: AsRef<Path>>(path: P) -> Result<()> {
    let entries = std::fs::read_dir(path)?
        .map(|ent| {
            ent.map(|ent| {
                ent.file_name()
                    .to_str()
                    .map(|s| s.split('.').into_iter().next().unwrap().to_string())
            })
        })
        .collect::<Result<Vec<_>>>()?;
    for entry in entries {
        if let Some(entry) = entry {
            println!("{entry}");
        } else {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid entry"));
        }
    }
    Ok(())
}

pub fn print_tldrlist(mut platform: Option<&str>) -> Result<()> {
    if platform.is_none() {
        platform = Some(get_platform());
    }
    let platform = platform.unwrap();
    if platform != "common" {
        let path = construct_path(None, platform)?;
        parse_tldrlist(path)?;
    }
    let path = construct_path(None, "common")?;
    parse_tldrlist(path)?;
    Ok(())
}

pub fn print_localpage<P: AsRef<Path>>(path: P) -> Result<()> {
    let output = get_file_content(path)?;
    parse_tldrpage(&output)?;

    Ok(())
}
