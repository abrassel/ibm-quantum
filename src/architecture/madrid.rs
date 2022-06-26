use super::{
    Architecture,
    Instruction::{self, *},
};

const MADRID_PULSE_1: &'static str = "Madrid_pulse_1";
const MADRID_PULSE_2: &'static str = "Madrid_pulse_2";
const MADRID_INITIAL_STATE_PULSE: &'static str = "Madrid_initial_state_pulse";

pub struct Madrid;

impl Architecture for Madrid {
    fn run(payload: &str) -> usize {
        todo!()
    }

    fn sum(rhs: usize) -> Vec<Instruction> {
        vec![Value(rhs), NamedInstruction(MADRID_PULSE_1)]
    }

    fn mul(rhs: usize) -> Vec<Instruction> {
        vec![
            Value(rhs),
            NamedInstruction(MADRID_PULSE_2),
            NamedInstruction(MADRID_PULSE_2),
        ]
    }

    fn div(rhs: usize) -> Vec<Instruction> {
        vec![
            Value(rhs),
            NamedInstruction(MADRID_PULSE_2),
            NamedInstruction(MADRID_PULSE_1),
        ]
    }

    fn initial_state(state: usize) -> Vec<Instruction> {
        vec![Value(state), NamedInstruction(MADRID_INITIAL_STATE_PULSE)]
    }
}