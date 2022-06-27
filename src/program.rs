use serde::Deserialize;

use crate::architecture::{Architecture, ArchitectureKind, Instruction};

use self::interpreted::InterpretedProgram;

pub mod deserialization;
pub(crate) mod interpreted;

#[derive(Deserialize)]
/// Represents a single program to execute.
pub struct Program {
    /// Each program has a unique identifer consisting of alphanumeric and "-"
    pub id: String,
    /// Each program is destined to be executed on a specific architecture.
    pub control_instrument: ArchitectureKind,
    initial_value: usize,
    operations: Vec<Operation>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
/// The current set of supported operations.
pub enum Operation {
    Sum { value: usize },
    Mul { value: usize },
    Div { value: usize },
}

impl Program {
    /// Run a program on a given [`Architecture`].
    pub fn interpret<Arch: Architecture>(
        &self,
        control_instrument: &Arch,
    ) -> anyhow::Result<usize> {
        let program_code = self.translate_operations(control_instrument);

        control_instrument.run(&InterpretedProgram { program_code })
    }

    /// Translate the operations into [`Architecture`] specific [`Instruction`]s.
    fn translate_operations<Arch: Architecture>(
        &self,
        control_instrument: &Arch,
    ) -> Vec<Instruction> {
        // Concat all instructions after interpretation
        let rest = self
            .operations
            .iter()
            .flat_map(|operation| control_instrument.apply_operation(operation));
        // prepend with initial state.
        let init = control_instrument
            .initial_state(self.initial_value)
            .into_iter();
        init.chain(rest).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::architecture::Instruction::*;
    use crate::architecture::{acme::Acme, madrid::Madrid};
    use crate::program::deserialization::ProgramInput;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_acme() {
        let program = ProgramInput::read_program_from_file("quantum_program_input.json")
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        let arch = Acme::new("http://foo").unwrap();

        let result = program.translate_operations(&arch);
        let expected = vec![
            NamedInstruction("Acme_initial_state_pulse"),
            Value(10),
            NamedInstruction("Acme_pulse_1"),
            NamedInstruction("Acme_pulse_2"),
            Value(120),
            NamedInstruction("Acme_pulse_2"),
            NamedInstruction("Acme_pulse_1"),
            NamedInstruction("Acme_pulse_1"),
            Value(3),
            NamedInstruction("Acme_pulse_2"),
            NamedInstruction("Acme_pulse_2"),
            Value(2),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_madrid() {
        let program = ProgramInput::read_program_from_file("quantum_program_input.json")
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        let arch = Madrid::new("http://foo").unwrap();

        let result = program.translate_operations(&arch);
        let expected = vec![
            Value(10),
            NamedInstruction("Madrid_initial_state_pulse"),
            Value(120),
            NamedInstruction("Madrid_pulse_1"),
            Value(3),
            NamedInstruction("Madrid_pulse_2"),
            NamedInstruction("Madrid_pulse_2"),
            Value(2),
            NamedInstruction("Madrid_pulse_2"),
            NamedInstruction("Madrid_pulse_1"),
        ];
        assert_eq!(result, expected);
    }
}
