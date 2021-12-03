use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn main() {
    part1();
    part2();
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Debug for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Floor => write!(f, "."),
            Self::Empty => write!(f, "L"),
            Self::Occupied => write!(f, "#"),
        }
    }
}

fn part1() {
    let file = File::open("day11/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut layout = Vec::new();
    for line in reader.lines().flatten() {
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            row.push(match c {
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                _ => panic!(),
            })
        }
        layout.push(row);
    }

    let height = layout.len();
    let width = layout[0].len();
    loop {
        let mut new_layout = layout.clone();
        let mut changes = 0;
        for y in 0..layout.len() {
            for x in 0..layout[0].len() {
                match layout[y][x] {
                    Seat::Floor => {}
                    Seat::Empty | Seat::Occupied => {
                        let mut count = 0;
                        for i in 0..3 {
                            for j in 0..3 {
                                if !(i == 1 && j == 1)
                                    && !(y == 0 && i == 0)
                                    && !(y == height - 1 && i == 2)
                                    && !(x == 0 && j == 0)
                                    && !(x == width - 1 && j == 2)
                                    && layout[y + i - 1][x + j - 1] == Seat::Occupied
                                {
                                    count += 1;
                                }
                            }
                        }
                        if count == 0 && layout[y][x] == Seat::Empty {
                            new_layout[y][x] = Seat::Occupied;
                            changes += 1;
                        } else if count >= 4 && layout[y][x] == Seat::Occupied {
                            new_layout[y][x] = Seat::Empty;
                            changes += 1;
                        }
                    }
                }
            }
        }
        layout = new_layout;

        if changes == 0 {
            break;
        }
    }
    let mut count = 0;
    for &seat in layout.iter().flatten() {
        if seat == Seat::Occupied {
            count += 1;
        }
    }
    println!("{}", count);
}
fn part2() {
    let file = File::open("day11/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut layout = Vec::new();
    for line in reader.lines().flatten() {
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            row.push(match c {
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                _ => panic!(),
            })
        }
        layout.push(row);
    }

    let height = layout.len() - 1;
    let width = layout[0].len() - 1;
    let neutral = |a, _| a;
    let less = |a, b| a - b;
    let greater = |a, b| a + b;

    loop {
        let mut new_layout = layout.clone();
        let mut changes = 0;
        for y in 0..layout.len() {
            for x in 0..layout[0].len() {
                match layout[y][x] {
                    Seat::Floor => {}
                    Seat::Empty | Seat::Occupied => {
                        let mut count = 0;
                        let left = x;
                        let right = width - x;
                        let up = y;
                        let down = height - y;
                        count += find(1..=left, neutral, less, y, x, &layout); // left
                        count += find(1..=right, neutral, greater, y, x, &layout); // right
                        count += find(1..=up, less, neutral, y, x, &layout); // up
                        count += find(1..=down, greater, neutral, y, x, &layout); // down
                        count += find(1..=up.min(left), less, less, y, x, &layout); // up left
                        count += find(1..=up.min(right), less, greater, y, x, &layout); // up right
                        count += find(1..=down.min(right), greater, greater, y, x, &layout); // down
                        count += find(1..=down.min(left), greater, less, y, x, &layout); // down left
                        if count == 0 && layout[y][x] == Seat::Empty {
                            new_layout[y][x] = Seat::Occupied;
                            changes += 1;
                        } else if count >= 5 && layout[y][x] == Seat::Occupied {
                            new_layout[y][x] = Seat::Empty;
                            changes += 1;
                        }
                    }
                }
            }
        }
        layout = new_layout;

        if changes == 0 {
            break;
        }
    }
    let mut count = 0;
    for &seat in layout.iter().flatten() {
        if seat == Seat::Occupied {
            count += 1;
        }
    }
    println!("{}", count);
}

fn find<Y, X>(
    range: RangeInclusive<usize>,
    fy: Y,
    fx: X,
    y: usize,
    x: usize,
    layout: &[Vec<Seat>],
) -> usize
where
    Y: Fn(usize, usize) -> usize,
    X: Fn(usize, usize) -> usize,
{
    for i in range {
        match layout[fy(y, i)][fx(x, i)] {
            Seat::Empty => return 0,
            Seat::Occupied => return 1,
            Seat::Floor => {}
        }
    }
    0
}
