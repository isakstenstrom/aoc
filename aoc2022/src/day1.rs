use std::{
    fs::File,
    io::{self, BufRead},
};

// Answer should be "69281"
pub fn run1() {
    let file = File::open("res/input/day1_1.txt").expect("Error: input file does not exist");
    let lines = io::BufReader::new(file).lines();
    let mut max_calories = 0;
    let mut calorie_counter = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            max_calories = std::cmp::max(max_calories, calorie_counter);
            calorie_counter = 0;
        } else {
            calorie_counter += str::parse::<i32>(&line).expect("Invalid calorie in the file.");
        }
    }
    max_calories = std::cmp::max(max_calories, calorie_counter);
    println!("{max_calories}");
}

// Answer should be "201524"
pub fn run2() {
    let file = File::open("res/input/day1_1.txt").expect("Error: input file does not exist");
    let lines = io::BufReader::new(file).lines();
    let mut max_calories1 = 0;
    let mut max_calories2 = 0;
    let mut max_calories3 = 0;
    let mut calorie_counter = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            if calorie_counter > max_calories1 {
                max_calories1 = calorie_counter;
            } else if calorie_counter > max_calories2 {
                max_calories2 = calorie_counter;
            } else if calorie_counter > max_calories3 {
                max_calories3 = calorie_counter;
            }
            calorie_counter = 0;
        } else {
            calorie_counter += str::parse::<i32>(&line).expect("Invalid calorie in the file.");
        }
    }
    if calorie_counter > max_calories1 {
        max_calories1 = calorie_counter;
    } else if calorie_counter > max_calories2 {
        max_calories2 = calorie_counter;
    } else if calorie_counter > max_calories3 {
        max_calories3 = calorie_counter;
    }
    println!("{}", max_calories1 + max_calories2 + max_calories3);
}
