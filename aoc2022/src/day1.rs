use crate::util::read_input_as_string;

// Answer should be "69281"
pub fn task1() -> u32 {
    read_input_as_string("day1.txt")
        .split("\n\n")
        .map(|l| {
            l.split('\n')
                .map(|c| c.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

// Answer should be "201524"
pub fn task2() -> u32 {
    let mut max1: u32 = 0;
    let mut max2: u32 = 0;
    let mut max3: u32 = 0;
    read_input_as_string("day1.txt")
        .split("\n\n")
        .map(|l| {
            l.split('\n')
                .map(|c| c.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .for_each(|c| {
            if c > max1 {
                max1 = c;
            } else if c > max2 {
                max2 = c;
            } else if c > max3 {
                max3 = c;
            }
        });
    max1 + max2 + max3
}

#[cfg(test)]
mod tests {
    use crate::day1::{task1, task2};

    #[test]
    fn test_task1() {
        assert_eq!(task1(), 69281);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(), 201524);
    }
}
