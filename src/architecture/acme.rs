use super::{
    Architecture,
    Instruction::{self, *},
};

const ACME_PULSE_1: &'static str = "Acme_pulse_1";
const ACME_PULSE_2: &'static str = "Acme_pulse_2";
const ACME_INITIAL_STATE_PULSE: &'static str = "Acme_initial_state_pulse";

pub struct Acme;

impl Architecture for Acme {
    fn run(&self, id: &str, instructions: &[Instruction]) -> usize {
        todo!()
    }

    fn sum(&self, rhs: usize) -> Vec<Instruction> {
        vec![
            NamedInstruction(ACME_PULSE_1),
            NamedInstruction(ACME_PULSE_2),
            Value(rhs),
        ]
    }

    fn mul(&self, rhs: usize) -> Vec<Instruction> {
        vec![
            NamedInstruction(ACME_PULSE_2),
            NamedInstruction(ACME_PULSE_1),
            NamedInstruction(ACME_PULSE_1),
            Value(rhs),
        ]
    }

    fn div(&self, rhs: usize) -> Vec<Instruction> {
        vec![
            NamedInstruction(ACME_PULSE_2),
            NamedInstruction(ACME_PULSE_2),
            Value(rhs),
        ]
    }

    fn initial_state(&self, state: usize) -> Vec<Instruction> {
        vec![NamedInstruction(ACME_INITIAL_STATE_PULSE), Value(state)]
    }
}
