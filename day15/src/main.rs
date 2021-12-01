use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = [2, 15, 0, 9, 1, 20];
    let mut last_said = HashMap::new();
    let mut next = 0;
    for (i, &n) in input.iter().enumerate() {
        let tmp = i - *last_said.entry(n).or_insert(i);
        last_said.insert(n, i);
        next = tmp;
    }
    for i in input.len()..2019 {
        let tmp = i - *last_said.entry(next).or_insert(i);
        last_said.insert(next, i);
        next = tmp;
    }
    println!("{}", next);
}

fn part2() {
    let input = [2, 15, 0, 9, 1, 20];
    let mut last_said = HashMap::new();
    let mut next = 0;
    for (i, &n) in input.iter().enumerate() {
        let tmp = i - *last_said.entry(n).or_insert(i);
        last_said.insert(n, i);
        next = tmp;
    }
    for i in input.len()..(30_000_000 - 1) {
        let tmp = i - *last_said.entry(next).or_insert(i);
        last_said.insert(next, i);
        next = tmp;
    }
    println!("{}", next);
}
