mod grid;
mod parser;
mod solutions;
mod utils;

#[macro_use]
extern crate lazy_static;

use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use rayon::prelude::*;

#[derive(Parser)]
#[command(name = "aoc2022")]
#[command(author = "Stephen Weinberg")]
#[command(about = "Solves Advent of Code 2022", long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        day: usize,
        problem: usize,
        #[arg(long)]
        input: Option<String>,
    },
    RunAll {
        #[arg(long)]
        parallel: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Run {
            day,
            problem,
            input,
        } => run(day, problem, input),
        Commands::RunAll { parallel } => run_all(parallel),
    }
}

fn run(day: usize, problem: usize, input: Option<String>) -> Result<()> {
    let flag_input = input
        .as_ref()
        .map(|x| std::fs::read_to_string(x).context("failed to read input file"))
        .transpose()?;

    let (ans, duration) = run_problem(day, problem, flag_input.as_deref())?;

    println!("{}", ans);
    println!("\nComputed in {:?}", duration);

    Ok(())
}

fn run_all(parallel: bool) -> Result<()> {
    let mut times = if parallel {
        (1..=25)
            .into_par_iter()
            .flat_map(|day| [(day, 1), (day, 2)])
            .map(|(day, problem)| (day, problem, run_problem(day, problem, None)))
            .try_fold_with(Vec::new(), |mut acc, (day, problem, res)| {
                acc.push((day, problem, res?.1));
                anyhow::Ok(acc)
            })
            .try_reduce(Vec::new, |mut a, b| {
                a.extend(b);
                Ok(a)
            })?
    } else {
        (1..=25)
            .flat_map(|day| [(day, 1), (day, 2)])
            .map(|(day, problem)| (day, problem, run_problem(day, problem, None)))
            .try_fold(Vec::new(), |mut acc, (day, problem, res)| {
                acc.push((day, problem, res?.1));
                anyhow::Ok(acc)
            })?
    };

    times.sort_by(|a, b| a.2.cmp(&b.2).reverse());
    for (day, problem, duration) in &times {
        println!("{:2}-{}: {:?}", day, problem, duration);
    }

    Ok(())
}

fn run_problem(day: usize, problem: usize, input: Option<&str>) -> Result<(String, Duration)> {
    let solution = solutions::SOLUTIONS
        .get(&day)
        .ok_or(anyhow!("unknown day: {}", day))?;
    let problem_fn = match problem {
        1 => solution.problem1,
        2 => solution.problem2,
        _ => return Err(anyhow!("unknown problem number: {}", problem)),
    };

    let input = input.unwrap_or(solution.input);

    let start = Instant::now();
    let ans = problem_fn(input).context("problemfn failed")?;
    let end = Instant::now();

    return Ok((ans, end.duration_since(start)));
}
