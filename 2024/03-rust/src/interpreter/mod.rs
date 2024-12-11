mod instructions;
mod parser;

use self::instructions::Instruction;

/// Methods that define the behavior of an `Interpreter`.
///
/// This Trait was intentionally defined as private to prevent `Interpreter` implementations
/// outside of the boundaries of this module.
trait InterpreterImplementation {
    fn handle_instruction(&mut self, instruction: &Instruction);

    fn accumulator(&self) -> u64;
}

#[allow(private_bounds)]
pub trait Interpreter: Default + InterpreterImplementation {
    /// Interprets a string of source code and returns it's result.
    fn eval(src: &str) -> u64 {
        let mut state = Self::default();

        parser::parse(src)
            .iter()
            .for_each(|instr| state.handle_instruction(instr));

        state.accumulator()
    }
}

impl<T> Interpreter for T where T: InterpreterImplementation + Default {}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_interpreter() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let ans = SimpleInterpreter::eval(input);
        assert_eq!(161, ans);
    }
}
