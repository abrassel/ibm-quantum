use super::{Architecture, Instruction};

pub struct Acme;

impl Architecture for Acme {
    fn run(payload: &str) -> usize {
        todo!()
    }

    fn sum(lhs: usize, rhs: usize) -> Vec<Instruction> {
        todo!()
    }

    fn mul(lhs: usize, rhs: usize) -> Vec<Instruction> {
        todo!()
    }

    fn div(lhs: usize, rhs: usize) -> Vec<Instruction> {
        todo!()
    }
}
