mod interpreter;

use std::{env, fs};

use interpreter::*;

fn read_input() -> String {
    let file_path: String = env::args().skip(1).next().expect("USAGE: cargo run -- FILE");
    fs::read_to_string(file_path).expect("Error reading the input file.")
}

fn main() {
    let input = read_input();

    let result_1 = SimpleInterpreter::eval(&input);
    let result_2 = ToggleableInterpreter::eval(&input);

    println!("Part 1 => {}", result_1);
    println!("Part 2 => {}", result_2);
}
