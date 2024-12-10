use std::{env, fs};

use regex::{Captures, Regex};
use chumsky::prelude::*;

#[derive(Clone, Debug)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
    Ignored
}

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

fn read_input() -> String {
    let file_path: String = env::args().skip(1).next().expect("USAGE: cargo run -- FILE");
    fs::read_to_string(file_path).expect("Error reading the input file.")
}

fn mul(captures: &Captures) -> u64 {
    let a: u64 = captures.get(1).unwrap().as_str().parse().unwrap();
    let b: u64 = captures.get(2).unwrap().as_str().parse().unwrap();

    a * b
}

fn cleanup_and_compute_result(s: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&s).map(|c| mul(&c)).sum()
}

fn main() {
    let input = read_input();

    let result = cleanup_and_compute_result(&input);

    match parser().parse(input) {
        Ok(ast) => ast.iter().for_each(|instruction| println!("{:?}", instruction)),
        Err(errs) => errs.into_iter().for_each(|e| println!("{:?}", e)),
    }

    println!("Part 1 => {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    static INPUT_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_cleanup_and_compute_result() {
        let ans = cleanup_and_compute_result(&INPUT_1);
        assert_eq!(161, ans);
    }
}
