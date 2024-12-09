use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn distance(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(r.to_owned()))
        .sum()
}

fn similarity(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    let mut right_count: HashMap<u32, u32> = HashMap::new();
    right.into_iter().for_each(|n| {
        *(right_count.entry(n.to_owned()).or_insert(0)) += 1;
    });
    left.into_iter().map(|x| x * right_count.get(x).unwrap_or(&0)).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path).expect("Failed to read input file");
    let reader = io::BufReader::new(file);

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        let left_value: u32 = parts[0].parse().unwrap();
        let right_value: u32 = parts[1].parse().unwrap();

        left.push(left_value);
        right.push(right_value);
    }

    left.sort();
    right.sort();

    let distance: u32 = distance(&left, &right);
    let similarity: u32 = similarity(&left, &right);

    println!("Part 1: {}", distance);
    println!("Part 2: {}", similarity);
}
