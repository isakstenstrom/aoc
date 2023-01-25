use crate::util::{ParsePointError, Point};
use std::cmp;

struct RockMaze {
    grid: Vec<Vec<bool>>,
    array_offset: usize,
    height: usize,
    amount_of_sand: usize,
    next_start_pos: Point<i32>,
}

impl RockMaze {
    fn new(input: &[String], add_floor: bool) -> Result<Self, ParsePointError> {
        let rock_rows: Vec<Vec<Point<i32>>> = input
            .iter()
            .map(|line| {
                line.split(" -> ")
                    .map(|s| s.parse::<Point<i32>>())
                    .collect::<Result<Vec<Point<i32>>, ParsePointError>>()
            })
            .collect::<Result<Vec<Vec<Point<i32>>>, ParsePointError>>()?;

        let mut x_min: usize = 500;
        let mut x_max: usize = 500;
        let mut y_max: usize = 0;

        for rocks in rock_rows.iter() {
            for rock in rocks {
                x_min = cmp::min(x_min, (rock.x) as usize);
                x_max = cmp::max(x_max, (rock.x) as usize);
                y_max = cmp::max(y_max, (rock.y) as usize);
            }
        }

        if add_floor {
            y_max += 2;
            x_min = cmp::min(x_min, 500 - y_max);
            x_max = cmp::max(x_max, 500 + y_max);
        }

        // Widens the maze so that sand can fall down on either side of the
        // outermost rocks.
        x_min -= 1;
        x_max += 1;

        let mut rock_maze = Self {
            grid: vec![vec![false; x_max - x_min + 1]; y_max + 1],
            array_offset: x_min,
            height: y_max,
            amount_of_sand: 0,
            next_start_pos: Point { x: 500, y: 0 },
        };

        for rocks in rock_rows {
            rocks.windows(2).for_each(|points| {
                let step = Point {
                    x: (points[1].x - points[0].x).signum(),
                    y: (points[1].y - points[0].y).signum(),
                };

                // Asserts that exactly one of x and y are 0
                assert!((step.x == 0) ^ (step.y == 0));

                for i in 0..=cmp::max(
                    (points[1].x - points[0].x).abs(),
                    (points[1].y - points[0].y).abs(),
                ) {
                    rock_maze.set_content_with_point(&(points[0] + step * i), true);
                }
            })
        }

        if add_floor {
            for x in (500 - rock_maze.height)..=(500 + rock_maze.height) {
                rock_maze.set_content(x, rock_maze.height, true);
            }
        }
        Ok(rock_maze)
    }

    fn get_content_with_point(&self, p: &Point<i32>) -> bool {
        self.grid[p.y as usize][(p.x as usize) - self.array_offset]
    }

    fn set_content(&mut self, x: usize, y: usize, content: bool) {
        self.grid[y][x - self.array_offset] = content;
    }

    fn set_content_with_point(&mut self, p: &Point<i32>, content: bool) {
        self.grid[p.y as usize][(p.x as usize) - self.array_offset] = content;
    }

    const POSSIBLE_NEIGHBORS: [Point<i32>; 3] = [
        Point { x: 0, y: 1 },
        Point { x: -1, y: 1 },
        Point { x: 1, y: 1 },
    ];

    fn simulate_sand(&mut self) -> bool {
        if self.get_content_with_point(&self.next_start_pos) {
            // Resets the starting position
            self.next_start_pos = Point { x: 500, y: 0 };
            if self.get_content_with_point(&self.next_start_pos) {
                return false;
            }
        }

        let mut current_point = self.next_start_pos;

        while (current_point.y as usize) < self.height {
            let mut found_empty_space = false;

            for step in Self::POSSIBLE_NEIGHBORS {
                let next_point = current_point + step;

                if !self.get_content_with_point(&next_point) {
                    self.next_start_pos = current_point;
                    current_point = next_point;
                    found_empty_space = true;
                    break;
                }
            }

            if !found_empty_space {
                self.set_content_with_point(&current_point, true);
                self.amount_of_sand += 1;
                return true;
            }
        }
        false
    }
}

pub fn task1(input: &[String]) -> usize {
    let mut maze = RockMaze::new(input, false).unwrap();

    while maze.simulate_sand() {}

    maze.amount_of_sand
}

pub fn task2(input: &[String]) -> usize {
    let mut maze = RockMaze::new(input, true).unwrap();

    while maze.simulate_sand() {}

    maze.amount_of_sand
}

#[cfg(test)]
mod tests {
    use super::{task1, task2};
    use crate::util::read_input_from_file;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&read_input_from_file("sample/day14.txt")), 24);
        assert_eq!(task1(&read_input_from_file("input/day14.txt")), 897);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&read_input_from_file("sample/day14.txt")), 93);
        assert_eq!(task2(&read_input_from_file("input/day14.txt")), 26683);
    }
}
