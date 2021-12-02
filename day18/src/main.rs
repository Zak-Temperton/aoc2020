use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
}

fn part1() {
    let mut sum = 0;
    let file = File::open("day18/res/input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            sum += evaluate(line.as_bytes()).0;
        }
    }
    println!("{}", sum);
}

enum Instruction {
    Add,
    Mull,
}

pub fn evaluate(line: &[u8]) -> (u64, usize) {
    let mut out = 0;
    let mut instruction = Instruction::Add;
    let mut i = 0;
    while i < line.len() {
        match line[i] {
            b'(' => {
                let (num, j) = evaluate(&line[(i + 1)..]);
                match instruction {
                    Instruction::Add => out += num,
                    Instruction::Mull => out *= num,
                }
                i += j;
            }
            b')' => return (out, i + 1),
            b'*' => instruction = Instruction::Mull,
            b'+' => instruction = Instruction::Add,
            b if b >= b'0' && b <= b'9' => match instruction {
                Instruction::Add => out += (b - b'0') as u64,
                Instruction::Mull => out *= (b - b'0') as u64,
            },
            _ => {}
        }
        i += 1;
    }
    (out, i)
}
