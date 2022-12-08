use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;

use std::collections::HashSet;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let mut visible = HashSet::new();

    for i in 0..grid.cells.len() {
        let left = Point::new(0, i);
        let right = Point::new(grid.cells[i].len() - 1, i);

        mark_visible(&mut visible, &grid, left, Direction::Right);
        mark_visible(&mut visible, &grid, right, Direction::Left);
    }

    for i in 0..grid.cells[0].len() {
        let bottom = Point::new(i, 0);
        let top = Point::new(i, grid.cells.len() - 1);

        mark_visible(&mut visible, &grid, bottom, Direction::Up);
        mark_visible(&mut visible, &grid, top, Direction::Down);
    }

    Ok(visible.len().to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);

    let ans = grid
        .iter_points()
        .map(|p| scenic_score(&grid, p))
        .max()
        .unwrap();

    Ok(ans.to_string())
}

fn scenic_score(grid: &Grid<u8>, p: Point) -> usize {
    let height = *grid.get(p).unwrap();
    Direction::iter()
        .map(|d| count_line(grid, p, height, d))
        .product()
}

fn count_line(grid: &Grid<u8>, start: Point, h: u8, d: Direction) -> usize {
    let Some(next) = start.next(d) else {return 0};

    grid.iter_line(next, d)
        .scan(false, |seen, (_, x)| {
            if !*seen {
                if *x >= h {
                    *seen = true;
                }
                Some(x)
            } else {
                None
            }
        })
        .count()
}

fn mark_visible(visible: &mut HashSet<Point>, grid: &Grid<u8>, start: Point, d: Direction) {
    let mut cells = grid.iter_line(start, d);

    let Some((first_point, &first_height)) = cells.next() else {return};
    let mut max_height = first_height;
    visible.insert(first_point);

    for (p, &h) in cells {
        if h > max_height {
            visible.insert(p);
            max_height = h;
        }
    }
}

mod parser {
    use crate::grid::Grid;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<u8>> {
        let digit = map(one_of::<_, _, nom::error::Error<_>>("0123456789"), |c| {
            c.to_digit(10).unwrap() as u8
        });
        let row = many1(digit);
        let grid = into(separated_list1(line_ending, row));
        complete(grid)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "21")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "8")
    }
}
