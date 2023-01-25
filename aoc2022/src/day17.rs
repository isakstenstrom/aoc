use std::{
    cmp,
    collections::{HashMap, HashSet},
    fmt,
};

use crate::util::Point;

const PART_1_MAX_ITERATIONS: u64 = 2022;
const PART_2_MAX_ITERATIONS: u64 = 1000000000000;

#[derive(Debug)]
struct Tower {
    blocks: HashSet<Point<i64>>,
    highest_blocks: [i64; Self::WIDTH],
}

impl Tower {
    const WIDTH: usize = 7;

    fn new() -> Self {
        let mut res = Self {
            blocks: HashSet::new(),
            highest_blocks: [-1; Self::WIDTH],
        };
        for x in 0..Self::WIDTH {
            res.blocks.insert(Point { x: x as i64, y: -1 });
        }
        res
    }

    fn is_within(&self, pos: &Point<i64>) -> bool {
        pos.y >= 0 && 0 <= pos.x && pos.x < Self::WIDTH as i64
    }

    fn is_empty(&self, pos: &Point<i64>) -> bool {
        !self.blocks.contains(pos)
    }

    fn get_height(&self) -> i64 {
        *self.highest_blocks.iter().max().unwrap() + 1
    }

    fn add_block(&mut self, pos: &Point<i64>) {
        self.blocks.insert(*pos);
        self.highest_blocks[pos.x as usize] = cmp::max(self.highest_blocks[pos.x as usize], pos.y);
    }

    fn get_relative_heights(&self) -> [i64; Self::WIDTH] {
        let highest_block = self.get_height();
        let mut res = [0; Self::WIDTH];

        for (i, block) in res.iter_mut().enumerate() {
            *block = highest_block - self.highest_blocks[i];
        }
        res
    }
}

impl fmt::Display for Tower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tower_height = self.get_height();
        for y in (0..tower_height).rev() {
            write!(f, "|")?;
            for x in 0..Self::WIDTH {
                if self.blocks.contains(&Point { x: x as i64, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        for _ in 0..Self::WIDTH + 2 {
            write!(f, "-")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Rock {
    num_blocks: usize,
    blocks: [Point<i64>; 5],
}

impl Rock {
    const NUM_PIECES: u64 = 5;

    #[rustfmt::skip]
    fn get_next_rock(index: u64) -> Rock {
        match index % Self::NUM_PIECES {
            0 => Rock{ num_blocks: 4, blocks: [Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 2, y: 0}, Point{x: 3, y: 0}, Point{x: 0, y: 0}]},
            1 => Rock{ num_blocks: 5, blocks: [Point{x: 1, y: 0}, Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 2, y: 1}, Point{x: 1, y: 2}]},
            2 => Rock{ num_blocks: 5, blocks: [Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 2, y: 0}, Point{x: 2, y: 1}, Point{x: 2, y: 2}]},
            3 => Rock{ num_blocks: 4, blocks: [Point{x: 0, y: 0}, Point{x: 0, y: 1}, Point{x: 0, y: 2}, Point{x: 0, y: 3}, Point{x: 0, y: 0}]},
            4 => Rock{ num_blocks: 4, blocks: [Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 0, y: 0}]},
            _ => panic!("Invalid index"),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct PatternMatchState {
    i_piece_cycle: u64,
    i_wind_cycle: u64,
    heights: [i64; Tower::WIDTH],
}

fn solve(input: &[String], max_num_rocks: u64) -> u64 {
    let mut wind_iter = input[0].chars().cycle();
    let mut tower = Tower::new();

    let mut prev_states: HashMap<PatternMatchState, (u64, i64)> = HashMap::new();
    let mut cycle_added_length: u64 = 0;
    let mut cycle_found = false;

    let mut rock_index: u64 = 0;
    let mut wind_index: u64 = 0;
    let wind_cycle_length = input[0].len() as u64;

    while rock_index < max_num_rocks {
        let rock = Rock::get_next_rock(rock_index);

        let mut rock_pos = Point {
            x: 2,
            y: tower.get_height() + 3,
        };

        let mut done_falling = false;
        while !done_falling {
            wind_index += 1;
            let horizontal_movement = match wind_iter.next().unwrap() {
                '<' => Point { x: -1, y: 0 },
                '>' => Point { x: 1, y: 0 },
                _ => panic!("Invalid input"),
            };

            if rock.blocks.iter().all(|p| {
                let tmp_pos = rock_pos + *p + horizontal_movement;
                tower.is_within(&tmp_pos) && tower.is_empty(&tmp_pos)
            }) {
                rock_pos += horizontal_movement
            }

            let vertical_movement = Point { x: 0, y: -1 };

            if rock.blocks[0..rock.num_blocks]
                .iter()
                .all(|p| tower.is_empty(&(rock_pos + *p + vertical_movement)))
            {
                rock_pos += vertical_movement;
            } else {
                done_falling = true;
            }
        }

        for p in rock.blocks[0..rock.num_blocks].iter() {
            tower.add_block(&(rock_pos + *p));
        }

        if !cycle_found {
            let pattern_match_state = PatternMatchState {
                i_piece_cycle: rock_index % Rock::NUM_PIECES,
                i_wind_cycle: wind_index % wind_cycle_length,
                heights: tower.get_relative_heights(),
            };

            let prev_state =
                prev_states.insert(pattern_match_state, (rock_index, tower.get_height()));

            if let Some((prev_index, prev_height)) = prev_state {
                let cycle_len = rock_index - prev_index;
                let cycle_height = tower.get_height() - prev_height;
                let remaining_cycles = (max_num_rocks - rock_index - 1) / cycle_len;

                cycle_added_length = remaining_cycles * u64::try_from(cycle_height).unwrap();

                rock_index += remaining_cycles * cycle_len;
                cycle_found = true;
            }
        }

        rock_index += 1;
    }

    u64::try_from(tower.get_height()).unwrap() + cycle_added_length
}

pub fn task1(input: &[String]) -> u64 {
    solve(input, PART_1_MAX_ITERATIONS)
}

pub fn task2(input: &[String]) -> u64 {
    solve(input, PART_2_MAX_ITERATIONS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&read_input_from_file("sample/day17.txt")), 3068);
        assert_eq!(task1(&read_input_from_file("input/day17.txt")), 3209);
    }

    #[test]
    fn test_task2() {
        assert_eq!(
            task2(&read_input_from_file("sample/day17.txt")),
            1514285714288
        );
        assert_eq!(
            task2(&read_input_from_file("input/day17.txt")),
            1580758017509
        );
    }
}
