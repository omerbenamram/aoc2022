use std::{io, ops::RangeInclusive};

use anyhow::{Context, Result};
use aoc2022::range_inclusive;
use itertools::Itertools;

type Input = Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>;

fn parse_input(input: &str) -> Result<Input> {
    let mut results = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let line = line.trim();

        let (elf1, elf2) = line
            .split(',')
            .collect_tuple()
            .context("Expected each line to contains 2 ranges delimited by ,")?;

        let r1 = range_inclusive(elf1)?;
        let r2 = range_inclusive(elf2)?;

        results.push((r1, r2))
    }

    Ok(results)
}

/// For every pair of ranges, chech in how many ranges one fully contains the other
fn part1(input: &Input) -> Result<i32> {
    Ok(input
        .iter()
        .filter(|(r1, r2)| {
            ((r1.start() <= r2.start()) && (r1.end() >= r2.end()))
                || (r2.start() <= r1.start()) && (r2.end() >= r1.end())
        })
        .count() as i32)
}

/// For every pair of ranges, chech in how many ranges overlap at all.
fn part2(input: &Input) -> Result<i32> {
    Ok(input
        .iter()
        // we can check if the absolute sum of the first start from the second end is less than the sum.
        .filter(|(r1, r2)| {
            let r1_span = r1.end().abs_diff(*r1.start()) as i32;
            let r2_span = r2.end().abs_diff(*r2.start()) as i32;
            let total_span = r1_span + r2_span as i32;
            let r1_r2_diff = r1.end().abs_diff(*r2.start()) as i32;
            let r2_r1_diff = r2.end().abs_diff(*r1.start()) as i32;

            (r1_r2_diff <= total_span) && (r2_r1_diff <= total_span)
        })
        .count() as i32)
}

#[test]
fn test() {
    let input = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    let parsed = parse_input(input).unwrap();
    assert_eq!(part1(&parsed).unwrap(), 2);
    assert_eq!(part2(&parsed).unwrap(), 4);
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
