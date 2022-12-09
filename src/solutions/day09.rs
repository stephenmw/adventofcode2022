use crate::grid::Direction;
use crate::solutions::prelude::*;

use std::collections::HashSet;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let instructions = parse!(input);

    let mut state = State::default();
    let mut tail_locations = HashSet::new();

    for inst in instructions {
        for _ in 0..inst.steps {
            state = state
                .step(inst.direction)
                .ok_or(anyhow!("steped off postive grid"))?;
            println!("{:?}", state);
            tail_locations.insert(state.tail);
        }
    }

    Ok(tail_locations.len().to_string())
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    todo!()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x: x, y: y }
    }

    pub fn next(&self, d: Direction) -> Option<Point> {
        let p = match d {
            Direction::Up => Point::new(self.x, self.y.checked_add(1)?),
            Direction::Down => Point::new(self.x, self.y.checked_sub(1)?),
            Direction::Left => Point::new(self.x.checked_sub(1)?, self.y),
            Direction::Right => Point::new(self.x.checked_add(1)?, self.y),
        };

        Some(p)
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    head: Point,
    tail: Point,
}

impl State {
    fn step(&self, d: Direction) -> Option<State> {
        let head = self.head.next(d)?;
        let mut tail = self.tail;

        let move_x = head.x.abs_diff(tail.x) > 1;
        let move_y = head.y.abs_diff(tail.y) > 1;
        let move_diag = head.x != tail.x && head.y != tail.y && (move_x || move_y);

        if move_x || move_diag {
            if head.x > tail.x {
                tail.x += 1;
            } else {
                tail.x = tail.x.checked_sub(1)?;
            }
        }

        if move_y || move_diag {
            if head.y > tail.y {
                tail.y += 1;
            } else {
                tail.y = tail.y.checked_sub(1)?;
            }
        }

        Some(State { head, tail })
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            head: Point::new(0, 0),
            tail: Point::new(0, 0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    direction: Direction,
    steps: usize,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
        let direction = alt((
            value(Direction::Up, char('U')),
            value(Direction::Down, char('D')),
            value(Direction::Left, char('L')),
            value(Direction::Right, char('R')),
        ));

        let instruction = separated_pair(direction, space0, uint)
            .map(|(direction, steps)| Instruction { direction, steps });
        let instructions = many1(ws_line(instruction));
        ws_all_consuming(instructions)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "13")
    }

    #[test]
    fn problem2_test() {
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "")
    }
}
