use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    panic,
    sync::mpsc::channel,
};

fn main() {
    part1();
    part2();
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn part1() {
    let file = File::open("day8/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut instructions = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let num = line[4..].parse().unwrap();
            let opp = &line[..3];
            instructions.push(match opp {
                "nop" => Instruction::Nop(num),
                "acc" => Instruction::Acc(num),
                "jmp" => Instruction::Jmp(num),
                _ => panic!(),
            });
        }
    }

    println!("{}", {
        let mut visited = HashSet::new();
        let mut acc = 0;
        let mut i = 0;
        loop {
            if (i as usize) >= instructions.len() || !visited.insert(i) {
                break acc;
            } else {
                match instructions[i as usize] {
                    Instruction::Acc(n) => {
                        acc += n;
                        i += 1;
                    }
                    Instruction::Jmp(n) => i += n,
                    Instruction::Nop(_) => i += 1,
                }
            }
        }
    });
}

fn part2() {
    let file = File::open("day8/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut instructions = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let num = line[4..].parse().unwrap();
            let opp = &line[..3];
            instructions.push(match opp {
                "nop" => Instruction::Nop(num),
                "acc" => Instruction::Acc(num),
                "jmp" => Instruction::Jmp(num),
                _ => panic!(),
            });
        }
    }
    println!("{}", {
        let mut changed = 0;
        loop {
            match instructions[changed] {
                Instruction::Acc(_) => {}
                Instruction::Jmp(n) => {
                    instructions[changed] = Instruction::Nop(n);
                    if let Some(ans) = test(&instructions) {
                        break ans;
                    } else {
                        instructions[changed] = Instruction::Jmp(n);
                    }
                }
                Instruction::Nop(n) => {
                    instructions[changed] = Instruction::Jmp(n);
                    if let Some(ans) = test(&instructions) {
                        break ans;
                    } else {
                        instructions[changed] = Instruction::Nop(n);
                    }
                }
            }
            changed += 1;
        }
    });
}
fn test(instructions: &Vec<Instruction>) -> Option<i32> {
    let mut visited = HashSet::new();
    let mut acc = 0;
    let mut i = 0;
    loop {
        if (i as usize) == instructions.len() {
            break Some(acc);
        } else if !visited.insert(i) || (i as usize) > instructions.len() {
            break None;
        } else {
            match instructions[i as usize] {
                Instruction::Acc(n) => {
                    acc += n;
                    i += 1;
                }
                Instruction::Jmp(n) => i += n,
                Instruction::Nop(_) => i += 1,
            }
        }
    }
}
