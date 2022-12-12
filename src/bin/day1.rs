use std::io;

use anyhow::{Context, Result};
use itertools::Itertools;

type Input = Vec<Vec<i32>>;

fn parse_input(input: &str) -> Result<Input> {
    let lines: Vec<&str> = input.lines().into_iter().collect();

    let per_elf: Vec<Vec<i32>> = lines
        .split(|line| line.is_empty())
        .map(|numbers| {
            numbers
                .iter()
                .map(|number| number.parse().context("Failed to parse number"))
                .collect::<Result<_>>()
        })
        .collect::<Result<_>>()
        .context("Failed to parse")?;

    Ok(per_elf)
}

fn part1(input: &Input) -> Result<i32> {
    input
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .context("Empty input")
}

fn part2(input: &Input) -> Result<i32> {
    Ok(input
        .iter()
        .map(|elf| elf.iter().sum::<i32>())
        .sorted()
        .rev()
        .take(3)
        .sum())
}

#[test]
fn test() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    let parsed = parse_input(input).unwrap();

    assert_eq!(part1(&parsed).unwrap(), 24000);
    assert_eq!(part2(&parsed).unwrap(), 45000);
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(io::stdin()).context("Failed to read input.")?;

    let (parsed, took) = aoc2022::timed(|| parse_input(&input).unwrap());
    println!("Parsing input took: {}ms", took.as_millis());

    let (results, took) = aoc2022::timed(|| part1(&parsed).unwrap());
    println!("Part 1 answer: `{}`; took: {}ms", results, took.as_millis());

    let (results, took) = aoc2022::timed(|| part2(&parsed).unwrap());
    println!("Part 2 answer: `{}`; took: {}ms", results, took.as_millis());

    Ok(())
}
