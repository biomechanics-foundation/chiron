use std::path::PathBuf;

use crate::args::file_arg;
use c3dio::prelude::*;
use clap::{ArgMatches, Command};
use colored::Colorize;

pub(super) fn info_command() -> Command {
    Command::new("info")
        .about("Prints information about a C3D file")
        .arg(file_arg().required(true))
}

pub(super) fn process_info_command(sub_matches: ArgMatches) {
    let file = sub_matches
        .get_one::<String>("FILE")
        .ok_or_else(|| println!("{}", "No file was provided".red()));
    let file = match file {
        Ok(file) => file,
        Err(e) => return,
    };
    println!("Opening {}", file.green());
    match C3d::load(&file.clone()) {
        Ok(c3d) => println!("{}", c3d.to_string()),
        Err(e) => println!("{}", e.to_string().red()),
    }
}
