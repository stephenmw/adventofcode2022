use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (mut layout, moves) = parse!(input);
    for m in moves {
        layout.move_9000(m).context("bad move")?;
    }

    Ok(layout.top())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (mut layout, moves) = parse!(input);
    for m in moves {
        layout.move_9001(m).context("bad move")?;
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
    fn move_9000(&mut self, m: Move) -> anyhow::Result<()> {
        for _ in 0..m.count {
            let val = self
                .crates
                .get_mut(m.from)
                .ok_or(anyhow!("from out of range"))?
                .pop()
                .ok_or(anyhow!("not enough values in column"))?;
            self.crates
                .get_mut(m.to)
                .ok_or(anyhow!("to out of range"))?
                .push(val);
        }

        Ok(())
    }

    fn move_9001(&mut self, m: Move) -> anyhow::Result<()> {
        let mut stack = Vec::new();

        for _ in 0..m.count {
            let val = self
                .crates
                .get_mut(m.from)
                .ok_or(anyhow!("from out of range"))?
                .pop()
                .ok_or(anyhow!("not enough values in column"))?;
            stack.push(val);
        }

        while let Some(v) = stack.pop() {
            self.crates
                .get_mut(m.to)
                .ok_or(anyhow!("to out of range"))?
                .push(v);
        }

        Ok(())
    }

    fn top(&self) -> String {
        self.crates
            .iter()
            .filter_map(|x| x.last())
            .cloned()
            .collect()
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
