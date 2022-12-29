use std::{
    collections::VecDeque,
    fmt::{Display, Write},
};

use ahash::HashMap;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let elves = parse!(input);

    let (_, mut grid) = simulate(&elves, 10);
    grid.trim();
    let ans = grid.area() - elves.len();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let mut elves = parse!(input);
    let (ans, _) = simulate(&mut elves, usize::MAX);
    Ok(ans.to_string())
}

fn simulate(elves: &[Point], max_iterations: usize) -> (usize, ExpandableGrid<bool>) {
    let mut grid = ExpandableGrid::default();
    for p in elves {
        *grid.get_mut_or_expand(p) = true;
    }
    grid.trim();

    let mut directions = VecDeque::from(vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    // Proposal of Value -> Key. If value is None, Key is blocked.
    let mut proposals: HashMap<Point, Option<Point>> = HashMap::default();

    for i in 0..max_iterations {
        proposals.drain();
        let elves = grid
            .iter()
            .filter_map(|(p, v)| if *v { Some(p) } else { None });
        for elf in elves {
            if elf.adjacent().all(|p| !grid.get(&p).map_or(false, |&x| x)) {
                continue;
            }

            let Some(d) = directions
                .iter()
                .find(|&d| !elf.adjacent_direction(*d).any(|p| grid.get(&p).map_or(false, |&x| x)))
                .copied() else {continue};

            proposals
                .entry(elf.step(d))
                .and_modify(|e| *e = None)
                .or_insert(Some(elf));
        }

        if proposals.values().all(|x| x.is_none()) {
            return (i + 1, grid);
        }

        for (to, from) in proposals.iter() {
            if let Some(from) = from {
                *grid.get_mut(from).unwrap() = false;
                *grid.get_mut_or_expand(to) = true;
            }
        }

        directions.rotate_left(1);
    }

    (max_iterations, grid)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn step(&self, d: Direction) -> Point {
        match d {
            Direction::South => Point::new(self.x, self.y + 1),
            Direction::North => Point::new(self.x, self.y - 1),
            Direction::West => Point::new(self.x - 1, self.y),
            Direction::East => Point::new(self.x + 1, self.y),
        }
    }

    fn adjacent(&self) -> impl Iterator<Item = Point> {
        let p = *self;
        (-1..=1).flat_map(move |x_offset| {
            (-1..=1)
                .map(move |y_offset| Point::new(p.x + x_offset, p.y + y_offset))
                .filter(move |x| x != &p)
        })
    }

    fn adjacent_direction(&self, d: Direction) -> impl Iterator<Item = Point> {
        let p = *self;
        (-1..=1).map(move |offset| match d {
            Direction::South => Point::new(p.x + offset, p.y + 1),
            Direction::North => Point::new(p.x + offset, p.y - 1),
            Direction::West => Point::new(p.x - 1, p.y + offset),
            Direction::East => Point::new(p.x + 1, p.y + offset),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Clone, Debug, Default)]
struct ExpandableGrid<T> {
    cells: VecDeque<VecDeque<T>>,
    x_offset: isize,
    y_offset: isize,
}

impl<T: Default + Clone + PartialEq> ExpandableGrid<T> {
    pub fn get(&self, p: &Point) -> Option<&T> {
        let x: usize = (p.x + self.x_offset as isize).try_into().ok()?;
        let y: usize = (p.y + self.y_offset as isize).try_into().ok()?;

        self.cells.get(y)?.get(x)
    }

    pub fn get_mut(&mut self, p: &Point) -> Option<&mut T> {
        let x: usize = (p.x + self.x_offset).try_into().ok()?;
        let y: usize = (p.y + self.y_offset).try_into().ok()?;

        self.cells.get_mut(y)?.get_mut(x)
    }

    pub fn get_mut_or_expand(&mut self, p: &Point) -> &mut T {
        let y = {
            let translated_y = p.y + self.y_offset;
            if translated_y >= 0 {
                translated_y as usize
            } else {
                let row_length = self.row_length();
                for _ in 0..translated_y.abs() {
                    self.cells
                        .push_front(new_vecdeque_with(row_length, T::default));
                }

                self.y_offset += translated_y.abs();

                0
            }
        };

        let x = {
            let translated_x = p.x + self.x_offset;
            if translated_x >= 0 {
                translated_x as usize
            } else {
                for row in self.cells.iter_mut() {
                    for _ in 0..translated_x.abs() {
                        row.push_front(T::default());
                    }
                }

                self.x_offset += translated_x.abs();

                0
            }
        };

        let row_length = self.row_length();

        if y >= self.cells.len() {
            self.cells
                .resize_with(y + 1, || VecDeque::from(vec![T::default(); row_length]));
        }

        if x >= row_length {
            for row in self.cells.iter_mut() {
                row.resize_with(x + 1, T::default);
            }
        }

        self.get_mut(p).unwrap()
    }

    fn row_length(&self) -> usize {
        self.cells.front().map(|row| row.len()).unwrap_or(0)
    }

    pub fn trim(&mut self) {
        let default = T::default();

        while self
            .cells
            .front()
            .map(|row| row.iter().all(|cell| cell == &default))
            .unwrap_or(false)
        {
            self.cells.pop_front();
            self.y_offset -= 1;
        }

        while self
            .cells
            .back()
            .map(|row| row.iter().all(|cell| cell == &default))
            .unwrap_or(false)
        {
            self.cells.pop_back();
        }

        let is_empty_column = |cells: &VecDeque<VecDeque<T>>, column| {
            cells
                .iter()
                .map(move |row| &row[column])
                .all(|cell| cell == &default)
        };

        while is_empty_column(&self.cells, 0) {
            self.cells.iter_mut().for_each(|x| {
                x.pop_front();
            });
            self.x_offset -= 1;
        }

        while is_empty_column(&self.cells, self.row_length() - 1) {
            self.cells.iter_mut().for_each(|x| {
                x.pop_back();
            });
        }
    }

    pub fn area(&self) -> usize {
        self.cells.len() * self.row_length()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> {
        self.cells.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().map(move |(x, cell)| {
                (
                    Point::new(
                        x as isize + -1 * self.x_offset,
                        y as isize + -1 * self.y_offset,
                    ),
                    cell,
                )
            })
        })
    }
}

impl Display for ExpandableGrid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = self.cells.iter();
        let Some(first) = rows.next() else {
            return write!(f, "<Empty Grid>");
        };

        fn write_row(f: &mut std::fmt::Formatter<'_>, row: &VecDeque<bool>) -> std::fmt::Result {
            for cell in row {
                match cell {
                    true => f.write_char('#')?,
                    false => f.write_char('.')?,
                };
            }

            Ok(())
        }

        write_row(f, first)?;

        for row in rows {
            f.write_char('\n')?;
            write_row(f, row)?;
        }

        Ok(())
    }
}

fn new_vecdeque_with<T>(len: usize, generator: impl FnMut() -> T) -> VecDeque<T> {
    let mut ret = VecDeque::with_capacity(len);
    ret.resize_with(len, generator);
    ret
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Point>> {
        let cell = alt((value(false, char('.')), value(true, char('#'))));
        let row = ws_line(many1(cell));
        let graph = many1(row).map(|graph| {
            graph
                .into_iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    row.into_iter()
                        .enumerate()
                        .filter(|(_, is_elf)| *is_elf)
                        .map(move |(j, _)| Point::new(j as isize, i as isize))
                })
                .collect::<Vec<_>>()
        });

        ws_all_consuming(graph)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        ..............
        ..............
        .......#......
        .....###.#....
        ...#...#.#....
        ....#...##....
        ...#.###......
        ...##.#.##....
        ....#..#......
        ..............
        ..............
        ..............
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "110")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "20")
    }
}
