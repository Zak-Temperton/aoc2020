use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("day07/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let r = Regex::new(r"(?:([\w\s]+)) bags contain [\d]+ (?:(.+)).").unwrap();
    let r_bags = Regex::new(r" (bags|bag)").unwrap();
    let r_split = Regex::new(r", [\d]+ ").unwrap();
    let mut bags = HashMap::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(captures) = r.captures(line.as_str()) {
                let parent = captures.get(1).unwrap().as_str().to_string();
                let tmp = &*r_bags.replace_all(&captures.get(2).unwrap().as_str(), "");
                let children = r_split.split(tmp).collect::<Vec<_>>();
                for child in children {
                    bags.entry(child.to_string())
                        .or_insert(Vec::new())
                        .push(parent.clone());
                }
            }
        }
    }
    let mut to_search = HashSet::new();
    to_search.insert("shiny gold".to_string());
    let mut searched = HashSet::new();
    let mut count = 0;
    while !to_search.is_empty() {
        let mut next_search = HashSet::new();
        for child in to_search.iter() {
            if bags.contains_key(child) {
                for parent in bags.get(child).unwrap().iter() {
                    if !searched.contains(&parent.clone()) && next_search.insert(parent.clone()) {
                        count += 1;
                        searched.insert(parent.clone());
                    }
                }
            }
        }
        to_search = next_search;
    }
    println!("{}", count);
}

fn part2() {
    let file = File::open("day07/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let r = Regex::new(r"(?:([\w\s]+)) bags contain (?:(.+)).").unwrap();
    let r_bags = Regex::new(r" (bags|bag)").unwrap();
    let r_split = Regex::new(r", ").unwrap();
    let r_child = Regex::new(r"(?:(\d+)) (?:([\w ]+))").unwrap();
    let mut bags = HashMap::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(captures) = r.captures(line.as_str()) {
                let parent = captures.get(1).unwrap().as_str().to_string();
                let tmp = &*r_bags.replace_all(&captures.get(2).unwrap().as_str(), "");
                let children = r_split.split(tmp).collect::<Vec<_>>();
                for &child in children.iter() {
                    if let Some(child) = r_child.captures(child) {
                        bags.entry(parent.clone()).or_insert(Vec::new()).push((
                            child.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                            child.get(2).unwrap().as_str().to_string(),
                        ));
                    }
                }
            }
        }
    }
    let count = get_bags_contained(&bags, &"shiny gold".to_string()) - 1;
    println!("{}", count);
}

fn get_bags_contained(bags: &HashMap<String, Vec<(u32, String)>>, bag: &String) -> u32 {
    let mut count = 1; //count itself
    if let Some(children) = bags.get(bag) {
        for child in children.iter() {
            count += child.0 * get_bags_contained(bags, &child.1);
        }
    }
    count
}
