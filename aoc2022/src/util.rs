use std::{
    cmp,
    fs::File,
    io::{self, BufRead},
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    str::FromStr,
};

use num::One;

pub fn read_input_from_file(filename: &str) -> Vec<String> {
    let file = File::open("res/".to_owned() + filename).expect("Error: input file does not exist");
    io::BufReader::new(file).lines().flatten().collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Point<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, factor: T) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePointError;

impl<T, U> FromStr for Point<T>
where
    T: FromStr<Err = U>,
{
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_s = s.split_once(',').ok_or(ParsePointError)?;
        Ok(Self {
            x: split_s.0.parse::<T>().map_err(|_| ParsePointError)?,
            y: split_s.1.parse::<T>().map_err(|_| ParsePointError)?,
        })
    }
}

impl Point<i32> {
    pub fn max_distance_to(&self, other: &Self) -> u32 {
        cmp::max(
            (self.x - other.x).unsigned_abs(),
            (self.y - other.y).unsigned_abs(),
        )
    }

    pub fn manhattan_distance_to(&self, other: &Self) -> u32 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}

impl<T> Point<T>
where
    T: PartialOrd,
{
    pub fn is_within(&self, lower_x: T, lower_y: T, upper_x: T, upper_y: T) -> bool {
        lower_x <= self.x && self.x < upper_x && lower_y <= self.y && self.y < upper_y
    }
}

impl<T> Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + num::One + Copy,
{
    #[rustfmt::skip]
    pub fn get_manhattan_neighbors(&self) -> [Self; 4] {
        [
            Self {x: self.x             , y: self.y - One::one()},
            Self {x: self.x - One::one(), y: self.y             },
            Self {x: self.x             , y: self.y + One::one()},
            Self {x: self.x + One::one(), y: self.y             },
        ]
    }

    #[rustfmt::skip]
    pub fn get_neighbors(&self) -> [Self; 8] {
        [
            Self {x: self.x - One::one(), y: self.y - One::one()},
            Self {x: self.x             , y: self.y - One::one()},
            Self {x: self.x + One::one(), y: self.y - One::one()},
            Self {x: self.x - One::one(), y: self.y             },
            Self {x: self.x + One::one(), y: self.y             },
            Self {x: self.x - One::one(), y: self.y + One::one()},
            Self {x: self.x             , y: self.y + One::one()},
            Self {x: self.x + One::one(), y: self.y + One::one()},
        ]
    }
}
