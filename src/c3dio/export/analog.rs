use crate::helpers::is_force_channel;
use grid::Grid;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use c3dio::prelude::*;
use yansi::Paint;

pub(super) fn export_analog(file: &str, output: &str) {
    let format = match output.split('.').last() {
        Some(format) => {
            let format = AnalogOutputFileTypes::from_str(format.trim().to_lowercase().as_str());
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
                AnalogOutputFileTypes::Sto => {
                    let sto = build_sto(&c3d);
                    match sto {
                        Some(sto) => sto.write(PathBuf::from(output)),
                        None => Err(C3dWriteError::InvalidForcePlatformInfo(
                            "Could not build sto file from c3d file".to_string(),
                        )),
                    }
                }
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
enum AnalogOutputFileTypes {
    Sto,
}

impl FromStr for AnalogOutputFileTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "sto" => Ok(AnalogOutputFileTypes::Sto),
            _ => Err(format!("{} is not a valid output format", s)),
        }
    }
}

impl Display for AnalogOutputFileTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalogOutputFileTypes::Sto => write!(f, "sto"),
        }
    }
}

fn build_sto(c3d: &C3d) -> Option<Sto> {
    let mut column_names = Vec::new();
    let mut data = Grid::new(c3d.analog.analog.size().0, 0);
    if c3d.analog.cols() > 0 {
        for (i, channel) in c3d.analog.iter().enumerate() {
            if !is_force_channel(c3d, (i + 1) as u8) {
                column_names.push(c3d.analog.labels[i].clone());
                data.push_col(vec![channel.clone()]);
            }
        }
        Some(Sto {
            file_description: None,
            version: 1,
            in_degrees: false,
            first_frame: c3d.points.first_frame as usize,
            column_names,
            data_rate: c3d.analog.rate,
            data,
        })
    } else {
        println!("{}", "No analog data was found in the C3D file".red());
        None
    }
}

