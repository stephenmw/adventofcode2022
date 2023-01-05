use std::collections::hash_map::Entry;

use ahash::HashMap;

use crate::grid::{Direction, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let directions = parse!(input);

    let mut dropper = Dropper::new(&directions);

    Ok(dropper.iterate(2022).to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    const TARGET: usize = 1000000000000;
    let directions = parse!(input);

    let (start, end) = find_periodic(&directions);

    let mut dropper = Dropper::new(&directions);
    let preceding_height = dropper.iterate(end);
    let block_size = end - start;
    let block_height = dropper.iterate(block_size);
    let remainder = (TARGET - end) % block_size;
    let remainder_height = dropper.iterate(remainder);

    let ans = preceding_height + block_height * ((TARGET - end) / block_size) + remainder_height;

    Ok(ans.to_string())
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Row(u8);

impl Row {
    const ALL: Self = Self(!(1 << 7));

    fn get(&self, i: usize) -> bool {
        self.0 & (1 << i) != 0
    }

    fn set(&mut self, i: usize) {
        self.0 = self.0 | (1 << i)
    }

    fn set_from(&mut self, other: &Self) {
        self.0 |= other.0
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Grid {
    cells: Vec<Row>,
}

impl Grid {
    fn get(&self, p: &Point) -> bool {
        self.cells.get(p.y).map(|row| row.get(p.x)).unwrap_or(false)
    }

    fn set_or_expand(&mut self, p: &Point) {
        if p.y >= self.cells.len() {
            self.cells.resize_with(p.y + 1, Row::default);
        }

        self.cells[p.y].set(p.x);
    }
}

fn find_periodic(directions: &[Direction]) -> (usize, usize) {
    let mut dropper = Dropper::new(directions);

    let mut seen = HashMap::default();

    for i in 0.. {
        dropper.drop_rock();
        match seen.entry(dropper.state()) {
            Entry::Occupied(e) => {
                return (*e.get(), i);
            }
            Entry::Vacant(e) => {
                e.insert(i);
            }
        }
    }

    unreachable!()
}

fn head(grid: &[Row]) -> usize {
    let mut seen = Row::default();
    for (i, row) in grid.iter().enumerate().rev() {
        seen.set_from(row);
        if seen == Row::ALL {
            return i;
        }
    }

    0
}

#[derive(Clone, Debug)]
struct Dropper<'a> {
    directions: &'a [Direction],

    grid: Grid,
    direction_index: usize,
    shape: usize,
}

impl<'a> Dropper<'a> {
    fn new(directions: &'a [Direction]) -> Self {
        Dropper {
            directions,
            grid: Grid::default(),
            direction_index: 0,
            shape: 0,
        }
    }

    fn iterate(&mut self, n: usize) -> usize {
        let before = self.height();
        for _ in 0..n {
            self.drop_rock();
        }
        self.height() - before
    }

    fn height(&self) -> usize {
        self.grid.cells.len()
    }

    fn state(&self) -> (Vec<Row>, usize, usize) {
        let first_head_row = head(&self.grid.cells);
        (
            self.grid.cells[first_head_row..].to_vec(),
            self.shape % SHAPES.len(),
            self.direction_index % self.directions.len(),
        )
    }

    fn drop_rock(&mut self) {
        let shape = &SHAPES[self.shape % SHAPES.len()];
        self.shape += 1;

        let mut cur = Point::new(2, self.grid.cells.len() + 3);

        loop {
            if let Some(next_cur) =
                cur.next(self.directions[self.direction_index % self.directions.len()])
            {
                let overlaps = shape
                    .iter()
                    .map(|&x| add_point(next_cur, x))
                    .any(|p| self.grid.get(&p) || p.x > 6);
                if !overlaps {
                    cur = next_cur;
                }
            }
            self.direction_index += 1;

            let Some(next_cur) = cur.next(Direction::Down) else {break};
            let overlaps = shape
                .iter()
                .map(|&x| add_point(next_cur, x))
                .any(|p| self.grid.get(&p));
            if !overlaps {
                cur = next_cur;
            } else {
                break;
            }
        }

        shape
            .iter()
            .map(|&x| add_point(cur, x))
            .for_each(|p| self.grid.set_or_expand(&p));
    }
}

fn add_point(a: Point, b: Point) -> Point {
    Point {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Direction>> {
        let direction = alt((
            value(Direction::Left, char('<')),
            value(Direction::Right, char('>')),
        ));
        ws_all_consuming(ws_line(many1(direction)))(input)
    }
}

lazy_static! {
    pub static ref SHAPES: [Vec<Point>; 5] = [
        // horizontal line
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
        ],
        // cross
        vec![
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ],
        // backwards L
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(2, 1),
            Point::new(2, 2),
        ],
        // vertical line
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(0, 3),
        ],
        // square
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
        ],
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "3068")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "1514285714288")
    }
}
