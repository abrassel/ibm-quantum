use serde::{Deserialize, Serialize};

use crate::program::{interpreted::InterpretedProgram, Operation};

pub mod acme;
pub mod madrid;
pub mod worker;

#[derive(Deserialize)]
pub enum ArchitectureKind {
    #[serde(alias = "ACME", alias = "acme")]
    Acme,
    Madrid,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum Instruction {
    NamedInstruction(&'static str),
    Value(usize),
}

pub trait Architecture: Send {
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

    /// Helper method to go from operation kind -> concrete operation call.
    /// It's unfortunate that this exists, but it seems to be a shortfall of the dynamicism of the json.
    fn apply_operation(&self, operation: &Operation) -> Vec<Instruction> {
        match operation {
            Operation::Sum { value } => self.sum(*value),
            Operation::Mul { value } => self.mul(*value),
            Operation::Div { value } => self.div(*value),
        }
    }
}
