use architecture::ArchitectureKind;
use std::{
    fmt,
    path::PathBuf,
    thread::{self, JoinHandle},
};

use clap::Parser;
use crossbeam_channel::Sender;
use program::{deserialization::ProgramInput, Program};

mod architecture;
mod program;

#[derive(Parser, Debug)]
#[clap(about)]
struct Args {
    /// Program file to interpret
    #[clap(value_parser)]
    program: PathBuf,
}

struct Worker {
    me: JoinHandle<anyhow::Result<()>>,
    input: Sender<Program>,
}

struct WorkerResult {
    id: String,
    output: usize,
}

impl fmt::Display for WorkerResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Result for id {}: {}", self.id, self.output)
    }
}

impl Worker {
    fn new(name: &str, output: Sender<WorkerResult>) -> anyhow::Result<Self> {
        let (tx, rx) = crossbeam_channel::unbounded::<Program>();
        let worker = thread::Builder::new()
            .name(name.to_owned())
            .spawn(move || {
                for program in rx.iter() {
                    let result = program.interpret()?;
                    output.send(WorkerResult {
                        id: program.id,
                        output: result,
                    })?;
                }

                Ok(())
            })?;

        return Ok(Worker {
            me: worker,
            input: tx,
        });
    }

    fn finish(self) -> anyhow::Result<()> {
        std::mem::drop(self.input);
        self.me.join().unwrap()
    }
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
            ArchitectureKind::Acme(_) => acme.input.send(program)?,
            ArchitectureKind::Madrid(_) => madrid.input.send(program)?,
        }
    }

    acme.finish()?;
    madrid.finish()?;
    printing_thread.join().unwrap();

    Ok(())
}
#[cfg(test)]
mod tests {}
