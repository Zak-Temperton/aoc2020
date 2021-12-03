use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

use regex::Regex;

fn main() {
    part1();
    // part2();
}

enum State {
    Rules,
    Transition1,
    MyTicket,
    Transition2,
    NearbyTickets,
}

fn part1() {
    let file = File::open("day16/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let r_rules =
        Regex::new(r"[\w\s]+: (?:([\d]+))\-(?:([\d]+)) or (?:([\d]+))\-(?:([\d]+))$").unwrap();
    let mut rules: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = Vec::new();
    let mut state = State::Rules;
    let mut total = 0;
    for line in reader.lines().flatten() {
        match state {
            State::Rules => {
                if line.is_empty() {
                    state = State::Transition1;
                    continue;
                }
                let captures = r_rules.captures(line.as_str()).unwrap();
                rules.push((
                    captures.get(1).unwrap().as_str().parse().unwrap()
                        ..=captures.get(2).unwrap().as_str().parse().unwrap(),
                    captures.get(3).unwrap().as_str().parse().unwrap()
                        ..=captures.get(4).unwrap().as_str().parse().unwrap(),
                ));
            }
            State::MyTicket => {
                if line.is_empty() {
                    state = State::Transition2
                }
            }
            State::NearbyTickets => {
                let fields = line
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>();
                for field in fields {
                    let mut valid = false;
                    for rule in rules.iter() {
                        if rule.0.contains(&field) || rule.1.contains(&field) {
                            valid = true;
                        }
                    }
                    if !valid {
                        total += field;
                    }
                }
            }
            State::Transition1 => state = State::MyTicket,
            State::Transition2 => state = State::NearbyTickets,
        }
    }
    println!("{}", total);
}
