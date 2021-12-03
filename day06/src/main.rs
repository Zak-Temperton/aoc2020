use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("day06/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut answers = HashSet::with_capacity(26);
    let mut total = 0;
    for line in reader.lines().flatten() {
        if !line.is_empty() {
            for c in line.chars() {
                answers.insert(c);
            }
        } else {
            total += answers.len();
            answers.clear();
        }
    }

    total += answers.len();
    println!("{}", total);
}

fn part2() {
    let file = File::open("day06/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut answers = HashMap::with_capacity(26);
    let mut total = 0;
    let mut people = 0;
    for line in reader.lines().flatten() {
        if !line.is_empty() {
            for c in line.chars() {
                *answers.entry(c).or_insert(0) += 1;
            }
            people += 1;
        } else {
            total += answers.iter().filter(|(_, &i)| i == people).count();
            answers.clear();
            people = 0;
        }
    }
    total += answers.len();
    println!("{}", total);
}
