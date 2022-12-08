use std::collections::HashMap;

macro_rules! days {
    ($($x:ident), *) => {
        $(
            mod $x;
        )*

        const SOLUTIONS_ARR: &'static [Solution] = &[
            $(
                Solution {
                    day: stringify!($x),
                    problem1: $x::problem1,
                    problem2: $x::problem2,
                    input: include_str!(concat!("../puzzle-inputs/", stringify!($x), ".txt"))
                },
            )*
        ];

    };
    ($($x:ident,) *) => (days!($($x),*););
}

lazy_static! {
    pub static ref SOLUTIONS: HashMap<usize, &'static Solution> =
        HashMap::from_iter(SOLUTIONS_ARR.iter().map(|x| (mod_name_to_num(x.day), x)));
}

// converts "day21" to 21;
fn mod_name_to_num(name: &str) -> usize {
    name.chars()
        .filter_map(|x| x.to_digit(10))
        .fold(0, |acc, x| acc * 10 + x as usize)
}

pub type ProblemFn = fn(&str) -> Result<String, anyhow::Error>;

pub struct Solution {
    pub day: &'static str,
    pub problem1: ProblemFn,
    pub problem2: ProblemFn,
    pub input: &'static str,
}

#[macro_use]
mod prelude {
    pub use anyhow::{anyhow, bail, Context};

    macro_rules! parse {
        ($input:expr) => {
            parser::parse($input)
                .map_err(|x| x.to_owned())
                .context("failed to parse input")?
                .1
        };
    }
}

days!(day01, day02, day03, day04, day05, day06, day08);
