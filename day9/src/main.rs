use std::{
    cmp,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("day9/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut preamble = VecDeque::with_capacity(25);
    for line in reader.lines() {
        if let Ok(line) = line {
            let num = line.parse::<u64>().unwrap();
            if preamble.len() == 25 {
                let mut contains = false;
                for &i in preamble.iter() {
                    if preamble.contains(&(num - i)) {
                        contains = true;
                        break;
                    }
                }
                if !contains {
                    println!("{}", &num);
                    return;
                }
                preamble.pop_front();
                preamble.push_back(num);
            } else {
                preamble.push_back(num);
            }
        }
    }
}

fn part2() {
    let file = File::open("day9/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut preamble = VecDeque::with_capacity(25);
    let mut numbers = Vec::new();
    let mut invalid_num = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let num = line.parse::<u64>().unwrap();
            if preamble.len() == 25 {
                let mut contains = false;
                for &i in preamble.iter() {
                    if preamble.contains(&(num - i)) {
                        contains = true;
                        break;
                    }
                }
                if !contains {
                    invalid_num = num;
                    break;
                }
                preamble.pop_front();
                preamble.push_back(num);
                numbers.push(num);
            } else {
                preamble.push_back(num);
                numbers.push(num);
            }
        }
    }
    let mut begin = numbers.len() - 1;
    let mut end = begin;
    let mut sum = numbers[end];
    loop {
        match sum.cmp(&invalid_num) {
            cmp::Ordering::Less => {
                begin -= 1;
                sum += numbers[begin];
            }
            cmp::Ordering::Equal => {
                let range = numbers[begin..=end].iter();

                println!("{}", range.clone().min().unwrap() + range.max().unwrap());
                break;
            }
            cmp::Ordering::Greater => {
                if end == begin {
                    end -= 1;
                    begin = end;
                    sum = numbers[end];
                } else {
                    sum -= numbers[end];
                    end -= 1;
                }
            }
        }
    }
}
