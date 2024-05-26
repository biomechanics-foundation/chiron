use argh::FromArgs;

mod check;
mod export;
mod filter;
mod info;
mod helpers;

#[derive(FromArgs)]
#[argh(description = "Command line tool for working with C3D files")]
struct C3dio {
    #[argh(subcommand)]
    nested: SubCommands,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum SubCommands {
    Check(check::Check),
    Info(info::Info),
    Export(export::Export),
    Filter(filter::Filter),
}

fn main() {
    let c3dio: C3dio = argh::from_env();
    match c3dio.nested {
        SubCommands::Check(check) => {
            check::print_check(check);
        }
        SubCommands::Info(info) => {
            info::print_info(info);
        }
        SubCommands::Export(export) => {
            export::export(export);
        }
        SubCommands::Filter(filter) => {
            filter::filter(filter);
        }
    }
}
