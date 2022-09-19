use clap::{ArgMatches, Command};

fn get_args() -> ArgMatches {
    Command::new("corncobc")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Dom Rodriguez <shymega@shymega.org.uk>")
        .about("COBOL compiler for the CornCob VM.")
        .get_matches()
        .clone()
}

fn main() {
    let _matches = get_args();

    todo!()
}
