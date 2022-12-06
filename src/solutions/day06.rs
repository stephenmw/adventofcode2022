use crate::solutions::prelude::*;

use std::collections::HashSet;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let ans = first_unique_str(input, 4).ok_or(anyhow!("no solution"))?;
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let ans = first_unique_str(input, 14).ok_or(anyhow!("no solution"))?;
    Ok(ans.to_string())
}

fn first_unique_str(s: &str, length: usize) -> Option<usize> {
    s.as_bytes()
        .windows(length)
        .enumerate()
        .find(|(_, xs)| unique(xs))
        .map(|(i, _)| i + length)
}

fn unique(xs: &[u8]) -> bool {
    let mut s = HashSet::new();

    for x in xs {
        if !s.insert(x) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "7")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "19")
    }
}
