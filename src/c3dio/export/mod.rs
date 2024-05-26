use argh::FromArgs;
use yansi::Paint;

mod forces;
mod markers;
mod analog;

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "export",
    description = "Exports C3D data to another format"
)]
pub(super) struct Export {
    #[argh(positional)]
    file: String,

    #[argh(
        option,
        short = 'm',
        long = "markers",
        description = "output file name for markers"
    )]
    markers: Option<String>,

    #[argh(
        option,
        short = 'f',
        long = "forces",
        description = "output file name for forces"
    )]
    forces: Option<String>,

    #[argh(
        option,
        short = 'a',
        long = "analog",
        description = "output file name for analog data"
    )]
    analog: Option<String>,
}

pub(super) fn export(export: Export) {
    println!("Exporting data from file: {}", export.file.bold());
    if let Some(markers) = &export.markers {
        println!("Exporting markers to: {}", markers);
        markers::export_markers(&export.file, &export.markers.unwrap());
    }
    if let Some(forces) = &export.forces {
        println!("Exporting forces to: {}", forces);
        forces::export_forces(&export.file, &export.forces.unwrap());
    }
    if let Some(analog) = &export.analog {
        println!("Exporting analog data to: {}", analog);
        analog::export_analog(&export.file, &export.analog.unwrap());
    }
}
