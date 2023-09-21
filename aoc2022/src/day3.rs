use std::collections::HashSet;

fn char_to_score(c: &char) -> u32 {
    let tmp = *c as u32;
    if tmp >= 97 {
        return tmp - 97 + 1;
    }
    tmp - 65 + 27
}

pub fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let h1: HashSet<char> = line[..(line.len() / 2)].chars().collect();
            let h2: HashSet<char> = line[(line.len() / 2)..].chars().collect();

            h1.intersection(&h2).map(char_to_score).sum::<u32>()
        })
        .sum()
}

pub fn part2(input: &[String]) -> u32 {
    input
        .chunks(3)
        .map(|lines| {
            let h1: HashSet<char> = lines[0].chars().collect();
            let h2: HashSet<char> = lines[1].chars().collect();
            let h3: HashSet<char> = lines[2].chars().collect();

            h1.iter()
                .filter(|c| h2.contains(c))
                .filter(|c| h3.contains(c))
                .map(char_to_score)
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day3.txt")), 157);
        assert_eq!(part1(&read_input_from_file("input/day3.txt")), 8176);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day3.txt")), 70);
        assert_eq!(part2(&read_input_from_file("input/day3.txt")), 2689);
    }
}
