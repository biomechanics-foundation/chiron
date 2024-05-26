use argh::FromArgs;
use c3dio::prelude::*;
use yansi::Paint;

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "info",
    description = "prints information about c3d file"
)]
pub(super) struct Info {
    #[argh(positional)]
    pub(super) file: String,

    #[argh(
        switch,
        short = 'v',
        long = "verbose",
        description = "prints everything about c3d file"
    )]
    pub(super) verbose: bool,

    #[argh(
        switch,
        short = 'm',
        long = "markers",
        description = "prints marker information"
    )]
    pub(super) markers: bool,

    #[argh(
        switch,
        short = 'f',
        long = "forces",
        description = "prints force information"
    )]
    pub(super) forces: bool,

    #[argh(
        switch,
        short = 'a',
        long = "analog",
        description = "prints analog information"
    )]
    pub(super) analog: bool,

    #[argh(
        switch,
        short = 'e',
        long = "events",
        description = "prints event information"
    )]
    pub(super) events: bool,

    #[argh(
        switch,
        long = "manufacturer",
        short = 'M',
        description = "prints manufacturer information"
    )]
    pub(super) manufacturer: bool,
}

pub(super) fn print_info(info: Info) {
    println!("\nAttempting to load file: {}", info.file.white().bold());
    let c3d = match C3d::load(&info.file) {
        Ok(c3d) => {
            println!("{}: C3D file loaded", "Success".green().bold());
            c3d
        }
        Err(e) => {
            println!("{}: {}", "Error".red().bold(), e.red().italic());
            return;
        }
    };
    println!(
        "\nPrinting information for file: {}\n",
        info.file.bold()
    );
    if info.verbose {
        println!("{}", "== Printing everything ==\n".yellow().bold());
    }
    println!("{}", "== General Information ==".bold());
    print_general_info(&c3d);
    if info.markers || info.verbose {
        println!("{}", "-- Markers --\n".bold());
        print_marker_info(&c3d);
    }
    if info.forces || info.verbose {
        println!("{}", "-- Forces --\n".bold());
        print_force_info(&c3d);
    }
    if info.analog || info.verbose {
        println!("{}", "-- Analog --\n".bold());
        print_analog_info(&c3d);
    }
    if info.events || info.verbose {
        println!("{}", "-- Events --\n".bold());
        print_event_info(&c3d);
    }
    if info.manufacturer || info.verbose {
        println!("{}", "-- Manufacturer --\n".bold());
        print_manufacturer_info(&c3d);
    }
}

fn print_general_info(c3d: &C3d) {
    println!("Number of markers: {}", c3d.points.cols().bold());
    println!(
        "Number of analog channels (incl. force): {}",
        c3d.analog.cols().bold()
    );
    println!("Number of events: {}", c3d.events.len().bold());
    println!("Number of forces: {}", c3d.forces.len().bold());
    println!("Number of frames: {}", c3d.points.rows().bold());
}

fn print_marker_info(c3d: &C3d) {
    println!("Number of markers: {}", c3d.points.cols().bold());
    println!("Marker names: {:?}", c3d.points.labels);
    println!("Units: {:?}", c3d.points.units.bold());
    println!("Frame rate: {}", c3d.points.frame_rate.bold());
    println!("Number of frames: {}", c3d.points.rows().bold());
}

fn print_force_info(c3d: &C3d) {
    println!("Number of force platforms: {}", c3d.forces.len().bold());
    println!("Force platform types:");
    for (i, platform) in c3d.forces.iter().enumerate() {
        println!("Force platform {}: {:?}", i + 1, platform.plate_type);
    }
    println!("Force platform corners:");
    for (i, platform) in c3d.forces.iter().enumerate() {
        println!("Force platform {}: {:?}", i + 1, platform.corners);
    }
    println!("Force platform origin in force plate reference frame:");
    for (i, platform) in c3d.forces.iter().enumerate() {
        println!("Force platform {}: {:?}", i + 1, platform.origin);
    }
}

fn print_analog_info(c3d: &C3d) {
    println!("Number of analog channels: {}", c3d.analog.cols().bold());
    println!("Analog channel names: {:?}", c3d.analog.labels);
    println!("Units: {:?}", c3d.analog.units.bold());
    println!(
        "Frame rate: {}",
        ((c3d.analog.samples_per_channel_per_frame as f32) * c3d.points.frame_rate).bold()
    );
    println!("Number of frames: {}", c3d.analog.rows().bold());
}

fn print_event_info(c3d: &C3d) {
    println!("Number of events: {}", c3d.events.len().bold());
    println!("Events:");
    for (i, event) in c3d.events.iter().enumerate() {
        println!("Event {}: {} {:?}", i + 1, event.label, event.time);
    }
}

fn print_manufacturer_info(c3d: &C3d) {
    if c3d.manufacturer.company.is_some() {
        println!(
            "Company: {}",
            c3d.manufacturer.company.as_ref().unwrap().bold()
        );
    }
    if c3d.manufacturer.software.is_some() {
        println!(
            "Software: {}",
            c3d.manufacturer.software.as_ref().unwrap().bold()
        );
    }
    if c3d.manufacturer.version.is_some() {
        let version = c3d.manufacturer.version.as_ref().unwrap();
        match version {
            ManufacturerVersion::String(version) => {
                println!("Version: {}", version.bold());
            }
            ManufacturerVersion::Float(version) => {
                println!("Version: {}", version.bold());
            }
            ManufacturerVersion::Array(version) => {
                println!("Version: {:?}", version.bold());
            }
        }
    }
    if c3d.manufacturer.edited.is_some() {
        println!(
            "Edited: {:?}",
            c3d.manufacturer.edited.as_ref().unwrap().bold()
        );
    }
}
