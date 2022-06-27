use architecture::madrid::Madrid;
use architecture::ArchitectureKind;
use architecture::{acme::Acme, worker::Worker};
use reqwest::Url;
use std::{io::Write, path::PathBuf, thread};

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
    let Args {
        program: filename,
        acme,
        madrid,
    } = Args::parse();
    let programs = ProgramInput::read_program_from_file(filename)?;

    // Set up acme, madrid, and printing thread
    let (tx, rx) = crossbeam_channel::unbounded();
    let acme = Worker::new(Acme::new(acme)?, tx.clone())?;
    let madrid = Worker::new(Madrid::new(madrid)?, tx)?;
    let printing_thread = thread::spawn(move || -> anyhow::Result<()> {
        // acquiring lock increases printing efficiency
        let mut stdout = std::io::stdout().lock();
        for result in rx.iter() {
            writeln!(stdout, "{}", result)?;
        }

        Ok(())
    });

    for program in programs {
        match program.control_instrument {
            ArchitectureKind::Acme => acme.send(program)?,
            ArchitectureKind::Madrid => madrid.send(program)?,
        }
    }

    acme.finish()?;
    madrid.finish()?;
    printing_thread.join().unwrap()?;

    Ok(())
}
