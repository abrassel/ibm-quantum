use std::{fs::File, io::BufReader, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum ProgramInput {
    Multi(Vec<Program>),
    Single(Program),
}

#[derive(Deserialize)]
pub struct Program {
    id: String,
    control_instrument: ArchitectureKind,
    initial_value: usize,
    operations: Vec<Operation>,
}

#[derive(Deserialize)]
pub enum ArchitectureKind {
    #[serde(rename = "ACME")]
    Acme,
    Madrid,
}

#[derive(Deserialize)]
pub struct Operation {
    r#type: OperationKind,
    value: usize,
}

#[derive(Deserialize)]
pub enum OperationKind {
    Sum,
    Mul,
    Div,
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
