use crate::util::Point;
use std::{cmp, collections::HashSet, vec};

const DEBUG_PRINT: bool = false;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(str: &str) -> Option<Self> {
        match str {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
    }

    fn get_move(&self) -> Point<i32> {
        match self {
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Down => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

struct Rope {
    knots: Vec<Point<i32>>,
}

impl Rope {
    fn new(length: usize) -> Rope {
        Rope {
            knots: vec![Point { x: 0, y: 0 }; length],
        }
    }

    fn move_head(&mut self, dir: &Direction) {
        let head = self.knots.first_mut().unwrap();
        *head += dir.get_move();
        let mut preceding_knot = *head;

        for knot in self.knots[1..].iter_mut() {
            if knot.max_distance_to(&preceding_knot) > 1 {
                knot.y += (preceding_knot.y - knot.y).signum();
                knot.x += (preceding_knot.x - knot.x).signum();
            }
            preceding_knot = *knot;
        }
    }

    fn get_tail(&self) -> &Point<i32> {
        self.knots.last().unwrap()
    }

    fn print(&self) {
        let mut upper_right = Point { x: 0, y: 0 };
        let mut lower_left = Point { x: 0, y: 0 };

        for knot in self.knots.iter() {
            upper_right.x = cmp::max(upper_right.x, knot.x);
            upper_right.y = cmp::max(upper_right.y, knot.y);
            lower_left.x = cmp::min(lower_left.x, knot.x);
            lower_left.y = cmp::min(lower_left.y, knot.y);
        }

        let width: usize = (upper_right.x - lower_left.x + 1).try_into().unwrap();
        let height: usize = (upper_right.y - lower_left.y + 1).try_into().unwrap();

        let mut grid = vec![vec!['.'; width]; height];

        for (i, knot) in self.knots.iter().enumerate().rev() {
            grid[(knot.y - lower_left.y) as usize][(knot.x - lower_left.x) as usize] =
                char::from_digit(i.try_into().unwrap(), 10).unwrap();
        }

        grid[(0 - lower_left.y) as usize][(0 - lower_left.x) as usize] = 's';

        println!("------------------------------");
        for row in grid.iter().rev() {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
        println!("------------------------------");
    }
}

fn simulate_rope(input: &[String], rope_length: usize) -> usize {
    let mut rope = Rope::new(rope_length);
    let mut visited_points: HashSet<Point<i32>> = HashSet::new();
    visited_points.insert(*rope.get_tail());

    input.iter().for_each(|line| {
        let mut split_line = line.split_whitespace();

        let dir = Direction::from_str(split_line.next().unwrap()).unwrap();
        let num_steps = str::parse::<u32>(split_line.next().unwrap()).unwrap();

        for _ in 0..num_steps {
            rope.move_head(&dir);
            if DEBUG_PRINT {
                rope.print();
            }
            visited_points.insert(*rope.get_tail());
        }
    });
    visited_points.len()
}

pub fn part1(input: &[String]) -> usize {
    simulate_rope(input, 2)
}

pub fn part2(input: &[String]) -> usize {
    simulate_rope(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day9_1.txt")), 13);
        assert_eq!(part1(&read_input_from_file("input/day9.txt")), 6311);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day9_1.txt")), 1);
        assert_eq!(part2(&read_input_from_file("sample/day9_2.txt")), 36);
        assert_eq!(part2(&read_input_from_file("input/day9.txt")), 2482);
    }
}
