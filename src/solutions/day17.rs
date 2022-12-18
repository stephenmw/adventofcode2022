use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::grid::{Direction, Grid, Point};
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

fn find_periodic(directions: &[Direction]) -> (usize, usize) {
    let mut dropper = Dropper::new(directions);

    let mut seen = HashMap::new();

    for i in 0.. {
        dropper.drop_rock();
        let mut d = dropper.clone();
        d.canonicalize();
        match seen.entry(d) {
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

fn head(grid: &Grid<bool>) -> usize {
    let mut seen = vec![false; 7];
    let mut last_row = 0;
    for (i, row) in grid.cells.iter().enumerate().rev() {
        for (i, b) in row.iter().enumerate() {
            if *b {
                seen[i] = true;
            }
        }

        if seen.iter().all(|x| *x) {
            last_row = i;
            break;
        }
    }

    last_row
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Dropper<'a> {
    directions: &'a [Direction],

    grid: Grid<bool>,
    direction_index: usize,
    shape: usize,
}

impl<'a> Dropper<'a> {
    fn new(directions: &'a [Direction]) -> Self {
        Dropper {
            directions,
            grid: Grid::new(Vec::new()),
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

    fn canonicalize(&mut self) {
        self.shape %= SHAPES.len();
        self.direction_index %= self.directions.len();

        let first_head_row = head(&self.grid);
        self.grid.cells.drain(..first_head_row);
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
                    .any(|p| self.grid.get(p).cloned().unwrap_or(false) || p.x > 6);
                if !overlaps {
                    cur = next_cur;
                }
            }
            self.direction_index += 1;

            let Some(next_cur) = cur.next(Direction::Down) else {break};
            let overlaps = shape
                .iter()
                .map(|&x| add_point(next_cur, x))
                .any(|p| self.grid.get(p).cloned().unwrap_or(false));
            if !overlaps {
                cur = next_cur;
            } else {
                break;
            }
        }

        shape
            .iter()
            .map(|&x| add_point(cur, x))
            .for_each(|p| *self.grid.get_mut_or_expand(p) = true);
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
