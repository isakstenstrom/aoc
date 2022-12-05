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
pub fn task1() -> u32 {
    read_input_as_lines("day3.txt")
        .iter()
        .map(|line| {
            let h1: HashSet<char> = line[..(line.len() / 2)].chars().into_iter().collect();
            let h2: HashSet<char> = line[(line.len() / 2)..].chars().into_iter().collect();

            h1.intersection(&h2).map(char_to_score).sum::<u32>()
        })
        .sum()
}

// Answer should be "2689"
pub fn task2() -> u32 {
    read_input_as_lines("day3.txt")
        .chunks(3)
        .map(|lines| {
            let h1: HashSet<char> = lines[0].chars().into_iter().collect();
            let h2: HashSet<char> = lines[1].chars().into_iter().collect();
            let h3: HashSet<char> = lines[2].chars().into_iter().collect();

            h1.iter()
                .filter(|c| h2.contains(c))
                .filter(|c| h3.contains(c))
                .map(char_to_score)
                .sum::<u32>()
        })
        .sum()
}
