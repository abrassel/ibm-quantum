use std::{fs::File, io::BufReader, path::Path};

use serde::Deserialize;

use super::Program;

#[derive(Deserialize)]
#[serde(untagged)]
/// Json input can be either a single program or a list.
/// Use an untagged enum to get either case.
pub(crate) enum ProgramInput {
    Multi(Vec<Program>),
    Single(Program),
}

impl ProgramInput {
    /// Given a file containing a json program(s), decode it.
    pub fn read_program_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Program>> {
        let reader = {
            let file = File::open(path)?;
            BufReader::new(file)
        };

        let input: Self = serde_json::from_reader(reader)?;
        Ok(input.into())
    }
}

impl Into<Vec<Program>> for ProgramInput {
    /// We want to have a list of programs ultimately, so singletons need to get packaged up.
    fn into(self) -> Vec<Program> {
        match self {
            ProgramInput::Multi(programs) => programs,
            ProgramInput::Single(program) => vec![program],
        }
    }
}
