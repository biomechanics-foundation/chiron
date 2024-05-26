use c3dio::prelude::*;
use std::str::FromStr;
use std::{fmt::Display, path::PathBuf};
use yansi::Paint;

pub(super) fn export_markers(file: &str, output: &str) {
    let format = match output.split('.').last() {
        Some(format) => {
            let format = MarkerOutputFileTypes::from_str(format.trim().to_lowercase().as_str());
            match format {
                Ok(format) => format,
                Err(e) => {
                    println!("{}", e.red());
                    return;
                }
            }
        }
        None => {
            println!(
                "{}",
                "No output format was provided, please provide a file with an extension".red()
            );
            return;
        }
    };

    println!("Opening {}", file.green());
    match C3d::load(file) {
        Ok(c3d) => {
            println!("Converting to {}", format.to_string().bright_yellow());
            let write_attempt = match format {
                MarkerOutputFileTypes::Trc => Trc::from_c3d(&c3d).write(PathBuf::from(output)),
            };
            match write_attempt {
                Ok(_) => println!("Wrote {}", output.green()),
                Err(e) => println!("{}", e.to_string().red()),
            }
        }
        Err(e) => println!("{}", e.to_string().red()),
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
