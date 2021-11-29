use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::RegexSet;

fn main() {
    check(valid1);
    check(valid2);
}

fn check<F>(valid: F)
where
    F: Fn(&str) -> bool,
{
    let file = File::open("day4/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut count = 0;
    let mut passport = String::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if line != "" {
                passport.push(' ');
                passport.push_str(line.as_str());
            } else {
                if valid(&passport) {
                    count += 1;
                }
                passport.clear();
            }
        }
    }
    if valid(&passport) {
        count += 1;
    }
    println!("{}", count);
}

fn valid1(passport: &str) -> bool {
    let set = RegexSet::new(&[
        r"byr:", r"iyr:", r"eyr:", r"hgt:", r"hcl:", r"ecl:", r"pid:",
    ])
    .unwrap();
    set.matches(passport).iter().count() == 7
}

fn valid2(passport: &str) -> bool {
    let set = RegexSet::new(&[
        r"byr:((19[2-9][0-9])|(200[0-2]))(\s|$)",
        r"iyr:(201[0-9]|2020)(\s|$)",
        r"eyr:(202[0-9]|2030)(\s|$)",
        r"hgt:((59|6[0-9]|7[0-6])in|(1[5-8][0-9]|19[0-3])cm)(\s|$)",
        r"hcl:\#[a-f0-9]{6}(\s|$)",
        r"ecl:(amb|blu|brn|gry|grn|hzl|oth)(\s|$)",
        r"pid:(\d{9})(\s|$)",
    ])
    .unwrap();
    set.matches(passport).iter().count() == 7
}
