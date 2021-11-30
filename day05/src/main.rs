use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("day05/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut max = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let chars = line.as_bytes(); //I know all chars are 1 byte in size
            let mut row_upper = 127;
            let mut row_lower = 0;
            for i in 0..7 {
                match chars[i] {
                    b'F' => row_upper -= (row_upper - row_lower) / 2,
                    b'B' => row_lower += (row_upper - row_lower + 1) / 2,
                    _ => {}
                }
            }
            let mut col_upper = 7;
            let mut col_lower = 0;
            for i in 0..3 {
                match chars[i + 7] {
                    b'L' => col_upper -= (col_upper - col_lower) / 2,
                    b'R' => col_lower += (col_upper - col_lower + 1) / 2,
                    _ => {}
                }
            }
            max = std::cmp::max(max, row_lower * 8 + col_lower);
        }
    }
    println!("{}", max);
}

fn part2() {
    let file = File::open("day05/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut seats = Vec::with_capacity(128 * 8);
    for line in reader.lines() {
        if let Ok(line) = line {
            let chars = line.as_bytes(); //I know all chars are 1 byte in size
            let mut row_upper = 127;
            let mut row_lower = 0;
            for i in 0..7 {
                match chars[i] {
                    b'F' => row_upper -= (row_upper - row_lower) / 2,
                    b'B' => row_lower += (row_upper - row_lower + 1) / 2,
                    _ => {}
                }
            }
            let mut col_upper = 7;
            let mut col_lower = 0;
            for i in 0..3 {
                match chars[i + 7] {
                    b'L' => col_upper -= (col_upper - col_lower) / 2,
                    b'R' => col_lower += (col_upper - col_lower + 1) / 2,
                    _ => {}
                }
            }
            seats.push(row_lower * 8 + col_lower);
        }
    }
    seats.sort_unstable();
    let mut last = seats[0] - 1;
    for s in seats {
        if s != last + 1 {
            println!("{}", s - 1);
            return;
        }
        last = s;
    }
}
