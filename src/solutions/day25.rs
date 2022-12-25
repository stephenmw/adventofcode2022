use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let nums = parse!(input);
    let sum: i64 = nums.iter().sum();
    let n = base_5(sum);
    Ok(base5_to_snafu(&n))
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    Ok("[Start The Blender]".to_owned())
}

// BCD... but base 5
fn base_5(mut n: i64) -> Vec<u8> {
    let mut ret = Vec::new();

    while n > 0 {
        ret.push((n % 5) as u8);
        n /= 5;
    }

    ret
}

fn base5_to_snafu(xs: &[u8]) -> String {
    let mut carry = 0;
    let mut ret = String::new();

    for d in xs.iter().copied() {
        let c = carry;
        carry = 0;
        let ch = match d + c {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                carry = 1;
                '='
            }
            4 => {
                carry = 1;
                '-'
            }
            5 => {
                carry = 1;
                '0'
            }
            _ => panic!("bad digit"),
        };

        ret.push(ch)
    }

    ret.chars().rev().collect()
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<i64>> {
        ws_all_consuming(many1(ws_line(snafu_num)))(input)
    }

    fn snafu_num(input: &str) -> IResult<&str, i64> {
        let digit = alt((
            value(2, char('2')),
            value(1, char('1')),
            value(0, char('0')),
            value(-1, char('-')),
            value(-2, char('=')),
        ));

        many1(digit)
            .map(|xs| xs.into_iter().fold(0, |acc, d| acc * 5 + d))
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "2=-1=0")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "[Start The Blender]")
    }
}
