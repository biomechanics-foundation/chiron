use grid::Grid;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use c3dio::prelude::*;
use yansi::Paint;

pub(super) fn export_forces(file: &str, output: &str) {
    let format = match output.split('.').last() {
        Some(format) => {
            let format = ForceOutputFileTypes::from_str(format.trim().to_lowercase().as_str());
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
                ForceOutputFileTypes::Sto => {
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
enum ForceOutputFileTypes {
    Sto,
}

impl FromStr for ForceOutputFileTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "sto" => Ok(ForceOutputFileTypes::Sto),
            _ => Err(format!("{} is not a valid output format", s)),
        }
    }
}

impl Display for ForceOutputFileTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForceOutputFileTypes::Sto => write!(f, "sto"),
        }
    }
}

fn build_sto(c3d: &C3d) -> Option<Sto> {
    let mut column_names = Vec::new();
    let mut data = Grid::new(c3d.analog.analog.size().0, 0);
    if c3d.forces.len() > 0 {
        for (i, plate) in c3d.forces.iter().enumerate() {
            for channel in plate.channels.into_iter() {
                let channel = channel as usize;
                if c3d.analog.labels.len() >= channel {
                    column_names.push(c3d.analog.labels[channel - 1].clone());
                } else {
                    column_names.push(format!("column_{}", channel));
                }
                data.push_col(c3d.analog.iter_col(channel - 1).cloned().collect());
            }
            let origin = plate.origin.as_ref();
            if origin.len() >= i + 1 {
                column_names.push(format!("EC{}X", i + 1));
                column_names.push(format!("EC{}Y", i + 1));
                column_names.push(format!("EC{}Z", i + 1));
                data.push_col(vec![origin[0] as f64; data.size().0]);
                data.push_col(vec![origin[1] as f64; data.size().0]);
                data.push_col(vec![origin[2] as f64; data.size().0]);
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
        println!("{}", "No force data was found in the C3D file".red());
        None
    }
}
