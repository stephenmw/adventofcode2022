use std::collections::{HashSet, VecDeque};

use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = Grid::new(
        input
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let start = grid
        .iter_points()
        .find(|p| grid.get(*p).map(|&x| x == 'S').unwrap_or(false))
        .ok_or(anyhow!("no starting location found"))?;

    let ans = bfs(&grid, [start])?;
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = Grid::new(
        input
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let start_locations = grid
        .iter_points()
        .filter(|p| grid.get(*p).map(|&x| x == 'S' || x == 'a').unwrap_or(false));

    let ans = bfs(&grid, start_locations)?;

    Ok(ans.to_string())
}

fn bfs<I>(grid: &Grid<char>, start_points: I) -> Result<usize, anyhow::Error>
where
    I: std::iter::IntoIterator<Item = Point>,
{
    let mut frontier = VecDeque::new();
    let mut seen = HashSet::new();

    for start in start_points {
        frontier.push_back((start, 0));
        seen.insert(start);
    }

    while let Some((p, steps)) = frontier.pop_front() {
        let cur_height = match grid.get(p) {
            Some('S') => 'a',
            Some(x) => *x,
            None => continue,
        } as u32;

        let next_directions = Direction::iter().filter_map(|d| p.next(d));
        for p in next_directions {
            let next_height = match grid.get(p) {
                Some('E') => {
                    if cur_height >= 'y' as u32 {
                        return Ok(steps + 1);
                    } else {
                        continue;
                    }
                }
                Some(x) => *x,
                None => continue,
            } as u32;

            if next_height <= cur_height + 1 {
                if seen.insert(p) {
                    frontier.push_back((p, steps + 1));
                }
            }
        }
    }

    bail!("no solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "31")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "29")
    }
}
