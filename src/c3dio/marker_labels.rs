use c3dio::C3d;
use clap::{ArgMatches, Command};
use colored::Colorize;
use glob::glob;
use std::path::PathBuf;

use crate::args::{file_arg, output_arg, reference_arg};

pub(super) fn marker_labels_command() -> Command {
    Command::new("marker-labels")
        .about("Changes the marker labels in a C3D to match the labels in a comma separated list")
        .arg(file_arg().required(true))
        .arg(reference_arg().required(true))
        .arg(output_arg())
}

pub(super) fn process_marker_labels_command(sub_matches: ArgMatches) {
    let file = sub_matches.get_one::<String>("FILE").unwrap();
    let reference_file = sub_matches.get_one::<String>("REFERENCE").unwrap();
    let output = sub_matches.get_one::<String>("OUTPUT");
    let output: PathBuf = match output {
        Some(output) => output.into(),
        None => {
            // set output to current directory
            println!(
                "{}",
                "No output file was provided, writing to current directory".yellow()
            );
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
    let reference_contents = std::fs::read_to_string(reference_file);
    let reference_contents: String = match reference_contents {
        Ok(reference_contents) => reference_contents,
        Err(e) => {
            println!("{}", e.to_string().red());
            return;
        }
    };
    println!("Opening {}", reference_file.green());
    let reference: Vec<String> = reference_contents
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let files = match glob(file) {
        Ok(files) => files,
        Err(e) => {
            println!("{}", e.to_string().red());
            return;
        }
    };
    let files: Vec<PathBuf> = files.map(|f| f.unwrap()).collect();
    for file in files {
        println!("Opening {}", file.to_string_lossy().green());
        let mut c3d = match C3d::load_path(file.clone()) {
            Ok(c3d) => {
                println!(
                    "Changing {} marker labels to match {}",
                    file.to_string_lossy().green(),
                    reference_file.green()
                );
                c3d
            }
            Err(e) => {
                println!("{}", e.to_string().red());
                return;
            }
        };
        c3d.points.labels = reference.clone();
        let output = match output.is_dir() {
            true => output.join(file.file_name().unwrap()),
            false => output.clone(),
        };
        match c3d.write_path(output.clone()) {
            Ok(_) => println!("Wrote {}", output.to_string_lossy().green()),
            Err(e) => println!("{}", e.to_string().red()),
        }
    }
}

