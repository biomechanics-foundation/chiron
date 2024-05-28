use crate::helpers::is_force_channel;
use argh::FromArgs;
use butterworth::{Cutoff, Filter as ButterFilter};
use c3dio::prelude::*;
use yansi::Paint;

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "filter",
    description = "Filters data in C3D file, writes to new file"
)]
pub(super) struct Filter {
    #[argh(positional)]
    file: String,

    #[argh(positional)]
    order: usize,

    #[argh(positional)]
    cutoff_freq: f32,

    #[argh(
        switch,
        short = 'm',
        long = "markers",
        description = "filter marker data"
    )]
    markers: bool,

    #[argh(
        switch,
        short = 'f',
        long = "forces",
        description = "filter force data"
    )]
    forces: bool,

    #[argh(
        switch,
        short = 'a',
        long = "analog",
        description = "filter analog data"
    )]
    analog: bool,

    #[argh(option, short = 'o', long = "output", description = "output file name")]
    output: Option<String>,
}

pub(super) fn filter(filter: Filter) {
    println!("\nAttempting to load file: {}", filter.file.bold());
    let mut c3d = match C3d::load(&filter.file) {
        Ok(c3d) => {
            println!("{}: C3D file loaded", "Success".green().bold());
            c3d
        }
        Err(e) => {
            println!("{}: {}", "Error".red().bold(), e.red().italic());
            return;
        }
    };
    println!("Filtering data in file: {}", filter.file.bold());
    println!(
        "Filter order: {} & cutoff frequency: {} Hz",
        filter.order, filter.cutoff_freq
    );
    if !filter.markers && !filter.forces && !filter.analog {
        println!(
            "{}: {}",
            "Error".red().bold(),
            "No data selected to filter".red().italic()
        );
        return;
    }
    if filter.markers {
        let butter_filter_markers = match ButterFilter::new(
            filter.order,
            c3d.points.frame_rate.into(),
            Cutoff::LowPass(filter.cutoff_freq.into()),
        ) {
            Ok(filter) => filter,
            Err(e) => {
                println!("{}: {}", "Error".red().bold(), e.red().italic());
                return;
            }
        };
        println!("Filtering marker data");
        filter_markers(&mut c3d, &butter_filter_markers);
    }
    if filter.forces || filter.analog {
        let butter_filter_analog = match ButterFilter::new(
            filter.order,
            c3d.points.frame_rate as f64 * c3d.analog.samples_per_channel_per_frame as f64,
            Cutoff::LowPass(filter.cutoff_freq.into()),
        ) {
            Ok(filter) => filter,
            Err(e) => {
                println!("{}: {}", "Error".red().bold(), e.red().italic());
                return;
            }
        };
        if filter.forces {
            println!("Filtering force data");
            filter_forces(&mut c3d, &butter_filter_analog);
        }
        if filter.analog {
            println!("Filtering analog data");
            filter_analog(&mut c3d, &butter_filter_analog);
        }
    }
    let output = match filter.output {
        Some(output) => output,
        None => {
            let temp = filter.file.split('.').collect::<Vec<&str>>();
            let temp = match temp.len() {
                0 => format!("filtered.c3d"),
                _ => format!("{}_filtered.c3d", temp[..temp.len() - 1].join(".")),
            };
            temp
        }
    };
    println!("Output file: {}", output.bold());
    match c3d.write(&output) {
        Ok(_) => println!("{}: C3D file written", "Success".green().bold()),
        Err(e) => println!("{}: {}", "Error".red().bold(), e.red().italic()),
    }
}

fn filter_markers(c3d: &mut C3d, filter: &ButterFilter) {
    for i in 0..c3d.points.cols() {
        for j in 0..2 {
            let mut temp: Vec<f64> = c3d
                .points
                .iter_col(i)
                .map(|x| x[j] as f64)
                .collect::<Vec<f64>>();
            let filtered = match filter.bidirectional(&mut temp) {
                Ok(filtered) => filtered,
                Err(e) => {
                    println!("{}: {}", "Error".red().bold(), e.red().italic());
                    return;
                }
            };
            for (k, val) in filtered.iter().enumerate() {
                c3d.points[k][i][j] = *val as f32;
            }
        }
    }
}

fn filter_forces(c3d: &mut C3d, filter: &ButterFilter) {
    for i in 0..c3d.forces.len() {
        for channel in c3d.forces[i].channels {
            if channel > 0 {
                let mut temp: Vec<f64> = c3d.analog[channel as usize - 1]
                    .iter()
                    .map(|x| *x as f64)
                    .collect::<Vec<f64>>();
                let filtered = match filter.bidirectional(&mut temp) {
                    Ok(filtered) => filtered,
                    Err(e) => {
                        println!("{}: {}", "Error".red().bold(), e.red().italic());
                        return;
                    }
                };
                for (k, val) in filtered.iter().enumerate() {
                    c3d.analog[k][channel as usize - 1] = *val;
                }
            }
        }
    }
}

fn filter_analog(c3d: &mut C3d, filter: &ButterFilter) {
    for i in 0..c3d.analog.cols() {
        if i > 255 {
            println!(
                "{}: {}",
                "Error".red().bold(),
                "Invalid channel number".red().italic()
            );
            return;
        }
        if !is_force_channel(&c3d, i as u8) {
            let mut temp: Vec<f64> = c3d.analog[i]
                .iter()
                .map(|x| *x as f64)
                .collect::<Vec<f64>>();
            let filtered = match filter.bidirectional(&mut temp) {
                Ok(filtered) => filtered,
                Err(e) => {
                    println!("{}: {}", "Error".red().bold(), e.red().italic());
                    return;
                }
            };
            for (k, val) in filtered.iter().enumerate() {
                c3d.analog[k][i] = *val;
            }
        }
    }
}
