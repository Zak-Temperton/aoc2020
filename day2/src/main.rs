use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("day2/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let r = Regex::new(r"(?:(\d+))-(?:(\d+)) (?:(\w)): (?:(\w+))").unwrap();
    let mut count = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(c) = r.captures(&line) {
                let min = c.get(1).unwrap().as_str().parse().unwrap();
                let max = c.get(2).unwrap().as_str().parse().unwrap();
                let k = c.get(3).unwrap().as_str().parse().unwrap();
                let password = c.get(4).unwrap().as_str();
                if is_valid1(k, min, max, password) {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

fn is_valid1(k: char, min: u8, max: u8, password: &str) -> bool {
    let mut count = 0;
    for c in password.chars() {
        if c == k {
            count += 1;
        }
    }
    count >= min && count <= max
}

fn part2() {
    let file = File::open("day2/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let r = Regex::new(r"(?:(\d+))-(?:(\d+)) (?:(\w)): (?:(\w+))").unwrap();
    let mut count = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(c) = r.captures(&line) {
                let i1 = c.get(1).unwrap().as_str().parse().unwrap();
                let i2 = c.get(2).unwrap().as_str().parse().unwrap();
                let k = c.get(3).unwrap().as_str().parse::<char>().unwrap() as u8;
                let password = c.get(4).unwrap().as_str().as_bytes();
                if is_valid2(k, i1, i2, password) {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

fn is_valid2(k: u8, i1: usize, i2: usize, password: &[u8]) -> bool {
    let len = password.len();
    match (
        i1 <= len && password[i1 - 1] == k as u8,
        i2 <= len && password[i2 - 1] == k as u8,
    ) {
        (true, true) | (false, false) => false,
        (true, false) | (false, true) => true,
    }
}
