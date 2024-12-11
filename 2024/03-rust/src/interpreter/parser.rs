use super::instructions::Instruction;
use chumsky::prelude::*;


fn operand_parser() -> impl Parser<char, u64, Error = Simple<char>> {
        filter(|c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1)
        .at_most(3)
        .map(|chars: Vec<char>| chars.iter().collect())
        .map(|s: String| s.parse::<u64>().unwrap())
}

fn mul_parser() -> impl Parser<char, (u64, u64), Error = Simple<char>> {
        just("mul(")
        .ignore_then(operand_parser())
        .then_ignore(just(","))
        .then(operand_parser())
        .then_ignore(just(")"))
}

fn parser() -> impl Parser<char, Vec<Instruction>, Error = Simple<char>> {
    choice((
        just("do()").to(Instruction::Do),
        just("don't()").to(Instruction::Dont),
        mul_parser().map(|(x, y)| Instruction::Mul(x, y)),
        any().to(Instruction::Ignored),
    ))
    .repeated()
    .collect()
}

pub fn parse<'a>(s: &'a str) -> Vec<Instruction> {
    parser().parse(s).unwrap().iter().cloned().collect()
}
