#[derive(Clone, Debug)]
pub struct Grid<T> {
    pub cells: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        assert!(!data.is_empty() && !data[0].is_empty());
        Grid { cells: data }
    }

    pub fn get(&self, p: Point) -> Option<&T> {
        self.cells.get(p.y)?.get(p.x)
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        self.cells.get_mut(p.y)?.get_mut(p.x)
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point> {
        let (x_len, y_len) = self.size();
        (0..y_len).flat_map(move |y| (0..x_len).map(move |x| Point::new(x, y)))
    }

    pub fn iter_line(&self, start: Point, d: Direction) -> impl Iterator<Item = (Point, &T)> {
        LineIterator {
            g: self,
            p: Some(start),
            d,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.cells[0].len(), self.cells.len())
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(cells: Vec<Vec<T>>) -> Self {
        Self::new(cells)
    }
}

struct LineIterator<'a, T> {
    g: &'a Grid<T>,
    p: Option<Point>,
    d: Direction,
}

impl<'a, T> Iterator for LineIterator<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.p?;
        let ret = self.g.get(cur)?;
        self.p = cur.next(self.d);

        Some((cur, ret))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
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
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
    }
}
