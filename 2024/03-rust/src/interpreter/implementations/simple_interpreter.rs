use super::interpreter_implementation_trait::InterpreterImplementation;
use super::super::instructions::Instruction;

/// This interpreter computes the solution for the first part of the problem. It will process all
/// the `mul` instructions while ignoring the `do()` and `don't()` instructions.
pub struct SimpleInterpreter {
    accumulator: u64
}

impl Default for SimpleInterpreter {
    fn default() -> Self {
        Self { accumulator: 0 }
    }
}

impl InterpreterImplementation for SimpleInterpreter {
    fn handle_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            &Instruction::Mul(x, y) => { self.accumulator += x * y; }
            _ => (),
        }
    }

    fn accumulator(&self) -> u64 {
        self.accumulator
    }
}
