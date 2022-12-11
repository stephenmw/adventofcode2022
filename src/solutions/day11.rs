use std::collections::VecDeque;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let monkeys = parse!(input);
    let mut group = MonkeyGroup::from(monkeys);
    (0..20).for_each(|_| group.round(3));

    let mut inspections: Vec<_> = group.monkeys.iter().map(|m| m.num_inspections).collect();
    inspections.sort_unstable_by(|a, b| b.cmp(a));

    let ans: u64 = (&inspections[..2]).iter().product();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let monkeys = parse!(input);
    let mut group = MonkeyGroup::from(monkeys);
    (0..10000).for_each(|_| group.round(1));

    let mut inspections: Vec<_> = group.monkeys.iter().map(|m| m.num_inspections).collect();
    inspections.sort_unstable_by(|a, b| b.cmp(a));

    let ans: u64 = (&inspections[..2]).iter().product();
    Ok(ans.to_string())
}

struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    multiple_divisors: u64,
}

impl MonkeyGroup {
    fn round(&mut self, worry_divisor: u64) {
        for id in 0..self.monkeys.len() {
            self.monkey_round(id, worry_divisor);
        }
    }

    fn monkey_round(&mut self, id: usize, worry_divisor: u64) {
        while let Some(item) = self.monkeys[id].items.pop_front() {
            self.monkeys[id].num_inspections += 1;

            let new_worry = (self.monkeys[id].operation.apply(item.worry) % self.multiple_divisors)
                / worry_divisor;

            let next_monkey = match new_worry % self.monkeys[id].test_divisor == 0 {
                true => self.monkeys[id].next_true,
                false => self.monkeys[id].next_false,
            };

            self.monkeys[next_monkey].items.push_back(new_worry.into());
        }
    }
}

impl From<Vec<Monkey>> for MonkeyGroup {
    fn from(monkeys: Vec<Monkey>) -> Self {
        let multiple_divisors = monkeys.iter().map(|m| m.test_divisor).product();
        MonkeyGroup {
            monkeys,
            multiple_divisors,
        }
    }
}

pub struct Item {
    worry: u64,
}

impl From<u64> for Item {
    fn from(worry: u64) -> Self {
        Item { worry }
    }
}

pub struct Monkey {
    operation: Operation,
    test_divisor: u64,
    next_true: usize,
    next_false: usize,

    items: VecDeque<Item>,
    num_inspections: u64,
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Add(Operand),
    Mul(Operand),
}

impl Operation {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Self::Add(Operand::Value(y)) => x + *y,
            Self::Mul(Operand::Value(y)) => x * *y,
            Self::Add(Operand::Old) => x + x,
            Self::Mul(Operand::Old) => x * x,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Value(u64),
    Old,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
        let id = delimited(tag("Monkey "), uint::<usize>, tag(":"));
        let starting_items = preceded(
            tag("Starting items: "),
            separated_list1(tag(", "), into(uint::<u64>)),
        );
        let operand = || {
            alt((
                value(Operand::Old, tag("old")),
                uint.map(|x| Operand::Value(x)),
            ))
        };
        let operation = preceded(
            tag("Operation: new = old "),
            alt((
                preceded(tag("+ "), operand()).map(|x| Operation::Add(x)),
                preceded(tag("* "), operand()).map(|x| Operation::Mul(x)),
            )),
        );
        let divisor = preceded(tag("Test: divisible by "), uint);
        let next_true = preceded(tag("If true: throw to monkey "), uint);
        let next_false = preceded(tag("If false: throw to monkey "), uint);

        let monkey = tuple((
            ws_line(id),
            ws_line(starting_items),
            ws_line(operation),
            ws_line(divisor),
            ws_line(next_true),
            ws_line(next_false),
        ))
        .map(
            |(_id, items, operation, test_divisor, next_true, next_false)| Monkey {
                items: VecDeque::from(items),
                operation,
                test_divisor,
                next_true,
                next_false,

                num_inspections: 0,
            },
        );

        let monkeys = separated_list1(delimited(space0, line_ending, space0), monkey);

        ws_all_consuming(monkeys)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        Monkey 0:
            Starting items: 79, 98
            Operation: new = old * 19
            Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
        
        Monkey 1:
            Starting items: 54, 65, 75, 74
            Operation: new = old + 6
            Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0
        
        Monkey 2:
            Starting items: 79, 60, 97
            Operation: new = old * old
            Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3
        
        Monkey 3:
            Starting items: 74
            Operation: new = old + 3
            Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "10605")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "2713310158")
    }
}
