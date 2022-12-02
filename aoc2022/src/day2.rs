use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Draw,
    Lose,
}

fn translate_to_move(input: char) -> Move {
    match input {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissors,
        _ => panic!("Invalid input: Move not in list"),
    }
}

fn translate_to_game_result(input: char) -> GameResult {
    match input {
        'X' => GameResult::Lose,
        'Y' => GameResult::Draw,
        'Z' => GameResult::Win,
        _ => panic!("Invalid input: Move not in list"),
    }
}

// Answer should be "15523"
pub fn run1() {
    let file = File::open("res/input/day2_1.txt").expect("Error: input file does not exist");
    let lines = io::BufReader::new(file).lines();

    let mut score = 0;
    for line in lines.flatten() {
        let p1 = translate_to_move(line.chars().next().expect("Invalid input in file"));
        let p2 = translate_to_move(line.chars().nth(2).expect("Invalid input in file"));

        match p2 {
            Move::Rock => score += 1,
            Move::Paper => score += 2,
            Move::Scissors => score += 3,
        }

        if p1 == p2 {
            score += 3;
        } else if (p1 == Move::Rock && p2 == Move::Paper)
            || (p1 == Move::Paper && p2 == Move::Scissors)
            || (p1 == Move::Scissors && p2 == Move::Rock)
        {
            score += 6;
        }
    }
    println!("{}", score);
}

// Answer should be "15702"
pub fn run2() {
    let file = File::open("res/input/day2_1.txt").expect("Error: input file does not exist");
    let lines = io::BufReader::new(file).lines();

    let mut score = 0;
    for line in lines.flatten() {
        let p1 = translate_to_move(line.chars().next().expect("Invalid input in file"));
        let game_result =
            translate_to_game_result(line.chars().nth(2).expect("Invalid input in file"));

        let p2 = match (&p1, &game_result) {
            (Move::Rock, GameResult::Lose) => Move::Scissors,
            (Move::Rock, GameResult::Draw) => Move::Rock,
            (Move::Rock, GameResult::Win) => Move::Paper,
            (Move::Paper, GameResult::Lose) => Move::Rock,
            (Move::Paper, GameResult::Draw) => Move::Paper,
            (Move::Paper, GameResult::Win) => Move::Scissors,
            (Move::Scissors, GameResult::Lose) => Move::Paper,
            (Move::Scissors, GameResult::Draw) => Move::Scissors,
            (Move::Scissors, GameResult::Win) => Move::Rock,
        };

        match p2 {
            Move::Rock => score += 1,
            Move::Paper => score += 2,
            Move::Scissors => score += 3,
        }

        match game_result {
            GameResult::Lose => score += 0,
            GameResult::Draw => score += 3,
            GameResult::Win => score += 6,
        }
    }
    println!("{}", score);
}
