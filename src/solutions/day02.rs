use crate::solutions::prelude::*;

pub fn problem1<'a>(input: &'a str) -> Result<String, anyhow::Error> {
    let data = parse!(input);

    let ans = data
        .iter()
        .map(|(a, b)| score(b.as_shape(), b.as_shape().outcome(a)))
        .sum::<u32>();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);

    let ans = data
        .iter()
        .map(|(a, b)| score(a.compliment(b.as_outcome().rev()), b.as_outcome()))
        .sum::<u32>();

    Ok(ans.to_string())
}

fn score(player_move: Shape, outcome: Outcome) -> u32 {
    player_move.value() + outcome.value()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn value(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn compliment(&self, outcome: Outcome) -> Shape {
        match outcome {
            Outcome::Lose => self.lose(),
            Outcome::Tie => *self,
            Outcome::Win => self.win(),
        }
    }

    fn win(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn lose(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn outcome(&self, other: &Self) -> Outcome {
        if self == other {
            return Outcome::Tie;
        }

        if other == &self.win() {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Lose,
    Tie,
    Win,
}

impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }

    fn rev(&self) -> Outcome {
        match self {
            Outcome::Lose => Outcome::Win,
            Outcome::Tie => Outcome::Tie,
            Outcome::Win => Outcome::Lose,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncryptedValue {
    X,
    Y,
    Z,
}

impl EncryptedValue {
    fn as_shape(&self) -> Shape {
        match self {
            EncryptedValue::X => Shape::Rock,
            EncryptedValue::Y => Shape::Paper,
            EncryptedValue::Z => Shape::Scissors,
        }
    }

    fn as_outcome(&self) -> Outcome {
        match self {
            EncryptedValue::X => Outcome::Lose,
            EncryptedValue::Y => Outcome::Tie,
            EncryptedValue::Z => Outcome::Win,
        }
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<(Shape, EncryptedValue)>> {
        let shape_a = alt((
            value(Shape::Rock, tag("A")),
            value(Shape::Paper, tag("B")),
            value(Shape::Scissors, tag("C")),
        ));

        let encrypted = alt((
            value(EncryptedValue::X, tag("X")),
            value(EncryptedValue::Y, tag("Y")),
            value(EncryptedValue::Z, tag("Z")),
        ));

        let inst = separated_pair(shape_a, space1, encrypted);
        let parser = separated_list1(line_ending, inst);

        complete(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "15")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "12")
    }
}
