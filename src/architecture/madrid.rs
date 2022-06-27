use reqwest::{blocking::Client, IntoUrl, Url};

use crate::program::interpreted::{Id, InterpretedProgram, ProgramResult};

use super::{
    Architecture,
    Instruction::{self, *},
};

const MADRID_PULSE_1: &'static str = "Madrid_pulse_1";
const MADRID_PULSE_2: &'static str = "Madrid_pulse_2";
const MADRID_INITIAL_STATE_PULSE: &'static str = "Madrid_initial_state_pulse";

/// Represents the Madrid architecture.
pub struct Madrid {
    client: Client,
    url: Url,
}

impl Madrid {
    /// Create new `Acme` architecture with target server.
    pub fn new<U: IntoUrl>(url: U) -> anyhow::Result<Self> {
        let client = Client::new();
        Ok(Self {
            client,
            url: url.into_url()?,
        })
    }

    /// Loading the program into the server is the first step to execute.
    fn load_program(&self, program: &InterpretedProgram) -> anyhow::Result<Id> {
        let res = self
            .client
            .post(format!("{}program/load", self.url))
            .json(program)
            .send()?;

        Ok(res.json()?)
    }

    /// After loading the program, it can be run with the Id handle returned.
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
