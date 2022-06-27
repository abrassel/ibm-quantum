use std::{
    fmt,
    thread::{self, JoinHandle},
};

use crossbeam_channel::Sender;

use crate::program::Program;

use super::Architecture;

/// Represents a worker for a given [`Architecture`]
pub struct Worker {
    me: JoinHandle<anyhow::Result<()>>,
    input: Sender<Program>,
}

/// Represents the result for a given [`Program`] after execution.
pub struct WorkerResult {
    id: String,
    output: usize,
}

impl fmt::Display for WorkerResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id {}: {}", self.id, self.output)
    }
}

impl Worker {
    pub fn new<Arch: Architecture + 'static>(
        arch: Arch,
        output: Sender<WorkerResult>,
    ) -> anyhow::Result<Self> {
        let (tx, rx) = crossbeam_channel::unbounded::<Program>();
        let worker = thread::spawn(move || {
            let arch = arch;
            for program in rx.iter() {
                let result = program.interpret(&arch)?;
                output.send(WorkerResult {
                    id: program.id,
                    output: result,
                })?;
            }

            Ok(())
        });

        Ok(Worker {
            me: worker,
            input: tx,
        })
    }

    /// Stand-in for a fallible drop.
    pub fn finish(self) -> anyhow::Result<()> {
        // Ensures that the worker ends, as the sender has closed.
        std::mem::drop(self.input);
        self.me.join().unwrap()
    }

    /// Send a program to the worker to complete.
    pub fn send(&self, program: Program) -> anyhow::Result<()> {
        Ok(self.input.send(program)?)
    }
}
