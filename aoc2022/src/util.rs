use std::{
    cmp,
    fs::File,
    io::{self, BufRead},
    ops::{Add, AddAssign, Mul},
    str::FromStr,
};

pub fn read_input_from_file(filename: &str) -> Vec<String> {
    let file = File::open("res/".to_owned() + filename).expect("Error: input file does not exist");
    io::BufReader::new(file).lines().flatten().collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, factor: i32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_s = s.split_once(',').ok_or(ParsePointError)?;
        Ok(Point {
            x: split_s.0.parse::<i32>().map_err(|_| ParsePointError)?,
            y: split_s.1.parse::<i32>().map_err(|_| ParsePointError)?,
        })
    }
}

impl Point {
    pub fn max_distance_to(&self, other: &Point) -> u32 {
        cmp::max(
            (self.x - other.x).unsigned_abs(),
            (self.y - other.y).unsigned_abs(),
        )
    }

    pub fn manhattan_distance_to(&self, other: &Point) -> u32 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    pub fn is_within(&self, lower_x: i32, lower_y: i32, upper_x: i32, upper_y: i32) -> bool {
        lower_x <= self.x && self.x < upper_x && lower_y <= self.y && self.y < upper_y
    }

    #[rustfmt::skip]
    pub fn get_neighbors(&self) -> [Point; 4] {
        [
            Point {x: self.x    , y: self.y - 1},
            Point {x: self.x - 1, y: self.y    },
            Point {x: self.x    , y: self.y + 1},
            Point {x: self.x + 1, y: self.y    },
        ]
    }
}
