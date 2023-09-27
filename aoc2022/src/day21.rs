use std::{collections::HashMap, str::FromStr};

type MonkeyId = i64;

fn string_to_id(s: &str) -> MonkeyId {
    s.chars()
        .enumerate()
        .map(|(i, c)| (c as i64) * 1000i64.pow(u32::try_from(i).unwrap()))
        .sum()
}

fn parse_monkeys(input: &[String]) -> HashMap<MonkeyId, Monkey> {
    let monkey_map = input
        .iter()
        .map(|s| {
            let (monkey_name, monkey_content) = s.split_once(": ").unwrap();
            (
                string_to_id(monkey_name),
                (monkey_content.parse::<Monkey>().unwrap()),
            )
        })
        .collect::<HashMap<MonkeyId, Monkey>>();
    monkey_map
}

#[derive(Debug, Clone)]
enum NumOp {
    Add,
    Sub,
    Mul,
    Div,
    Eql,
}

impl NumOp {
    fn do_op(&self, a: i64, b: i64) -> i64 {
        match self {
            NumOp::Add => a + b,
            NumOp::Sub => a - b,
            NumOp::Mul => a * b,
            NumOp::Div => a / b,
            NumOp::Eql => panic!("Cannot perform equal operation."),
        }
    }

    fn undo_op(&self, target: i64, num: i64, num_is_lhs: bool) -> i64 {
        match (self, num_is_lhs) {
            (NumOp::Add, _) => target - num,
            (NumOp::Sub, true) => target + num,
            (NumOp::Sub, false) => num - target,
            (NumOp::Mul, _) => target / num,
            (NumOp::Div, true) => target * num,
            (NumOp::Div, false) => num / target,
            (NumOp::Eql, _) => num,
        }
    }
}

#[derive(Debug, Clone)]
struct MonkeyOp {
    lhs: MonkeyId,
    rhs: MonkeyId,
    op: NumOp,
}

#[derive(Debug, Clone)]
enum MonkeyType {
    YellNum(i64),
    YellOp(MonkeyOp),
}

#[derive(Debug, Clone)]
struct Monkey {
    monkey_type: MonkeyType,
    is_in_human_branch: bool,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<i64>() {
            return Ok(Monkey {
                monkey_type: MonkeyType::YellNum(num),
                is_in_human_branch: false,
            });
        }
        let mut it = s.split(' ');
        Ok(Monkey {
            monkey_type: MonkeyType::YellOp(MonkeyOp {
                lhs: string_to_id(it.next().unwrap()),
                op: match it.next().unwrap() {
                    "+" => NumOp::Add,
                    "-" => NumOp::Sub,
                    "*" => NumOp::Mul,
                    "/" => NumOp::Div,
                    invalid_operator => panic!("Invalid operator \"{invalid_operator}\""),
                },
                rhs: string_to_id(it.next().unwrap()),
            }),
            is_in_human_branch: false,
        })
    }
}

fn get_value(map: &HashMap<MonkeyId, Monkey>, m_id: MonkeyId) -> i64 {
    match map.get(&m_id).unwrap().clone().monkey_type {
        MonkeyType::YellNum(i) => i,
        MonkeyType::YellOp(operation) => {
            let lhs = get_value(map, operation.lhs);
            let rhs = get_value(map, operation.rhs);

            // Not updating the map is faster in this case

            operation.op.do_op(lhs, rhs)
        }
    }
}

fn mark_human_branch(map: &mut HashMap<MonkeyId, Monkey>, m_id: MonkeyId) -> bool {
    if m_id == string_to_id("humn") {
        map.get_mut(&m_id).unwrap().is_in_human_branch = true;
        return true;
    }

    match map.get(&m_id).unwrap().clone().monkey_type {
        MonkeyType::YellNum(_) => false,
        MonkeyType::YellOp(op) => {
            let lhs = mark_human_branch(map, op.lhs);
            let rhs = mark_human_branch(map, op.rhs);
            if lhs || rhs {
                map.get_mut(&m_id).unwrap().is_in_human_branch = true
            }
            lhs || rhs
        }
    }
}

fn get_human_yell(map: &HashMap<MonkeyId, Monkey>, m_id: MonkeyId, target: i64) -> i64 {
    if m_id == string_to_id("humn") {
        return target;
    }

    let op: MonkeyOp = match map.get(&m_id).unwrap().clone().monkey_type {
        MonkeyType::YellOp(op) => op,
        MonkeyType::YellNum(_) => panic!(),
    };

    let (human_branch, other_branch, num_is_lhs) = {
        if map.get(&op.lhs).unwrap().is_in_human_branch {
            (op.lhs, op.rhs, true)
        } else {
            (op.rhs, op.lhs, false)
        }
    };

    let other_branch_value = get_value(map, other_branch);
    let new_target = op.op.undo_op(target, other_branch_value, num_is_lhs);
    get_human_yell(map, human_branch, new_target)
}

pub fn part1(input: &[String]) -> i64 {
    let monkey_map = parse_monkeys(input);
    get_value(&monkey_map, string_to_id("root"))
}

pub fn part2(input: &[String]) -> i64 {
    let mut monkey_map = parse_monkeys(input);

    match &mut monkey_map
        .get_mut(&string_to_id("root"))
        .unwrap()
        .monkey_type
    {
        MonkeyType::YellOp(op) => op.op = NumOp::Eql,
        _ => panic!("root node should be an operation"),
    }

    mark_human_branch(&mut monkey_map, string_to_id("root"));
    get_human_yell(&monkey_map, string_to_id("root"), 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day21.txt")), 152);
        assert_eq!(
            part1(&read_input_from_file("input/day21.txt")),
            158661812617812
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day21.txt")), 301);
        assert_eq!(
            part2(&read_input_from_file("input/day21.txt")),
            3352886133831
        );
    }
}
