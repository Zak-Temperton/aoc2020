use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("day17/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            lines.push(line.as_bytes().to_vec());
        }
    }
    let mut world = vec![vec![vec![false; 1 + 12]; lines.len() + 12]; lines.len() + 12];
    for (y, line) in lines.iter().enumerate() {
        for (x, &b) in line.iter().enumerate() {
            if b == b'#' {
                world[x + 6][y + 6][6] = true;
            }
        }
    }
    for _ in 0..6 {
        let mut new_world = vec![vec![vec![false; 1 + 12]; lines.len() + 12]; lines.len() + 12];
        for x in 0..world.len() {
            for y in 0..world[0].len() {
                for z in 0..world[0][0].len() {
                    let mut count = 0;
                    for i in 0..3 {
                        if x == 0 && i == 0 || x == world.len() - 1 && i == 2 {
                            continue;
                        }
                        for j in 0..3 {
                            if y == 0 && j == 0 || y == world[0].len() - 1 && j == 2 {
                                continue;
                            }
                            for k in 0..3 {
                                if z == 0 && k == 0
                                    || (z == world[0][0].len() - 1 && k == 2)
                                    || (i == 1 && j == 1 && k == 1)
                                {
                                    continue;
                                }
                                if world[x + i - 1][y + j - 1][z + k - 1] {
                                    count += 1;
                                }
                            }
                        }
                    }

                    if world[x][y][z] {
                        if count == 2 || count == 3 {
                            new_world[x][y][z] = true;
                        }
                    } else if count == 3 {
                        new_world[x][y][z] = true;
                    }
                }
            }
        }
        world = new_world;
    }
    println!(
        "{}",
        world.iter().flatten().flatten().fold(0, |i, &c| {
            if c {
                i + 1
            } else {
                i
            }
        })
    );
}
fn part2() {
    let file = File::open("day17/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            lines.push(line.as_bytes().to_vec());
        }
    }
    let mut world =
        vec![vec![vec![vec![false; 1 + 12]; 1 + 12]; lines.len() + 12]; lines.len() + 12];
    for (y, line) in lines.iter().enumerate() {
        for (x, &b) in line.iter().enumerate() {
            if b == b'#' {
                world[x + 6][y + 6][6][6] = true;
            }
        }
    }
    for _ in 0..6 {
        let mut new_world =
            vec![vec![vec![vec![false; 1 + 12]; 1 + 12]; lines.len() + 12]; lines.len() + 12];
        for x in 0..world.len() {
            for y in 0..world[0].len() {
                for z in 0..world[0][0].len() {
                    for w in 0..world[0][0].len() {
                        let mut count = 0;
                        for i in 0..3 {
                            if x == 0 && i == 0 || x == world.len() - 1 && i == 2 {
                                continue;
                            }
                            for j in 0..3 {
                                if y == 0 && j == 0 || y == world[0].len() - 1 && j == 2 {
                                    continue;
                                }
                                for k in 0..3 {
                                    if z == 0 && k == 0 || (z == world[0][0].len() - 1 && k == 2) {
                                        continue;
                                    }
                                    for l in 0..3 {
                                        if w == 0 && l == 0
                                            || (w == world[0][0][0].len() - 1 && l == 2)
                                            || (i == 1 && j == 1 && k == 1 && l == 1)
                                        {
                                            continue;
                                        }
                                        if world[x + i - 1][y + j - 1][z + k - 1][w + l - 1] {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }

                        if world[x][y][z][w] {
                            if count == 2 || count == 3 {
                                new_world[x][y][z][w] = true;
                            }
                        } else if count == 3 {
                            new_world[x][y][z][w] = true;
                        }
                    }
                }
            }
        }
        world = new_world;
    }
    println!(
        "{}",
        world.iter().flatten().flatten().flatten().fold(0, |i, &c| {
            if c {
                i + 1
            } else {
                i
            }
        })
    );
}
