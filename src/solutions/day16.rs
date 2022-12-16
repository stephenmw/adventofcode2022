use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let valves = parse!(input);

    let mut graph = Graph::new(valves);
    graph.make_complete();
    graph.trim();

    let valves: Vec<_> = graph.nodes.into_values().collect();
    let start_index = valves
        .iter()
        .enumerate()
        .find(|(_, v)| v.name == ['A', 'A'])
        .ok_or(anyhow!("start not found"))?
        .0;
    let mut stack = vec![State {
        time_remaining: 30,
        value: 0,
        visited: BitSet::default().with_set(start_index),
        cur_valve: start_index,
    }];

    let mut max_pressure_released = 0;

    while let Some(mut state) = stack.pop() {
        if state.time_remaining == 0 {
            continue;
        }

        let cur_valve = &valves[state.cur_valve];

        if cur_valve.flow_rate > 0 {
            state.time_remaining -= 1;
            state.value += cur_valve.flow_rate * state.time_remaining;
            max_pressure_released = max_pressure_released.max(state.value);
        }

        for i in 0..valves.len() {
            if !state.visited.get(i) {
                let next_name = valves[i].name;
                let cost = *cur_valve.tunnels.get(&next_name).unwrap();
                let Some(next_state) = state.next_valve(i, cost) else {continue};
                stack.push(next_state);
            }
        }
    }

    Ok(max_pressure_released.to_string())
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    todo!()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    time_remaining: usize,
    value: usize,
    visited: BitSet,
    cur_valve: usize,
}

impl State {
    fn next_valve(&self, valve: usize, cost: usize) -> Option<Self> {
        let mut ret = self.clone();
        ret.cur_valve = valve;
        ret.visited = ret.visited.with_set(valve);
        ret.time_remaining = ret.time_remaining.checked_sub(cost)?;
        Some(ret)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "1707")
    }
}
