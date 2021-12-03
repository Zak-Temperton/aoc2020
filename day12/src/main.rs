use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

struct Ship1 {
    facing: u8,
    pos: (i32, i32),
}

impl Ship1 {
    pub fn new() -> Self {
        Ship1 {
            facing: 0,
            pos: (0, 0),
        }
    }

    pub fn rotate(&mut self, deg: i32) {
        self.facing += (deg / 90) as u8;
        self.facing %= 4;
    }

    pub fn move_dir(&mut self, dir: u8, distance: i32) {
        match dir {
            0 => self.pos.0 += distance, //E
            1 => self.pos.1 += distance, //S
            2 => self.pos.0 -= distance, //W
            3 => self.pos.1 -= distance, //N
            _ => panic!(),
        }
    }

    pub fn move_facing(&mut self, distance: i32) {
        match self.facing {
            0 => self.pos.0 += distance, //E
            1 => self.pos.1 += distance, //S
            2 => self.pos.0 -= distance, //W
            3 => self.pos.1 -= distance, //N
            _ => panic!(),
        }
    }
}

fn part1() {
    let file = File::open("day12/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut ship = Ship1::new();
    for line in reader.lines().flatten() {
        match &line[..1] {
            "F" => ship.move_facing(line[1..].parse().unwrap()),
            "L" => ship.rotate(360 - line[1..].parse::<i32>().unwrap()),
            "R" => ship.rotate(line[1..].parse().unwrap()),
            "E" => ship.move_dir(0, line[1..].parse().unwrap()),
            "S" => ship.move_dir(1, line[1..].parse().unwrap()),
            "W" => ship.move_dir(2, line[1..].parse().unwrap()),
            "N" => ship.move_dir(3, line[1..].parse().unwrap()),
            _ => panic!(),
        }
    }
    println!("{}", ship.pos.0.abs() + ship.pos.1.abs());
}

struct Ship2 {
    waypoint: (i32, i32),
    pos: (i32, i32),
}

impl Ship2 {
    pub fn new() -> Self {
        Ship2 {
            waypoint: (10, -1),
            pos: (0, 0),
        }
    }
    pub fn move_waypoint(&mut self, dir: u8, distance: i32) {
        match dir {
            0 => self.waypoint.0 += distance, //E
            1 => self.waypoint.1 += distance, //S
            2 => self.waypoint.0 -= distance, //W
            3 => self.waypoint.1 -= distance, //N
            _ => panic!(),
        }
    }
    pub fn forward(&mut self, distance: i32) {
        self.pos.0 += distance * self.waypoint.0;
        self.pos.1 += distance * self.waypoint.1;
    }
    pub fn rotate_waypoint(&mut self, deg: i32) {
        match deg {
            90 => {
                let tmp = self.waypoint;
                self.waypoint.0 = -tmp.1;
                self.waypoint.1 = tmp.0;
            }
            180 => {
                let tmp = self.waypoint;
                self.waypoint.0 = -tmp.0;
                self.waypoint.1 = -tmp.1;
            }
            270 => {
                let tmp = self.waypoint;
                self.waypoint.0 = tmp.1;
                self.waypoint.1 = -tmp.0;
            }
            _ => {}
        }
    }
}

fn part2() {
    let file = File::open("day12/res/input.txt").expect("Failed to load input.txt");
    let reader = BufReader::new(file);
    let mut ship = Ship2::new();
    for line in reader.lines().flatten() {
        match &line[..1] {
            "F" => ship.forward(line[1..].parse().unwrap()),
            "L" => ship.rotate_waypoint(360 - line[1..].parse::<i32>().unwrap()),
            "R" => ship.rotate_waypoint(line[1..].parse().unwrap()),
            "E" => ship.move_waypoint(0, line[1..].parse().unwrap()),
            "S" => ship.move_waypoint(1, line[1..].parse().unwrap()),
            "W" => ship.move_waypoint(2, line[1..].parse().unwrap()),
            "N" => ship.move_waypoint(3, line[1..].parse().unwrap()),
            _ => panic!(),
        }
    }
    println!("{}", ship.pos.0.abs() + ship.pos.1.abs());
}
