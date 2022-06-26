use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::program::{interpreted::InterpretedProgram, Operation, OperationKind};

use self::{acme::Acme, madrid::Madrid};

mod acme;
mod madrid;

#[derive(Deserialize)]
struct Id {
    program_id: String,
}

#[derive(Deserialize)]
struct ProgramResult {
    result: usize,
}

#[derive(Deserialize)]
pub enum ArchitectureKindDeserializer {
    #[serde(alias = "ACME", alias = "acme")]
    Acme,
    Madrid,
}

#[enum_dispatch]
#[derive(Deserialize)]
#[serde(from = "ArchitectureKindDeserializer")]
pub enum ArchitectureKind {
    Acme(Acme),
    Madrid(Madrid),
}

impl From<ArchitectureKindDeserializer> for ArchitectureKind {
    fn from(other: ArchitectureKindDeserializer) -> Self {
        match other {
            ArchitectureKindDeserializer::Acme => Self::Acme(Acme::new()),
            ArchitectureKindDeserializer::Madrid => Self::Madrid(Madrid::new()),
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Instruction {
    NamedInstruction(&'static str),
    Value(usize),
}

#[enum_dispatch(ArchitectureKind)]
pub trait Architecture {
    /// Call this endpoint to execute a full program
    fn run(&self, program: &InterpretedProgram) -> anyhow::Result<usize>;

    /// Issue the instruction set for adding two numbers
    fn sum(&self, rhs: usize) -> Vec<Instruction>;
    /// Issue the instruction set for multiplying two numbers
    fn mul(&self, rhs: usize) -> Vec<Instruction>;
    /// Issue the instruction set for dividing two numbers
    fn div(&self, rhs: usize) -> Vec<Instruction>;
    /// Set the initial state
    fn initial_state(&self, state: usize) -> Vec<Instruction>;

    fn apply_operation(&self, operation: &Operation) -> Vec<Instruction> {
        let Operation { r#type, value } = operation;
        match r#type {
            OperationKind::Sum => self.sum(*value),
            OperationKind::Mul => self.mul(*value),
            OperationKind::Div => self.div(*value),
        }
    }
}
