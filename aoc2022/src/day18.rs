use std::{collections::VecDeque, error::Error, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',');
        let x = it.next().unwrap().parse::<i32>().unwrap() + 1;
        let y = it.next().unwrap().parse::<i32>().unwrap() + 1;
        let z = it.next().unwrap().parse::<i32>().unwrap() + 1;
        Ok(Cube { x, y, z })
    }
}

impl Cube {
    fn get_neighbors(&self) -> [Cube; 6] {
        [
            Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ]
    }
}

struct Grid {
    arr: [[[bool; Self::GRID_SIZE]; Self::GRID_SIZE]; Self::GRID_SIZE],
}

impl Grid {
    const GRID_SIZE: usize = 22;

    fn new() -> Self {
        Self {
            arr: [[[false; Self::GRID_SIZE]; Self::GRID_SIZE]; Self::GRID_SIZE],
        }
    }

    fn is_within(cube: &Cube) -> bool {
        let upper = i32::try_from(Self::GRID_SIZE).unwrap();
        0 <= cube.x
            && cube.x < upper
            && 0 <= cube.y
            && cube.y < upper
            && 0 <= cube.z
            && cube.z < upper
    }

    fn get_cell(&self, cube: &Cube) -> bool {
        self.arr[usize::try_from(cube.x).unwrap()][usize::try_from(cube.y).unwrap()]
            [usize::try_from(cube.z).unwrap()]
    }

    fn set_cell(&mut self, cube: &Cube, val: bool) {
        self.arr[usize::try_from(cube.x).unwrap()][usize::try_from(cube.y).unwrap()]
            [usize::try_from(cube.z).unwrap()] = val;
    }

    fn get_num_neighbors(&self, cube: &Cube) -> usize {
        let mut count = 0;
        for neighbor in cube.get_neighbors().iter() {
            if Self::is_within(neighbor) && self.get_cell(neighbor) {
                count += 1;
            }
        }
        count
    }

    fn calculate_accessible_cells(&self) -> usize {
        let mut exposed_faces = 0usize;

        let mut visited = Grid::new();
        let mut q = VecDeque::<Cube>::new();
        q.push_back(Cube { x: 0, y: 0, z: 0 });

        while let Some(cube) = q.pop_front() {
            if self.get_cell(&cube) {
                exposed_faces += 1;
            } else if !visited.get_cell(&cube) {
                visited.set_cell(&cube, true);
                for neighbor in cube.get_neighbors().iter() {
                    if Self::is_within(neighbor) {
                        q.push_back(*neighbor);
                    }
                }
            }
        }
        exposed_faces
    }
}

pub fn part1(input: &[String]) -> usize {
    let mut grid = Grid::new();

    let cubes = input
        .iter()
        .map(|s| s.parse::<Cube>().unwrap())
        .collect::<Vec<Cube>>();

    cubes.iter().for_each(|cube| {
        grid.set_cell(cube, true);
    });

    cubes.iter().fold(0, |mut acc, cube| {
        acc += 6 - grid.get_num_neighbors(cube);
        acc
    })
}

pub fn part2(input: &[String]) -> usize {
    let mut grid = Grid::new();

    let cubes = input
        .iter()
        .map(|s| s.parse::<Cube>().unwrap())
        .collect::<Vec<Cube>>();

    cubes.iter().for_each(|cube| {
        grid.set_cell(cube, true);
    });

    grid.calculate_accessible_cells()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day18.txt")), 64);
        assert_eq!(part1(&read_input_from_file("input/day18.txt")), 3448);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day18.txt")), 58);
        assert_eq!(part2(&read_input_from_file("input/day18.txt")), 2052);
    }
}
