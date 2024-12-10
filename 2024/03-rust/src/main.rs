use std::{env, fs};

use regex::{Captures, Regex};

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

    println!("Part 1 => {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_cleanup_and_compute_result() {
        let ans = cleanup_and_compute_result(&INPUT);
        assert_eq!(161, ans);
    }
}
