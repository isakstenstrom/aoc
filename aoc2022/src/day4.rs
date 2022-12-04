use crate::util::read_input_as_lines;

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

// Answer should be "538"
pub fn task1() {
    let lines = read_input_as_lines("day4.txt");
    println!(
        "{}",
        lines
            .iter()
            .map(|l| ElfPair::from_string(l))
            .filter(|p| p.fully_contains())
            .count()
    );
}

// Answer should be "792"
pub fn task2() {
    let lines = read_input_as_lines("day4.txt");
    println!(
        "{}",
        lines
            .iter()
            .map(|l| ElfPair::from_string(l))
            .filter(|p| p.partly_contains())
            .count()
    );
}
