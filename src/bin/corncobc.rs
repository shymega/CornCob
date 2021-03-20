use clap::{App, ArgMatches};

fn parse_arguments() -> ArgMatches<'static> {
    App::new("corncobc")
        .version(env!("CARGO_PKG_VERSION"))
        .author("The CornCob Developers")
        .about(
            "Program for compiling COBOL code for the CornCob virtual machine",
        )
        .get_matches()
}

fn main() {
    let _matches = parse_arguments();

    unimplemented!();
}
