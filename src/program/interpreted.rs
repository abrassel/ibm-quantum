use serde::Serialize;

use crate::architecture::Instruction;

#[derive(Serialize)]
pub struct InterpretedProgram {
    pub program_code: Vec<Instruction>,
}
