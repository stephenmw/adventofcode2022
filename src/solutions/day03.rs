use crate::solutions::prelude::*;

use std::collections::HashSet;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let ans = input
        .split_whitespace()
        .map(|x| x.split_at(x.len() / 2))
        .flat_map(|(a, b)| {
            let a_s = HashSet::<char>::from_iter(a.chars());
            let b_s = HashSet::<char>::from_iter(b.chars());
            a_s.intersection(&b_s).cloned().collect::<Vec<char>>()
        })
        .map(|x| priority(x))
        .sum::<u32>();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let sacks: Vec<_> = input
        .split_whitespace()
        .map(|x| HashSet::<char>::from_iter(x.chars()))
        .collect();

    let items = sacks.chunks_exact(3).map(|x| {
        find_intersection(&x[0], &x[1], &x[2]).ok_or(anyhow!("group with no intersection"))
    });

    items
        .map(|x| x.map(priority))
        .try_fold(0, |acc, x| Ok(acc + x?))
        .map(|x| x.to_string())
}

fn find_intersection(a: &HashSet<char>, b: &HashSet<char>, c: &HashSet<char>) -> Option<char> {
    a.intersection(b)
        .cloned()
        .collect::<HashSet<char>>()
        .intersection(c)
        .cloned()
        .next()
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => 27 + c as u32 - 'A' as u32,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "157")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "70")
    }
}
