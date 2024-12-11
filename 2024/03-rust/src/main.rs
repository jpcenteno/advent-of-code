mod interpreter;

use std::{env, fs};

use interpreter::{Interpreter, SimpleInterpreter};

fn read_input() -> String {
    let file_path: String = env::args().skip(1).next().expect("USAGE: cargo run -- FILE");
    fs::read_to_string(file_path).expect("Error reading the input file.")
}

fn main() {
    let input = read_input();

    let result = SimpleInterpreter::eval(&input);

    println!("Part 1 => {}", result);
}
