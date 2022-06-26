use std::{
    fmt,
    thread::{self, JoinHandle},
};

use crossbeam_channel::Sender;

use crate::program::Program;

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
    pub fn new(name: &str, output: Sender<WorkerResult>) -> anyhow::Result<Self> {
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

    pub fn finish(self) -> anyhow::Result<()> {
        std::mem::drop(self.input);
        self.me.join().unwrap()
    }

    pub fn send(&self, program: Program) -> anyhow::Result<()> {
        Ok(self.input.send(program)?)
    }
}
