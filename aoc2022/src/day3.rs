use std::collections::HashSet;

use crate::util::read_input_as_lines;

fn char_to_score(c: &char) -> u32 {
    let tmp = *c as u32;
    if tmp >= 97 {
        return tmp - 97 + 1;
    }
    tmp - 65 + 27
}

// Answer should be "8176"
pub fn task1() {
    let lines = read_input_as_lines("day3.txt");

    let mut sum: u32 = 0;
    for line in lines.iter() {
        let h1: HashSet<char> = line[..(line.len() / 2)].chars().into_iter().collect();
        let h2: HashSet<char> = line[(line.len() / 2)..].chars().into_iter().collect();

        sum += h1.intersection(&h2).map(char_to_score).sum::<u32>();
    }
    println!("{}", sum);
}

// Answer should be "2689"
pub fn task2() {
    let lines = read_input_as_lines("day3.txt");

    let mut sum: u32 = 0;
    for line_chunk in lines.chunks(3) {
        let h1: HashSet<char> = line_chunk[0].chars().into_iter().collect();
        let h2: HashSet<char> = line_chunk[1].chars().into_iter().collect();
        let h3: HashSet<char> = line_chunk[2].chars().into_iter().collect();

        sum += h1
            .iter()
            .filter(|c| h2.contains(c))
            .filter(|c| h3.contains(c))
            .map(char_to_score)
            .sum::<u32>();
    }
    println!("{}", sum);
}
