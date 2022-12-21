use std::collections::HashMap;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let mut monkeys = parse!(input);
    let root = monkeys
        .get("root")
        .ok_or(anyhow!("root not found"))?
        .clone();
    let ans = root.find_value(&mut monkeys)?;
    Ok(ans.to_string())
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    todo!()
}

type MonkeyName = String;

#[derive(Clone, Debug)]
pub struct Monkey {
    name: MonkeyName,
    task: MonkeyTask,
}

impl Monkey {
    fn find_value(&self, m: &mut HashMap<MonkeyName, Monkey>) -> anyhow::Result<u64> {
        let res = match &self.task {
            MonkeyTask::Value(x) => *x,
            MonkeyTask::Operation(op) => {
                let val = op.execute(m)?;
                m.get_mut(&self.name).unwrap().task = MonkeyTask::Value(val);
                val
            }
        };

        Ok(res)
    }
}

#[derive(Clone, Debug)]
pub enum MonkeyTask {
    Operation(Operation),
    Value(u64),
}

impl MonkeyTask {}

#[derive(Clone, Debug)]
struct Operation {
    op: Op,
    a: MonkeyName,
    b: MonkeyName,
}

impl Operation {
    fn execute(&self, m: &mut HashMap<MonkeyName, Monkey>) -> anyhow::Result<u64> {
        let a = m.get(&self.a).ok_or(anyhow!("dangling monkey"))?.clone();
        let b = m.get(&self.b).ok_or(anyhow!("dangling monkey"))?.clone();

        let a_val = a.find_value(m)?;
        let b_val = b.find_value(m)?;

        Ok(self.op.execute(a_val, b_val))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn execute(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
        }
    }
}

mod parser {
    use std::collections::HashMap;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, HashMap<MonkeyName, Monkey>> {
        let parser =
            many1(ws_line(monkey)).map(|xs| xs.into_iter().map(|x| (x.name.clone(), x)).collect());
        ws_all_consuming(parser)(input)
    }

    fn monkey(input: &str) -> IResult<&str, Monkey> {
        let op = alt((
            value(Op::Add, char('+')),
            value(Op::Sub, char('-')),
            value(Op::Mul, char('*')),
            value(Op::Div, char('/')),
        ));
        let operation = tuple((identifier, delimited(space0, op, space0), identifier))
            .map(|(a, op, b)| Operation { op, a, b });

        let task = alt((
            uint.map(|x| MonkeyTask::Value(x)),
            operation.map(|x| MonkeyTask::Operation(x)),
        ));

        separated_pair(identifier, tag(": "), task)
            .map(|(name, task)| Monkey { name, task })
            .parse(input)
    }

    fn identifier(input: &str) -> IResult<&str, String> {
        alphanumeric1.map(|x: &str| x.to_owned()).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "152")
    }

    #[test]
    fn problem2_test() {
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "")
    }
}
