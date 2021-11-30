use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("day14/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let r_mask = Regex::new(r"mask = (?:([X01]+))").unwrap();
    let r_mem = Regex::new(r"mem\[(:?([\d]+))\] = (?:([\d]+))").unwrap();
    let mut mask = Vec::new();
    let mut map = HashMap::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if line[..4].to_string() == "mask".to_string() {
                mask = r_mask
                    .captures(line.as_str())
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .as_bytes()
                    .to_vec();
            } else {
                let captures = r_mem.captures(line.as_str()).unwrap();
                let address: u64 = captures.get(2).unwrap().as_str().parse().unwrap();
                let mem: u64 = captures.get(3).unwrap().as_str().parse().unwrap();
                let mut ans: u64 = 0;
                for i in (0..mask.len()).rev() {
                    ans <<= 1;
                    if (mask[mask.len() - 1 - i] == b'X' && (mem >> i) & 1 == 1)
                        || mask[mask.len() - 1 - i] == b'1'
                    {
                        ans += 1;
                    }
                }
                *map.entry(address).or_default() = ans;
            }
        }
    }
    println!("{}", map.iter().fold(0_u64, |j, (_, &i)| j + i));
}

fn part2() {
    let file = File::open("day14/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let r_mask = Regex::new(r"mask = (?:([X01]+))").unwrap();
    let r_mem = Regex::new(r"mem\[(:?([\d]+))\] = (?:([\d]+))").unwrap();
    let mut mask = 0;
    let mut map = HashMap::new(); // memory
    let mut xi = Vec::new(); //indeces of 'X's
    for line in reader.lines() {
        if let Ok(line) = line {
            if line[..4].to_string() == "mask".to_string() {
                let mask_raw = r_mask
                    .captures(line.as_str())
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str();

                xi = mask_raw
                    .bytes()
                    .enumerate()
                    .fold(Vec::new(), |mut j, (i, b)| {
                        if b == b'X' {
                            j.push(mask_raw.len() - 1 - i);
                        }
                        j
                    });

                mask = 0;
                for n in mask_raw.chars() {
                    mask <<= 1;
                    if n == '1' {
                        mask |= 1;
                    }
                }
            } else {
                let captures = r_mem.captures(line.as_str()).unwrap();
                let mut address_raw: Vec<u64> =
                    vec![captures.get(2).unwrap().as_str().parse::<u64>().unwrap() | mask];
                let mem: u64 = captures.get(3).unwrap().as_str().parse().unwrap();

                for &x in xi.iter() {
                    let mut new_addresses = Vec::new();
                    for &address in address_raw.iter() {
                        let mut address = address;
                        address ^= 1 << x;
                        new_addresses.push(address);
                    }
                    address_raw.append(&mut new_addresses);
                }
                for address in address_raw {
                    *map.entry(address).or_default() = mem;
                }
            }
        }
    }
    println!("{}", map.iter().fold(0_u64, |j, (_, &i)| j + i));
}
