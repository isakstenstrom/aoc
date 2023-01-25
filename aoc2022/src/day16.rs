use std::{
    cmp,
    collections::{BinaryHeap, HashMap},
};

const START_POINT: &str = "AA";

#[derive(Debug)]
struct Valve {
    index: usize,
    flowrate: usize,
    connections: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseValveError;

impl Valve {
    fn from_str(s: &str, valve_to_index: &HashMap<&str, usize>) -> Result<Self, ParseValveError> {
        let mut parts = s.split(&[' ', '=', ';', ','][..]);
        let name = parts.nth(1).ok_or(ParseValveError)?;
        let flowrate: usize = parts
            .nth(3)
            .ok_or(ParseValveError)?
            .parse()
            .map_err(|_| ParseValveError)?;

        parts.nth(4);

        Ok(Valve {
            index: *valve_to_index.get(name).unwrap(),
            flowrate,
            connections: parts
                .filter_map(|neighbor| {
                    if neighbor.is_empty() {
                        return None;
                    }
                    Some(*valve_to_index.get(neighbor).unwrap())
                })
                .collect(),
        })
    }
}

fn calculate_all_pairs(adj_matrix: &mut Vec<Vec<usize>>) {
    for via in 0..adj_matrix.len() {
        for from in 0..adj_matrix.len() {
            for to in 0..adj_matrix.len() {
                if adj_matrix[from][via] != usize::MAX && adj_matrix[via][to] != usize::MAX {
                    adj_matrix[from][to] = cmp::min(
                        adj_matrix[from][to],
                        adj_matrix[from][via] + adj_matrix[via][to],
                    );
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SearchState {
    pos1: usize,
    pos2: usize,
    flowrate: usize,
    heuristic: usize,
    time_left1: usize,
    time_left2: usize,
    opened_valves: u16,
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(other.heuristic.cmp(&self.heuristic))
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

impl SearchState {
    fn new(
        pos1: usize,
        pos2: usize,
        flowrate: usize,
        time_left1: usize,
        time_left2: usize,
        opened_valves: u16,
        valves: &[Valve],
    ) -> Self {
        let mut s = Self {
            pos1,
            pos2,
            flowrate,
            heuristic: 0,
            time_left1,
            time_left2,
            opened_valves,
        };
        s.heuristic = heuristic(valves, &s);
        s
    }
}

fn heuristic_helper(valves: &[Valve], search_state: &SearchState) -> usize {
    let mut best_flowrate = search_state.flowrate;
    let mut time_left1: i32 = i32::try_from(search_state.time_left1).unwrap() - 2;
    let mut time_left2: i32 = i32::try_from(search_state.time_left2).unwrap() - 2;

    for (i, valve) in valves.iter().enumerate() {
        if time_left1 <= 0 && time_left2 <= 0 {
            return best_flowrate;
        }
        if search_state.opened_valves & (1 << i) == 0 {
            best_flowrate +=
                valve.flowrate * usize::try_from(cmp::max(time_left1, time_left2)).unwrap();

            if time_left1 > time_left2 {
                time_left1 -= 2;
            } else {
                time_left2 -= 2;
            }
        }
    }

    best_flowrate
}

fn heuristic(valves: &[Valve], state: &SearchState) -> usize {
    let mut best_flowrate = state.flowrate;
    if state.time_left1 >= 2 && state.opened_valves & (1 << state.pos1) == 0 {
        best_flowrate = cmp::max(
            best_flowrate,
            heuristic_helper(
                valves,
                &SearchState {
                    flowrate: state.flowrate + valves[state.pos1].flowrate * (state.time_left1 - 1),
                    time_left1: state.time_left1 - 1,
                    opened_valves: state.opened_valves | 1 << state.pos1,
                    ..*state
                },
            ),
        );
    } else if state.time_left2 >= 2 && state.opened_valves & (1 << state.pos2) == 0 {
        best_flowrate = cmp::max(
            best_flowrate,
            heuristic_helper(
                valves,
                &SearchState {
                    flowrate: state.flowrate + valves[state.pos2].flowrate * (state.time_left2 - 1),
                    time_left2: state.time_left2 - 1,
                    opened_valves: state.opened_valves | 1 << state.pos2,
                    ..*state
                },
            ),
        );
    } else if state.time_left1 >= 2
        && state.opened_valves & (1 << state.pos1) == 0
        && state.time_left2 >= 2
        && state.opened_valves & (1 << state.pos2) == 0
    {
        best_flowrate = cmp::max(
            best_flowrate,
            heuristic_helper(
                valves,
                &SearchState {
                    flowrate: state.flowrate
                        + valves[state.pos1].flowrate * (state.time_left1 - 1)
                        + valves[state.pos2].flowrate * (state.time_left2 - 1),
                    time_left1: state.time_left1 - 1,
                    time_left2: state.time_left2 - 1,
                    opened_valves: state.opened_valves | 1 << state.pos1 | 1 << state.pos2,
                    ..*state
                },
            ),
        );
    }

    cmp::max(best_flowrate, heuristic_helper(valves, state))
}

fn get_valve_index_map(input: &[String]) -> HashMap<&str, usize> {
    let mut ordered_valves = input
        .iter()
        .map(|line| {
            let mut parts = line.split(&[' ', '=', ';', ','][..]);
            (
                parts.nth(1).unwrap(),
                parts.nth(3).unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    ordered_valves.sort_unstable_by(|a, b| {
        if a.1 == b.1 {
            if a.0 == START_POINT {
                return cmp::Ordering::Less;
            } else if b.0 == START_POINT {
                return cmp::Ordering::Greater;
            }
            return cmp::Ordering::Equal;
        }
        b.1.cmp(&a.1)
    });

    ordered_valves
        .into_iter()
        .enumerate()
        .map(|(i, (valve_identifier, _))| (valve_identifier, i))
        .collect()
}

fn parse_input(
    input: &[String],
    valve_to_index_map: &HashMap<&str, usize>,
) -> (Vec<Valve>, Vec<Vec<usize>>) {
    let mut valves: Vec<Valve> = input
        .iter()
        .map(|line| Valve::from_str(line, valve_to_index_map).unwrap())
        .collect();

    valves.sort_unstable_by(|a, b| a.index.cmp(&b.index));

    let mut big_adj_matrix: Vec<Vec<usize>> = vec![vec![usize::MAX; valves.len()]; valves.len()];

    for (i, valve) in valves.iter().enumerate() {
        big_adj_matrix[i][i] = 0;
        for neighbor in &valve.connections {
            big_adj_matrix[i][*neighbor] = 1;
        }
    }

    calculate_all_pairs(&mut big_adj_matrix);

    valves.retain(|valve| valve.flowrate != 0 || valve.index == valve_to_index_map[START_POINT]);
    let mut adj_matrix: Vec<Vec<usize>> = vec![vec![usize::MAX; valves.len()]; valves.len()];

    for y in 0..valves.len() {
        for x in 0..valves.len() {
            adj_matrix[y][x] = big_adj_matrix[y][x];
        }
    }

    (valves, adj_matrix)
}

fn search(
    valves: &[Valve],
    adj_matrix: &[Vec<usize>],
    time_for_user_1: usize,
    time_for_user_2: usize,
    start_index: usize,
) -> usize {
    let mut queue: BinaryHeap<SearchState> = BinaryHeap::new();
    queue.push(SearchState::new(
        start_index,
        start_index,
        0,
        time_for_user_1,
        time_for_user_2,
        0,
        valves,
    ));

    let mut best_result = 0usize;

    while let Some(state) = queue.pop() {
        best_result = cmp::max(best_result, state.flowrate);
        if best_result >= state.heuristic || (state.time_left1 <= 1 && state.time_left2 <= 1) {
            continue;
        }

        if state.time_left1 >= state.time_left2 {
            for (i, valve) in valves.iter().enumerate() {
                if valve.flowrate != 0 && state.opened_valves & (1 << i) == 0 {
                    let time_needed = adj_matrix[state.pos1][valve.index] + 1;

                    if time_needed <= state.time_left1 {
                        let new_state = SearchState::new(
                            valve.index,
                            state.pos2,
                            state.flowrate + (valve.flowrate * (state.time_left1 - time_needed)),
                            state.time_left1 - time_needed,
                            state.time_left2,
                            state.opened_valves | (1 << i),
                            valves,
                        );

                        if new_state.heuristic > best_result {
                            queue.push(new_state);
                        }
                    }
                }
            }
        } else {
            for (i, valve) in valves.iter().enumerate() {
                if valve.flowrate != 0 && state.opened_valves & (1 << i) == 0 {
                    let action_time = adj_matrix[state.pos2][valve.index] + 1;

                    if action_time <= state.time_left2 {
                        let new_state = SearchState::new(
                            state.pos1,
                            valve.index,
                            state.flowrate + (valve.flowrate * (state.time_left2 - action_time)),
                            state.time_left1,
                            state.time_left2 - action_time,
                            state.opened_valves | (1 << i),
                            valves,
                        );

                        if new_state.heuristic > best_result {
                            queue.push(new_state);
                        }
                    }
                }
            }
        }
    }
    best_result
}

fn solve(input: &[String], time_for_user_1: usize, time_for_user_2: usize) -> usize {
    let valve_to_index_map = get_valve_index_map(input);

    let (valves, adj_matrix) = parse_input(input, &valve_to_index_map);

    search(
        &valves,
        &adj_matrix,
        time_for_user_1,
        time_for_user_2,
        valve_to_index_map[START_POINT],
    )
}

pub fn task1(input: &[String]) -> usize {
    solve(input, 30, 0)
}

pub fn task2(input: &[String]) -> usize {
    solve(input, 26, 26)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&read_input_from_file("sample/day16.txt")), 1651);
        assert_eq!(task1(&read_input_from_file("input/day16.txt")), 1944);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&read_input_from_file("sample/day16.txt")), 1707);
        assert_eq!(task2(&read_input_from_file("input/day16.txt")), 2679);
    }
}
