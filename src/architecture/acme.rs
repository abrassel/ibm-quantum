use reqwest::{blocking::Client, Url};

use crate::program::interpreted::{Id, InterpretedProgram, ProgramResult};

use super::{
    Architecture,
    Instruction::{self, *},
};

const ACME_PULSE_1: &'static str = "Acme_pulse_1";
const ACME_PULSE_2: &'static str = "Acme_pulse_2";
const ACME_INITIAL_STATE_PULSE: &'static str = "Acme_initial_state_pulse";

pub struct Acme {
    client: Client,
    url: Url,
}

impl Acme {
    pub fn new(url: Url) -> Self {
        let client = Client::new();

        Self { client, url }
    }

    fn load_program(&self, program: &InterpretedProgram) -> anyhow::Result<Id> {
        let res = self
            .client
            // note there is room for improvement here, duplicating string.  this happens elsewhere.
            .post(format!("{}load_program", self.url))
            .json(program)
            .send()?;

        Ok(res.json()?)
    }

    fn run_program(&self, prog_id: Id) -> anyhow::Result<ProgramResult> {
        let res = self
            .client
            .get(format!("{}run_program/{}", self.url, prog_id.program_id))
            .send()?;
        Ok(res.json()?)
    }
}

impl Architecture for Acme {
    fn run(&self, program: &InterpretedProgram) -> anyhow::Result<usize> {
        let id = self.load_program(program)?;
        Ok(self.run_program(id)?.result)
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
