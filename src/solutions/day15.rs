use crate::solutions::prelude::*;

use rayon::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    problem1_(input, 2000000)
}

fn problem1_(input: &str, row: isize) -> Result<String, anyhow::Error> {
    let sensors = parse!(input);

    let ranges = find_range_for_row(&sensors, row);

    let spaces_covered: usize = ranges.iter().map(|x| x.len()).sum();
    let beacons = unique_beacons(&sensors)
        .iter()
        .filter(|b| b.y == row)
        .count();
    let ans = spaces_covered - beacons;

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    problem2_(input, 4000000)
}

fn problem2_(input: &str, max_coordinate: isize) -> Result<String, anyhow::Error> {
    let sensors = parse!(input);

    let (row, ranges) = (0..=max_coordinate)
        .into_par_iter()
        .map(|row| (row, find_range_for_row(&sensors, row)))
        .find_any(|(_, ranges)| ranges.len() > 1)
        .ok_or(anyhow!("no solution"))?;

    let x = ranges[0].end;
    let y = row;
    let ans = x * 4000000 + y;
    Ok(ans.to_string())
}

fn unique_beacons(sensors: &[Sensor]) -> Vec<Point> {
    let mut beacons: Vec<_> = sensors.iter().map(|s| s.closest_beacon).collect();
    beacons.sort_unstable();
    beacons.dedup();

    beacons
}

fn find_range_for_row(sensors: &[Sensor], row: isize) -> Vec<Range> {
    let mut ranges: Vec<_> = sensors
        .iter()
        .filter_map(|s| {
            let dist = s.manhattan_distance();
            let dist_to_row = (row - s.location.y).abs();
            let row_len = dist - dist_to_row;
            let row_start = s.location.x;

            Some(Range::new(row_start - row_len, row_start + row_len + 1)).filter(|r| r.len() > 0)
        })
        .collect();

    ranges.sort_unstable();

    let simplified_ranges = ranges.iter().fold(Vec::new(), |mut acc, r| {
        let Some(cur) = acc.last_mut() else {
            acc.push(*r);
            return acc
        };

        if let Some(combined) = cur.add(r) {
            *cur = combined;
        } else {
            acc.push(*r);
        }

        acc
    });

    simplified_ranges
}

// Range [start, end)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn new(start: isize, end: isize) -> Self {
        Range { start, end }
    }

    fn add(&self, other: &Self) -> Option<Range> {
        if self.start > other.end || self.end < other.start {
            None
        } else {
            Some(Range {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        }
    }

    fn len(&self) -> usize {
        (self.end - self.start).try_into().unwrap_or(0)
    }
}

pub struct Sensor {
    location: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn manhattan_distance(&self) -> isize {
        self.location.manhattan_distance(&self.closest_beacon)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Sensor>> {
        let point = || {
            separated_pair(
                preceded(tag("x="), int),
                tag(", "),
                preceded(tag("y="), int),
            )
            .map(|(x, y)| Point::new(x, y))
        };

        let sensor = pair(
            preceded(tag("Sensor at "), point()),
            preceded(tag(": closest beacon is at "), point()),
        )
        .map(|(location, closest_beacon)| Sensor {
            location,
            closest_beacon,
        });

        let parser = many1(ws_line(sensor));
        ws_all_consuming(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1_(EXAMPLE_INPUT, 10).unwrap(), "26")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2_(EXAMPLE_INPUT, 20).unwrap(), "56000011")
    }
}
