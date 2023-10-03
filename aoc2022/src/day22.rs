use itertools::Itertools;
use std::fmt;

use crate::util::Point;

const NUM_EDGES: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Wall,
    Outside,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Open => ".",
                Tile::Wall => "#",
                Tile::Outside => " ",
            }
        )
    }
}

impl Tile {
    fn from_char(s: char) -> Result<Self, ()> {
        match s {
            '.' => Ok(Tile::Open),
            '#' => Ok(Tile::Wall),
            ' ' => Ok(Tile::Outside),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn step(&self, p: &Point<i32>) -> Point<i32> {
        match self {
            Direction::Up => Point { x: p.x, y: p.y - 1 },
            Direction::Right => Point { x: p.x + 1, y: p.y },
            Direction::Down => Point { x: p.x, y: p.y + 1 },
            Direction::Left => Point { x: p.x - 1, y: p.y },
        }
    }
}

fn is_on_line(p: Point<i32>, line: &(Point<i32>, Point<i32>)) -> bool {
    let p1x = i32::min(line.0.x, line.1.x);
    let p2x = i32::max(line.0.x, line.1.x);
    let p1y = i32::min(line.0.y, line.1.y);
    let p2y = i32::max(line.0.y, line.1.y);
    p1x <= p.x && p.x <= p2x && p1y <= p.y && p.y <= p2y
}

fn map_onto_line(v: i32, f1: i32, f2: i32, t1: i32, t2: i32) -> i32 {
    (v - f1) * i32::signum(f2 - f1) * i32::signum(t2 - t1) + t1
}

fn map_point(
    p: Point<i32>,
    f: &(Point<i32>, Point<i32>),
    t: &(Point<i32>, Point<i32>),
    flip: bool,
) -> Point<i32> {
    let (f1, f2) = f;
    let (t1, t2) = t;

    let (t1, t2) = if flip { (t2, t1) } else { (t1, t2) };

    debug_assert!((f1.x == f2.x) != (f1.y == f2.y));
    debug_assert!((t1.x == t2.x) != (t1.y == t2.y));

    if (f1.x == f2.x) != (t1.x == t2.x) {
        return Point {
            x: map_onto_line(p.y, f1.y, f2.y, t1.x, t2.x),
            y: map_onto_line(p.x, f1.x, f2.x, t1.y, t2.y),
        };
    }

    Point {
        x: map_onto_line(p.x, f1.x, f2.x, t1.x, t2.x),
        y: map_onto_line(p.y, f1.y, f2.y, t1.y, t2.y),
    }
}

struct BoxWrap {
    edges: [(Point<i32>, Point<i32>); NUM_EDGES],
    map_to: [usize; NUM_EDGES],
    edge_dirs: [Direction; NUM_EDGES],
}

const BOX_WRAP_SAMPLE_INPUT: BoxWrap = BoxWrap {
    edges: [
        (Point { x: 9, y: 0 }, Point { x: 12, y: 0 }),
        (Point { x: 13, y: 1 }, Point { x: 13, y: 4 }),
        (Point { x: 13, y: 5 }, Point { x: 13, y: 8 }),
        (Point { x: 13, y: 8 }, Point { x: 16, y: 8 }),
        (Point { x: 17, y: 9 }, Point { x: 17, y: 12 }),
        (Point { x: 16, y: 13 }, Point { x: 13, y: 13 }),
        (Point { x: 12, y: 13 }, Point { x: 9, y: 13 }),
        (Point { x: 8, y: 12 }, Point { x: 8, y: 9 }),
        (Point { x: 8, y: 9 }, Point { x: 5, y: 9 }),
        (Point { x: 4, y: 9 }, Point { x: 1, y: 9 }),
        (Point { x: 0, y: 8 }, Point { x: 0, y: 5 }),
        (Point { x: 1, y: 4 }, Point { x: 4, y: 4 }),
        (Point { x: 5, y: 4 }, Point { x: 8, y: 4 }),
        (Point { x: 8, y: 4 }, Point { x: 8, y: 1 }),
    ],
    map_to: [11, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 0, 13, 12],
    edge_dirs: [
        Direction::Up,
        Direction::Right,
        Direction::Right,
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Down,
        Direction::Left,
        Direction::Down,
        Direction::Down,
        Direction::Left,
        Direction::Up,
        Direction::Up,
        Direction::Left,
    ],
};

const BOX_WRAP_REAL_INPUT: BoxWrap = BoxWrap {
    edges: [
        (Point { x: 51, y: 0 }, Point { x: 100, y: 0 }),
        (Point { x: 101, y: 0 }, Point { x: 150, y: 0 }),
        (Point { x: 151, y: 1 }, Point { x: 151, y: 50 }),
        (Point { x: 150, y: 51 }, Point { x: 101, y: 51 }),
        (Point { x: 101, y: 51 }, Point { x: 101, y: 100 }),
        (Point { x: 101, y: 101 }, Point { x: 101, y: 150 }),
        (Point { x: 100, y: 151 }, Point { x: 51, y: 151 }),
        (Point { x: 51, y: 151 }, Point { x: 51, y: 200 }),
        (Point { x: 50, y: 201 }, Point { x: 1, y: 201 }),
        (Point { x: 0, y: 200 }, Point { x: 0, y: 151 }),
        (Point { x: 0, y: 150 }, Point { x: 0, y: 101 }),
        (Point { x: 1, y: 100 }, Point { x: 50, y: 100 }),
        (Point { x: 50, y: 100 }, Point { x: 50, y: 51 }),
        (Point { x: 50, y: 50 }, Point { x: 50, y: 1 }),
    ],
    map_to: [9, 8, 5, 4, 3, 2, 7, 6, 1, 0, 13, 12, 11, 10],
    edge_dirs: [
        Direction::Up,
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Right,
        Direction::Right,
        Direction::Down,
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Left,
        Direction::Up,
        Direction::Left,
        Direction::Left,
    ],
};

struct Board {
    arr: Vec<Vec<Tile>>,
    pos: Point<i32>,
    dir: Direction,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.arr.iter().for_each(|v| {
            v.iter().for_each(|e| {
                write!(f, "{}", e).unwrap();
            });
            writeln!(f).unwrap();
        });
        Ok(())
    }
}

impl Board {
    fn create_board(input: &[String]) -> Self {
        let board_height = input.len() + 2;
        let board_width = input.iter().map(|s| s.len()).max().unwrap() + 2;

        let mut board = Board {
            arr: vec![vec![Tile::Outside; board_width]; board_height],
            pos: Point { x: 0, y: 0 },
            dir: Direction::Right,
        };

        input.iter().enumerate().for_each(|(y, s)| {
            s.chars()
                .enumerate()
                .for_each(|(x, c)| board.arr[y + 1][x + 1] = Tile::from_char(c).unwrap())
        });

        board.pos = Point {
            x: board.arr[1]
                .iter()
                .find_position(|elem| **elem == Tile::Open)
                .unwrap()
                .0
                .try_into()
                .unwrap(),
            y: 1,
        };
        board
    }

