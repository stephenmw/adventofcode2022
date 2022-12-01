use prelude::*;
use std::str::FromStr;

pub mod prelude {
    pub use nom::{
        branch::alt,
        bytes::complete::{is_a, tag, take_while},
        character::complete::{anychar, line_ending, multispace0, one_of, space0, space1},
        combinator::{eof, into, map, map_res, opt, recognize, value, verify},
        error::ParseError,
        multi::{count, many1, separated_list1},
        sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
        IResult, Parser,
    };

    pub use super::{complete, int, uint};
}

pub fn uint<T: FromStr>(input: &str) -> IResult<&str, T> {
    let digits = is_a("0123456789");
    let mut parser = map_res(digits, |x: &str| x.parse());
    parser(input)
}

pub fn int<T: FromStr>(input: &str) -> IResult<&str, T> {
    let digits = is_a("0123456789");
    let num = tuple((opt(tag("-")), digits));
    let mut parser = map_res(recognize(num), |x: &str| x.parse());
    parser(input)
}

// Ensures that parser F
pub fn complete<I, O1, E, P>(parser: P) -> impl FnMut(I) -> IResult<I, O1, E>
where
    I: nom::InputLength + nom::InputTakeAtPosition + Clone,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    P: nom::Parser<I, O1, E>,
    E: nom::error::ParseError<I>,
{
    terminated(parser, tuple((multispace0, eof)))
}
