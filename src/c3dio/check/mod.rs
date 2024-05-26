use argh::FromArgs;
use c3dio::prelude::*;
use yansi::Paint;

#[derive(FromArgs)]
#[argh(subcommand, name = "check", description = "Checks C3D file for errors")]
pub(super) struct Check {
    #[argh(positional)]
    file: String,
}

pub(super) fn print_check(check: Check) {
    println!("Checking file: {}", check.file);
    match C3d::load(&check.file) {
        Ok(_c3d) => {
            println!("{}: The file is a valid C3D file", "SUCCESS".green().bold());
        }
        Err(e) => {
            println!("{}: {}", "ERROR".red().bold(), e.red().italic());
        }
    }
}
