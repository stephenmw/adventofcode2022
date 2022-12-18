use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let points = parse!(input);
    let grid = build_grid(&points)?;

    let ans: usize = points
        .iter()
        .map(|&p| {
            6 - Direction::iter_neighbors(p)
                .filter_map(|p| grid.get(p).filter(|x| **x == Space::Magma))
                .count()
        })
        .sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let points = parse!(input);

    // Ensure no points are on the zero border.
    let translated_points: Vec<_> = points
        .into_iter()
        .map(|p| Point {
            x: p.x + 1,
            y: p.y + 1,
            z: p.z + 1,
        })
        .collect();

    let mut grid = build_grid(&translated_points)?;
    let mut frontier = vec![Point::new(0, 0, 0)];

    while let Some(p) = frontier.pop() {
        let Some(cell) = grid.get_mut(p) else {continue};
        if *cell != Space::Empty {
            continue;
        }
        *cell = Space::Water;
        frontier.extend(Direction::iter_neighbors(p));
    }

    let ans: usize = translated_points
        .iter()
        .map(|&p| {
            Direction::iter_neighbors(p)
                .filter_map(|p| grid.get(p).filter(|x| **x == Space::Water))
                .count()
        })
        .sum();

    Ok(ans.to_string())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Space {
    Empty,
    Magma,
    Water,
}

impl Default for Space {
    fn default() -> Self {
        Self::Empty
    }
}

fn build_grid(points: &[Point]) -> Result<Grid<Space>, anyhow::Error> {
    // Size must be big enough to hold all points and for points not to touch
    // the far border.
    let size = points
        .iter()
        .flat_map(|p| [p.x, p.y, p.z])
        .max()
        .map(|s| s + 2)
        .unwrap_or(0);

    let mut grid = Grid {
        cells: vec![vec![vec![Space::Empty; size]; size]; size],
    };
    for &p in points {
        *grid.get_mut(p).ok_or(anyhow!("grid not big enough"))? = Space::Magma;
    }

    Ok(grid)
}

#[derive(Clone, Debug, Default)]
struct Grid<T> {
    cells: Vec<Vec<Vec<T>>>,
}

impl<T> Grid<T> {
    pub fn get(&self, p: Point) -> Option<&T> {
        self.cells.get(p.z)?.get(p.y)?.get(p.x)
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        self.cells.get_mut(p.z)?.get_mut(p.y)?.get_mut(p.x)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    fn step(&self, d: Direction) -> Option<Point> {
        let p = match d {
            Direction::Up => Point::new(self.x, self.y.checked_add(1)?, self.z),
            Direction::Down => Point::new(self.x, self.y.checked_sub(1)?, self.z),
            Direction::Left => Point::new(self.x.checked_sub(1)?, self.y, self.z),
            Direction::Right => Point::new(self.x.checked_add(1)?, self.y, self.z),
            Direction::Forward => Point::new(self.x, self.y, self.z.checked_add(1)?),
            Direction::Back => Point::new(self.x, self.y, self.z.checked_sub(1)?),
        };

        Some(p)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Back,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Forward,
            Direction::Back,
        ]
        .into_iter()
    }

    pub fn iter_neighbors(p: Point) -> impl Iterator<Item = Point> {
        Self::iter().filter_map(move |d| p.step(d))
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Point>> {
        let point = tuple((uint, char(','), uint, char(','), uint))
            .map(|(x, _, y, _, z)| Point::new(x, y, z));
        ws_all_consuming(many1(ws_line(point)))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1("1,1,1\n2,1,1").unwrap(), "10");
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "64");
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "58")
    }
}
