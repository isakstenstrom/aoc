fn simulate_register(input: &[String]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut register: i32 = 1;
    for line in input {
        result.push(register);
        if line.starts_with("addx") {
            result.push(register);
            register += line[5..].parse::<i32>().unwrap();
        }
    }
    result
}

pub fn task1(input: &[String]) -> i32 {
    simulate_register(input)
        .iter()
        .enumerate()
        .filter(|(i, _)| (i + 20 + 1) % 40 == 0)
        .map(|(i, value)| ((i as i32) + 1) * value)
        .sum()
}

pub fn task2(input: &[String]) -> String {
    simulate_register(input)
        .iter()
        .enumerate()
        .map(|(i, value)| {
            let mut pixel: String = "".to_string();
            let horizontal_index = (i as i32) % 40;

            if horizontal_index == 0 {
                pixel.push('\n');
            }

            if horizontal_index - 1 <= *value && *value <= horizontal_index + 1 {
                pixel.push('#')
            } else {
                pixel.push('.')
            }

            pixel
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&read_input_from_file("sample/day10.txt")), 13140);
        assert_eq!(task1(&read_input_from_file("input/day10.txt")), 14360);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&read_input_from_file("sample/day10.txt")), "\n##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....");

        // Correct task result is "BGKAEREZ"
        assert_eq!(task2(&read_input_from_file("input/day10.txt")), "\n###...##..#..#..##..####.###..####.####.\n#..#.#..#.#.#..#..#.#....#..#.#.......#.\n###..#....##...#..#.###..#..#.###....#..\n#..#.#.##.#.#..####.#....###..#.....#...\n#..#.#..#.#.#..#..#.#....#.#..#....#....\n###...###.#..#.#..#.####.#..#.####.####.");
    }
}
