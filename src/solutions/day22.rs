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

    let ans = 1000 * (cur.y + 1) + 4 * (cur.x + 1) + direction_value(dir);
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    problem2_(input, false)
}

fn problem2_(input: &str, is_test: bool) -> Result<String, anyhow::Error> {
    let (map, instructions) = parse!(input);

    let mut walker = CubeWalker {
        sides: if is_test { SIDES_TEST } else { SIDES },
        cube_length: if is_test { 4 } else { 50 },
        direction: Direction::Right,
        side: 1,
        point: Point::new(0, 0),
    };

    let mut m = HashMap::new();
    m.insert(walker.as_point(), walker.direction);

    for inst in instructions {
        match inst {
            Instruction::Turn(t) => walker = walker.turn(t),
            Instruction::Step(n) => {
                for _ in 0..n {
                    let next = walker.step();
                    if map.get(&next.as_point()).unwrap() == Cell::Wall {
                        break;
                    }
                    walker = next;
                    m.insert(walker.as_point(), walker.direction);
                }
            }
        }
    }

    let cur = walker.as_point();
    let ans = 1000 * (cur.y + 1) + 4 * (cur.x + 1) + direction_value(walker.direction);
    Ok(ans.to_string())
}

fn direction_value(d: Direction) -> usize {
    match d {
        Direction::Right => 0,
        Direction::Up => 1,
        Direction::Left => 2,
        Direction::Down => 3,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
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
    cells: Vec<Vec<Option<Cell>>>,
}

impl Map {
    pub fn get(&self, p: &Point) -> Option<Cell> {
        self.cells.get(p.y)?.get(p.x).copied().flatten()
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
        self.get(p).is_some()
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

#[derive(Clone, Debug)]
struct CubeWalker {
    sides: [Side; 7],
    cube_length: usize,
    direction: Direction,
    side: u8,
    point: Point,
}

impl CubeWalker {
    fn step(&self) -> CubeWalker {
        let next = self
            .point
            .next(self.direction)
            .filter(|p| p.x < self.cube_length && p.y < self.cube_length);

        if let Some(next) = next {
            return CubeWalker {
                sides: self.sides,
                cube_length: self.cube_length,
                direction: self.direction,
                side: self.side,
                point: next,
            };
        }

        let other_coord = match self.direction {
            Direction::Up | Direction::Down => self.point.x,
            Direction::Left | Direction::Right => self.point.y,
        };

        let r = self.sides[self.side as usize].step(self.direction);
        let other_coord = if r.reverse {
            self.cube_length - other_coord - 1
        } else {
            other_coord
        };

        self.new_side(r.next_side, r.border, other_coord)
    }

    fn turn(&self, t: TurnDirection) -> Self {
        Self {
            sides: self.sides,
            cube_length: self.cube_length,
            direction: t.apply(self.direction),
            side: self.side,
            point: self.point,
        }
    }

    fn new_side(&self, side: u8, border: SquareBorder, other_coordinate: usize) -> Self {
        let point = match border {
            SquareBorder::Top => Point::new(other_coordinate, 0),
            SquareBorder::Bottom => Point::new(other_coordinate, self.cube_length - 1),
            SquareBorder::Left => Point::new(0, other_coordinate),
            SquareBorder::Right => Point::new(self.cube_length - 1, other_coordinate),
        };

        Self {
            sides: self.sides,
            cube_length: self.cube_length,
            direction: border.inward(),
            side,
            point,
        }
    }

    fn as_point(&self) -> Point {
        let index = self.sides[self.side as usize].index;
        Point {
            x: index.x * self.cube_length + self.point.x,
            y: index.y * self.cube_length + self.point.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SideRelation {
    next_side: u8,
    border: SquareBorder,
    reverse: bool,
}

#[derive(Clone, Copy, Debug)]
struct Side {
    index: Point,

    left: SideRelation,
    right: SideRelation,
    top: SideRelation,
    bottom: SideRelation,
}

impl Side {
    fn step(&self, d: Direction) -> &SideRelation {
        match d {
            Direction::Up => &self.bottom,
            Direction::Down => &self.top,
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

macro_rules! r {
    ($next:expr, $border:ident, $reverse:expr) => {
        SideRelation {
            next_side: $next,
            border: SquareBorder::$border,
            reverse: $reverse,
        }
    };
}

const SIDES: [Side; 7] = [
    // Dummy
    Side {
        index: Point { x: 0, y: 0 },
        left: r!(0, Top, false),
        top: r!(0, Top, false),
        right: r!(0, Top, false),
        bottom: r!(0, Top, false),
    },
    // Side 1
    Side {
        index: Point { x: 1, y: 0 },
        left: r!(5, Left, true),
        top: r!(6, Left, false),
        right: r!(2, Left, false),
        bottom: r!(3, Top, false),
    },
    // Side 2
    Side {
        index: Point { x: 2, y: 0 },
        left: r!(1, Right, false),
        top: r!(6, Bottom, false),
        right: r!(4, Right, true),
        bottom: r!(3, Right, false),
    },
    // Side 3
    Side {
        index: Point { x: 1, y: 1 },
        left: r!(5, Top, false),
        top: r!(1, Bottom, false),
        right: r!(2, Bottom, false),
        bottom: r!(4, Top, false),
    },
    // Side 4
    Side {
        index: Point { x: 1, y: 2 },
        left: r!(5, Right, false),
        top: r!(3, Bottom, false),
        right: r!(2, Right, true),
        bottom: r!(6, Right, false),
    },
    // Side 5
    Side {
        index: Point { x: 0, y: 2 },
        left: r!(1, Left, true),
        top: r!(3, Left, false),
        right: r!(4, Left, false),
        bottom: r!(6, Top, false),
    },
    // Side 6
    Side {
        index: Point { x: 0, y: 3 },
        left: r!(1, Top, false),
        top: r!(5, Bottom, false),
        right: r!(4, Bottom, false),
        bottom: r!(2, Top, false),
    },
];

const SIDES_TEST: [Side; 7] = [
    // Dummy
    Side {
        index: Point { x: 0, y: 0 },
        left: r!(0, Top, false),
        top: r!(0, Top, false),
        right: r!(0, Top, false),
        bottom: r!(0, Top, false),
    },
    // Side 1
    Side {
        index: Point { x: 2, y: 0 },
        left: r!(3, Top, false),
        top: r!(2, Top, true),
        right: r!(6, Right, true),
        bottom: r!(4, Top, false),
    },
    // Side 2
    Side {
        index: Point { x: 0, y: 1 },
        left: r!(6, Bottom, true),
        top: r!(1, Top, true),
        right: r!(3, Left, false),
        bottom: r!(5, Bottom, true),
    },
    // Side 3
    Side {
        index: Point { x: 1, y: 1 },
        left: r!(2, Right, false),
        top: r!(1, Left, false),
        right: r!(4, Left, false),
        bottom: r!(5, Left, true),
    },
    // Side 4
    Side {
        index: Point { x: 2, y: 1 },
        left: r!(3, Right, false),
        top: r!(1, Bottom, false),
        right: r!(6, Top, true),
        bottom: r!(5, Top, false),
    },
    // Side 5
    Side {
        index: Point { x: 2, y: 2 },
        left: r!(3, Bottom, true),
        top: r!(4, Bottom, false),
        right: r!(6, Left, false),
        bottom: r!(2, Bottom, true),
    },
    // Side 6
    Side {
        index: Point { x: 3, y: 2 },
        left: r!(5, Right, false),
        top: r!(4, Right, true),
        right: r!(1, Right, true),
        bottom: r!(2, Left, true),
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SquareBorder {
    Top,
    Bottom,
    Left,
    Right,
}

impl SquareBorder {
    fn inward(&self) -> Direction {
        match self {
            Self::Top => Direction::Up,
            Self::Bottom => Direction::Down,
            Self::Left => Direction::Right,
            Self::Right => Direction::Left,
        }
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, (Map, Vec<Instruction>)> {
        let cell = alt((
            value(None, char(' ')),
            value(Some(Cell::Walkable), char('.')),
            value(Some(Cell::Wall), char('#')),
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
        assert_eq!(problem2_(EXAMPLE_INPUT, true).unwrap(), "5031")
    }
}
