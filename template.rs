use std::io;

use anyhow::{Context, Result};
use itertools::Itertools;

type Input = ();

fn parse_input(input: &str) -> Result<Input> {
    todo!()
}

fn part1(input: &Input) -> Result<i32> {
    todo!()
}

fn part2(input: &Input) -> Result<i32> {
    todo!()
}

#[test]
fn test() {
    todo!()
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(io::stdin()).context("Failed to read input.")?;

    let (parsed, took) = aoc2022::timed(|| parse_input(&input).unwrap());
    println!("Parsing input took: {}ms", took.as_millis());

    // let (results, took) = aoc2022::timed(|| part1(&parsed).unwrap());
    // println!("Part 1 answer: `{}`; took: {}ms", results, took.as_millis());

    // let (results, took) = aoc2022::timed(|| part2(&parsed).unwrap());
    // println!("Part 2 answer: `{}`; took: {}ms", results, took.as_millis());

    Ok(())
}
