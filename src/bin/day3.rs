use std::{collections::HashSet, io};

use anyhow::{Context, Result};
use itertools::Itertools;

type Input = Vec<Rucksack>;

#[derive(Debug)]
struct Rucksack(HashSet<char>, HashSet<char>);

impl Rucksack {
    fn common(&self) -> Vec<char> {
        self.0
            .intersection(&self.1)
            .into_iter()
            .cloned()
            .collect_vec()
    }

    fn joined(&self) -> HashSet<char> {
        self.0.union(&self.1).cloned().collect()
    }
}

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => (c as u32) as i32 - 96,
        'A'..='Z' => (c as u32) as i32 - 38,
        _ => todo!(),
    }
}

#[test]
fn test_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('z'), 26);
    assert_eq!(priority('A'), 27);
    assert_eq!(priority('Z'), 52);
}

fn parse_input(input: &str) -> Result<Input> {
    let mut rucksacks = vec![];

    for line in input.lines() {
        let line = line.trim();
        let n = line.len();
        let half = n / 2;

        let chars = line.chars().collect_vec();
        let mut h1: HashSet<char> = HashSet::new();
        let mut h2: HashSet<char> = HashSet::new();

        h1.extend(&chars[..half]);
        h2.extend(&chars[half..]);

        rucksacks.push(Rucksack(h1, h2))
    }

    Ok(rucksacks)
}

fn part1(input: &Input) -> Result<i32> {
    Ok(input
        .iter()
        .map(|sack| sack.common().iter().cloned().map(priority).sum::<i32>())
        .sum())
}

fn part2(input: &Input) -> Result<i32> {
    Ok(input
        .chunks(3)
        .into_iter()
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| {
            let mut common = chunk[0].joined();
            for elf in chunk.iter().skip(1) {
                common = common.intersection(&elf.joined()).cloned().collect()
            }

            common.iter().cloned().map(priority).sum::<i32>()
        })
        .sum())
}

#[test]
fn test() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    ";

    let parsed = parse_input(input).unwrap();

    assert_eq!(part1(&parsed).unwrap(), 157);
    assert_eq!(part2(&parsed).unwrap(), 70);
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
