use crate::grid::Direction;
use crate::solutions::prelude::*;

use std::collections::HashSet;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let instructions = parse!(input);
    Ok(num_tail_locations(&instructions, 2).to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let instructions = parse!(input);
    Ok(num_tail_locations(&instructions, 10).to_string())
}

fn num_tail_locations(instructions: &[Instruction], rope_length: usize) -> usize {
    let mut state = State::new(rope_length);
    let mut tail_locations = HashSet::new();

    for inst in instructions {
        for _ in 0..inst.steps {
            state.step(inst.direction);
            tail_locations.insert(state.tail());
        }
    }

    tail_locations.len()
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x: x, y: y }
    }

    pub fn next(&self, d: Direction) -> Point {
        match d {
            Direction::Up => Point::new(self.x, self.y + 1),
            Direction::Down => Point::new(self.x, self.y - 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    knots: Vec<Point>,
}

impl State {
    fn new(n: usize) -> Self {
        State {
            knots: vec![Point::default(); n],
        }
    }

    fn step(&mut self, d: Direction) {
        fn step_next_knot(prev: Point, mut next: Point) -> Point {
            let move_x = prev.x.abs_diff(next.x) > 1;
            let move_y = prev.y.abs_diff(next.y) > 1;
            let move_diag = prev.x != next.x && prev.y != next.y && (move_x || move_y);

            if move_x || move_diag {
                if prev.x > next.x {
                    next.x += 1;
                } else {
                    next.x -= 1;
                }
            }

            if move_y || move_diag {
                if prev.y > next.y {
                    next.y += 1;
                } else {
                    next.y = next.y - 1;
                }
            }

            next
        }

        let Some(head) = self.knots.first_mut() else {return};
        *head = head.next(d);

        for i in 1..self.knots.len() {
            let prev = self.knots[i - 1];
            let next = self.knots[i];

            self.knots[i] = step_next_knot(prev, next);
        }
    }

    fn tail(&self) -> Point {
        self.knots[self.knots.len() - 1]
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
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "1")
    }
}
