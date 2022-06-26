use serde::{Deserialize, Serialize};

mod acme;
mod madrid;

#[derive(Deserialize)]
pub enum ArchitectureKind {
    #[serde(rename = "ACME")]
    Acme,
    Madrid,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Instruction {
    NamedInstruction(&'static str),
    Value(usize),
}

pub trait Architecture {
    /// Call this endpoint to execute a full program
    fn run(payload: &str) -> usize;

    /// Issue the instruction set for adding two numbers
    fn sum(lhs: usize, rhs: usize) -> Vec<Instruction>;
    /// Issue the instruction set for multiplying two numbers
    fn mul(lhs: usize, rhs: usize) -> Vec<Instruction>;
    /// Issue the instruction set for dividing two numbers
    fn div(lhs: usize, rhs: usize) -> Vec<Instruction>;
}
