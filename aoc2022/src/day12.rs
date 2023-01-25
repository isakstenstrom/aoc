use crate::util::Point;
use std::collections::{HashSet, VecDeque};

struct HeightMap {
    heights: Vec<Vec<u32>>,
    start: Point<i32>,
    end: Point<i32>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn new(input: &[String]) -> HeightMap {
        let mut map = HeightMap {
            heights: vec![Vec::new(); input.len()],
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 0 },
            width: input[0].len(),
            height: input.len(),
        };

        for (y, line) in input.iter().enumerate() {
            for (x, mut c) in line.chars().enumerate() {
                if c == 'S' {
                    map.start = Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    };
                    c = 'a';
                } else if c == 'E' {
                    map.end = Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    };
                    c = 'z';
                }
                map.heights[y].push((c as u32) - 97);
            }
        }
        map
    }

    fn height_at(&self, p: &Point<i32>) -> u32 {
        self.heights[p.y as usize][p.x as usize]
    }

    fn find_path_length<T>(&self, start: &Point<i32>, stop_condition: &T) -> Option<usize>
    where
        T: Fn(&Point<i32>) -> bool,
    {
        let mut visited: HashSet<Point<i32>> = HashSet::new();
        let mut queue: VecDeque<(Point<i32>, usize)> = VecDeque::new();

        visited.insert(*start);
        queue.push_back((*start, 0));

        while let Some((node, num_steps)) = queue.pop_front() {
            if stop_condition(&node) {
                return Some(num_steps);
            }

            visited.insert(node);
            for neighbor in node.get_neighbors() {
                if neighbor.is_within(
                    0,
                    0,
                    self.width.try_into().unwrap(),
                    self.height.try_into().unwrap(),
                ) && !visited.contains(&neighbor)
                    && self.height_at(&neighbor) + 1 >= self.height_at(&node)
                {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, num_steps + 1));
                }
            }
        }
        None
    }
}

pub fn task1(input: &[String]) -> usize {
    let map = HeightMap::new(input);
    map.find_path_length(&map.end, &|node| *node == map.start)
        .expect("No solution found!")
}

pub fn task2(input: &[String]) -> usize {
    let map = HeightMap::new(input);
    map.find_path_length(&map.end, &|node| map.height_at(node) == 0)
        .expect("No solution found!")
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
