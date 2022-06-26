use std::path::PathBuf;

use clap::Parser;
use program::ProgramInput;

mod program;

#[derive(Parser, Debug)]
#[clap(about)]
struct Args {
    /// Program file to interpret
    #[clap(value_parser)]
    program: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let filename = Args::parse().program;
    let programs = ProgramInput::read_program_from_file(filename)?;

    Ok(())
}
#[cfg(test)]
mod tests {}
