mod parser;
mod solutions;
#[macro_use]
extern crate lazy_static;

use std::time::Instant;

use anyhow::{anyhow, Context, Result};

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next();
    let day: usize = args
        .next()
        .context("not enough args")?
        .parse()
        .context("failed to parse day")?;
    let problem: usize = args
        .next()
        .context("not enough args")?
        .parse()
        .context("failed to parse problem")?;

    let s = solutions::SOLUTIONS
        .get(&day)
        .ok_or(anyhow!("day not found"))?;
    let problem_fn = match problem {
        1 => s.problem1,
        2 => s.problem2,
        _ => return Err(anyhow!("unknown problem number: {}", problem)),
    };

    let start = Instant::now();
    let ans = problem_fn(s.input).context("problemfn failed")?;
    let end = Instant::now();

    println!("{}", ans);
    let duration = end.duration_since(start);
    println!("\nComputed in {:?}", duration);

    Ok(())
}
