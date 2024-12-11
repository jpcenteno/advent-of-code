mod instructions;
mod parser;

use self::instructions::Instruction;

pub struct SimpleInterpreter {
    accumulator: u64
}

impl SimpleInterpreter {
    pub fn eval(src: &str) -> u64 {
        let mut state = Self::default();

        parser::parse(src)
            .iter()
            .for_each(|instr| state.handle_instruction(instr));

        state.accumulator
    }

    fn handle_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            &Instruction::Mul(x, y) => { self.accumulator += x * y; }
            _ => (),
        }
    }
}

impl Default for SimpleInterpreter {
    fn default() -> Self {
        Self { accumulator: 0 }
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
