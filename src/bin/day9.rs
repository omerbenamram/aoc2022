use core::panic;
use std::io;

use anyhow::{Context, Result};

use aoc2022::grid::{InfiniteGrid, Point};
use itertools::Itertools;
use log::trace;
use rstest::rstest;

type Input = Vec<Instruction>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    steps: usize,
}

struct Rope {
    inner: Vec<Point>,
}

impl Rope {
    pub fn new(num_knots: usize) -> Self {
        assert!(num_knots >= 2);
        Rope {
            inner: vec![(0, 0).into(); num_knots],
        }
    }

    pub fn step_head(&mut self, direction: Direction) {
        let head = *self.inner.get(0).expect("checked len");

        self.inner[0] = match direction {
            Direction::Right => head.right(1),
            Direction::Left => head.left(1),
            Direction::Up => head.up(1),
            Direction::Down => head.down(1),
        };

        for k in 1..self.inner.len() {
            let knot = *self.inner.get(k).expect("checked len");
            let prev = *self.inner.get(k - 1).expect("checked len");

            self.inner[k] = Self::new_knot_position(knot, prev);
        }
    }

    pub fn tail_position(&self) -> Point {
        self.inner[self.inner.len() - 1]
    }

    /// Calculate new tail position given head position.
    fn new_knot_position<T: Into<Point>>(tail: T, head: T) -> Point {
        let tail = tail.into();
        let head = head.into();

        match (head.x - tail.x, head.y - tail.y) {
            (-1, 2) | (-2, 1) | (-2, 2) => tail.up(1).left(1),
            (1, 2) | (2, 1) | (2, 2) => tail.up(1).right(1),
            (1, -2) | (2, -1) | (2, -2) => tail.down(1).right(1),
            (-1, -2) | (-2, -1) | (-2, -2) => tail.down(1).left(1),
            (2, 0) => tail.right(1),
            (-2, 0) => tail.left(1),
            (0, 2) => tail.up(1),
            (0, -2) => tail.down(1),
            _ => tail,
        }
    }
}

fn parse_input(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|l| l.split(' ').collect_tuple())
        .map(|(d, steps)| {
            let direction = match d {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("Unknown direction"),
            };
            Instruction {
                direction,
                steps: steps.parse().unwrap(),
            }
        })
        .collect_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((0, 0), (2, 0) , (1, 0))]
    #[case((0, 0), (-2, 0),(-1, 0))]
    #[case((0, 0), (0, 2) ,(0, 1))]
    #[case((0, 0), (0, -2),(0, -1))]
    #[case((0, 0), (1, 2),(1, 1))]
    #[case((0, 0), (-1, 2),(-1, 1))]
    #[case((0, 0), (1, -2),(1, -1))]
    #[case((0, 0), (-1, -2),(-1, -1))]
    #[case((0, 0), (2, 1),(1, 1))]
    fn test_tail_position(
        #[case] head: (i32, i32),
        #[case] tail: (i32, i32),
        #[case] expected: (i32, i32),
    ) {
        assert_eq!(
            Rope::new_knot_position::<(i32, i32)>(head.into(), tail.into()),
            expected.into()
        );
    }
}

fn part1(input: &Input) -> Result<i32> {
    let mut state: InfiniteGrid<bool> = InfiniteGrid::new();
    let mut rope = Rope::new(2);

    for instruction in input {
        trace!("{:?} {}", instruction.direction, instruction.steps);
        for _ in 0..instruction.steps {
            rope.step_head(instruction.direction);
            state.entry(rope.tail_position()).or_insert(true);
        }
        state.entry(rope.tail_position()).or_insert(true);
    }

    Ok(state.num_points() as i32)
}

fn part2(input: &Input) -> Result<i32> {
    let mut state: InfiniteGrid<bool> = InfiniteGrid::new();
    let mut rope = Rope::new(10);

    for instruction in input {
        trace!("{:?} {}", instruction.direction, instruction.steps);
        for _ in 0..instruction.steps {
            rope.step_head(instruction.direction);
            state.entry(rope.tail_position()).or_insert(true);
        }
        state.entry(rope.tail_position()).or_insert(true);
    }

    Ok(state.num_points() as i32)
}

fn dump(grid: &InfiniteGrid<bool>) {
    let (min_x, max_x, min_y, max_y) = grid.dimensions();
    println!("x = [{}:{}], y = [{}:{}]\n", min_x, max_x, min_y, max_y);

    for y in (min_y..=max_y).rev() {
        print!("{:2}| ", y);
        for x in min_x..=max_x {
            // dbg!((x, y));
            if (x, y) == (0, 0) {
                print!("O ");
                continue;
            }
            match grid.get((x, y).into()) {
                Some(_) => print!("#"),
                None => print!("."),
            }
            print!(" ")
        }
        println!()
    }
    print!("   ");
    for _x in min_x..=max_x {
        print!("--")
    }
    println!();
    print!("   ");
    for x in min_x..=max_x {
        print!("{:2}", x)
    }
    println!()
}

#[test]
fn test_day9() {
    env_logger::init();
    let input = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    let parsed = parse_input(input).unwrap();
    assert_eq!(part1(&parsed).unwrap(), 13);
    assert_eq!(part2(&parsed).unwrap(), 1);

    let larger_input = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    let parsed = parse_input(larger_input).unwrap();
    assert_eq!(part2(&parsed).unwrap(), 36);
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
