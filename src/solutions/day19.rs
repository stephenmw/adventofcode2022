use anyhow::Ok;
use arrayvec::ArrayVec;

use rayon::prelude::*;

use crate::solutions::prelude::*;

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
        unaffordable_robots: blueprint.robot_costs.clone().into(),
    };

    let mut frontier = vec![initial_state];
    let mut max_geodes = 0;

    while let Some(mut state) = frontier.pop() {
        state.remove_never_affordable_robots();

        while state.time_remaining > 0 {
            state.time_remaining -= 1;

            let ready_robots = state.remove_affordable_robots();

            state.resources.add(&state.robots);

            for robot in ready_robots {
                let mut new_state = state.clone();
                new_state.buy(&robot).expect("robot should be affordable");
                new_state.unaffordable_robots = blueprint.robot_costs.clone().into();
                frontier.push(new_state);
            }

            // speed up time if no more unaffordable robots
            if state.unaffordable_robots.is_empty() {
                state.resources.geode += state.robots.geode * state.time_remaining;
                state.time_remaining = 0;
                break;
            }
        }

        max_geodes = max_geodes.max(state.resources.geode);
    }

    max_geodes
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blueprint {
    id: usize,
    robot_costs: [RobotCost; 4],
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct State {
    time_remaining: usize,
    resources: ResourceState,
    robots: ResourceState,
    unaffordable_robots: ArrayVec<RobotCost, 4>,
}

impl State {
    fn buy(&mut self, robot: &RobotCost) -> anyhow::Result<()> {
        self.resources = self
            .resources
            .sub_resources(&robot.resources)
            .ok_or(anyhow!("not enough minerals"))?;
        *self.robots.get_mut(robot.robot_type) += 1;
        Ok(())
    }

    fn remove_affordable_robots(&mut self) -> ArrayVec<RobotCost, 4> {
        let mut ret = ArrayVec::new();
        let mut i = 0;
        while i < self.unaffordable_robots.len() {
            if self
                .resources
                .is_affordable(&self.unaffordable_robots[i].resources)
            {
                ret.push(self.unaffordable_robots.swap_remove(i));
            } else {
                i += 1;
            }
        }

        ret
    }

    fn remove_never_affordable_robots(&mut self) {
        let mut i = 0;
        while i < self.unaffordable_robots.len() {
            let never_affordable = self.unaffordable_robots[i]
                .resources
                .iter()
                .any(|(r, _)| self.robots.get(*r) == 0);

            if never_affordable {
                self.unaffordable_robots.swap_remove(i);
            } else {
                i += 1;
            }
        }
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

    fn is_affordable<'a>(&self, iter: impl IntoIterator<Item = &'a (Resource, usize)>) -> bool {
        for &(resource, amount) in iter {
            if self.get(resource) < amount {
                return false;
            }
        }

        true
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

    fn add(&mut self, other: &Self) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
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
        .map(|(id, ore, clay, obsidian, geode)| Blueprint {
            id,
            robot_costs: [
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
            ],
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
