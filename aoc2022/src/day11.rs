use std::mem::swap;

#[derive(Debug)]
struct Monkey {
    items: Vec<i128>,
    operator: String,
    operator_argument: String,
    test_divide_by: i128,
    send_if_true: usize,
    send_if_false: usize,
    number_of_inspected_items: usize,
}

impl Monkey {
    fn new(
        item_line: &str,
        operation_line: &str,
        divide_line: &str,
        true_line: &str,
        false_line: &str,
    ) -> Self {
        let mut monkey = Monkey {
            items: Vec::new(),
            operator: operation_line[23..24].to_string(),
            operator_argument: operation_line[25..].to_string(),
            test_divide_by: divide_line[21..].parse().unwrap(),
            send_if_true: true_line[29..].parse().unwrap(),
            send_if_false: false_line[30..].parse().unwrap(),
            number_of_inspected_items: 0,
        };

        for elem in item_line[18..].split(", ") {
            monkey.items.push(elem.parse().unwrap());
        }
        monkey
    }

    fn inspect(&mut self, divide_by: i128, mod_value_by: i128) -> Option<(i128, usize)> {
        let mut item = self.items.pop()?;

        let int_to_apply: i128 = if self.operator_argument == "old" {
            item
        } else {
            self.operator_argument.parse::<i128>().unwrap()
        };

        if self.operator == "+" {
            item += int_to_apply;
        } else {
            item *= int_to_apply;
        }

        item /= divide_by;
        item %= mod_value_by;

        self.number_of_inspected_items += 1;

        if item % self.test_divide_by == 0 {
            Some((item, self.send_if_true))
        } else {
            Some((item, self.send_if_false))
        }
    }
}

fn parse_monkeys(input: &[String]) -> Vec<Monkey> {
    let mut res: Vec<Monkey> = Vec::new();

    let mut monkey_iter = input.iter();

    while let Some(_) = monkey_iter.next() {
        res.push(Monkey::new(
            monkey_iter.next().unwrap(),
            monkey_iter.next().unwrap(),
            monkey_iter.next().unwrap(),
            monkey_iter.next().unwrap(),
            monkey_iter.next().unwrap(),
        ));
        monkey_iter.next();
    }

    res
}

fn simulate_monkeys(monkeys: &mut [Monkey], num_iterations: usize, divide_worry_by: i128) {
    let mod_value_by: i128 = monkeys.iter().map(|monkey| monkey.test_divide_by).product();

    for _ in 0..num_iterations {
        for i in 0..monkeys.len() {
            while let Some((val, dest)) = monkeys[i].inspect(divide_worry_by, mod_value_by) {
                monkeys[dest].items.push(val);
            }
        }
    }
}

pub fn part1(input: &[String]) -> usize {
    let mut monkeys = parse_monkeys(input);
    simulate_monkeys(&mut monkeys, 20, 3);

    let mut max1 = 0usize;
    let mut max2 = 0usize;
    for monkey in monkeys.iter_mut() {
        if monkey.number_of_inspected_items > max1 {
            swap(&mut monkey.number_of_inspected_items, &mut max1);
        }
        if monkey.number_of_inspected_items > max2 {
            swap(&mut monkey.number_of_inspected_items, &mut max2);
        }
    }
    max1 * max2
}

pub fn part2(input: &[String]) -> usize {
    let mut monkeys = parse_monkeys(input);
    simulate_monkeys(&mut monkeys, 10000, 1);

    let mut max1 = 0usize;
    let mut max2 = 0usize;
    for monkey in monkeys.iter_mut() {
        if monkey.number_of_inspected_items > max1 {
            swap(&mut monkey.number_of_inspected_items, &mut max1);
        }
        if monkey.number_of_inspected_items > max2 {
            swap(&mut monkey.number_of_inspected_items, &mut max2);
        }
    }
    max1 * max2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day11.txt")), 10605);
        assert_eq!(part1(&read_input_from_file("input/day11.txt")), 72884);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day11.txt")), 2713310158);
        assert_eq!(part2(&read_input_from_file("input/day11.txt")), 15310845153);
    }
}
