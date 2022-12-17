use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let valves = parse!(input);
    open_valves::<1>(valves, 30).map(|x| x.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let valves = parse!(input);
    open_valves::<2>(valves, 26).map(|x| x.to_string())
}

fn open_valves<const RUNNERS: usize>(
    valves: Vec<Valve>,
    time: usize,
) -> Result<usize, anyhow::Error> {
    let mut graph = Graph::new(valves);
    graph.make_complete();
    graph.trim();

    let valves = convert_valves(&graph.nodes.into_values().collect::<Vec<_>>());
    let start_index = valves
        .iter()
        .enumerate()
        .find(|(_, v)| v.name == ['A', 'A'])
        .ok_or(anyhow!("start not found"))?
        .0;

    let mut stack = vec![State {
        time_remaining: time,
        opening_last_paid: time + 1,
        value: 0,
        visited: BitSet::default().with_set(start_index),
        runners: [Runner::new(start_index, 0); RUNNERS],
    }];

    let mut max_pressure_released = 0;

    while let Some(mut state) = stack.pop() {
        if state.time_remaining == 0 {
            continue;
        }

        let cur_valve = &valves[state.runners[0].cur_valve];
        let open_valve = cur_valve.flow_rate > 0;
        if open_valve {
            state.value += cur_valve.flow_rate * (state.time_remaining - 1);
            max_pressure_released = max_pressure_released.max(state.value);
        }

        for &(valve_id, distance) in &cur_valve.tunnels {
            if !state.visited.get(valve_id) {
                let next_state = {
                    let mut s = state.clone();
                    s.visited = s.visited.with_set(valve_id);
                    s.runners[0] = Runner::new(valve_id, distance + if open_valve { 1 } else { 0 });
                    s.advance();
                    s
                };
                stack.push(next_state);
            }
        }

        // at each step, kill the current runner so the other can work unimpeded.
        assert!(RUNNERS == 1 || RUNNERS == 2); // the hack below only works for R=1 or R=2
        if RUNNERS == 2 {
            let next_state = {
                let mut s = state.clone();
                s.runners[0] = Runner::new(0, usize::MAX);
                s.advance();
                s
            };
            stack.push(next_state);
        }
    }

    Ok(max_pressure_released)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State<const RUNNERS: usize> {
    time_remaining: usize,
    opening_last_paid: usize,
    value: usize,
    visited: BitSet,
    runners: [Runner; RUNNERS],
}

impl<const RUNNERS: usize> State<RUNNERS> {
    fn advance(&mut self) {
        self.runners.sort_by_key(|r| r.wait);
        let time_to_skip = self.runners[0].wait;

        self.time_remaining = self.time_remaining.saturating_sub(time_to_skip);
        self.runners.iter_mut().for_each(|r| r.wait -= time_to_skip);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Runner {
    cur_valve: usize,
    wait: usize,
}

impl Runner {
    fn new(cur_valve: usize, wait: usize) -> Self {
        Self { cur_valve, wait }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct BitSet(u32);

impl BitSet {
    fn get(&self, bit: usize) -> bool {
        assert!(bit < 32);
        let flag = 1 << bit;
        self.0 & flag > 0
    }

    fn with_set(&self, bit: usize) -> Self {
        assert!(bit < 32);
        let flag = 1 << bit;
        BitSet(self.0 | flag)
    }
}

type ValveName = [char; 2];

#[derive(Clone, Debug, Default)]
struct Graph {
    nodes: HashMap<ValveName, Valve>,
}

impl Graph {
    fn new(valves: Vec<Valve>) -> Self {
        let nodes = valves.into_iter().map(|n| (n.name, n)).collect();
        Self { nodes }
    }

    // Makes the graph a complete graph.
    fn make_complete(&mut self) {
        let nodes: Vec<_> = self.nodes.keys().cloned().collect();
        for node in nodes {
            self.connect_node(node);
        }
    }

    // Connect a node to all other nodes
    fn connect_node(&mut self, name: ValveName) {
        let known_tunnels = self.nodes.get(&name).unwrap().tunnels.clone();
        let mut heap: BinaryHeap<_> = known_tunnels
            .iter()
            .map(|(&n, &d)| Reverse((d, n)))
            .collect();

        let mut tunnels = HashMap::new();
        while let Some(Reverse((cost, node))) = heap.pop() {
            if !tunnels.contains_key(&node) {
                tunnels.insert(node, cost);
                heap.extend(
                    self.nodes
                        .get(&node)
                        .unwrap()
                        .tunnels
                        .iter()
                        .map(|(&n, &d)| Reverse((cost + d, n))),
                );
            }
        }

        tunnels.remove(&name);

        self.nodes.get_mut(&name).unwrap().tunnels = tunnels;
    }

    // Remove all nodes which have zero flow except "AA".
    fn trim(&mut self) {
        let dead_nodes: Vec<_> = self
            .nodes
            .iter()
            .filter(|(name, valve)| valve.flow_rate == 0 && name != &&['A', 'A'])
            .map(|(name, _)| name)
            .cloned()
            .collect();

        for node in &dead_nodes {
            self.nodes.remove(node);
        }

        for valve in self.nodes.values_mut() {
            for node in &dead_nodes {
                valve.tunnels.remove(node);
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Valve {
    name: ValveName,
    flow_rate: usize,
    // distance to other valves
    tunnels: HashMap<ValveName, usize>,
}

#[derive(Clone, Debug, Default)]
struct Valve2 {
    name: ValveName,
    flow_rate: usize,
    // (valve, distance)
    tunnels: Vec<(usize, usize)>,
}

fn convert_valves(valves: &[Valve]) -> Vec<Valve2> {
    let valve_ids: HashMap<_, _> = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name, i))
        .collect();
    let mut new_valves = vec![Valve2::default(); valve_ids.len()];

    for v in valves {
        let new_valve = Valve2 {
            name: v.name,
            flow_rate: v.flow_rate,
            tunnels: v
                .tunnels
                .iter()
                .map(|(name, distance)| (*valve_ids.get(name).unwrap(), *distance))
                .collect(),
        };

        let id = *valve_ids.get(&new_valve.name).unwrap();
        new_valves[id] = new_valve;
    }

    new_valves
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Valve>> {
        ws_all_consuming(many1(ws_line(valve)))(input)
    }

    fn valve(input: &str) -> IResult<&str, Valve> {
        let name = || pair(anychar, anychar).map(|(a, b)| [a, b]);
        tuple((
            tag("Valve "),
            name(),
            tag(" has flow rate="),
            uint,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), name()),
        ))
        .map(|(_, name, _, flow_rate, _, tunnels)| Valve {
            name,
            flow_rate,
            tunnels: tunnels.into_iter().map(|name| (name, 1)).collect(),
        })
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "1651")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "1707")
    }
}
