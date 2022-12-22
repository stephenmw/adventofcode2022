use std::collections::HashMap;

use crate::grid::{Direction, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (map, instructions) = parse!(input);
    let mut cur = map.starting_location();
    let mut dir = Direction::Right;

    let mut m = HashMap::new();
    m.insert(cur, dir);

    for inst in instructions {
        match inst {
            Instruction::Turn(t) => dir = t.apply(dir),
            Instruction::Step(n) => {
                for _ in 0..n {
                    let next = map.step(&cur, dir);
                    if map.get(&next).unwrap() == Cell::Wall {
                        break;
                    }
                    cur = next;
                    m.insert(cur, dir);
                }
            }
        }
    }

    println!("{}", debug_map(&map, &m));

    let ans = 1000 * (cur.y + 1) + 4 * (cur.x + 1) + direction_value(dir);
    Ok(ans.to_string())
}

fn debug_map(map: &Map, points: &HashMap<Point, Direction>) -> String {
    let mut ret = String::new();

    for (y, row) in map.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let p = Point::new(x, y);
            ret.push(match points.get(&p) {
                Some(Direction::Right) => '>',
                Some(Direction::Down) => 'v',
                Some(Direction::Left) => '<',
                Some(Direction::Up) => '^',
                None => match cell {
                    Cell::Empty => ' ',
                    Cell::Walkable => '.',
                    Cell::Wall => '#',
                },
            });
        }
        ret.push('\n');
    }

    ret
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    todo!()
}

fn direction_value(d: Direction) -> usize {
    match d {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Walkable,
    Wall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    fn apply(&self, d: Direction) -> Direction {
        match self {
            Self::Left => match d {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            Self::Right => match d {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    Turn(TurnDirection),
    Step(usize),
}

pub struct Map {
    cells: Vec<Vec<Cell>>,
}

impl Map {
    pub fn get(&self, p: &Point) -> Option<Cell> {
        self.cells.get(p.y)?.get(p.x).copied()
    }

    pub fn get_mut(&mut self, p: &Point) -> Option<&mut Cell> {
        self.cells.get_mut(p.y)?.get_mut(p.x)
    }

    // Behavior is undefined if you step from an invalid starting location
    fn step(&self, p: &Point, d: Direction) -> Point {
        fn single_step(map: &Map, p: &Point, d: Direction) -> Point {
            let next = p.next(d);
            match d {
                Direction::Up => {
                    let next = next.unwrap();
                    if next.y >= map.cells.len() {
                        Point::new(next.x, 0)
                    } else {
                        next
                    }
                }
                Direction::Right => {
                    let next = next.unwrap();
                    if next.x >= map.cells[next.y].len() {
                        Point::new(0, next.y)
                    } else {
                        next
                    }
                }
                Direction::Down => {
                    if let Some(next) = next {
                        next
                    } else {
                        Point::new(p.x, map.cells.len() - 1)
                    }
                }
                Direction::Left => {
                    if let Some(next) = next {
                        next
                    } else {
                        Point::new(map.cells[p.y].len() - 1, p.y)
                    }
                }
            }
        }

        let mut next = single_step(self, p, d);
        while !self.is_valid_location(&next) {
            next = single_step(self, &next, d);
        }

        next
    }

    fn is_valid_location(&self, p: &Point) -> bool {
        self.get(p).unwrap_or(Cell::Empty) != Cell::Empty
    }

    fn starting_location(&self) -> Point {
        let p = Point::new(0, 0);
        if self.is_valid_location(&p) {
            p
        } else {
            self.step(&p, Direction::Right)
        }
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, (Map, Vec<Instruction>)> {
        let cell = alt((
            value(Cell::Empty, char(' ')),
            value(Cell::Walkable, char('.')),
            value(Cell::Wall, char('#')),
        ));
        let map_line = terminated(many1(cell), line_ending);
        let map = many1(map_line).map(|cells| Map { cells });

        let turn_direction = alt((
            value(TurnDirection::Left, char('L')),
            value(TurnDirection::Right, char('R')),
        ));
        let instruction = alt((
            uint.map(|x| Instruction::Step(x)),
            turn_direction.map(|x| Instruction::Turn(x)),
        ));
        let instructions = many1(instruction);

        let parser = separated_pair(map, line_ending, instructions);

        complete(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "6032")
    }

    #[test]
    fn problem2_test() {
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "5031")
    }
}
