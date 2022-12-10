use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let states: Vec<(usize, i32)> = MachineStateIter::new(&data).collect();
    let total_cycles = states.last().unwrap().0;

    let ans: i32 = (20..total_cycles + 1)
        .step_by(40)
        .map(|cycle| {
            let loc = states.binary_search_by_key(&cycle, |x| x.0);
            let i = match loc {
                Ok(x) => x,
                Err(x) => x,
            };
            cycle as i32 * states[i - 1].1
        })
        .sum();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let states: Vec<(usize, i32)> = MachineStateIter::new(&data).collect();

    let mut ans = String::new();

    let mut x = 1;
    let mut cur = 0;

    for i in 0..240 {
        if i % 40 == 0 && i != 0 {
            ans.push('\n');
        }

        if i >= states[cur].0 {
            x = states[cur].1;
            cur += 1;
        }

        let pos = (i % 40) as i32;
        if pos >= x - 1 && pos <= x + 1 {
            ans.push('#');
        } else {
            ans.push('.');
        }
    }

    Ok(ans)
}

struct MachineStateIter<'a> {
    instructions: &'a [Instruction],
    x: i32,
    pc: usize,
    cycle: usize,
}

impl<'a> MachineStateIter<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        MachineStateIter {
            instructions,
            x: 1,
            pc: 0,
            cycle: 0,
        }
    }
}

impl<'a> std::iter::Iterator for MachineStateIter<'a> {
    type Item = (usize, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.instructions.get(self.pc).is_none() {
            return None;
        }

        while let Some(inst) = self.instructions.get(self.pc) {
            self.pc += 1;
            self.cycle += inst.cycles();
            match inst {
                Instruction::Noop => (),
                Instruction::Addx(x) => {
                    self.x += x;
                    return Some((self.cycle, self.x));
                }
            };
        }

        // Program has completed. Send final results.
        Some((self.cycle, self.x))
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
        let inst = alt((
            value(Instruction::Noop, tag("noop")),
            preceded(tag("addx "), int).map(|x| Instruction::Addx(x)),
        ));

        let parser = many1(ws_line(inst));
        ws_all_consuming(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "13140")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(
            problem2(EXAMPLE_INPUT).unwrap(),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        )
    }
}
