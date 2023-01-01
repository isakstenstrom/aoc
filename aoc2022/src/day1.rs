use std::mem::swap;

pub fn task1(input: &[String]) -> u32 {
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

pub fn task2(input: &[String]) -> u32 {
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
    use super::{task1, task2};
    use crate::util::read_input_from_file;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&read_input_from_file("sample/day1.txt")), 24000);
        assert_eq!(task1(&read_input_from_file("input/day1.txt")), 69281);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&read_input_from_file("sample/day1.txt")), 45000);
        assert_eq!(task2(&read_input_from_file("input/day1.txt")), 201524);
    }
}
