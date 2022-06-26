use super::{
    Architecture,
    Instruction::{self, *},
};

pub struct Acme;

const ACME_PULSE_1: &'static str = "Acme_pulse_1";
const ACME_PULSE_2: &'static str = "Acme_pulse_2";
const ACME_INITIAL_STATE_PULSE: &'static str = "Acme_initial_state_pulse";

impl Architecture for Acme {
    fn run(payload: &str) -> usize {
        todo!()
    }

    fn sum(rhs: usize) -> Vec<Instruction> {
        vec![
            NamedInstruction(ACME_PULSE_1),
            NamedInstruction(ACME_PULSE_2),
            Value(rhs),
        ]
    }

    fn mul(rhs: usize) -> Vec<Instruction> {
        vec![
            NamedInstruction(ACME_PULSE_2),
            NamedInstruction(ACME_PULSE_1),
            NamedInstruction(ACME_PULSE_1),
            Value(rhs),
        ]
    }

    fn div(rhs: usize) -> Vec<Instruction> {
        vec![
            NamedInstruction(ACME_PULSE_2),
            NamedInstruction(ACME_PULSE_2),
            Value(rhs),
        ]
    }

    fn initial_state(state: usize) -> Vec<Instruction> {
        vec![NamedInstruction(ACME_INITIAL_STATE_PULSE), Value(state)]
    }
}