    fn get_height(&self) -> usize {
        self.arr.len()
    }

    fn get_width(&self) -> usize {
        self.arr[0].len()
    }

    fn get_tile_at(&self, p: &Point<i32>) -> Tile {
        self.arr[p.y as usize][p.x as usize]
    }

    fn wrap_flat(&mut self, mut p: Point<i32>, d: Direction) -> (Point<i32>, Direction) {
        p = match &self.dir {
            Direction::Up => Point {
                x: p.x,
                y: self.get_height() as i32 - 1,
            },
            Direction::Right => Point { x: 0, y: p.y },
            Direction::Down => Point { x: p.x, y: 0 },
            Direction::Left => Point {
                x: self.get_width() as i32 - 1,
                y: p.y,
            },
        };
        while self.get_tile_at(&p) == Tile::Outside {
            p = self.dir.step(&p);
        }
        (p, d)
    }

    fn wrap_as_cube(
        &mut self,
        mut p: Point<i32>,
        mut d: Direction,
        box_wrap: &BoxWrap,
    ) -> (Point<i32>, Direction) {
        for i in 0..NUM_EDGES {
            if is_on_line(p, &box_wrap.edges[i]) && d == box_wrap.edge_dirs[i] {
                p = map_point(
                    p,
                    &box_wrap.edges[i],
                    &box_wrap.edges[box_wrap.map_to[i]],
                    true,
                );
                d = box_wrap.edge_dirs[box_wrap.map_to[i]].reverse();
                p = d.step(&p);
                return (p, d);
            }
        }
        panic!("Not in a wrapping position");
    }

