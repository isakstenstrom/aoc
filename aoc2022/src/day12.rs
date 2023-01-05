use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn within(&self, lower_x: isize, lower_y: isize, upper_x: isize, upper_y: isize) -> bool {
        lower_x <= self.x && self.x < upper_x && lower_y <= self.y && self.y < upper_y
    }
}

struct HeightMap {
    heights: Vec<Vec<u32>>,
    start: Point,
    end: Point,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn new(input: &[String]) -> HeightMap {
        let mut map = HeightMap {
            heights: Vec::new(),
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 0 },
            width: 0,
            height: 0,
        };

        for (y, line) in input.iter().enumerate() {
            map.heights.push(Vec::new());
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        map.start = Point {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        };
                        map.heights.last_mut().unwrap().push(('a' as u32) - 97);
                    }
                    'E' => {
                        map.end = Point {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        };
                        map.heights.last_mut().unwrap().push(('z' as u32) - 97);
                    }
                    _ => map.heights.last_mut().unwrap().push((c as u32) - 97),
                }
            }
        }
        map.height = map.heights.len();
        map.width = map.heights[0].len();

        map
    }

    fn height_at(&self, p: &Point) -> u32 {
        self.heights[p.y as usize][p.x as usize]
    }

    fn find_path_length(&self, search_for_ground_level: bool) -> usize {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<(Point, usize)> = VecDeque::new();

        // Search from end to start to make part 2 easier
        visited.insert(self.end);
        queue.push_back((self.end, 0));

        while let Some((node, num_steps)) = queue.pop_front() {
            if (search_for_ground_level && self.height_at(&node) == 0)
                || (!search_for_ground_level && node == self.start)
            {
                return num_steps;
            }

            visited.insert(node);
            for adjacent_point in [
                Point { x: 0, y: -1 },
                Point { x: -1, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
            ] {
                let mut neighbour = node;
                neighbour.add(&adjacent_point);
                if neighbour.within(
                    0,
                    0,
                    self.width.try_into().unwrap(),
                    self.height.try_into().unwrap(),
                ) && !visited.contains(&neighbour)
                    && self.height_at(&neighbour) + 1 >= self.height_at(&node)
                {
                    visited.insert(neighbour);
                    queue.push_back((neighbour, num_steps + 1));
                }
            }
        }
        0
    }
}

pub fn task1(input: &[String]) -> usize {
    let map = HeightMap::new(input);
    map.find_path_length(false)
}

pub fn task2(input: &[String]) -> usize {
    let map = HeightMap::new(input);
    map.find_path_length(true)
}

#[cfg(test)]
mod tests {
    use super::{task1, task2};
    use crate::util::read_input_from_file;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&read_input_from_file("sample/day12.txt")), 31);
        assert_eq!(task1(&read_input_from_file("input/day12.txt")), 490);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&read_input_from_file("sample/day12.txt")), 29);
        assert_eq!(task2(&read_input_from_file("input/day12.txt")), 488);
    }
}
