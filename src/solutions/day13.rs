use std::{cmp::Ordering, fmt::Write};

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);

    let ans: usize = data
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| {
            if pair.0.cmp(&pair.1).is_lt() {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let mut modified_input = input.trim_end().to_owned();
    modified_input += "\n\n[[2]]\n[[6]]";
    let data = parse!(&modified_input);
    let mut packets: Vec<_> = data.into_iter().flat_map(|(a, b)| [a, b]).collect();
    packets.sort_unstable();

    let divider_locations = packets
        .iter()
        .enumerate()
        .filter(|(_, x)| {
            let s = format!("{}", x);
            s == "[[2]]" || s == "[[6]]"
        })
        .map(|(i, _)| i + 1);

    let ans: usize = divider_locations.product();

    Ok(ans.to_string())
}

#[derive(Clone, Debug, Default, Eq)]
pub struct List {
    values: Vec<Element>,
}

impl List {
    fn from_value(v: u32) -> List {
        List {
            values: vec![Element::Value(v)],
        }
    }
}

impl std::cmp::Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.values.iter().zip(other.values.iter()) {
            let ord = a.cmp(b);
            if !ord.is_eq() {
                return ord;
            }
        }

        self.values.len().cmp(&other.values.len())
    }
}

impl std::cmp::PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        if let Some((first, rest)) = self.values.split_first() {
            write!(f, "{}", first)?;
            for v in rest {
                f.write_char(',')?;
                write!(f, "{}", v)?;
            }
        }
        f.write_char(']')
    }
}

#[derive(Clone, Debug, Eq)]
enum Element {
    List(List),
    Value(u32),
}

impl std::cmp::Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::Value(a), Self::Value(b)) => a.cmp(b),
            (Self::List(a), Self::Value(b)) => a.cmp(&List::from_value(*b)),
            (Self::Value(a), Self::List(b)) => (List::from_value(*a)).cmp(b),
        }
    }
}

impl std::cmp::PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(l) => write!(f, "{}", l),
            Self::Value(v) => write!(f, "{}", v),
        }
    }
}

mod parser {
    use nom::multi::separated_list0;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<(List, List)>> {
        let list_pair = pair(ws_line(list), ws_line(list));
        let pairs = separated_list1(delimited(space0, line_ending, space0), list_pair);
        ws_all_consuming(pairs)(input)
    }

    fn list(input: &str) -> IResult<&str, List> {
        delimited(char('['), separated_list0(char(','), element), char(']'))
            .map(|values| List { values })
            .parse(input)
    }

    fn element(input: &str) -> IResult<&str, Element> {
        alt((
            uint::<u32>.map(|v| Element::Value(v)),
            list.map(|l| Element::List(l)),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        [1,1,3,1,1]
        [1,1,5,1,1]
        
        [[1],[2,3,4]]
        [[1],4]
        
        [9]
        [[8,7,6]]
        
        [[4,4],4,4]
        [[4,4],4,4,4]
        
        [7,7,7,7]
        [7,7,7]
        
        []
        [3]
        
        [[[]]]
        [[]]
        
        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "13")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "140")
    }
}
