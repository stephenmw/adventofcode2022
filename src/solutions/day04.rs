use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let ans = data.iter().filter(|(a, b)| a.fully_overlaps(b)).count();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let ans = data.iter().filter(|(a, b)| a.has_overlap(b)).count();
    Ok(ans.to_string())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RangeInclusive {
    start: u32,
    end: u32,
}

impl RangeInclusive {
    fn fully_overlaps(&self, other: &Self) -> bool {
        if self.start <= other.start && self.end >= other.end {
            true
        } else if other.start <= self.start && other.end >= self.end {
            true
        } else {
            false
        }
    }

    fn has_overlap(&self, other: &Self) -> bool {
        let (a, b) = (self.min(other), self.max(other));
        a.end >= b.start
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<(RangeInclusive, RangeInclusive)>> {
        let range = || {
            map(separated_pair(uint, tag("-"), uint), |(start, end)| {
                RangeInclusive { start, end }
            })
        };
        let pair = separated_pair(range(), tag(","), range());
        let parser = separated_list1(line_ending, pair);
        complete(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "2")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "4")
    }
}
