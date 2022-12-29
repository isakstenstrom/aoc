use std::{
    cmp,
    thread::{self, JoinHandle},
};

use crate::util::read_input_as_lines;

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

    fn add(&mut self, other: &Resources) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }

    fn sub(&mut self, other: &Resources) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }

    fn affordable(&self, cost: &Resources) -> bool {
        self.ore >= cost.ore
            && self.clay >= cost.clay
            && self.obsidian >= cost.obsidian
            && self.geode >= cost.geode
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
        if res.affordable(&costs.ore) {
            new_production.ore += 1;
        }
        if res.affordable(&costs.clay) {
            new_production.clay += 1;
            res.sub(&costs.clay);
        }
        if res.affordable(&costs.obsidian) {
            new_production.obsidian += 1;
            res.sub(&costs.obsidian);
        }
        if res.affordable(&costs.geode) {
            new_production.geode += 1;
            res.sub(&costs.geode);
        }
        res.add(&old_production);
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

            while !stack.is_empty() {
                let state = stack.pop().unwrap();
                if state.time == 1 {
                    max_score = cmp::max(
                        max_score,
                        state.resources.geode + state.production.geode as usize,
                    );
                    continue;
                }

                if heuristic2(&state, &costs) <= max_score {
                    continue;
                }

                let mut new_state = state.clone();
                new_state.resources.add(&new_state.production);
                new_state.time -= 1;
                stack.push(new_state);

                if state.time >= 4 + costs.ore.ore
                    && state.production.ore < max_costs.ore
                    && state.resources.affordable(&costs.ore)
                {
                    let mut new_state = state.clone();
                    new_state.resources.sub(&costs.ore);
                    new_state.resources.add(&new_state.production);
                    new_state.production.ore += 1;
                    new_state.time -= 1;
                    stack.push(new_state);
                }
                if state.time >= 6
                    && state.production.clay < max_costs.clay
                    && state.resources.affordable(&costs.clay)
                {
                    let mut new_state = state.clone();
                    new_state.resources.sub(&costs.clay);
                    new_state.resources.add(&new_state.production);
                    new_state.production.clay += 1;
                    new_state.time -= 1;
                    stack.push(new_state);
                }
                if state.time >= 4
                    && state.production.obsidian < max_costs.obsidian
                    && state.resources.affordable(&costs.obsidian)
                {
                    let mut new_state = state.clone();
                    new_state.resources.sub(&costs.obsidian);
                    new_state.resources.add(&new_state.production);
                    new_state.production.obsidian += 1;
                    new_state.time -= 1;
                    stack.push(new_state);
                }
                if state.resources.affordable(&costs.geode) {
                    let mut new_state = state.clone();
                    new_state.resources.sub(&costs.geode);
                    new_state.resources.add(&new_state.production);
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

fn parse_blueprints(filename: &str) -> Vec<BlueprintCosts> {
    let lines = read_input_as_lines(filename);
    let mut result: Vec<BlueprintCosts> = Vec::new();

    for line in lines.iter() {
        let mut split_line = line.split(' ');

        let mut costs = BlueprintCosts::new();
        costs.ore.ore = str::parse::<usize>(split_line.nth(6).unwrap()).unwrap();
        costs.clay.ore = str::parse::<usize>(split_line.nth(5).unwrap()).unwrap();
        costs.obsidian.ore = str::parse::<usize>(split_line.nth(5).unwrap()).unwrap();
        costs.obsidian.clay = str::parse::<usize>(split_line.nth(2).unwrap()).unwrap();
        costs.geode.ore = str::parse::<usize>(split_line.nth(5).unwrap()).unwrap();
        costs.geode.obsidian = str::parse::<usize>(split_line.nth(2).unwrap()).unwrap();

        result.push(costs);
    }
    result
}

fn solve(filename: &str, num_blueprints: isize, num_minutes: usize) -> Vec<usize> {
    let blueprints = parse_blueprints(filename);
    if num_blueprints == -1 {
        simulate_robot_production(&blueprints, num_minutes)
    } else {
        simulate_robot_production(&blueprints[..(num_blueprints as usize)], num_minutes)
    }
}

pub fn task1() -> usize {
    let scores = solve("day19.txt", -1, 24);

    scores
        .into_iter()
        .enumerate()
        .map(|(i, score)| (i + 1) * score)
        .sum()
}

pub fn task2() -> usize {
    let scores = solve("day19.txt", 3, 32);
    scores.into_iter().reduce(|res, score| res * score).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day19::{heuristic, task1, task2, Resources, SearchState};

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
    fn test_task1() {
        assert_eq!(task1(), 1365);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(), 4864);
    }
}
