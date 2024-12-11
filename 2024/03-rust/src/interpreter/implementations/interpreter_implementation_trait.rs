use super::super::instructions::Instruction;

/// Methods that define the behavior of an `Interpreter`.
///
/// This Trait was intentionally defined as private to prevent `Interpreter` implementations
/// outside of the boundaries of this module.
pub trait InterpreterImplementation {
    fn handle_instruction(&mut self, instruction: &Instruction);

    fn accumulator(&self) -> u64;
}
