use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
}

#[derive(Debug)]
struct Piece {
    id: u32,
    top: u16,
    bottom: u16,
    left: u16,
    right: u16,
}

impl Piece {
    pub fn new(id: u32, top: u16, bottom: u16, left: u16, right: u16) -> Self {
        fn reverse_bits(n: u16) -> u16 {
            let mut r = 0;
            for b in 0..10 {
                r <<= 1;
                r += (n >> b) & 1;
            }
            r
        }
        Self {
            id,
            top: top.max(reverse_bits(top)),
            bottom: bottom.max(reverse_bits(bottom)),
            left: left.max(reverse_bits(left)),
            right: right.max(reverse_bits(right)),
        }
    }

    pub fn unique_sides(&self, other: &Piece) -> (bool, bool, bool, bool) {
        (
            self.top != other.top
                && self.top != other.bottom
                && self.top != other.left
                && self.top != other.right,
            self.bottom != other.top
                && self.bottom != other.bottom
                && self.bottom != other.left
                && self.bottom != other.right,
            self.left != other.top
                && self.left != other.bottom
                && self.left != other.left
                && self.left != other.right,
            self.right != other.top
                && self.right != other.bottom
                && self.right != other.left
                && self.right != other.right,
        )
    }
}

fn gen_piece(lines: &mut Vec<Vec<u8>>, pieces: &mut Vec<Piece>) {
    let id = lines[0][5..9]
        .iter()
        .fold(0, |i, &b| (i * 10) + (b - b'0') as u32);
    let top = lines[1].iter().fold(0, |mut i, &b| {
        i <<= 1;
        if b == b'#' {
            i |= 1
        }
        i
    });
    let bottom = lines[10].iter().fold(0, |mut i, &b| {
        i <<= 1;
        if b == b'#' {
            i |= 1
        }
        i
    });
    let left = lines[1..=10].iter().fold(0, |mut i, b| {
        i <<= 1;
        if b[0] == b'#' {
            i |= 1
        }
        i
    });
    let right = lines[1..=10].iter().fold(0, |mut i, b| {
        i <<= 1;
        if b[9] == b'#' {
            i |= 1
        }
        i
    });
    pieces.push(Piece::new(id, top, bottom, left, right));
    *lines = Vec::new();
}

fn part1() {
    let file = File::open("day20/res/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut pieces = Vec::new();
    let mut lines: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines().flatten() {
        if line.is_empty() {
            gen_piece(&mut lines, &mut pieces);
        } else {
            lines.push(line.as_bytes().to_vec());
        }
    }
    gen_piece(&mut lines, &mut pieces);

    let mut product = 1;
    for i in 0..pieces.len() {
        let mut unique = (true, true, true, true);
        let mut s = 4;
        for j in 0..pieces.len() {
            if i != j {
                let sides = pieces[i].unique_sides(&pieces[j]);
                if !sides.0 && unique.0 {
                    unique.0 = false;
                    s -= 1;
                }
                if !sides.1 && unique.1 {
                    unique.1 = false;
                    s -= 1;
                }
                if !sides.2 && unique.2 {
                    unique.2 = false;
                    s -= 1;
                }
                if !sides.3 && unique.3 {
                    unique.3 = false;
                    s -= 1;
                }
            }
        }
        if s == 2 {
            product *= pieces[i].id as u64;
            // println!("{}", pieces[i].id);
        }
    }
    println!("{}", product);
    // println!("{:?}", pieces);
}
