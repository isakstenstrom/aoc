use std::collections::VecDeque;

pub fn part1(input: &[String]) -> String {
    let num_columns = (input[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); num_columns];

    let mut index: usize = 0;
    for line in input {
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

    for line in &input[index..] {
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

pub fn part2(input: &[String]) -> String {
    let num_columns = (input[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); num_columns];

    let mut index: usize = 0;
    for line in input {
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

    for line in &input[index..] {
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
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day5.txt")), "CMZ");
        assert_eq!(part1(&read_input_from_file("input/day5.txt")), "RTGWZTHLD");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day5.txt")), "MCD");
        assert_eq!(part2(&read_input_from_file("input/day5.txt")), "STHGRZZFR");
    }
}
