use std::{
    cmp::{self, max},
    collections::HashSet,
    vec,
};

use crate::util::read_input_as_lines;

const DEBUG_PRINT: bool = false;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn distance_to(&mut self, other: &Point) -> u32 {
        max(
            (self.x - other.x).unsigned_abs(),
            (self.y - other.y).unsigned_abs(),
        )
    }
}

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

    fn get_move(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Down => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn new(length: usize) -> Rope {
        Rope {
            knots: vec![Point { x: 0, y: 0 }; length],
        }
    }

    fn move_head(&mut self, dir: &Direction) {
        let head = self.knots.first_mut().unwrap();
        head.add(&dir.get_move());
        let mut preceding_knot = head.clone();

        for knot in self.knots[1..].iter_mut() {
            if knot.distance_to(&preceding_knot) > 1 {
                if preceding_knot.x == knot.x {
                    knot.y += (preceding_knot.y - knot.y).signum();
                } else if preceding_knot.y == knot.y {
                    knot.x += (preceding_knot.x - knot.x).signum();
                } else {
                    for neighbor in [
                        Point { x: -1, y: -1 },
                        Point { x: 1, y: -1 },
                        Point { x: -1, y: 1 },
                        Point { x: 1, y: 1 },
                    ] {
                        let mut diagonal = knot.clone();
                        diagonal.add(&neighbor);
                        if preceding_knot.distance_to(&diagonal) <= 1 {
                            *knot = diagonal;
                            break;
                        }
                    }
                }
            }
            preceding_knot = knot.clone();
        }
    }

    fn get_tail(&self) -> &Point {
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

fn simulate_rope(rope_length: usize) -> usize {
    let mut visited_points: HashSet<Point> = HashSet::new();
    let mut rope = Rope::new(rope_length);
    visited_points.insert(rope.get_tail().clone());

    read_input_as_lines("day9.txt").iter().for_each(|line| {
        let mut split_line = line.split_whitespace();

        let dir = Direction::from_str(split_line.next().unwrap()).unwrap();
        let num_steps = str::parse::<u32>(split_line.next().unwrap()).unwrap();

        for _ in 0..num_steps {
            rope.move_head(&dir);
            if DEBUG_PRINT {
                rope.print();
            }
            visited_points.insert(rope.get_tail().clone());
        }
    });
    visited_points.len()
}

pub fn task1() -> usize {
    simulate_rope(2)
}

pub fn task2() -> usize {
    simulate_rope(10)
}

#[cfg(test)]
mod tests {
    use crate::day9::{task1, task2};

    #[test]
    fn test_task1() {
        assert_eq!(task1(), 6311);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(), 2482);
    }
}
