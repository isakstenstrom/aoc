use crate::util::read_input_as_string;

fn get_index_of_first_substring_with_unique_letters(input: &str, substring_len: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(substring_len)
        .enumerate()
        .find(|(_, window)| {
            let mut w: Vec<char> = Vec::from_iter(window.iter().cloned());
            w.sort();
            w.dedup();
            w.len() == substring_len
        })
        .unwrap()
        .0
        + substring_len // since we index from the last char
}

pub fn task1() -> usize {
    get_index_of_first_substring_with_unique_letters(&read_input_as_string("day6.txt"), 4)
}

pub fn task2() -> usize {
    get_index_of_first_substring_with_unique_letters(&read_input_as_string("day6.txt"), 14)
}

#[cfg(test)]
mod tests {
    use crate::day6::{task1, task2};

    #[test]
    fn test_task1() {
        assert_eq!(task1(), 1300);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(), 3986);
    }
}
