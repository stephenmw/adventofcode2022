use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let valves = parse!(input);
    let best_seen = open_valves(valves, 30)?;

    let ans = best_seen
        .iter()
        .map(|(_, released)| released)
        .max()
        .ok_or(anyhow!("no paths followed"))?;

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let valves = parse!(input);
    let best_seen = open_valves(valves, 26)?;

    let ans = best_seen
        .iter()
        .flat_map(|x| best_seen.iter().zip(std::iter::repeat(x)))
        .filter_map(|(a, b)| Some(a.1 + b.1).filter(|_| a.0.is_disjoint(&b.0)))
        .max()
        .ok_or(anyhow!("no disjoint paths"))?;

    Ok(ans.to_string())
}

const START_VALVE: ValveName = ['A', 'A'];

fn simplify_valves(valves: Vec<Valve>) -> Vec<Valve2> {
    let mut graph = Graph::new(valves);
    graph.make_complete();
    graph.trim();
    let trimmed_valves: Vec<_> = graph.nodes.into_values().collect();
    convert_valves(&trimmed_valves)
}

fn open_valves(valves: Vec<Valve>, time: usize) -> Result<Vec<(BitSet, usize)>, anyhow::Error> {
    let valves = simplify_valves(valves);

    let start_index = valves
        .iter()
        .enumerate()
        .find(|(_, v)| v.name == START_VALVE)
        .ok_or(anyhow!("start not found"))?
        .0;

    let initial_state = State {
        time_remaining: time,
        pressure_released: 0,
        opened: BitSet::default().with_set(start_index),
        cur_valve: start_index,
    };

    let mut stack = vec![initial_state];
    let mut best = HashMap::<BitSet, usize>::new();

    while let Some(mut state) = stack.pop() {
        if state.time_remaining == 0 {
            continue;
        }

        let cur_valve = &valves[state.cur_valve];

        if cur_valve.flow_rate > 0 {
            state.time_remaining -= 1;
            state.pressure_released += cur_valve.flow_rate * state.time_remaining;
            state.opened = state.opened.with_set(state.cur_valve);
        }
        let max_seen = best.entry(state.opened).or_insert(0);
        *max_seen = (*max_seen).max(state.pressure_released);

        let next_states = cur_valve.tunnels.iter().filter_map(|&(v, d)| {
            if !state.opened.contains(v) {
                let mut new_state = state.clone();
                new_state.time_remaining = state.time_remaining.checked_sub(d)?;
                new_state.cur_valve = v;
                Some(new_state)
            } else {
                None
            }
        });

        stack.extend(next_states);
    }

    // The starting valve was never opened so we need to unset it.
    let best_seen: Vec<_> = best
        .into_iter()
        .map(|(open, released)| (open.with_unset(start_index), released))
        .collect();

    Ok(best_seen)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    time_remaining: usize,
    pressure_released: usize,
    opened: BitSet,
    cur_valve: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct BitSet(u32);

impl BitSet {
    fn contains(&self, bit: usize) -> bool {
        assert!(bit < 32);
        let flag = 1 << bit;
        self.0 & flag > 0
    }

    fn with_set(&self, bit: usize) -> Self {
        assert!(bit < 32);
        let flag = 1 << bit;
        BitSet(self.0 | flag)
    }

    fn with_unset(&self, bit: usize) -> Self {
        assert!(bit < 32);
        let flag = 1 << bit;
        BitSet(self.0 & !flag)
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        self.0 & other.0 == 0
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
            .filter(|(name, valve)| valve.flow_rate == 0 && **name != START_VALVE)
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
