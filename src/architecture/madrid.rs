use crate::program::interpreted::InterpretedProgram;

use super::{
    Architecture,
    Instruction::{self, *},
};

const MADRID_PULSE_1: &'static str = "Madrid_pulse_1";
const MADRID_PULSE_2: &'static str = "Madrid_pulse_2";
const MADRID_INITIAL_STATE_PULSE: &'static str = "Madrid_initial_state_pulse";

pub struct Madrid;

impl Architecture for Madrid {
    fn run(&self, program: &InterpretedProgram) -> anyhow::Result<usize> {
        todo!()
    }

    fn sum(&self, rhs: usize) -> Vec<Instruction> {
        vec![Value(rhs), NamedInstruction(MADRID_PULSE_1)]
    }

    fn mul(&self, rhs: usize) -> Vec<Instruction> {
        vec![
            Value(rhs),
            NamedInstruction(MADRID_PULSE_2),
            NamedInstruction(MADRID_PULSE_2),
        ]
    }

    fn div(&self, rhs: usize) -> Vec<Instruction> {
        vec![
            Value(rhs),
            NamedInstruction(MADRID_PULSE_2),
            NamedInstruction(MADRID_PULSE_1),
        ]
    }

    fn initial_state(&self, state: usize) -> Vec<Instruction> {
        vec![Value(state), NamedInstruction(MADRID_INITIAL_STATE_PULSE)]
    }
}