    fn make_move(&mut self, s: &str, cube_wrap_map: Option<&BoxWrap>) {
        if s == "R" {
            self.dir = self.dir.turn_right();
            return;
        }
        if s == "L" {
            self.dir = self.dir.turn_left();
            return;
        }

        let steps = s.parse::<i32>().unwrap();
        for _ in 0..steps {
            let mut next_pos = self.dir.step(&self.pos);
            let mut next_dir = self.dir;

            if self.get_tile_at(&next_pos) == Tile::Outside {
                if let Some(box_wrap) = cube_wrap_map {
                    (next_pos, next_dir) = self.wrap_as_cube(next_pos, next_dir, box_wrap);
                } else {
                    (next_pos, next_dir) = self.wrap_flat(next_pos, next_dir);
                }
            }

            if self.get_tile_at(&next_pos) == Tile::Wall {
                break;
            }
            self.pos = next_pos;
            self.dir = next_dir;
        }
    }

    fn get_result(&self) -> i32 {
        self.pos.y * 1000
            + self.pos.x * 4
            + match self.dir {
                Direction::Up => 3,
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 2,
            }
    }
}

fn solve(input: &[String], wrap_as_cube: bool) -> i32 {
    let mut board = Board::create_board(&input[..(input.len() - 2)]);

    let moves = input
        .last()
        .unwrap()
        .replace('R', "_R_")
        .replace('L', "_L_")
        .split('_')
        .map(String::from)
        .collect_vec();

    let box_wrap = if wrap_as_cube {
        if board.get_height() >= 20 {
            Some(&BOX_WRAP_REAL_INPUT)
        } else {
            Some(&BOX_WRAP_SAMPLE_INPUT)
        }
    } else {
        None
    };

    moves.iter().for_each(|m| board.make_move(m, box_wrap));

    board.get_result()
}

pub fn part1(input: &[String]) -> i32 {
    solve(input, false)
}

pub fn part2(input: &[String]) -> i32 {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[rustfmt::skip]
    #[test]
    fn test_map_point() {

        assert_eq!(map_onto_line(1, 1, 4, 5, 8), 5);
        assert_eq!(map_onto_line(2, 1, 4, 5, 8), 6);
        assert_eq!(map_onto_line(3, 1, 4, 5, 8), 7);
        assert_eq!(map_onto_line(4, 1, 4, 5, 8), 8);

        assert_eq!(map_onto_line(1, 1, 4, 8, 5), 8);
        assert_eq!(map_onto_line(2, 1, 4, 8, 5), 7);
        assert_eq!(map_onto_line(3, 1, 4, 8, 5), 6);
        assert_eq!(map_onto_line(4, 1, 4, 8, 5), 5);


        let f1 = Point { x: 0, y: 0 };
        let f2 = Point { x: 0, y: 3 };
        let t1 = Point { x: 4, y: 4 };
        let t2 = Point { x: 4, y: 7 };
        assert_eq!(map_point(Point { x: 0, y: 0 }, &(f1, f2), &(t1, t2), false), Point { x: 4, y: 4 });
        assert_eq!(map_point(Point { x: 0, y: 1 }, &(f1, f2), &(t1, t2), false), Point { x: 4, y: 5 });
        assert_eq!(map_point(Point { x: 0, y: 2 }, &(f1, f2), &(t1, t2), false), Point { x: 4, y: 6 });
        assert_eq!(map_point(Point { x: 0, y: 3 }, &(f1, f2), &(t1, t2), false), Point { x: 4, y: 7 });

        let f1 = Point { x: 0, y: 0 };
        let f2 = Point { x: 3, y: 0 };
        let t1 = Point { x: 4, y: 4 };
        let t2 = Point { x: 7, y: 4 };
        assert_eq!(map_point(Point { x: 0, y: 0 }, &(f1, f2), &(t1, t2), false), Point { x: 4, y: 4 });
        assert_eq!(map_point(Point { x: 1, y: 0 }, &(f1, f2), &(t1, t2), false), Point { x: 5, y: 4 });
        assert_eq!(map_point(Point { x: 2, y: 0 }, &(f1, f2), &(t1, t2), false), Point { x: 6, y: 4 });
        assert_eq!(map_point(Point { x: 3, y: 0 }, &(f1, f2), &(t1, t2), false), Point { x: 7, y: 4 });

        let f1 = Point { x: 0, y: 0 };
        let f2 = Point { x: 0, y: 3 };
        let t1 = Point { x: 4, y: 4 };
        let t2 = Point { x: 4, y: 7 };
        assert_eq!(map_point(Point { x: 0, y: 0 }, &(f1, f2), &(t1, t2), true), Point { x: 4, y: 7 });
        assert_eq!(map_point(Point { x: 0, y: 1 }, &(f1, f2), &(t1, t2), true), Point { x: 4, y: 6 });
        assert_eq!(map_point(Point { x: 0, y: 2 }, &(f1, f2), &(t1, t2), true), Point { x: 4, y: 5 });
        assert_eq!(map_point(Point { x: 0, y: 3 }, &(f1, f2), &(t1, t2), true), Point { x: 4, y: 4 });

        let f1 = Point { x: 0, y: 0 };
        let f2 = Point { x: 3, y: 0 };
        let t1 = Point { x: 4, y: 4 };
        let t2 = Point { x: 7, y: 4 };
        assert_eq!(map_point(Point { x: 0, y: 0 }, &(f1, f2), &(t1, t2), true), Point { x: 7, y: 4 });
        assert_eq!(map_point(Point { x: 1, y: 0 }, &(f1, f2), &(t1, t2), true), Point { x: 6, y: 4 });
        assert_eq!(map_point(Point { x: 2, y: 0 }, &(f1, f2), &(t1, t2), true), Point { x: 5, y: 4 });
        assert_eq!(map_point(Point { x: 3, y: 0 }, &(f1, f2), &(t1, t2), true), Point { x: 4, y: 4 });

        let f1 = Point { x: 0, y: 0 };
        let f2 = Point { x: 0, y: 3 };
        let t1 = Point { x: 4, y: 4 };
        let t2 = Point { x: 7, y: 4 };
        assert_eq!(map_point(Point { x: 0, y: 0 }, &(f1, f2), &(t1, t2), false), Point { x: 4, y: 4 });
        assert_eq!(map_point(Point { x: 0, y: 1 }, &(f1, f2), &(t1, t2), false), Point { x: 5, y: 4 });
        assert_eq!(map_point(Point { x: 0, y: 2 }, &(f1, f2), &(t1, t2), false), Point { x: 6, y: 4 });
        assert_eq!(map_point(Point { x: 0, y: 3 }, &(f1, f2), &(t1, t2), false), Point { x: 7, y: 4 });

        let f1 = Point { x: 0, y: 0 };
        let f2 = Point { x: 0, y: 3 };
        let t1 = Point { x: 4, y: 4 };
        let t2 = Point { x: 7, y: 4 };
        assert_eq!(map_point(Point { x: 0, y: 0 }, &(f1, f2), &(t1, t2), true), Point { x: 7, y: 4 });
        assert_eq!(map_point(Point { x: 0, y: 1 }, &(f1, f2), &(t1, t2), true), Point { x: 6, y: 4 });
        assert_eq!(map_point(Point { x: 0, y: 2 }, &(f1, f2), &(t1, t2), true), Point { x: 5, y: 4 });
        assert_eq!(map_point(Point { x: 0, y: 3 }, &(f1, f2), &(t1, t2), true), Point { x: 4, y: 4 });
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day22.txt")), 6032);
        assert_eq!(part1(&read_input_from_file("input/day22.txt")), 181128);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day22.txt")), 5031);
        assert_eq!(part2(&read_input_from_file("input/day22.txt")), 52311);
    }
}
