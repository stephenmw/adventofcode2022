use std::collections::BinaryHeap;

use arrayvec::ArrayVec;
use rayon::prelude::*;

use crate::solutions::prelude::*;
use crate::utils::HeapElement;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let blueprints = parse!(input);
    let ans: usize = blueprints
        .par_iter()
        .map(|b| b.id * simulate_blueprint(b, 24))
        .sum();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let mut blueprints = parse!(input);
    blueprints.truncate(3);
    let ans: usize = blueprints
        .par_iter()
        .map(|b| simulate_blueprint(b, 32))
        .product();

    Ok(ans.to_string())
}

fn simulate_blueprint(blueprint: &Blueprint, time: usize) -> usize {
    let initial_state = State {
        time_remaining: time,
        resources: ResourceState::default(),
        robots: ResourceState {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    };

    let mut frontier = BinaryHeap::new();
    frontier.push(HeapElement::from((
        initial_state.high_mark(&blueprint),
        initial_state,
    )));

    while let Some(state) = frontier.pop().map(|x| x.value) {
        if state.time_remaining == 0 {
            return state.resources.geode;
        }

        let mut robot_costs = ArrayVec::from(blueprint.robot_costs.clone());
        robot_costs.retain(|c| {
            c.robot_type == Resource::Geode
                || state.robots.get(c.robot_type) < blueprint.max_needed.get(c.robot_type)
        });

        let mut resources = state.resources;

        for time_remaining in (0..state.time_remaining).rev() {
            let mut i = 0;
            while i < robot_costs.len() {
                if let Some(r) = resources.sub_resources(&robot_costs[i].resources) {
                    let costs = robot_costs.swap_remove(i);
                    let new_state = State {
                        time_remaining,
                        resources: r.add(&state.robots),
                        robots: state.robots.add_resource(costs.robot_type, 1),
                    };
                    frontier.push(HeapElement::from((
                        new_state.high_mark(&blueprint),
                        new_state,
                    )));
                } else {
                    i += 1;
                }
            }

            resources = resources.add(&state.robots);
        }

        let new_state = State {
            time_remaining: 0,
            robots: state.robots,
            resources,
        };
        frontier.push(HeapElement::from((
            new_state.high_mark(&blueprint),
            new_state,
        )));
    }

    unreachable!()
}

const NUM_ROBOT_TYPES: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blueprint {
    id: usize,
    robot_costs: [RobotCost; NUM_ROBOT_TYPES],
    max_needed: ResourceState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RobotCost {
    robot_type: Resource,
    resources: ArrayVec<(Resource, usize), 2>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct State {
    time_remaining: usize,
    resources: ResourceState,
    robots: ResourceState,
}

impl State {
    // A number guaranteed to be higher than geodes possible on this path.
    fn high_mark(&self, blueprint: &Blueprint) -> usize {
        let mut resources = [self.resources; NUM_ROBOT_TYPES];
        let mut robots = self.robots;
        for _ in 0..self.time_remaining {
            let additional_robots = blueprint
                .robot_costs
                .iter()
                .enumerate()
                .filter_map(|(i, cost)| {
                    let r = &mut resources[i];
                    if let Some(new_r) = r.sub_resources(&cost.resources) {
                        *r = new_r;
                        Some(cost.robot_type)
                    } else {
                        None
                    }
                })
                .fold(ResourceState::default(), |acc, resource| {
                    acc.add_resource(resource, 1)
                });

            resources.iter_mut().for_each(|x| *x = x.add(&robots));
            robots = robots.add(&additional_robots);
        }

        resources[0].geode
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ResourceState {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl ResourceState {
    fn get(&self, resource: Resource) -> usize {
        match resource {
            Resource::Ore => self.ore,
            Resource::Clay => self.clay,
            Resource::Obsidian => self.obsidian,
            Resource::Geode => self.geode,
        }
    }

    fn get_mut(&mut self, resource: Resource) -> &mut usize {
        match resource {
            Resource::Ore => &mut self.ore,
            Resource::Clay => &mut self.clay,
            Resource::Obsidian => &mut self.obsidian,
            Resource::Geode => &mut self.geode,
        }
    }

    fn sub_resources<'a>(
        &self,
        iter: impl IntoIterator<Item = &'a (Resource, usize)>,
    ) -> Option<Self> {
        let mut ret = *self;

        for &(resource, amount) in iter {
            let val = ret.get_mut(resource);
            *val = val.checked_sub(amount)?;
        }

        Some(ret)
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }

    fn add_resource(&self, resource: Resource, amount: usize) -> Self {
        let mut ret = *self;
        *ret.get_mut(resource) += amount;
        ret
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Blueprint>> {
        macro_rules! mtag {
            ($x:expr) => {
                delimited(multispace0, tag($x), multispace0)
            };
        }

        let blueprint = tuple((
            delimited(mtag!("Blueprint "), uint, mtag!(":")),
            delimited(mtag!("Each ore robot costs"), uint, mtag!("ore.")),
            delimited(mtag!("Each clay robot costs"), uint, mtag!("ore.")),
            delimited(
                mtag!("Each obsidian robot costs"),
                separated_pair(uint, mtag!("ore and"), uint),
                mtag!("clay."),
            ),
            delimited(
                mtag!("Each geode robot costs"),
                separated_pair(uint, mtag!("ore and"), uint),
                mtag!("obsidian."),
            ),
        ))
        .map(|(id, ore, clay, obsidian, geode)| {
            let robot_costs = [
                RobotCost {
                    robot_type: Resource::Ore,
                    resources: ArrayVec::from_iter([(Resource::Ore, ore)]),
                },
                RobotCost {
                    robot_type: Resource::Clay,
                    resources: ArrayVec::from_iter([(Resource::Ore, clay)]),
                },
                RobotCost {
                    robot_type: Resource::Obsidian,
                    resources: ArrayVec::from_iter([
                        (Resource::Ore, obsidian.0),
                        (Resource::Clay, obsidian.1),
                    ]),
                },
                RobotCost {
                    robot_type: Resource::Geode,
                    resources: ArrayVec::from_iter([
                        (Resource::Ore, geode.0),
                        (Resource::Obsidian, geode.1),
                    ]),
                },
            ];

            let max_needed = robot_costs.iter().flat_map(|r| &r.resources).fold(
                ResourceState::default(),
                |mut state, &(resource, amount)| {
                    let cur = state.get_mut(resource);
                    *cur = (*cur).max(amount);
                    state
                },
            );

            Blueprint {
                id,
                robot_costs,
                max_needed,
            }
        });

        ws_all_consuming(many1(blueprint))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        Blueprint 1:
            Each ore robot costs 4 ore.
            Each clay robot costs 2 ore.
            Each obsidian robot costs 3 ore and 14 clay.
            Each geode robot costs 2 ore and 7 obsidian.
        
        Blueprint 2:
            Each ore robot costs 2 ore.
            Each clay robot costs 3 ore.
            Each obsidian robot costs 3 ore and 8 clay.
            Each geode robot costs 3 ore and 12 obsidian.
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "33")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "3472")
    }
}
