use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("day10/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut adapters = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            adapters.push(line.parse().unwrap());
        }
    }
    adapters.sort_unstable();
    let mut last = 0;
    let mut j1 = 0;
    let mut j3 = 1;
    for i in adapters {
        if i - last == 1 {
            j1 += 1;
        } else if i - last == 3 {
            j3 += 1;
        }
        last = i;
    }
    println!("{}", j1 * j3);
}

fn part2() {
    let file = File::open("day10/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut adapters: Vec<usize> = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            adapters.push(line.parse().unwrap());
        }
    }
    adapters.sort_unstable_by(|&a, &b| b.partial_cmp(&a).unwrap()); //sort in reverse
    adapters.push(0); //add starting point
    let mut paths: Vec<usize> = vec![0; adapters[0] + 4];
    *paths.last_mut().unwrap() = 1;
    for &adapter in adapters.iter() {
        paths[adapter] = paths[adapter + 1] + paths[adapter + 2] + paths[adapter + 3];
    }
    println!("{}", paths[0]);
}
