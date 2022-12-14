use std::cmp::Ordering;

use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let lines = parse!(input);
    let max_x = lines.iter().flat_map(|l| [l.a.x, l.b.x]).max().unwrap();
    let max_y = lines.iter().flat_map(|l| [l.a.y, l.b.y]).max().unwrap();

    let mut grid = Grid::new(vec![vec![GridValue::Air; max_x + 1]; max_y + 1]);
    for line in lines {
        for point in line.points() {
            *grid.get_mut(point).unwrap() = GridValue::Rock;
        }
    }

    let mut count = 0;
    while drop_sand(&mut grid) {
        count += 1;
    }

    Ok(count.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let mut lines = parse!(input);
    let max_x = lines.iter().flat_map(|l| [l.a.x, l.b.x]).max().unwrap();
    let max_y = lines.iter().flat_map(|l| [l.a.y, l.b.y]).max().unwrap() + 2;

    lines.push(Line::new(
        Point { x: 0, y: max_y },
        Point {
            x: max_x + 500,
            y: max_y,
        },
    ));

    let mut grid = Grid::new(vec![vec![GridValue::Air; max_x + 501]; max_y + 1]);
    for line in lines {
        for point in line.points() {
            *grid.get_mut(point).unwrap() = GridValue::Rock;
        }
    }

    let mut count = 0;
    while *grid.get(Point::new(500, 0)).unwrap() == GridValue::Air {
        if !drop_sand(&mut grid) {
            bail!("grid not big enough");
        }
        count += 1;
    }

    Ok(count.to_string())
}

fn drop_sand(grid: &mut Grid<GridValue>) -> bool {
    let mut cur = Point::new(500, 0);

    'outer: loop {
        let next_options = [
            cur.next(Direction::Up),
            cur.next(Direction::Up)
                .and_then(|c| c.next(Direction::Left)),
            cur.next(Direction::Up)
                .and_then(|c| c.next(Direction::Right)),
        ];

        for next in next_options {
            let Some(next) = next else {return false};
            let Some(&v) = grid.get(next) else {return false};
            if v == GridValue::Air {
                cur = next;
                continue 'outer;
            }
        }

        *grid.get_mut(cur).unwrap() = GridValue::Sand;
        return true;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GridValue {
    Air,
    Sand,
    Rock,
}

pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        assert!(a.x == b.x || a.y == b.y);
        Line { a, b }
    }

    fn points(&self) -> Vec<Point> {
        if self.a.x == self.b.x {
            let (min_y, max_y) = match self.a.y.cmp(&self.b.y) {
                Ordering::Less => (self.a.y, self.b.y),
                Ordering::Equal => (self.a.y, self.b.y),
                Ordering::Greater => (self.b.y, self.a.y),
            };

            (min_y..=max_y)
                .map(move |y| Point::new(self.a.x, y))
                .collect()
        } else if self.a.y == self.b.y {
            let (min_x, max_x) = match self.a.x.cmp(&self.b.x) {
                Ordering::Less => (self.a.x, self.b.x),
                Ordering::Equal => (self.a.x, self.b.x),
                Ordering::Greater => (self.b.x, self.a.x),
            };

            (min_x..=max_x)
                .map(move |x| Point::new(x, self.a.y))
                .collect()
        } else {
            panic!("bad point")
        }
    }
}

mod parser {
    use super::*;
    use crate::grid::Point;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Line>> {
        let stream = separated_list1(tag(" -> "), point).map(|points| {
            points
                .windows(2)
                .map(|w| Line::new(w[0], w[1]))
                .collect::<Vec<_>>()
        });

        let parser =
            many1(ws_line(stream)).map(|lines| lines.into_iter().flatten().collect::<Vec<_>>());
        ws_all_consuming(parser)(input)
    }

    fn point(input: &str) -> IResult<&str, Point> {
        separated_pair(uint, char(','), uint)
            .map(|(x, y)| Point::new(x, y))
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "24")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "93")
    }
}
