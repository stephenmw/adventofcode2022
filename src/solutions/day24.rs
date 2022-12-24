use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Write;

use bitflags::bitflags;

use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let initial_state = parse!(input);
    let valley = BlizzardValley::new(initial_state);

    let ans = shortest_path(&valley, valley.start, valley.end, 0)?;

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let initial_state = parse!(input);
    let valley = BlizzardValley::new(initial_state);

    let trip1 = shortest_path(&valley, valley.start, valley.end, 0)?;
    let trip2 = shortest_path(&valley, valley.end, valley.start, trip1)?;
    let trip3 = shortest_path(&valley, valley.start, valley.end, trip2)?;

    Ok(trip3.to_string())
}

fn shortest_path(
    valley: &BlizzardValley,
    start: Point,
    end: Point,
    start_minute: usize,
) -> anyhow::Result<usize> {
    let mut frontier = BinaryHeap::new();
    frontier.push((
        Reverse(end.manhattan_distance(&start)),
        ExpeditionState {
            point: start,
            minute: start_minute,
        },
    ));

    let mut seen = HashSet::new();
    seen.insert((start, start_minute));

    while let Some((_, state)) = frontier.pop() {
        let next_points = state.point.iter_adjacent().chain([state.point]);
        let grid = valley.get(state.minute + 1);
        let minute = state.minute + 1;

        for p in next_points {
            if p == end {
                return Ok(minute);
            } else if grid.get(p).map(|c| c.is_empty()).unwrap_or(false) {
                if seen.insert((p, minute % valley.repeat_interval())) {
                    frontier.push((
                        Reverse(end.manhattan_distance(&p) + minute),
                        ExpeditionState { point: p, minute },
                    ));
                }
            }
        }
    }

    bail!("no solution")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ExpeditionState {
    point: Point,
    minute: usize,
}

pub struct BlizzardValley {
    blizzard_locations: Vec<Grid<Cell>>,
    start: Point,
    end: Point,
}

impl BlizzardValley {
    fn new(grid: Grid<Cell>) -> Self {
        let start_x = grid
            .cells
            .first()
            .unwrap()
            .iter()
            .position(|x| !x.contains(Cell::WALL))
            .unwrap();

        let end_x = grid
            .cells
            .last()
            .unwrap()
            .iter()
            .position(|x| !x.contains(Cell::WALL))
            .unwrap();

        let end_y = grid.cells.len() - 1;

        let x_len = grid.cells.first().unwrap().len() - 2;
        let y_len = grid.cells.len() - 2;

        let mut ret = BlizzardValley {
            blizzard_locations: vec![grid],
            start: Point::new(start_x, 0),
            end: Point::new(end_x, end_y),
        };

        ret.grow(x_len * y_len - 1);
        ret
    }

    fn get(&self, time: usize) -> &Grid<Cell> {
        let index = time % self.repeat_interval();
        &self.blizzard_locations[index]
    }

    fn repeat_interval(&self) -> usize {
        self.blizzard_locations.len()
    }

    fn grow(&mut self, target_time: usize) {
        let needed = target_time + 1 - self.blizzard_locations.len();
        for _ in 0..needed {
            let cur = self.blizzard_locations.last().unwrap();
            self.blizzard_locations.push(Self::step_blizzard(cur));
        }
    }

    fn step_blizzard(old: &Grid<Cell>) -> Grid<Cell> {
        let mut new = Grid::new(
            old.cells
                .iter()
                .map(|r| vec![Cell::empty(); r.len()])
                .collect(),
        );

        for p in old.iter_points() {
            let cur = old.get(p).unwrap();
            if cur.contains(Cell::WALL) {
                *new.get_mut(p).unwrap() = Cell::WALL;
            } else {
                for cell in cur.set_blizzards() {
                    let d = cell.as_direction();
                    let next = p
                        .next(d)
                        .filter(|&p| !old.get(p).unwrap().contains(Cell::WALL));
                    let new_p = match next {
                        Some(x) => x,
                        None => match d {
                            Direction::Left => Point::new(old.cells[p.y].len() - 2, p.y),
                            Direction::Right => Point::new(1, p.y),
                            Direction::Up => Point::new(p.x, 1),
                            Direction::Down => Point::new(p.x, old.cells.len() - 2),
                        },
                    };

                    new.get_mut(new_p).unwrap().set(cell, true);
                }
            }
        }

        new
    }
}

bitflags! {
    pub struct Cell: u8 {
        const WALL = 0b00000001;
        const LEFT = 0b00000010;
        const RIGHT = 0b00000100;
        const UP = 0b00001000;
        const DOWN = 0b00010000;
    }
}

impl Cell {
    fn set_blizzards(&self) -> impl Iterator<Item = Cell> {
        let cell = *self;
        [Cell::LEFT, Cell::RIGHT, Cell::UP, Cell::DOWN]
            .into_iter()
            .filter(move |&f| cell.contains(f))
    }

    fn as_direction(&self) -> Direction {
        match *self {
            Self::LEFT => Direction::Left,
            Self::RIGHT => Direction::Right,
            Self::UP => Direction::Down,
            Self::DOWN => Direction::Up,
            _ => panic!("cannot take as_direction of cell with multiple flags"),
        }
    }
}

impl std::fmt::Display for Grid<Cell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn render_row(f: &mut std::fmt::Formatter<'_>, row: &[Cell]) -> std::fmt::Result {
            for cell in row.iter() {
                let blizzards: Vec<_> = cell.set_blizzards().collect();
                if cell.contains(Cell::WALL) {
                    f.write_char('#')?;
                } else {
                    if blizzards.is_empty() {
                        f.write_char('.')?;
                    } else if blizzards.len() == 1 {
                        f.write_char(match blizzards[0] {
                            Cell::LEFT => '<',
                            Cell::RIGHT => '>',
                            Cell::UP => '^',
                            Cell::DOWN => 'v',
                            _ => unreachable!(),
                        })?;
                    } else {
                        let l = blizzards.len().to_string();
                        f.write_str(&l)?;
                    }
                };
            }

            Ok(())
        }

        let Some((first, rest)) = self.cells.split_first() else {
            return f.write_str("<EMPTY GRID>")
        };

        render_row(f, first)?;
        for row in rest {
            f.write_char('\n')?;
            render_row(f, row)?;
        }

        Ok(())
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<Cell>> {
        let cell = alt((
            value(Cell::empty(), char('.')),
            value(Cell::WALL, char('#')),
            value(Cell::LEFT, char('<')),
            value(Cell::RIGHT, char('>')),
            value(Cell::UP, char('^')),
            value(Cell::DOWN, char('v')),
        ));

        let row = many1(cell);
        let grid = many1(ws_line(row)).map(|x| Grid::new(x));

        ws_all_consuming(grid)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "18")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "54")
    }
}
