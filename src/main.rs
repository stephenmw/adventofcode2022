mod grid;
mod parser;
mod solutions;
#[macro_use]
extern crate lazy_static;

use std::time::Instant;

use anyhow::{anyhow, Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "aoc2022")]
#[command(author = "Stephen Weinberg")]
#[command(about = "Solves Advent of Code 2022", long_about = None)]
struct Cli {
    day: usize,
    problem: usize,
    #[arg(long)]
    input: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let s = solutions::SOLUTIONS
        .get(&cli.day)
        .ok_or(anyhow!("day not found"))?;
    let problem_fn = match cli.problem {
        1 => s.problem1,
        2 => s.problem2,
        _ => return Err(anyhow!("unknown problem number: {}", cli.problem)),
    };

    let flag_input = cli
        .input
        .as_ref()
        .map(|x| std::fs::read_to_string(x).context("failed to read input file"))
        .transpose()?;

    let input = flag_input.as_deref().unwrap_or(s.input);

    let start = Instant::now();
    let ans = problem_fn(input).context("problemfn failed")?;
    let end = Instant::now();

    println!("{}", ans);
    let duration = end.duration_since(start);
    println!("\nComputed in {:?}", duration);

    Ok(())
}
