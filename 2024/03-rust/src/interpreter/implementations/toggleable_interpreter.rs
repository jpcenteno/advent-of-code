use super::interpreter_implementation_trait::InterpreterImplementation;
use super::super::instructions::Instruction;

#[derive(PartialEq, Eq)]
enum State {
    Do,
    Dont,
}

pub struct ToggleableInterpreter {
    accumulator: u64,
    state: State,
}

impl ToggleableInterpreter {
    fn handle_mul(&mut self, x: u64, y: u64) {
        if self.state == State::Do {
            self.accumulator += x * y;
        }
    }
}

impl Default for ToggleableInterpreter {
    fn default() -> Self {
        Self { accumulator: 0, state: State::Do }
    }
}

impl InterpreterImplementation for ToggleableInterpreter {
    fn handle_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            &Instruction::Do => { self.state = State::Do; },
            &Instruction::Dont => { self.state = State::Dont },
            &Instruction::Mul(x, y) => self.handle_mul(x, y),
            &Instruction::Ignored => ()
        }
    }

    fn accumulator(&self) -> u64 {
        self.accumulator
    }
}
