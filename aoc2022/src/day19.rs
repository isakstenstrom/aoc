use std::{
    cmp,
    ops::{AddAssign, SubAssign},
    thread::{self, JoinHandle},
};

const DEBUG_PRINT: bool = false;

#[derive(Debug, Clone)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Resources {
    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn can_afford(&self, cost: &Resources) -> bool {
        self.ore >= cost.ore
            && self.clay >= cost.clay
            && self.obsidian >= cost.obsidian
            && self.geode >= cost.geode
    }
}

impl AddAssign<&Resources> for Resources {
    fn add_assign(&mut self, other: &Self) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }
}

impl SubAssign<&Resources> for Resources {
    fn sub_assign(&mut self, other: &Self) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }
}

#[derive(Debug, Clone)]
struct BlueprintCosts {
    ore: Resources,
    clay: Resources,
    obsidian: Resources,
    geode: Resources,
}

impl BlueprintCosts {
    fn new() -> BlueprintCosts {
        BlueprintCosts {
            ore: Resources::new(),
            clay: Resources::new(),
            obsidian: Resources::new(),
            geode: Resources::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct SearchState {
    time: usize,
    resources: Resources,
    production: Resources,
}

#[allow(dead_code)]
fn heuristic(state: &SearchState) -> usize {
    let current_production = state.production.geode;
    let ending_production = state.production.geode + (state.time - 1);
    state.resources.geode
        + (ending_production - current_production + 1) * (current_production + ending_production)
            / 2
}

fn heuristic2(state: &SearchState, blueprint_costs: &BlueprintCosts) -> usize {
    let mut res = state.resources.clone();
    let mut old_production = state.production.clone();
    let mut new_production = state.production.clone();
    let mut costs = blueprint_costs.clone();
    costs.obsidian.ore = 0;
    costs.geode.ore = 0;

    for _ in (1..=state.time).rev() {
        if res.can_afford(&costs.ore) {
            new_production.ore += 1;
        }
        if res.can_afford(&costs.clay) {
            new_production.clay += 1;
            res -= &costs.clay;
        }
        if res.can_afford(&costs.obsidian) {
            new_production.obsidian += 1;
            res -= &costs.obsidian;
        }
        if res.can_afford(&costs.geode) {
            new_production.geode += 1;
            res -= &costs.geode;
        }
        res += &old_production;
        old_production = new_production.clone();
    }
    res.geode
}

fn simulate_robot_production(blueprints: &[BlueprintCosts], num_minutes: usize) -> Vec<usize> {
    let mut threads: Vec<JoinHandle<usize>> = Vec::new();

    for (i, blueprint) in blueprints.iter().enumerate() {
        let costs = blueprint.clone();
        threads.push(thread::spawn(move || {
            let max_costs = Resources {
                ore: cmp::max(
                    cmp::max(costs.ore.ore, costs.clay.ore),
                    cmp::max(costs.obsidian.ore, costs.geode.ore),
                ),
                clay: costs.obsidian.clay,
                obsidian: costs.geode.obsidian,
                geode: 0,
            };

            let mut max_score: usize = 0;
            let mut stack: Vec<SearchState> = Vec::new();
            stack.push(SearchState {
                time: num_minutes,
                resources: Resources::new(),
                production: Resources {
                    ore: 1,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
            });

            while let Some(state) = stack.pop() {
                if state.time == 1 {
                    max_score = cmp::max(max_score, state.resources.geode + state.production.geode);
                    continue;
                }

                if heuristic2(&state, &costs) <= max_score {
                    continue;
                }

                let mut new_state = state.clone();
                new_state.resources += &new_state.production;
                new_state.time -= 1;
                stack.push(new_state);

                // TODO: Cleanup
                if state.time >= 4 + costs.ore.ore
                    && state.production.ore < max_costs.ore
                    && state.resources.can_afford(&costs.ore)
                {
                    let mut new_state = state.clone();
                    new_state.resources -= &costs.ore;
                    new_state.resources += &new_state.production;
                    new_state.production.ore += 1;
                    new_state.time -= 1;
                    stack.push(new_state);
                }
                if state.time >= 6
                    && state.production.clay < max_costs.clay
                    && state.resources.can_afford(&costs.clay)
                {
                    let mut new_state = state.clone();
                    new_state.resources -= &costs.clay;
                    new_state.resources += &new_state.production;
                    new_state.production.clay += 1;
                    new_state.time -= 1;
                    stack.push(new_state);
                }
                if state.time >= 4
                    && state.production.obsidian < max_costs.obsidian
                    && state.resources.can_afford(&costs.obsidian)
                {
                    let mut new_state = state.clone();
                    new_state.resources -= &costs.obsidian;
                    new_state.resources += &new_state.production;
                    new_state.production.obsidian += 1;
                    new_state.time -= 1;
                    stack.push(new_state);
                }
                if state.resources.can_afford(&costs.geode) {
                    let mut new_state = state.clone();
                    new_state.resources -= &costs.geode;
                    new_state.resources += &new_state.production;
                    new_state.production.geode += 1;
                    new_state.time -= 1;
                    stack.push(new_state);
                }
            }
            if DEBUG_PRINT {
                println!("Max score of iter {} is {}", i + 1, max_score);
            }

            max_score
        }));
    }

    let mut result: Vec<usize> = Vec::new();
    for thread in threads {
        result.push(thread.join().unwrap());
    }
    result
}

fn parse_blueprints(input: &[String]) -> Vec<BlueprintCosts> {
    input
        .iter()
        .map(|line| {
            let mut split_line = line.split(' ');

            let mut costs = BlueprintCosts::new();
            costs.ore.ore = str::parse::<usize>(split_line.nth(6).unwrap()).unwrap();
            costs.clay.ore = str::parse::<usize>(split_line.nth(5).unwrap()).unwrap();
            costs.obsidian.ore = str::parse::<usize>(split_line.nth(5).unwrap()).unwrap();
            costs.obsidian.clay = str::parse::<usize>(split_line.nth(2).unwrap()).unwrap();
            costs.geode.ore = str::parse::<usize>(split_line.nth(5).unwrap()).unwrap();
            costs.geode.obsidian = str::parse::<usize>(split_line.nth(2).unwrap()).unwrap();
            costs
        })
        .collect()
}

// TODO: Make this more clean
fn solve(input: &[String], num_blueprints: isize, num_minutes: usize) -> Vec<usize> {
    let blueprints = parse_blueprints(input);
    if num_blueprints == -1 || num_blueprints >= blueprints.len().try_into().unwrap() {
        simulate_robot_production(&blueprints, num_minutes)
    } else {
        simulate_robot_production(&blueprints[..(num_blueprints as usize)], num_minutes)
    }
}

pub fn part1(input: &[String]) -> usize {
    let scores = solve(input, -1, 24);

    scores
        .into_iter()
        .enumerate()
        .map(|(i, score)| (i + 1) * score)
        .sum()
}

pub fn part2(input: &[String]) -> usize {
    let scores = solve(input, 3, 32);
    scores.into_iter().reduce(|res, score| res * score).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_heuristic() {
        let state = SearchState {
            time: 3,
            resources: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 8,
            },
            production: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 3,
            },
        };
        assert_eq!(heuristic(&state), 8 + 3 + 4 + 5);

        let state = SearchState {
            time: 4,
            resources: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 8,
            },
            production: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 3,
            },
        };
        assert_eq!(heuristic(&state), 8 + 3 + 4 + 5 + 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day19.txt")), 33);
        assert_eq!(part1(&read_input_from_file("input/day19.txt")), 1365);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day19.txt")), 56 * 62);
        assert_eq!(part2(&read_input_from_file("input/day19.txt")), 4864);
    }
}
