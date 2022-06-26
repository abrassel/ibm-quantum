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
pub struct Operation {
    pub(crate) r#type: OperationKind,
    pub(crate) value: usize,
}

#[derive(Deserialize)]
pub enum OperationKind {
    Sum,
    Mul,
    Div,
}

impl Program {
    pub fn interpret(&self) -> anyhow::Result<usize> {
        let Self {
            id: _,
            control_instrument,
            initial_value,
            operations,
        } = self;
        let program_code: Vec<Instruction> = {
            let rest = operations
                .iter()
                .flat_map(|operation| control_instrument.apply_operation(operation));
            let init = control_instrument.initial_state(*initial_value).into_iter();
            init.chain(rest).collect()
        };

        control_instrument.run(&InterpretedProgram { program_code })
    }
}
