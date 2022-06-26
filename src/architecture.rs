use serde::Deserialize;

use self::{acme::Acme, madrid::Madrid};

mod acme;
mod madrid;

#[derive(Deserialize)]
pub enum ArchitectureKind {
    #[serde(rename = "ACME")]
    Acme,
    Madrid,
}

pub trait Architecture {
    type Instruction;

    /// Call this endpoint to execute a full program
    fn run() -> usize;

    /// Issue the instruction set for adding two numbers
    fn sum(lhs: usize, rhs: usize) -> Vec<String>;
    /// Issue the instruction set for multiplying two numbers
    fn mul(lhs: usize, rhs: usize) -> Vec<String>;
    /// Issue the instruction set for dividing two numbers
    fn div(lhs: usize, rhs: usize) -> Vec<String>;
}
