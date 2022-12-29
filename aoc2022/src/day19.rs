use std::cmp;

use crate::util::read_input_as_lines;

#[derive(Debug, Clone)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
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

#[derive(Debug)]
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
    time: u32,
    resources: Resources,
    production: Resources,
}

fn heuristic(state: &SearchState) -> u32 {
    let current_production = state.production.geode;
    let ending_production = state.production.geode + (state.time - 1);
    state.resources.geode
        + (ending_production - current_production + 1) * (current_production + ending_production)
            / 2
}

pub fn task1() -> u32 {
    let lines = read_input_as_lines("day19.txt");

    let mut result: u32 = 0;
    for (i, line) in lines.iter().enumerate() {
        let mut split_line = line.split(' ');

        let mut costs = BlueprintCosts::new();

        costs.ore.ore = str::parse::<u32>(split_line.nth(6).unwrap()).unwrap();
        costs.clay.ore = str::parse::<u32>(split_line.nth(5).unwrap()).unwrap();
        costs.obsidian.ore = str::parse::<u32>(split_line.nth(5).unwrap()).unwrap();
        costs.obsidian.clay = str::parse::<u32>(split_line.nth(2).unwrap()).unwrap();
        costs.geode.ore = str::parse::<u32>(split_line.nth(5).unwrap()).unwrap();
        costs.geode.obsidian = str::parse::<u32>(split_line.nth(2).unwrap()).unwrap();

        let max_costs = Resources {
            ore: cmp::max(
                cmp::max(costs.ore.ore, costs.clay.ore),
                cmp::max(costs.obsidian.ore, costs.geode.ore),
            ),
            clay: costs.obsidian.clay,
            obsidian: costs.geode.obsidian,
            geode: 0,
        };

        let mut max_score: u32 = 0;
        let mut stack: Vec<SearchState> = Vec::new();
        stack.push(SearchState {
            time: 24,
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
                max_score = cmp::max(max_score, state.resources.geode + state.production.geode);
                continue;
            }

            if heuristic(&state) <= max_score {
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
        println!("Max score of iter {} is {}", i + 1, max_score);
        result += (u32::try_from(i).unwrap() + 1) * max_score;
    }
    result
}

pub fn task2() -> u32 {
    let lines = read_input_as_lines("day19.txt");

    1337
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

    // #[test]
    // fn test_task1() {
    //     assert_eq!(task1(), 1644735);
    // }

    // #[test]
    // fn test_task2() {
    //     assert_eq!(task2(), 1300850);
    // }
}
