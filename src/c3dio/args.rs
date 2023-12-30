use clap::Arg;

pub(super) fn file_arg() -> Arg {
    Arg::new("FILE").help("The C3D file to use")
}

pub(super) fn output_arg() -> Arg {
    Arg::new("OUTPUT").help("The output file to write to")
}

pub(super) fn format_arg() -> Arg {
    Arg::new("FORMAT")
        .short('f')
        .long("format")
        .help("The format to output the data in")
}

pub(super) fn reference_arg() -> Arg {
    Arg::new("REFERENCE")
        .short('r')
        .long("reference")
        .help("The reference file to use")
}