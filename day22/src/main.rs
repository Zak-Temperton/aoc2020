use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

enum State {
    Player1,
    Transition,
    Player2,
}

fn part1() {
    let mut player1 = VecDeque::new();
    let mut player2 = VecDeque::new();
    let file = File::open("day22/res/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut state = State::Player1;
    for line in reader.lines().flatten().skip(1) {
        match state {
            State::Player1 => {
                if line.is_empty() {
                    state = State::Transition;
                } else {
                    player1.push_back(line.parse::<usize>().unwrap());
                }
            }
            State::Transition => state = State::Player2,
            State::Player2 => player2.push_back(line.parse::<usize>().unwrap()),
        }
    }
    while !player1.is_empty() && !player2.is_empty() {
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();
        if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }
    let mut score = 0;
    if !player1.is_empty() {
        for (i, &card) in player1.iter().enumerate() {
            score += (player1.len() - i) * card;
        }
    } else {
        for (i, &card) in player2.iter().enumerate() {
            score += (player2.len() - i) * card;
        }
    }
    println!("{}", score);
}

fn part2() {
    let mut player1 = VecDeque::new();
    let mut player2 = VecDeque::new();
    let file = File::open("day22/res/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut state = State::Player1;
    for line in reader.lines().flatten().skip(1) {
        match state {
            State::Player1 => {
                if line.is_empty() {
                    state = State::Transition;
                } else {
                    player1.push_back(line.parse::<usize>().unwrap());
                }
            }
            State::Transition => state = State::Player2,
            State::Player2 => player2.push_back(line.parse::<usize>().unwrap()),
        }
    }
    let mut player1_played_hands = HashSet::new();
    let mut player2_played_hands = HashSet::new();
    while !player1.is_empty() && !player2.is_empty() {
        if !player1_played_hands.insert(player1.clone())
            || !player2_played_hands.insert(player2.clone())
        {
            player2.clear();
        }
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();
        if p1 <= player1.len() && p2 <= player2.len() {
            if play_game(
                player1.range(..p1).copied().collect(),
                player2.range(..p2).copied().collect(),
            ) {
                player1.push_back(p1);
                player1.push_back(p2);
            } else {
                player2.push_back(p2);
                player2.push_back(p1);
            }
        } else if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }
    let mut score = 0;
    if !player1.is_empty() {
        for (i, &card) in player1.iter().enumerate() {
            score += (player1.len() - i) * card;
        }
    } else {
        for (i, &card) in player2.iter().enumerate() {
            score += (player2.len() - i) * card;
        }
    }
    println!("{}", score);
}

fn play_game(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) -> bool {
    let mut player1_played_hands = HashSet::new();
    let mut player2_played_hands = HashSet::new();
    while !player1.is_empty() && !player2.is_empty() {
        if !player1_played_hands.insert(player1.clone())
            || !player2_played_hands.insert(player2.clone())
        {
            player2.clear();
            break;
        }
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();
        if p1 <= player1.len() && p2 <= player2.len() {
            if play_game(
                player1.range(..p1).copied().collect(),
                player2.range(..p2).copied().collect(),
            ) {
                player1.push_back(p1);
                player1.push_back(p2);
            } else {
                player2.push_back(p2);
                player2.push_back(p1);
            }
        } else if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }
    player2.is_empty()
}
