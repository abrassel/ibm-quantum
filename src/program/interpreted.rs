use serde::{Deserialize, Serialize};

use crate::architecture::Instruction;

#[derive(Serialize)]
/// The code after we have initially interpreted it to a given [`Architecture`](crate::architecture::Architecture).
pub struct InterpretedProgram {
    pub program_code: Vec<Instruction>,
}

#[derive(Deserialize)]
/// The id for a given program after it has been loaded into an [`Architecture`](crate::architecture::Architecture).
pub(crate) struct Id {
    pub program_id: String,
}

#[derive(Deserialize)]
/// The result of a program after the [`Architecture`](crate::architecture::Architecture) has executed it.
pub(crate) struct ProgramResult {
    pub result: usize,
}
