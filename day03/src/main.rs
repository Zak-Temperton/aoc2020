use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("day03/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut count = 0;
    for (y, line) in reader.lines().flatten().enumerate() {
        let x = (3 * y) % line.len();
        if line.as_bytes()[x] == b'#' {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part2() {
    let file = File::open("day03/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut counts = [0; 5];
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]; //(across, down)
    for (y, line) in reader.lines().flatten().enumerate() {
        for (i, slope) in slopes.iter().enumerate() {
            if y % slope.0 == 0 {
                let x = (slope.1 * y / slope.0) % line.len();
                if line.as_bytes()[x] == b'#' {
                    counts[i] += 1;
                }
            }
        }
    }
    println!("{}", counts.iter().product::<usize>());
}
