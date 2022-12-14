use crate::solutions::prelude::*;
use crate::utils;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);

    data.iter()
        .map(|x| x.iter().sum::<usize>())
        .max()
        .ok_or(anyhow!("no elves"))
        .map(|x| x.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);

    let counts = data.iter().map(|x| x.iter().sum::<usize>());
    let ans: usize = utils::top_n(counts, 3).sum();

    Ok(ans.to_string())
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
        let elf = separated_list1(line_ending, uint);
        let elves = separated_list1(tuple((line_ending, line_ending)), elf);
        complete(elves)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "24000")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "45000")
    }
}
