use std::{fs::File, io::BufReader, path::Path};

use serde::Deserialize;

use super::Program;

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum ProgramInput {
    Multi(Vec<Program>),
    Single(Program),
}

impl ProgramInput {
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
    fn into(self) -> Vec<Program> {
        match self {
            ProgramInput::Multi(programs) => programs,
            ProgramInput::Single(program) => vec![program],
        }
    }
}
