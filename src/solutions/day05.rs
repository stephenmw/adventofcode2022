use crate::solutions::prelude::*;

use std::cmp::Ordering;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (mut layout, moves) = parse!(input);
    for m in &moves {
        layout.apply_move(m, true).context("bad move")?;
    }

    Ok(layout.top())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (mut layout, moves) = parse!(input);
    for m in &moves {
        layout.apply_move(m, false).context("bad move")?;
    }

    Ok(layout.top())
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Clone, Debug)]
pub struct Layout {
    crates: Vec<Vec<char>>,
}

impl Layout {
    fn apply_move(&mut self, m: &Move, reverse: bool) -> anyhow::Result<()> {
        if m.to.max(m.from) >= self.crates.len() {
            bail!("invalid column");
        }

        if m.to == m.from {
            bail!("the source and destination of a crate cannot be the same");
        }

        let (to, from) = get_dual_mut(&mut self.crates, m.to, m.from);

        let start = from
            .len()
            .checked_sub(m.count)
            .ok_or(anyhow!("not enough crates in column"))?;

        let to_move = from.drain(start..);

        if reverse {
            to.extend(to_move.rev());
        } else {
            to.extend(to_move);
        }

        Ok(())
    }

    fn top(&self) -> String {
        self.crates.iter().filter_map(|x| x.last()).collect()
    }
}

// This function gets mutable elements from two distinct indexes of a slice.
fn get_dual_mut<T>(s: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    let (a, b) = s.split_at_mut(i.max(j));

    match i.cmp(&j) {
        Ordering::Less => (&mut a[i], &mut b[0]),
        Ordering::Equal => panic!("cannot get two pointers to the same index"),
        Ordering::Greater => (&mut b[0], &mut a[j]),
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, (Layout, Vec<Move>)> {
        let mov = map(
            tuple((
                tag("move "),
                uint,
                tag(" from "),
                uint::<usize>,
                tag(" to "),
                uint::<usize>,
            )),
            |(_, count, _, from, _, to)| Move {
                count: count,
                from: from - 1,
                to: to - 1,
            },
        );
        let moves = separated_list1(line_ending, mov);

        let parser = separated_pair(crate_layout, tuple((line_ending, line_ending)), moves);
        complete(parser)(input)
    }

    fn crate_layout(input: &str) -> IResult<&str, Layout> {
        let crate_ = delimited(char('['), one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), char(']'));
        let crate_space = alt((map(crate_, |x| Some(x)), value(None, tag("   "))));
        let crate_line = separated_list1(char(' '), crate_space);
        let crate_lines = separated_list1(line_ending, crate_line);
        let numbers_line = many1(delimited(space0, uint::<usize>, space0));
        let layout = separated_pair(crate_lines, line_ending, numbers_line);

        let mut parser = map_res(layout, |(lines, numbers)| {
            let num_columns = numbers.len();
            let mut crates = vec![Vec::new(); num_columns];
            for l in lines.iter().rev() {
                for (i, v) in l.iter().enumerate() {
                    if let Some(val) = v {
                        crates
                            .get_mut(i)
                            .ok_or(anyhow!("invalid input: too many crates on a line"))?
                            .push(*val);
                    }
                }
            }
            Ok::<_, anyhow::Error>(Layout { crates })
        });

        parser(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "CMZ")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "MCD")
    }
}
