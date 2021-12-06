use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}
fn part1() {
    let file = File::open("day21/res/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut all = HashMap::new();
    let mut allergens_map = HashMap::new();
    for line in reader.lines().flatten() {
        let mut split = line.split(" (contains ");
        let ingredients = split.next().unwrap().split(' ').map(|s| s.to_string());
        for i in ingredients.clone() {
            *all.entry(i).or_insert(0) += 1;
        }
        let allergens = split
            .next()
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(", ")
            .map(|s| s.to_string());
        for allergen in allergens {
            let mut new_set = HashSet::new();
            for i in ingredients.clone() {
                new_set.insert(i);
            }
            let set = allergens_map
                .entry(allergen.to_string())
                .or_insert_with(|| new_set.clone());
            for ingredient in set.clone().iter() {
                if !new_set.contains(ingredient) {
                    set.remove(ingredient);
                }
            }
        }
    }
    for allergen in allergens_map.values().flatten() {
        all.remove(allergen);
    }

    println!("{}", all.values().sum::<i32>())
}

fn part2() {
    let file = File::open("day21/res/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut all = HashMap::new();
    let mut allergens_map = HashMap::new();
    for line in reader.lines().flatten() {
        let mut split = line.split(" (contains ");
        let ingredients = split.next().unwrap().split(' ').map(|s| s.to_string());
        for i in ingredients.clone() {
            *all.entry(i).or_insert(0) += 1;
        }
        let allergens = split
            .next()
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(", ")
            .map(|s| s.to_string());
        for allergen in allergens {
            let mut new_set = HashSet::new();
            for i in ingredients.clone() {
                new_set.insert(i);
            }
            let set = allergens_map
                .entry(allergen.to_string())
                .or_insert_with(|| new_set.clone());
            for ingredient in set.clone().iter() {
                if !new_set.contains(ingredient) {
                    set.remove(ingredient);
                }
            }
        }
    }
    let mut identified = Vec::new();
    while !allergens_map.is_empty() {
        let mut remove = Vec::new();
        for (k, set) in allergens_map.iter_mut() {
            if set.len() == 1 {
                identified.push((k.clone(), set.iter().next().unwrap().clone()));
                remove.push(k.clone());
            }
            for i in identified.iter() {
                set.remove(&i.1);
            }
        }
        for k in remove {
            allergens_map.remove(&k);
        }
    }

    identified.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let len = identified.len() - 2;
    for (_, i) in identified[0..=len].iter() {
        print!("{},", i);
    }
    print!("{}", identified[len + 1].1);
}
