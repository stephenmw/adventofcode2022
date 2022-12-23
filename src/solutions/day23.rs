use std::collections::{HashMap, HashSet, VecDeque};

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let mut elves = parse!(input);
    simulate(&mut elves, 10);
    Ok(empty_spaces(&elves).to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let mut elves = parse!(input);
    let ans = simulate(&mut elves, usize::MAX);
    Ok(ans.to_string())
}

fn empty_spaces(elves: &HashSet<Point>) -> usize {
    let (min_x, max_x, min_y, max_y) = elves.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(min_x, max_x, min_y, max_y), p| {
            (
                min_x.min(p.x),
                max_x.max(p.x),
                min_y.min(p.y),
                max_y.max(p.y),
            )
        },
    );

    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
    area as usize - elves.len()
}

fn simulate(elves: &mut HashSet<Point>, max_iterations: usize) -> usize {
    let mut directions = VecDeque::from(vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    // Proposal of Value -> Key. If value is None, Key is blocked.
    let mut proposals: HashMap<Point, Option<Point>> = HashMap::new();

    for i in 0..max_iterations {
        proposals.drain();
        for elf in elves.iter() {
            if elf.adjacent().all(|p| !elves.contains(&p)) {
                continue;
            }

            let Some(d) = directions
                .iter()
                .find(|&d| !elf.adjacent_direction(*d).any(|p| elves.contains(&p)))
                .copied() else {continue};

            proposals
                .entry(elf.step(d))
                .and_modify(|e| *e = None)
                .or_insert(Some(*elf));
        }

        if proposals.values().all(|x| x.is_none()) {
            return i + 1;
        }

        for (to, from) in proposals.iter() {
            if let Some(from) = from {
                elves.remove(from);
                elves.insert(*to);
            }
        }

        directions.rotate_left(1);
    }

    max_iterations
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn step(&self, d: Direction) -> Point {
        match d {
            Direction::South => Point::new(self.x, self.y + 1),
            Direction::North => Point::new(self.x, self.y - 1),
            Direction::West => Point::new(self.x - 1, self.y),
            Direction::East => Point::new(self.x + 1, self.y),
        }
    }

    fn adjacent(&self) -> impl Iterator<Item = Point> {
        let p = *self;
        (-1..=1).flat_map(move |x_offset| {
            (-1..=1)
                .map(move |y_offset| Point::new(p.x + x_offset, p.y + y_offset))
                .filter(move |x| x != &p)
        })
    }

    fn adjacent_direction(&self, d: Direction) -> impl Iterator<Item = Point> {
        let p = *self;
        (-1..=1).map(move |offset| match d {
            Direction::South => Point::new(p.x + offset, p.y + 1),
            Direction::North => Point::new(p.x + offset, p.y - 1),
            Direction::West => Point::new(p.x - 1, p.y + offset),
            Direction::East => Point::new(p.x + 1, p.y + offset),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, HashSet<Point>> {
        let cell = alt((value(false, char('.')), value(true, char('#'))));
        let row = ws_line(many1(cell));
        let graph = many1(row).map(|graph| {
            graph
                .into_iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    row.into_iter()
                        .enumerate()
                        .filter(|(_, is_elf)| *is_elf)
                        .map(move |(j, _)| Point::new(j as isize, i as isize))
                })
                .collect::<HashSet<_>>()
        });

        ws_all_consuming(graph)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        ..............
        ..............
        .......#......
        .....###.#....
        ...#...#.#....
        ....#...##....
        ...#.###......
        ...##.#.##....
        ....#..#......
        ..............
        ..............
        ..............
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "110")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "20")
    }
}
