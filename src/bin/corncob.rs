use clap::{App, ArgMatches};

fn parse_arguments() -> ArgMatches<'static> {
    App::new("corncob")
        .version(env!("CARGO_PKG_VERSION"))
        .author("The CornCob Developers")
        .about("Program for running compiled COBOL programs in the CornCob virtual machine")
        .get_matches()
}

fn main() {
    let _matches = parse_arguments();

    unimplemented!();
}
