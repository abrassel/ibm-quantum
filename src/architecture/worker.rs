use std::{
    fmt,
    thread::{self, JoinHandle},
};

use crossbeam_channel::Sender;

use crate::program::Program;

use super::Architecture;

pub struct Worker {
    me: JoinHandle<anyhow::Result<()>>,
    input: Sender<Program>,
}

pub struct WorkerResult {
    id: String,
    output: usize,
}

impl fmt::Display for WorkerResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Result for id {}: {}", self.id, self.output)
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

        return Ok(Worker {
            me: worker,
            input: tx,
        });
    }

    pub fn finish(self) -> anyhow::Result<()> {
        std::mem::drop(self.input);
        self.me.join().unwrap()
    }

    pub fn send(&self, program: Program) -> anyhow::Result<()> {
        Ok(self.input.send(program)?)
    }
}
