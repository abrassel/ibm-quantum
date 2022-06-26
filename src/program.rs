use serde::Deserialize;

use crate::architecture::{Architecture, ArchitectureKind, Instruction};

use self::interpreted::InterpretedProgram;

pub mod deserialization;
pub(crate) mod interpreted;

#[derive(Deserialize)]
pub struct Program {
    pub id: String,
    pub control_instrument: ArchitectureKind,
    initial_value: usize,
    operations: Vec<Operation>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Operation {
    Sum { value: usize },
    Mul { value: usize },
    Div { value: usize },
}

impl Program {
    pub fn interpret<Arch: Architecture>(
        &self,
        control_instrument: &Arch,
    ) -> anyhow::Result<usize> {
        let program_code: Vec<Instruction> = {
            let rest = self
                .operations
                .iter()
                .flat_map(|operation| control_instrument.apply_operation(operation));
            let init = control_instrument
                .initial_state(self.initial_value)
                .into_iter();
            init.chain(rest).collect()
        };

        control_instrument.run(&InterpretedProgram { program_code })
    }
}
