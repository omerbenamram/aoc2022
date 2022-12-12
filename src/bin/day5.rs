use std::{
    collections::{BTreeMap, VecDeque},
    fmt::{Display, Write},
    io,
};

use anyhow::{Context, Result};

use aoc2022::regex;

type Stack<T> = Vec<T>;
type Input = CraneProblem;

#[derive(Clone, Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Clone, Debug)]
struct CraneProblem {
    state: BTreeMap<usize, Stack<String>>,
    operations: Vec<Instruction>,
}

impl Display for CraneProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut state = self.clone();
        let mut tallest = state.state.values().map(|v| v.len()).max().unwrap();

        while tallest > 0 {
            for stack in state.state.values_mut() {
                if stack.len() < tallest {
                    f.write_str("   ")?;
                } else {
                    f.write_char('[')?;
                    f.write_char(stack.pop().unwrap().chars().next().unwrap())?;
                    f.write_char(']')?;
                }
                f.write_char(' ')?;
            }
            f.write_char('\n')?;

            tallest -= 1;
        }

        for idx in state.state.keys() {
            f.write_fmt(format_args!("{:3} ", idx))?;
        }

        Ok(())
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let mut lines: VecDeque<&str> = input.lines().collect();

    if lines[0].is_empty() {
        lines.pop_front();
    }

    let mut state_lines = vec![];

    for line in lines.iter() {
        if line.is_empty() {
            break;
        }

        state_lines.push(line)
    }

    // Skip the numbers line
    // Parse the stacks reversed;
    let mut problem_state = BTreeMap::new();

    for line in state_lines.iter().rev().skip(1) {
        let boxes = regex!(r"(?P<empty>\s\s\s\s)|(\[(?P<letter>\w)\])").captures_iter(line);

        for (i, b) in boxes.into_iter().enumerate() {
            if b.name("empty").is_some() {
                continue;
            }
            problem_state
                .entry(i + 1)
                .or_insert_with(Stack::new)
                .push(b["letter"].to_string())
        }
    }

    let mut problem_instructions = vec![];

    for instruction in lines.iter().skip(state_lines.len() + 1) {
        let captures = regex!(r"move (?P<quantity>\d+) from (?P<from>\d) to (?P<to>\d)")
            .captures(instruction)
            .context(format!(
                "Expect line to match instruction `{}`",
                instruction
            ))?;

        problem_instructions.push(Instruction {
            quantity: captures["quantity"]
                .parse()
                .context("Error parsing quantity")?,
            from: captures["from"].parse().context("Error parsing from")?,
            to: captures["to"].parse().context("Error parsing to")?,
        });
    }

    Ok(CraneProblem {
        state: problem_state,
        operations: problem_instructions,
    })
}

fn part1(input: &Input) -> Result<String> {
    let mut board = input.clone();

    for instruction in board.operations {
        for _ in 0..instruction.quantity {
            let item = board
                .state
                .get_mut(&instruction.from)
                .context(format!("Expected a stack at {}", instruction.from))?
                .pop()
                .context(format!("Expected a box at {}", instruction.from))?;

            board
                .state
                .get_mut(&instruction.to)
                .context(format!("Expected a stack at {}", instruction.from))?
                .push(item);
        }
    }

    let mut result = String::new();

    for letters in board.state.values() {
        let letter = letters.last();

        if let Some(l) = letter {
            result.push_str(l);
        }
    }

    Ok(result)
}

fn part2(input: &Input) -> Result<String> {
    let mut board = input.clone();

    for instruction in board.operations {
        let mut buffer = VecDeque::new();

        for _ in 0..instruction.quantity {
            let item = board
                .state
                .get_mut(&instruction.from)
                .context(format!("Expected a stack at {}", instruction.from))?
                .pop()
                .context(format!("Expected a box at {}", instruction.from))?;

            buffer.push_front(item);
        }

        board
            .state
            .get_mut(&instruction.to)
            .context(format!("Expected a stack at {}", instruction.from))?
            .extend(buffer.iter().cloned())
    }

    let mut result = String::new();

    for letters in board.state.values() {
        let letter = letters.last();

        if let Some(l) = letter {
            result.push_str(l);
        }
    }

    Ok(result)
}

#[test]
fn test() {
    let input = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let parsed = parse_input(input).unwrap();
    println!("{}", parsed);

    assert_eq!(part1(&parsed).unwrap(), "CMZ".to_string());
    assert_eq!(part2(&parsed).unwrap(), "MCD".to_string());
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(io::stdin()).context("Failed to read input.")?;

    let (parsed, took) = aoc2022::timed(|| parse_input(&input).unwrap());
    println!("{}", parsed);
    println!("Parsing input took: {}ms", took.as_millis());

    let (results, took) = aoc2022::timed(|| part1(&parsed).unwrap());
    println!("Part 1 answer: `{}`; took: {}ms", results, took.as_millis());

    let (results, took) = aoc2022::timed(|| part2(&parsed).unwrap());
    println!("Part 2 answer: `{}`; took: {}ms", results, took.as_millis());

    Ok(())
}
