use std::cmp::max;

pub fn part1(input: &[String]) -> usize {
    let mut grid: Vec<Vec<(i32, bool)>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| (i32::try_from(c.to_digit(10).unwrap()).unwrap(), false))
                .collect::<Vec<(i32, bool)>>()
        })
        .collect();

    let mut visible_vertical: Vec<i32> = vec![-1; grid.len()];
    let mut visible_horizontal: Vec<i32> = vec![-1; grid.len()];
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, tree) in row.iter_mut().enumerate() {
            if visible_vertical[y] < tree.0 {
                visible_vertical[y] = tree.0;
                tree.1 = true;
            }
            if visible_horizontal[x] < tree.0 {
                visible_horizontal[x] = tree.0;
                tree.1 = true;
            }
        }
    }

    let mut visible_vertical: Vec<i32> = vec![-1; grid.len()];
    let mut visible_horizontal: Vec<i32> = vec![-1; grid.len()];
    for (y, row) in grid.iter_mut().enumerate().rev() {
        for (x, tree) in row.iter_mut().enumerate().rev() {
            if visible_vertical[y] < tree.0 {
                visible_vertical[y] = tree.0;
                tree.1 = true;
            }
            if visible_horizontal[x] < tree.0 {
                visible_horizontal[x] = tree.0;
                tree.1 = true;
            }
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|tree| tree.1).count())
        .sum()
}

fn get_scenic_score(grid: &mut Vec<Vec<(i32, bool)>>, x: usize, y: usize) -> u32 {
    let tree_height: i32 = grid[y][x].0;
    let mut score_up: u32 = 0;
    for i in (0..y).rev() {
        score_up += 1;
        if grid[i][x].0 >= tree_height {
            break;
        }
    }
    let mut score_down: u32 = 0;
    for i in (y + 1)..grid.len() {
        score_down += 1;
        if grid[i][x].0 >= tree_height {
            break;
        }
    }
    let mut score_left: u32 = 0;
    for i in (0..x).rev() {
        score_left += 1;
        if grid[y][i].0 >= tree_height {
            break;
        }
    }
    let mut score_right: u32 = 0;
    for i in (x + 1)..grid.len() {
        score_right += 1;
        if grid[y][i].0 >= tree_height {
            break;
        }
    }
    score_up * score_down * score_left * score_right
}

pub fn part2(input: &[String]) -> u32 {
    let mut grid: Vec<Vec<(i32, bool)>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| (i32::try_from(c.to_digit(10).unwrap()).unwrap(), false))
                .collect::<Vec<(i32, bool)>>()
        })
        .collect();

    let mut max_score = 0;
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            max_score = max(max_score, get_scenic_score(&mut grid, x, y));
        }
    }
    max_score
}

#[cfg(test)]
mod tests {
    use crate::{
        day8::{part1, part2},
        util::read_input_from_file,
    };

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day8.txt")), 21);
        assert_eq!(part1(&read_input_from_file("input/day8.txt")), 1672);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day8.txt")), 8);
        assert_eq!(part2(&read_input_from_file("input/day8.txt")), 327180);
    }
}
