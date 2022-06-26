use reqwest::{blocking::Client, Url};

use crate::program::interpreted::{Id, InterpretedProgram, ProgramResult};

use super::{
    Architecture,
    Instruction::{self, *},
};

const MADRID_PULSE_1: &'static str = "Madrid_pulse_1";
const MADRID_PULSE_2: &'static str = "Madrid_pulse_2";
const MADRID_INITIAL_STATE_PULSE: &'static str = "Madrid_initial_state_pulse";

pub struct Madrid {
    client: Client,
    url: Url,
}

impl Madrid {
    pub fn new(url: Url) -> Self {
        let client = Client::new();
        Self { client, url }
    }

    fn load_program(&self, program: &InterpretedProgram) -> anyhow::Result<Id> {
        let res = self
            .client
            .post(format!("{}program/load", self.url))
            .json(program)
            .send()?;

        Ok(res.json()?)
    }

    fn run_program(&self, prog_id: Id) -> anyhow::Result<ProgramResult> {
        let res = self
            .client
            .get(format!("{}program/run/{}", self.url, prog_id.program_id))
            .send()?;
        Ok(res.json()?)
    }
}

impl Architecture for Madrid {
    fn run(&self, program: &InterpretedProgram) -> anyhow::Result<usize> {
        let id = self.load_program(program)?;
        Ok(self.run_program(id)?.result)
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
