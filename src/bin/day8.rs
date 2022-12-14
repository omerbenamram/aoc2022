use std::{io, ops::Index};

use anyhow::{bail, Context, Result};
use aoc2022::grid::Grid;
use itertools::Itertools;
use log::trace;

type Input = Grid;

fn parse_input(input: &str) -> Result<Input> {
    Grid::new(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect_vec()
            })
            .collect_vec(),
    )
}

fn is_visible(grid: &Grid, idx: (usize, usize)) -> bool {
    let tree = grid[idx];
    trace!("{:?} - value {}", idx, tree);

    let visible_right = grid.iter_row(idx.1).skip(idx.0 + 1).all(|v| v < tree);
    let visible_left = grid.iter_row(idx.1).take(idx.0).all(|v| v < tree);
    let visible_bottom = grid.iter_col(idx.0).skip(idx.1 + 1).all(|v| v < tree);
    let visible_top = grid.iter_col(idx.0).take(idx.1).all(|v| v < tree);

    visible_left || visible_right || visible_top || visible_bottom
}

fn scenic_score(grid: &Grid, idx: (usize, usize)) -> usize {
    let tree = grid[idx];

    trace!("{:?} - value {}", idx, tree);
    let visible_right = grid.iter_row(idx.1).skip(idx.0 + 1).all(|v| v < tree);

    let mut visible_right_pos = grid
        .iter_row(idx.1)
        .skip(idx.0 + 1)
        .take_while(|v| *v < tree)
        .count();

    if !visible_right {
        visible_right_pos += 1;
    }
    trace!("right {:?}", visible_right_pos);

    let visible_left = grid.iter_row(idx.1).take(idx.0).all(|v| v < tree);
    let mut visible_left_pos = grid
        .iter_row(idx.1)
        .take(idx.0)
        .collect_vec()
        .into_iter()
        .rev()
        .take_while(|v| *v < tree)
        .count();

    if !visible_left {
        visible_left_pos += 1;
    }

    trace!("left {:?}", visible_left_pos);

    let visible_bottom = grid.iter_col(idx.0).skip(idx.1 + 1).all(|v| v < tree);
    let mut visible_bottom_pos = grid
        .iter_col(idx.0)
        .skip(idx.1 + 1)
        .take_while(|v| *v < tree)
        .count();

    if !visible_bottom {
        visible_bottom_pos += 1;
    }

    trace!("bottom {:?}", visible_bottom_pos);

    let visible_top = grid.iter_col(idx.0).take(idx.1).all(|v| v < tree);
    let mut visible_top_pos = grid
        .iter_col(idx.0)
        .take(idx.1)
        .collect_vec()
        .into_iter()
        .rev()
        .take_while(|v| *v < tree)
        .count();

    if !visible_top {
        visible_top_pos += 1;
    }
    trace!("top {:?}", visible_top_pos);

    visible_left_pos * visible_right_pos * visible_top_pos * visible_bottom_pos
}

fn part1(input: &Input) -> Result<i32> {
    let mut visible = 0;

    for i in 0..input.nrows() {
        for j in 0..input.ncols() {
            if is_visible(input, (i, j)) {
                visible += 1;
            }
        }
    }

    Ok(visible)
}

fn part2(input: &Input) -> Result<i32> {
    let mut best = 0;

    for i in 0..input.nrows() {
        for j in 0..input.ncols() {
            let score = scenic_score(input, (i, j));
            if score > best {
                best = score
            }
        }
    }

    Ok(best as i32)
}

#[test]
fn test() {
    let input = "
30373
25512
65332
33549
35390";

    let parsed = parse_input(input).unwrap();
    assert!(!is_visible(&parsed, (1, 3)));

    assert_eq!(part1(&parsed).unwrap(), 21);

    assert_eq!(scenic_score(&parsed, (2, 3)), 8);

    assert_eq!(part2(&parsed).unwrap(), 8);
}

fn main() -> Result<()> {
    env_logger::init();

    let input = std::io::read_to_string(io::stdin()).context("Failed to read input.")?;

    let (parsed, took) = aoc2022::timed(|| parse_input(&input).unwrap());
    println!("Parsing input took: {}ms", took.as_millis());

    let (results, took) = aoc2022::timed(|| part1(&parsed).unwrap());
    println!("Part 1 answer: `{}`; took: {}ms", results, took.as_millis());

    let (results, took) = aoc2022::timed(|| part2(&parsed).unwrap());
    println!("Part 2 answer: `{}`; took: {}ms", results, took.as_millis());

    Ok(())
}
