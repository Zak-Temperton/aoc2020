use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2b();
}

fn part1() {
    let file = File::open("day13/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut min_wait = 0;
    let mut min_depart = 0;
    let mut min_bus = 0;
    for (i, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if i == 0 {
                min_wait = line.parse::<i32>().unwrap();
                min_depart = min_wait;
            } else {
                for s in line.split(',') {
                    if s != "x" {
                        let bus = s.parse::<i32>().unwrap();
                        let wait = bus - min_depart % bus;
                        if wait < min_wait {
                            min_wait = wait;
                            min_bus = bus;
                        }
                    }
                }
            }
        }
    }
    println!("{}", min_wait * min_bus);
}

//my brute force method
#[allow(dead_code)]
fn part2() {
    let file = File::open("day13/res/example.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut busses = Vec::new();
    for line in reader.lines().skip(1) {
        if let Ok(line) = line {
            for s in line.split(',').enumerate() {
                if s.1 != "x" {
                    busses.push((s.0 as i128, s.1.parse::<i128>().unwrap()));
                }
            }
        }
    }

    let max = busses
        .iter()
        .max_by(|&a, &b| a.1.cmp(&b.1))
        .unwrap()
        .clone();

    for bus in busses.iter_mut() {
        bus.0 -= max.0;
    }

    let mut min_wait = 0;
    println!(
        "{}",
        loop {
            let mut found = true;

            for bus in busses.iter() {
                if (min_wait + bus.0) % bus.1 != 0 {
                    found = false;
                    break;
                }
            }

            if found {
                break min_wait - max.0;
            }
            min_wait += max.1;
        }
    );
}

// solution credit to https://github.com/LennardF1989/AdventOfCode2020/blob/6f011bb607ae562daf008682defda44d47f2e30d/Src/AdventOfCode2020/Days/Day13.cs#L198
fn part2b() {
    let file = File::open("day13/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut busses = Vec::new();
    for line in reader.lines().skip(1) {
        if let Ok(line) = line {
            for s in line.split(',') {
                if s != "x" {
                    busses.push(s.parse::<usize>().unwrap());
                } else {
                    busses.push(0);
                }
            }
        }
    }

    let mut wait = 0;
    let mut increment = busses[0];
    let mut offset = 1;
    while offset < busses.len() {
        if busses[offset] == 0 {
            offset += 1;
        } else {
            wait += increment;
            if (wait + offset) % busses[offset] == 0 {
                increment *= busses[offset];
                offset += 1;
            }
        }
    }

    println!("{}", wait);
}
