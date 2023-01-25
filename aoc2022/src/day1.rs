use std::mem::swap;

pub fn part1(input: &[String]) -> u32 {
    input
        .join("\n")
        .split("\n\n")
        .map(|l| {
            l.split('\n')
                .map(|c| c.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn part2(input: &[String]) -> u32 {
    let mut max: Vec<u32> = vec![0; 3];
    input
        .join("\n")
        .split("\n\n")
        .map(|l| {
            l.split('\n')
                .map(|c| c.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .for_each(|mut c| {
            for m in max.iter_mut() {
                if c > *m {
                    swap(&mut c, m);
                }
            }
        });
    max.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day1.txt")), 24000);
        assert_eq!(part1(&read_input_from_file("input/day1.txt")), 69281);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day1.txt")), 45000);
        assert_eq!(part2(&read_input_from_file("input/day1.txt")), 201524);
    }
}
