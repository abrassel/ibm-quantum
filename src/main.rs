use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about)]
struct Args {
    /// Program file to interpret
    #[clap(value_parser)]
    program: PathBuf,
}

fn main() {
    let _filename = Args::parse().program;
}
#[cfg(test)]
mod tests {}
