mod instructions;
mod parser;
mod implementations;

use self::implementations::interpreter_implementation_trait::InterpreterImplementation;

pub use self::implementations::simple_interpreter::SimpleInterpreter;
pub use self::implementations::toggleable_interpreter::ToggleableInterpreter;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_interpreter() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let ans = SimpleInterpreter::eval(input);
        assert_eq!(161, ans);
    }

    #[test]
    fn test_toggleable_interpreter() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let ans = ToggleableInterpreter::eval(input);
        assert_eq!(48, ans);
    }
}
