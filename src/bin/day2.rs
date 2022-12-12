use std::io;

use anyhow::{Context, Result};
use itertools::Itertools;

type Input = Vec<Round>;

enum Startegy {
    X,
    Y,
    Z,
}

enum Play {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    me: Startegy,
    opponent: Play,
}

impl Round {
    fn score(&self) -> i32 {
        let my_shape = match &self.me {
            Startegy::X => Play::Rock,
            Startegy::Y => Play::Paper,
            Startegy::Z => Play::Scissors,
        };

        let shape = match &my_shape {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };

        let results = match (&my_shape, &self.opponent) {
            (Play::Rock, Play::Rock) => 3,
            (Play::Rock, Play::Paper) => 0,
            (Play::Rock, Play::Scissors) => 6,
            (Play::Paper, Play::Rock) => 6,
            (Play::Paper, Play::Paper) => 3,
            (Play::Paper, Play::Scissors) => 0,
            (Play::Scissors, Play::Rock) => 0,
            (Play::Scissors, Play::Paper) => 6,
            (Play::Scissors, Play::Scissors) => 3,
        };

        shape + results
    }

    fn p2_score(&self) -> i32 {
        // X - Need to lose
        // Y - Need to draw
        // Z - Need to win
        let my_shape = match (&self.me, &self.opponent) {
            (Startegy::X, Play::Rock) => Play::Scissors,
            (Startegy::X, Play::Paper) => Play::Rock,
            (Startegy::X, Play::Scissors) => Play::Paper,
            (Startegy::Y, Play::Rock) => Play::Rock,
            (Startegy::Y, Play::Paper) => Play::Paper,
            (Startegy::Y, Play::Scissors) => Play::Scissors,
            (Startegy::Z, Play::Rock) => Play::Paper,
            (Startegy::Z, Play::Paper) => Play::Scissors,
            (Startegy::Z, Play::Scissors) => Play::Rock,
        };

        let shape = match &my_shape {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };

        let score = match &self.me {
            Startegy::X => 0,
            Startegy::Y => 3,
            Startegy::Z => 6,
        };

        shape + score
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let lines: Vec<&str> = input.lines().into_iter().collect();
    let mut results = vec![];

    for line in lines {
        let (opponent, me) = line
            .split(' ')
            .collect_tuple()
            .context("Expected exactly two letters seperated by a single space.")?;

        let opponent_play = match opponent {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => anyhow::bail!("Expected exactly one of `'A', 'B', 'C'` for opponent play"),
        };

        let my_play = match me {
            "X" => Startegy::X,
            "Y" => Startegy::Y,
            "Z" => Startegy::Z,
            _ => anyhow::bail!(format!(
                "Expected exactly one of `'X', 'Y', 'Z'` for my play, found `{}`",
                me
            )),
        };

        let round = Round {
            me: my_play,
            opponent: opponent_play,
        };

        results.push(round)
    }

    Ok(results)
}

fn part1(input: &Input) -> Result<i32> {
    Ok(input.iter().map(|i| i.score()).sum())
}

fn part2(input: &Input) -> Result<i32> {
    Ok(input.iter().map(|i| i.p2_score()).sum())
}

#[test]
fn test() {
    let input = "A Y
B X
C Z
";
    let parsed = parse_input(input).unwrap();

    assert_eq!(part1(&parsed).unwrap(), 15);
    assert_eq!(part2(&parsed).unwrap(), 12);
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
