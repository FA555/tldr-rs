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
    if let Ok(()) = real_main() {}
}

fn real_main() -> Result<()> {
    let args = Arguments::parse();
    let platform = args.platform.map(|platform| platform.to_str());

    if args.version {
        print_version();
        return Ok(());
    }

    if args.clear_cache {
        clear_localdb()?;
        return Ok(());
    }

    if args.update {
        update_localdb()?;
        return Ok(());
    } else if args.item.is_some() {
        match check_localtime() {
            Ok(_) => {}
            Err(_) => update_localdb()?,
        }
    }

    if let Some(path) = args.render {
        print_localpage(path)?;
        return Ok(());
    }

    if !has_localdb()? {
        update_localdb()?;
    }
    if args.list {
        print_tldrlist(platform)?;
        return Ok(());
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
