fn mix(input: &[String], num_iterations: usize, decryption_key: i64) -> i64 {
    let mut v: Vec<(usize, i64)> = input
        .iter()
        .enumerate()
        .map(|(i, line)| (i, line.parse::<i64>().unwrap() * decryption_key))
        .collect();

    for _ in 0..num_iterations {
        for mix_num in 0..v.len() {
            let old_pos = v.iter().position(|(i, _)| *i == mix_num).unwrap();
            let elem = v.remove(old_pos);
            let new_pos = usize::try_from(
                (i64::try_from(old_pos).unwrap() + elem.1)
                    .rem_euclid(i64::try_from(v.len()).unwrap()),
            )
            .unwrap();
            v.insert(new_pos, elem);
        }
    }

    let zero_index = v.iter().position(|(_, val)| *val == 0).unwrap();

    let mut sum = 0;
    sum += v[(zero_index + 1000).rem_euclid(v.len())].1;
    sum += v[(zero_index + 3000).rem_euclid(v.len())].1;
    sum += v[(zero_index + 2000).rem_euclid(v.len())].1;
    sum
}

pub fn part1(input: &[String]) -> i64 {
    mix(input, 1, 1)
}

pub fn part2(input: &[String]) -> i64 {
    mix(input, 10, 811589153)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day20.txt")), 3);
        assert_eq!(part1(&read_input_from_file("input/day20.txt")), 13883);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day20.txt")), 1623178306);
        assert_eq!(
            part2(&read_input_from_file("input/day20.txt")),
            19185967576920
        );
    }
}
