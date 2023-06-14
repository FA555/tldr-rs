mod cli;
mod constant;
mod local;
mod parser;
mod util;

use crate::cli::Arguments;
use crate::parser::*;

use std::io::{Error, Result};

use clap::{CommandFactory, Parser};
use colored::Colorize;

use crate::local::*;

fn print_version() {
    if let Some(version) = option_env!("CARGO_PKG_VERSION") {
        println!("{}", format!("tldr-rs {version}").bold());
    } else {
        println!("tldr-rs");
    }

    if let Some(author) = option_env!("CARGO_PKG_AUTHORS") {
        println!("Copyright (C) 2023 {author}");
    }

    if let Some(source) = option_env!("CARGO_PKG_REPOSITORY") {
        if !source.is_empty() {
            println!("Source available at {source}");
        }
    }
}

fn main() {
    match real_main() {
        Ok(_) => {}
        Err(err) => {
            println!("{} {err}", "Error:".to_string().red());
            std::process::exit(1);
        }
    }
}

fn real_main() -> Result<()> {
    let args = Arguments::parse();
    let platform = args.platform.map(|platform| platform.to_str());

    if args.version {
        print_version();
        return Ok(());
    }

    if args.clear_cache {
        return clear_localdb();
    }

    if args.update {
        return update_localdb();
    } else if args.item.is_some() && check_localtime().is_err() {
        update_localdb()?;
    }

    if let Some(path) = args.render {
        return print_localpage(path);
    }

    if !has_localdb()? {
        update_localdb()?;
    }
    if args.list {
        return print_tldrlist(platform);
    }

    match args.item {
        Some(item) => match print_tldrpage(&item, platform) {
            Ok(_) => Ok(()),
            Err(_) => {
                println!("This page doesn't exist yet!");
                println!("Submit new pages here: https://github.com/tldr-pages/tldr");
                Err(Error::new(std::io::ErrorKind::NotFound, "Page not found"))
            }
        },
        None => {
            Arguments::command().print_help()?;
            std::process::exit(1);
        }
    }
}
