use std::{fs::File, io::BufReader, path::Path};

use serde::Deserialize;

use crate::architecture::{Architecture, ArchitectureKind};

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

impl Program {
    pub fn intepret(&self) -> usize {
        todo!()
    }

    fn interpret_instrs<Arch: Architecture>(
        initial_value: usize,
        operations: Vec<Operation>,
    ) -> Vec<Arch::Instruction> {
        todo!()
    }
}
