use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const TARGET: i32 = 2020;

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("day01/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut numbers = HashSet::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let num = line.parse::<i32>().unwrap();
            let needed = TARGET - num;
            if numbers.contains(&needed) {
                println!("{} {} {} {}", num, needed, num + needed, needed * num);
                return;
            } else {
                numbers.insert(num);
            }
        }
    }
}

fn part2() {
    let file = File::open("day01/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let num = line.parse::<i32>().unwrap();
            numbers.push(num);
        }
    }
    let mut set = HashSet::new();
    for &n in numbers.iter() {
        set.insert(n);
    }
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            let needed = TARGET - numbers[i] - numbers[j];
            if set.contains(&needed) {
                println!("{}", numbers[i] * numbers[j] * needed);
                return;
            }
        }
    }
}
