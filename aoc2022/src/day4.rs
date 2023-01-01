struct ElfPair {
    first: (u32, u32),
    second: (u32, u32),
}

impl ElfPair {
    fn from_string(input: &str) -> ElfPair {
        let mut values = input.split(&['-', ',']).map(|v| v.parse::<u32>().unwrap());
        ElfPair {
            first: (values.next().unwrap(), values.next().unwrap()),
            second: (values.next().unwrap(), values.next().unwrap()),
        }
    }

    fn fully_contains(&self) -> bool {
        (self.first.0 <= self.second.0 && self.first.1 >= self.second.1)
            || (self.first.0 >= self.second.0 && self.first.1 <= self.second.1)
    }

    fn partly_contains(&self) -> bool {
        !(self.first.1 < self.second.0 || self.first.0 > self.second.1)
    }
}

pub fn task1(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| ElfPair::from_string(l))
        .filter(|p| p.fully_contains())
        .count()
}

pub fn task2(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| ElfPair::from_string(l))
        .filter(|p| p.partly_contains())
        .count()
}

#[cfg(test)]
mod tests {
    use super::{task1, task2};
    use crate::util::read_input_from_file;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&read_input_from_file("sample/day4.txt")), 2);
        assert_eq!(task1(&read_input_from_file("input/day4.txt")), 538);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&read_input_from_file("sample/day4.txt")), 4);
        assert_eq!(task2(&read_input_from_file("input/day4.txt")), 792);
    }
}
