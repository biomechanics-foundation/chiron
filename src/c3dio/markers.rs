use c3dio::file_formats::trc;
use c3dio::prelude::*;
use clap::{ArgMatches, Command};
use colored::Colorize;
use glob::glob;
use std::{fmt::Display, path::PathBuf};
use std::str::FromStr;

use crate::args::{file_arg, output_arg, format_arg};

pub(super) fn markers_command() -> Command {
    Command::new("markers")
        .about("Prints the marker data from a C3D file to a supported format")
        .arg(file_arg().required(true))
        .arg(format_arg().required(true))
        .arg(output_arg())
        //allow explicitly setting the output format
}

pub(super) fn process_markers_command(sub_matches: ArgMatches) {
    let file = sub_matches.get_one::<String>("FILE").unwrap();
    let format = match sub_matches.get_one::<String>("FORMAT") {
        Some(format) => format,
        None => {
            println!("{}", "No output format was provided".red());
            return;
        }
    };
    let format = match MarkerOutputFileTypes::from_str(format.trim().to_lowercase().as_str()) {
        Ok(format) => format,
        Err(e) => {
            println!("{}", e.red());
            return;
        }
    };
    let output = sub_matches.get_one::<String>("OUTPUT");
    let output: PathBuf = match output {
        Some(output) => output.into(),
        None => {
            // set output to current directory
            println!("{}", "No output file was provided, writing to current directory".yellow());
            let output = std::env::current_dir();
            match output {
                Ok(output) => output,
                Err(e) => {
                    println!("{}", e.to_string().red());
                    return;
                }
            }
        }
    };

    // if file contains a wildcard, use glob to find all matching files
    // make sure to check that the output file is a directory
    let files = match glob(file) {
        Ok(files) => files,
        Err(e) => {
            println!("{}", e.to_string().red());
            return;
        }
    };
    let files = files
        .filter_map(|file| match file {
            Ok(file) => {
                if file.is_file() {
                    Some(file)
                } else {
                    None
                }
            }
            Err(e) => {
                println!("{}", e.to_string().red());
                None
            }
        })
        .collect::<Vec<_>>();
    for file in files {
        println!("Opening {}", file.to_string_lossy().green());
        match C3d::load_path(file.clone()) {
            Ok(c3d) => {
                println!("Converting {} to {}", file.to_string_lossy().green(), format);
                let output = match output.is_dir() {
                    true => {
                        let mut output = output.clone();
                        output.push(file.file_name().unwrap());
                        output.set_extension(format!("{}", format));
                        output
                    }
                    false => output.clone(),
                };
                let write_attempt = match format {
                    MarkerOutputFileTypes::Trc => Trc::from_c3d(&c3d).write(output.clone()),
                };
                match write_attempt {
                    Ok(_) => println!("Wrote {}", output.to_string_lossy().green()),
                    Err(e) => println!("{}", e.to_string().red()),
                }
            }
            Err(e) => println!("{}", e.to_string().red()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum MarkerOutputFileTypes {
    Trc,
}

impl FromStr for MarkerOutputFileTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "trc" => Ok(MarkerOutputFileTypes::Trc),
            _ => Err(format!(
                "{} is not a valid output file type, types allowed: .trc",
                s
            )),
        }
    }
}

impl Display for MarkerOutputFileTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkerOutputFileTypes::Trc => write!(f, "trc"),
        }
    }
}
