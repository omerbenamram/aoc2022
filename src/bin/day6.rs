use std::{collections::HashSet, io};

use anyhow::{Context, Result};
use itertools::Itertools;

type Input = String;

fn parse_input(input: &str) -> Result<Input> {
    Ok(input.to_string())
}

fn start_of_signal(s: &str, sz: usize) -> Option<usize> {
    let chars = s.chars().collect_vec();

    for (i, v) in chars.windows(sz).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(v);
        if set.len() == sz {
            return Some(i + sz);
        }
    }

    None
}

fn part1(input: &Input) -> Result<i32> {
    Ok(start_of_signal(input, 4).unwrap().try_into().unwrap())
}

fn part2(input: &Input) -> Result<i32> {
    Ok(start_of_signal(input, 14).unwrap().try_into().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        start_of_signal("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(),
        5
    );
    assert_eq!(
        start_of_signal("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(),
        6
    );
    assert_eq!(
        start_of_signal("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(),
        10
    );
    assert_eq!(
        start_of_signal("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(),
        19
    );
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(io::stdin()).context("Failed to read input.")?;

    let (parsed, took) = aoc2022::timed(|| parse_input(&input).unwrap());
    println!("Parsing input took: {}ms", took.as_millis());

    let (results, took) = aoc2022::timed(|| part1(&parsed).unwrap());
    println!("Part 1 answer: `{}`; took: {}ns", results, took.as_nanos());

    let (results, took) = aoc2022::timed(|| part2(&parsed).unwrap());
    println!("Part 2 answer: `{}`; took: {}ns", results, took.as_nanos());

    Ok(())
}
