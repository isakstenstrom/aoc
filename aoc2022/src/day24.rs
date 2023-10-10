use std::collections::{HashSet, VecDeque};

use crate::util::Point;

fn arrow_to_usize(arrow: char) -> Option<usize> {
    match arrow {
        '^' => Some(0),
        '>' => Some(1),
        'v' => Some(2),
        '<' => Some(3),
        _ => None,
    }
}

struct Grid {
    height: usize,
    width: usize,
    blizzards: [VecDeque<VecDeque<bool>>; 4],
    walls: Vec<Vec<bool>>,
}

impl Grid {
    fn new(input: &[String]) -> Self {
        let grid_height = input.len() + 2;
        let grid_width = input[0].len() + 2;

        let blizzard = VecDeque::from(vec![
            VecDeque::from(vec![false; grid_width - 4]);
            grid_height - 4
        ]);

        let mut g = Grid {
            height: grid_height,
            width: grid_width,
            walls: vec![vec![true; grid_width]; grid_height],
            blizzards: [
                blizzard.clone(),
                blizzard.clone(),
                blizzard.clone(),
                blizzard.clone(),
            ],
        };

        input.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if let Some(blizzard_id) = arrow_to_usize(c) {
                    g.blizzards[blizzard_id][y - 1][x - 1] = true;
                }
                if c != '#' {
                    g.walls[y + 1][x + 1] = false;
                }
            })
        });

        g
    }

    fn get_cell_as_char(&self, x: usize, y: usize) -> char {
        if self.walls[y][x] {
            return '#';
        }

        if !self.in_blizzard_range(Point { x, y }) {
            return '.';
        }

        let num_blizzards = self.get_num_blizzards(Point { x, y });
        if num_blizzards > 1 {
            return num_blizzards.to_string().chars().last().unwrap();
        }

        if self.blizzards[0][y - 2][x - 2] {
            return '^';
        }
        if self.blizzards[1][y - 2][x - 2] {
            return '>';
        }
        if self.blizzards[2][y - 2][x - 2] {
            return 'v';
        }
        if self.blizzards[3][y - 2][x - 2] {
            return '<';
        }
        '.'
    }

    #[allow(dead_code)]
    fn print(&self, p: Point<usize>) {
        for y in 0..self.height {
            for x in 0..self.width {
                if p.x == x && p.y == y {
                    print!("E");
                } else {
                    print!("{}", self.get_cell_as_char(x, y));
                }
            }
            println!();
        }
    }

    fn get_start(&self) -> Point<usize> {
        Point { x: 2, y: 1 }
    }

    fn get_end(&self) -> Point<usize> {
        Point {
            x: self.width - 3,
            y: self.height - 2,
        }
    }

    fn in_blizzard_range(&self, p: Point<usize>) -> bool {
        p.is_within(2, 2, self.width - 2, self.height - 2)
    }

    fn get_num_blizzards(&self, p: Point<usize>) -> usize {
        if !self.in_blizzard_range(p) {
            return 0;
        }
        return [
            self.blizzards[0][p.y - 2][p.x - 2],
            self.blizzards[1][p.y - 2][p.x - 2],
            self.blizzards[2][p.y - 2][p.x - 2],
            self.blizzards[3][p.y - 2][p.x - 2],
        ]
        .iter()
        .filter(|a| **a)
        .count();
    }

    fn iterate(&mut self) {
        self.blizzards[0].rotate_left(1);
        self.blizzards[1].iter_mut().for_each(|a| a.rotate_right(1));
        self.blizzards[2].rotate_right(1);
        self.blizzards[3].iter_mut().for_each(|a| a.rotate_left(1));
    }

    fn search(&mut self, source: Point<usize>, target: Point<usize>) -> usize {
        let mut time = 0usize;

        let mut q: VecDeque<(Point<usize>, usize)> = VecDeque::new();
        let mut visited: HashSet<(Point<usize>, usize)> = HashSet::new();

        q.push_back((source, time));
        visited.insert((source, time));

        while let Some((v, v_time)) = q.pop_front() {
            if time == v_time {
                self.iterate();
                time += 1;
            }

            let mut neighbors = Vec::from(v.get_manhattan_neighbors());
            neighbors.push(v);

            for n in neighbors {
                if !self.walls[n.y][n.x] && self.get_num_blizzards(n) == 0 {
                    if n == target {
                        return time;
                    }
                    if visited.insert((n, time)) {
                        q.push_back((n, time));
                    }
                }
            }
        }
        panic!("No path found!")
    }
}

pub fn part1(input: &[String]) -> usize {
    let mut g = Grid::new(input);
    g.search(g.get_start(), g.get_end())
}

pub fn part2(input: &[String]) -> usize {
    let mut g = Grid::new(input);

    let mut total_time = 0usize;
    total_time += g.search(g.get_start(), g.get_end());
    total_time += g.search(g.get_end(), g.get_start());
    total_time += g.search(g.get_start(), g.get_end());
    total_time
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day24.txt")), 18);
        assert_eq!(part1(&read_input_from_file("input/day24.txt")), 292);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day24.txt")), 54);
        assert_eq!(part2(&read_input_from_file("input/day24.txt")), 816);
    }
}
