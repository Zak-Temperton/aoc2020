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
    for line in reader.lines() {
        if let Ok(line) = line {
            match state {
                State::Rules => {
                    if line == "" {
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
                    if line == "" {
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
    }
    println!("{}", total);
}

// fn part2() {
//     let file = File::open("day16/res/input.txt").expect("Failed to load input.txt");
//     let reader = BufReader::new(file);
//     let r_rules =
//         Regex::new(r"[\w\s]+: (?:([\d]+))\-(?:([\d]+)) or (?:([\d]+))\-(?:([\d]+))$").unwrap();
//     let mut rules: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = Vec::new();
//     let mut state = State::Rules;
//     let mut tickets = Vec::new();
//     for line in reader.lines() {
//         if let Ok(line) = line {
//             match state {
//                 State::Rules => {
//                     if line == "" {
//                         state = State::Transition1;
//                         continue;
//                     }
//                     let captures = r_rules.captures(line.as_str()).unwrap();
//                     rules.push((
//                         captures.get(1).unwrap().as_str().parse().unwrap()
//                             ..=captures.get(2).unwrap().as_str().parse().unwrap(),
//                         captures.get(3).unwrap().as_str().parse().unwrap()
//                             ..=captures.get(4).unwrap().as_str().parse().unwrap(),
//                     ));
//                 }
//                 State::MyTicket => {
//                     if line == "" {
//                         state = State::Transition2
//                     }
//                 }
//                 State::NearbyTickets => {
//                     let fields = line
//                         .split(',')
//                         .map(|s| s.parse().unwrap())
//                         .collect::<Vec<_>>();
//                     let mut valid = false;
//                     for field in fields.iter() {
//                         for rule in rules.iter() {
//                             if rule.0.contains(field) || rule.1.contains(field) {
//                                 valid = true;
//                             }
//                         }
//                         if !valid {
//                             break;
//                         }
//                     }
//                     if valid {
//                         tickets.push(fields);
//                     }
//                 }
//                 State::Transition1 => state = State::MyTicket,
//                 State::Transition2 => state = State::NearbyTickets,
//             }
//         }
//     }

//     let mut valid_rules = vec![vec![true; rules.len()]; tickets[0].len()];
//     for f in 0..tickets[0].len() {
//         for ticket in tickets.iter() {
//             for (r, rule) in rules.iter().enumerate() {
//                 if !(rule.0.contains(&ticket[f]) || rule.1.contains(&ticket[f])) {
//                     valid_rules[f][r] = false;
//                 }
//             }
//         }
//     }
//     println!("{:?}", &valid_rules);
//     let mut sums = vec![0; tickets[0].len()];
//     for (i, r) in valid_rules.iter().enumerate() {
//         for &b in r {
//             if b {
//                 sums[i] += 1;
//             }
//         }
//     }
//     println!("{:?}", sums);
// }
