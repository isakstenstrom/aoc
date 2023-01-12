use std::{cmp, str::FromStr};

use itertools::Itertools;

use crate::util::Point;

#[derive(Debug, Clone)]
struct Line {
    k: i32,
    m: i32,
}

impl Line {
    fn from_points(p1: &Point, p2: &Point) -> Self {
        let k = (p2.y - p1.y) / (p2.x - p1.x);
        let m = p1.y - k * p1.x;
        Self { k, m }
    }

    fn intersection(&self, other: &Line) -> Option<Point> {
        if self.k == other.k {
            return None;
        }

        let x = (other.m - self.m) / (self.k - other.k);
        let y = self.k * x + self.m;

        Some(Point { x, y })
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    beacon: Point,
    range: usize,
}

impl Sensor {
    fn new(x: i32, y: i32, beacon_x: i32, beacon_y: i32) -> Self {
        let mut s = Self {
            location: Point { x, y },
            beacon: Point {
                x: beacon_x,
                y: beacon_y,
            },
            range: 0,
        };
        s.range = s.location.manhattan_distance_to(&s.beacon) as usize;
        s
    }

    fn covers(&self, x: i32, y: i32) -> bool {
        (self.location.manhattan_distance_to(&Point { x, y }) as usize) <= self.range
    }

    // Returns the range of x values that are covered by this sensor at the
    // given y height (inclusive)
    fn range_at_y(&self, y: i32) -> Option<(i32, i32)> {
        let range: i32 = self.range.try_into().unwrap();
        if (self.location.y - y).abs() > range {
            return None;
        }

        Some((
            self.location.x - (range - (self.location.y - y).abs()),
            self.location.x + (range - (self.location.y - y).abs()),
        ))
    }

    fn get_lines(&self) -> [Line; 4] {
        let range: i32 = self.range.try_into().unwrap();
        [
            // Upper left
            Line::from_points(
                &Point {
                    x: self.location.x - (range + 1),
                    y: self.location.y,
                },
                &Point {
                    x: self.location.x,
                    y: self.location.y - (range + 1),
                },
            ),
            // Upper right
            Line::from_points(
                &Point {
                    x: self.location.x,
                    y: self.location.y - (range + 1),
                },
                &Point {
                    x: self.location.x + (range + 1),
                    y: self.location.y,
                },
            ),
            // Lower right
            Line::from_points(
                &Point {
                    x: self.location.x + (range + 1),
                    y: self.location.y,
                },
                &Point {
                    x: self.location.x,
                    y: self.location.y + (range + 1),
                },
            ),
            // Lower left
            Line::from_points(
                &Point {
                    x: self.location.x,
                    y: self.location.y + (range + 1),
                },
                &Point {
                    x: self.location.x - (range + 1),
                    y: self.location.y,
                },
            ),
        ]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSensorError;

impl FromStr for Sensor {
    type Err = ParseSensorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_s = s.split(&['=', ',', ' ', ':'][..]);

        Ok(Self::new(
            split_s
                .nth(3)
                .ok_or(ParseSensorError)?
                .parse::<i32>()
                .map_err(|_| ParseSensorError)?,
            split_s
                .nth(2)
                .ok_or(ParseSensorError)?
                .parse::<i32>()
                .map_err(|_| ParseSensorError)?,
            split_s
                .nth(6)
                .ok_or(ParseSensorError)?
                .parse::<i32>()
                .map_err(|_| ParseSensorError)?,
            split_s
                .nth(2)
                .ok_or(ParseSensorError)?
                .parse::<i32>()
                .map_err(|_| ParseSensorError)?,
        ))
    }
}

fn merge_ranges(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    ranges.sort_by_key(|range| range.0);

    let mut merged_ranges = Vec::new();
    if ranges.is_empty() {
        return merged_ranges;
    }

    let mut current_range = ranges[0];
    for range in ranges.iter().skip(1) {
        if range.0 <= current_range.1 {
            current_range.1 = cmp::max(current_range.1, range.1);
        } else {
            merged_ranges.push(current_range);
            current_range = *range;
        }
    }
    merged_ranges.push(current_range);

    merged_ranges
}

fn task1_solver(input: &[String], y: i32) -> usize {
    let sensors = input
        .iter()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect::<Vec<Sensor>>();

    let ranges = merge_ranges(
        sensors
            .iter()
            .filter_map(|sensor| sensor.range_at_y(y))
            .collect::<Vec<_>>(),
    );

    let num_covered_points = ranges.iter().fold(0usize, |acc, range| {
        acc + usize::try_from(range.1 - range.0 + 1).unwrap()
    });

    let beacons_on_same_line = sensors
        .iter()
        .filter(|sensor| sensor.beacon.y == y)
        .map(|sensor| sensor.beacon.x)
        .unique()
        .count();

    num_covered_points - beacons_on_same_line
}

fn task2_solver(input: &[String], max_x: i32, max_y: i32) -> Option<u64> {
    let sensors = input
        .iter()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect::<Vec<Sensor>>();

    let mut lines: Vec<Line> = Vec::new();

    for sensor in sensors.iter() {
        lines.extend(sensor.get_lines());
    }

    for (i, line1) in lines.iter().enumerate() {
        for line2 in lines[(i + 1)..].iter() {
            if let Some(intersection) = line1.intersection(line2) {
                if 0 <= intersection.x
                    && intersection.x <= max_x
                    && 0 <= intersection.y
                    && intersection.y <= max_y
                    && !sensors
                        .iter()
                        .any(|sensor| sensor.covers(intersection.x, intersection.y))
                {
                    return Some(
                        u64::try_from(intersection.x).unwrap() * 4000000
                            + u64::try_from(intersection.y).unwrap(),
                    );
                }
            }
        }
    }
    None
}

pub fn task1(input: &[String]) -> usize {
    task1_solver(input, 2000000)
}

pub fn task2(input: &[String]) -> u64 {
    task2_solver(input, 4000000, 4000000).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{task1, task1_solver, task2, task2_solver};
    use crate::util::read_input_from_file;

    #[test]
    fn test_task1() {
        assert_eq!(
            task1_solver(&read_input_from_file("sample/day15.txt"), 10),
            26
        );
        assert_eq!(task1(&read_input_from_file("input/day15.txt")), 5525990);
    }

    #[test]
    fn test_task2() {
        assert_eq!(
            task2_solver(&read_input_from_file("sample/day15.txt"), 20, 20).unwrap(),
            56000011
        );
        assert_eq!(
            task2(&read_input_from_file("input/day15.txt")),
            11756174628223
        );
    }
}
