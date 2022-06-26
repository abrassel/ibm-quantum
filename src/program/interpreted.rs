use serde::{Deserialize, Serialize};

use crate::architecture::Instruction;

#[derive(Serialize)]
pub struct InterpretedProgram {
    pub program_code: Vec<Instruction>,
}

#[derive(Deserialize)]
pub(crate) struct Id {
    pub program_id: String,
}

#[derive(Deserialize)]
pub(crate) struct ProgramResult {
    pub result: usize,
}
