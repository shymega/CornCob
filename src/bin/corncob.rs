use clap::{ArgMatches, Command};

fn get_args() -> ArgMatches {
    Command::new("corncob")
        .version(env!("CARGO_PKG_VERSION"))
        .author("The CornCob Developers")
        .about("Program for running compiled COBOL programs in the CornCob virtual machine")
        .get_matches()
        .clone()
}

fn main() {
    let _matches = get_args();

    todo!()
}
