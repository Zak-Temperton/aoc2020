use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    part1();
}
#[derive(Debug)]
enum Rule {
    Ordered(Vec<usize>),
    Divergent(Vec<usize>, Vec<usize>),
    Char(u8),
}
enum State {
    Rules,
    Messages,
}
fn part1() {
    let mut rules = HashMap::new();
    let mut state = State::Rules;
    let r_rule_ordered = Regex::new(r"(?:(\d+)): (?:([\d\s]+))$").unwrap();
    let r_rule_divergent = Regex::new(r"(?:(\d+)): (?:([\d\s]+)) \| (?:([\d\s]+))").unwrap();
    let r_rule_char = Regex::new(r#"(?:(\d+)): "(?:([a-z]))"$"#).unwrap();
    let mut count = 0;
    let mut r_rules = Regex::new("").unwrap();

    let file = File::open("day19/res/input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            match state {
                State::Rules => {
                    if line == "" {
                        state = State::Messages;
                        let r = format!("^{}$", parse_to_regex_str(0, &rules));
                        println!("{}", r);
                        r_rules = Regex::new(r.as_str()).unwrap();
                    } else {
                        if let Some(captures) = r_rule_ordered.captures(line.as_str()) {
                            let r = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            let v = captures
                                .get(2)
                                .unwrap()
                                .as_str()
                                .split(' ')
                                .map(|s| s.parse().unwrap())
                                .collect();
                            rules.insert(r, Rule::Ordered(v));
                        } else if let Some(captures) = r_rule_divergent.captures(line.as_str()) {
                            let r = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            let v1 = captures
                                .get(2)
                                .unwrap()
                                .as_str()
                                .split(' ')
                                .map(|s| s.parse().unwrap())
                                .collect();
                            let v2 = captures
                                .get(3)
                                .unwrap()
                                .as_str()
                                .split(' ')
                                .map(|s| s.parse().unwrap())
                                .collect();
                            rules.insert(r, Rule::Divergent(v1, v2));
                        } else if let Some(captures) = r_rule_char.captures(line.as_str()) {
                            let r = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            let a = captures.get(2).unwrap().as_str().as_bytes();
                            rules.insert(r, Rule::Char(a[0]));
                        } else {
                            println!("{}", line);
                        }
                    }
                }
                State::Messages => {
                    if r_rules.is_match(line.as_str()) {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
}

//solution credit to https://github.com/sporksmith/aoc2020/blob/0ae0b27417f72c18b0236f4d706ea52f01dabeab/src/d19_messages.rs
fn parse_to_regex_str(rule: usize, rules: &HashMap<usize, Rule>) -> String {
    match rules.get(&rule).unwrap() {
        Rule::Ordered(a) => a.iter().map(|&i| parse_to_regex_str(i, &rules)).collect(),
        Rule::Divergent(a, b) => {
            format!(
                "({}|{})",
                a.iter()
                    .map(|&i| parse_to_regex_str(i, &rules))
                    .collect::<String>(),
                b.iter()
                    .map(|&i| parse_to_regex_str(i, &rules))
                    .collect::<String>()
            )
        }
        Rule::Char(c) => format!("{}", *c as char),
    }
}
