use std::{
    collections::{HashSet, VecDeque},
    fmt, ops,
};

use crate::util::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn get_adjacent(&self) -> [Direction; 3] {
        match self {
            Direction::North => [Direction::NorthWest, Direction::North, Direction::NorthEast],
            Direction::NorthEast => [Direction::North, Direction::NorthEast, Direction::East],
            Direction::East => [Direction::NorthEast, Direction::East, Direction::SouthEast],
            Direction::SouthEast => [Direction::East, Direction::SouthEast, Direction::South],
            Direction::South => [Direction::SouthEast, Direction::South, Direction::SouthWest],
            Direction::SouthWest => [Direction::South, Direction::SouthWest, Direction::West],
            Direction::West => [Direction::SouthWest, Direction::West, Direction::NorthWest],
            Direction::NorthWest => [Direction::West, Direction::NorthWest, Direction::North],
        }
    }
}

impl ops::Add<Direction> for Point<i16> {
    type Output = Point<i16>;

    #[rustfmt::skip]
    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North =>     Point {x: self.x,       y: self.y - 1   },
            Direction::NorthEast => Point {x: self.x + 1,   y: self.y - 1   },
            Direction::East =>      Point {x: self.x + 1,   y: self.y       },
            Direction::SouthEast => Point {x: self.x + 1,   y: self.y + 1   },
            Direction::South =>     Point {x: self.x,       y: self.y + 1   },
            Direction::SouthWest => Point {x: self.x - 1,   y: self.y + 1   },
            Direction::West =>      Point {x: self.x - 1,   y: self.y       },
            Direction::NorthWest => Point {x: self.x - 1,   y: self.y - 1   },
        }
    }
}

struct Grid {
    map: HashSet<Point<i16>>,
    dir_queue: VecDeque<Direction>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (min_x, max_x, min_y, max_y) = self.get_bounding_box();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.map.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(input: &[String]) -> Self {
        let mut g = Self {
            map: HashSet::new(),
            dir_queue: VecDeque::from_iter([
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ]),
        };

        input.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    g.map.insert(Point {
                        x: i16::try_from(x).unwrap(),
                        y: i16::try_from(y).unwrap(),
                    });
                }
            })
        });

        g
    }

    fn has_neighbor(&self, p: &Point<i16>) -> bool {
        for neighbor in p.get_neighbors() {
            if self.map.contains(&neighbor) {
                return true;
            }
        }
        false
    }

    fn can_move_to_pos(&self, p: &Point<i16>, dir: &Direction) -> bool {
        for adj_dir in dir.get_adjacent() {
            if self.map.contains(&(*p + adj_dir)) {
                return false;
            }
        }
        true
    }

    fn get_next_pos(&self, old_p: &Point<i16>) -> Option<Point<i16>> {
        if !self.has_neighbor(old_p) {
            return None;
        }

        for dir in self.dir_queue.iter() {
            if self.can_move_to_pos(old_p, dir) {
                return Some(*old_p + *dir);
            }
        }
        None
    }

    fn perform_iteration(&mut self) -> bool {
        let mut new_map: HashSet<Point<i16>> = HashSet::new();

        let mut map_changed = false;
        for old_p in self.map.iter() {
            let new_p = self.get_next_pos(old_p);

            if let Some(new_p) = new_p {
                map_changed = true;
                if !new_map.insert(new_p) {
                    new_map.remove(&new_p);
                    new_map.insert(*old_p);
                    new_map.insert(new_p * 2 - *old_p);
                }
            } else {
                new_map.insert(*old_p);
            }
        }
        if !map_changed {
            return false;
        }

        self.map = new_map;
        self.dir_queue.rotate_left(1);

        true
    }

    fn get_bounding_box(&self) -> (i16, i16, i16, i16) {
        let mut min_x = i16::MAX;
        let mut max_x = i16::MIN;
        let mut min_y = i16::MAX;
        let mut max_y = i16::MIN;

        for p in self.map.iter() {
            min_x = i16::min(min_x, p.x);
            max_x = i16::max(max_x, p.x);
            min_y = i16::min(min_y, p.y);
            max_y = i16::max(max_y, p.y);
        }

        (min_x, max_x, min_y, max_y)
    }

    fn get_grid_size(&self) -> i16 {
        let (min_x, max_x, min_y, max_y) = self.get_bounding_box();
        (max_x - min_x + 1) * (max_y - min_y + 1) - self.map.len() as i16
    }
}

pub fn part1(input: &[String]) -> i16 {
    let mut grid = Grid::new(input);

    for _ in 0..10 {
        grid.perform_iteration();
    }

    grid.get_grid_size()
}

pub fn part2(input: &[String]) -> i16 {
    let mut grid = Grid::new(input);

    let mut i = 1;
    while grid.perform_iteration() {
        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day23.txt")), 110);
        assert_eq!(part1(&read_input_from_file("input/day23.txt")), 4068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day23.txt")), 20);
        assert_eq!(part2(&read_input_from_file("input/day23.txt")), 968);
    }
}
