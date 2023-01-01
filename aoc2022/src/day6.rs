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

pub fn task1(input: &[String]) -> usize {
    get_index_of_first_substring_with_unique_letters(&input.join("\n"), 4)
}

pub fn task2(input: &[String]) -> usize {
    get_index_of_first_substring_with_unique_letters(&input.join("\n"), 14)
}

#[cfg(test)]
mod tests {
    use super::{task1, task2};
    use crate::util::read_input_from_file;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&read_input_from_file("sample/day6_1.txt")), 7);
        assert_eq!(task1(&read_input_from_file("sample/day6_2.txt")), 5);
        assert_eq!(task1(&read_input_from_file("sample/day6_3.txt")), 6);
        assert_eq!(task1(&read_input_from_file("sample/day6_4.txt")), 10);
        assert_eq!(task1(&read_input_from_file("sample/day6_5.txt")), 11);
        assert_eq!(task1(&read_input_from_file("input/day6.txt")), 1300);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&read_input_from_file("sample/day6_1.txt")), 19);
        assert_eq!(task2(&read_input_from_file("sample/day6_2.txt")), 23);
        assert_eq!(task2(&read_input_from_file("sample/day6_3.txt")), 23);
        assert_eq!(task2(&read_input_from_file("sample/day6_4.txt")), 29);
        assert_eq!(task2(&read_input_from_file("sample/day6_5.txt")), 26);
        assert_eq!(task2(&read_input_from_file("input/day6.txt")), 3986);
    }
}
