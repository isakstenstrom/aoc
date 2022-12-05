use std::collections::VecDeque;

use crate::util::read_input_as_lines;

// Answer should be "RTGWZTHLD"
pub fn task1() -> String {
    let lines = read_input_as_lines("day5.txt");
    let num_columns = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); num_columns];

    let mut index: usize = 0;
    for line in &lines {
        index += 1;
        if line.chars().nth(1).unwrap() == '1' {
            break;
        }
        line[1..]
            .chars()
            .into_iter()
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if c != ' ' {
                    stacks[i].push_front(c);
                }
            })
    }
    index += 1;

    for line in &lines[index..] {
        let mut split_line = line.split(' ');

        let amount = split_line.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split_line.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = split_line.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..amount {
            let c = stacks[from].pop_back().unwrap();
            stacks[to].push_back(c);
        }
    }
    stacks.iter().map(|s| s.back().unwrap()).collect()
}

// Answer should be "STHGRZZFR"
pub fn task2() -> String {
    let lines = read_input_as_lines("day5.txt");
    let num_columns = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); num_columns];

    let mut index: usize = 0;
    for line in &lines {
        index += 1;
        if line.chars().nth(1).unwrap() == '1' {
            break;
        }
        line[1..]
            .chars()
            .into_iter()
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if c != ' ' {
                    stacks[i].push_front(c);
                }
            })
    }
    index += 1;

    for line in &lines[index..] {
        let mut split_line = line.split(' ');

        let amount = split_line.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split_line.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = split_line.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        let mut tmp: Vec<char> = Vec::new();

        for _ in 0..amount {
            tmp.push(stacks[from].pop_back().unwrap());
        }

        for _ in 0..amount {
            stacks[to].push_back(tmp.pop().unwrap());
        }
    }
    stacks.iter().map(|s| s.back().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use crate::day5::{task1, task2};

    #[test]
    fn test_task1() {
        assert_eq!(task1(), "RTGWZTHLD");
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(), "STHGRZZFR");
    }
}
