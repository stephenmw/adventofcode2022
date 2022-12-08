use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = input.lines().map(line_to_row).collect::<Vec<_>>();
    let mut grid = Grid::new(data);

    for i in 0..grid.cells.len() {
        let left = Point::new(0, i);
        let right = Point::new(grid.cells[i].len() - 1, i);

        println!("lr: {:?}, {:?}", left, right);

        mark_visible(&mut grid, left, Direction::Right);
        mark_visible(&mut grid, right, Direction::Left);
    }

    for i in 0..grid.cells[0].len() {
        let bottom = Point::new(i, 0);
        let top = Point::new(i, grid.cells.len() - 1);

        println!("bt: {:?}, {:?}", bottom, top);

        mark_visible(&mut grid, bottom, Direction::Up);
        mark_visible(&mut grid, top, Direction::Down);
    }

    let ans = grid
        .cells
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| x.visible)
        .count();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let data = input.lines().map(line_to_row).collect::<Vec<_>>();
    let grid = Grid::new(data);

    let ans = grid.iter().map(|p| scenic_score(&grid, p)).max().unwrap();
    Ok(ans.to_string())
}

fn scenic_score(grid: &Grid<GridValue>, p: Point) -> usize {
    let height = grid.get(p).unwrap().height;
    Direction::iter()
        .map(|d| count_available(grid, p, height, d))
        .product()
}

fn count_available(grid: &Grid<GridValue>, start: Point, h: u32, d: Direction) -> usize {
    let mut cur = start;
    let mut ret = 0;

    while let Some(p) = cur.next(d) {
        cur = p;

        let cell = match grid.get(cur) {
            Some(x) => x,
            None => break,
        };

        ret += 1;
        if cell.height >= h {
            break;
        }
    }

    ret
}

fn mark_visible(grid: &mut Grid<GridValue>, start: Point, d: Direction) {
    let start_cell = grid.get_mut(start).unwrap();
    let mut max_height = start_cell.height;
    start_cell.visible = true;

    let mut cur = start;
    while let Some(p) = cur.next(d) {
        cur = p;
        let cell = match grid.get_mut(cur) {
            Some(x) => x,
            None => break,
        };
        if cell.height > max_height {
            cell.visible = true;
            max_height = cell.height;
        }
    }
}

fn line_to_row(l: &str) -> Vec<GridValue> {
    l.chars()
        .filter_map(|x| {
            Some(GridValue {
                height: x.to_digit(10)?,
                visible: false,
            })
        })
        .collect()
}

struct GridValue {
    height: u32,
    visible: bool,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, ()> {
        unimplemented!()
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
