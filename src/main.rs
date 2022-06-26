use architecture::worker::Worker;
use architecture::ArchitectureKind;
use reqwest::Url;
use std::{path::PathBuf, thread};

use clap::Parser;
use program::deserialization::ProgramInput;

mod architecture;
mod program;

#[derive(Parser, Debug)]
#[clap(about)]
struct Args {
    #[clap(value_parser)]
    /// Program file to interpret
    program: PathBuf,

    #[clap(long, value_parser)]
    /// Location of Acme server
    acme: Url,

    #[clap(long, value_parser)]
    /// Location of Madrid server
    madrid: Url,
}

fn main() -> anyhow::Result<()> {
    let filename = Args::parse().program;
    let programs = ProgramInput::read_program_from_file(filename)?;
    let (tx, rx) = crossbeam_channel::unbounded();
    let acme = Worker::new("Acme", tx.clone())?;
    let madrid = Worker::new("Madrid", tx)?;
    let printing_thread = thread::spawn(move || {
        for result in rx.iter() {
            println!("{}", result);
        }
    });

    for program in programs {
        match program.control_instrument {
            ArchitectureKind::Acme(_) => acme.send(program)?,
            ArchitectureKind::Madrid(_) => madrid.send(program)?,
        }
    }

    acme.finish()?;
    madrid.finish()?;
    printing_thread.join().unwrap();

    Ok(())
}
#[cfg(test)]
mod tests {}
