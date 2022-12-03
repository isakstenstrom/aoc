use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn char_to_score(c: &char) -> u32 {
    let tmp = *c as u32;
    if tmp >= 97 {
        return tmp - 97 + 1;
    }
    tmp - 65 + 27
}

// Answer should be "8176"
pub fn run1() {
    let file = File::open("res/input/day3_1.txt").expect("Error: input file does not exist");
    let lines = io::BufReader::new(file).lines();

    let mut sum: u32 = 0;
    for line in lines.flatten() {
        let h1: HashSet<char> = line[..(line.len() / 2)].chars().into_iter().collect();
        let h2: HashSet<char> = line[(line.len() / 2)..].chars().into_iter().collect();

        for char_in_both in h1.intersection(&h2) {
            sum += char_to_score(char_in_both);
        }
    }
    println!("{}", sum);
}

// Answer should be "2689"
pub fn run2() {
    let file = File::open("res/input/day3_1.txt").expect("Error: input file does not exist");
    let lines = io::BufReader::new(file).lines();

    let mut sum: u32 = 0;
    let mut line_iter = lines.flatten().peekable();

    while line_iter.peek().is_some() {
        let h1: HashSet<char> = line_iter.next().unwrap().chars().into_iter().collect();
        let h2: HashSet<char> = line_iter.next().unwrap().chars().into_iter().collect();
        let h3: HashSet<char> = line_iter.next().unwrap().chars().into_iter().collect();

        for char_in_both in h1
            .iter()
            .filter(|k| h2.contains(k))
            .filter(|k| h3.contains(k))
        {
            sum += char_to_score(char_in_both);
        }
    }
    println!("{}", sum);
}
