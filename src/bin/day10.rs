use std::{cell::RefCell, collections::VecDeque, io, ops::AddAssign, rc::Rc};

use anyhow::{bail, Context, Result};

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

type Input = VecDeque<Instruction>;

fn parse_input(input: &str) -> Result<Input> {
    let mut instructions = VecDeque::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut words = line.split_whitespace();
        let word = words.next().context("expected word")?;
        match word {
            "noop" => instructions.push_back(Instruction::Noop),
            "addx" => {
                let amount = words
                    .next()
                    .context("expected number")?
                    .parse::<i32>()
                    .context("expected number")?;

                instructions.push_back(Instruction::Addx(amount))
            }
            _ => bail!("Unknown instruction {}", word),
        }
    }

    Ok(instructions)
}

struct Cpu {
    register: i32,
    clock: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            register: 1,
            clock: 1,
        }
    }

    pub fn simulate_program_for(
        &mut self,
        mut program: VecDeque<Instruction>,
        until: usize,
        mut hook: Box<dyn FnMut(i32, usize)>,
    ) {
        let mut running_for = 1;
        let mut current_instruction = program.pop_front();

        while self.clock < until {
            match current_instruction.as_ref() {
                Some(Instruction::Addx(n)) => {
                    if running_for == 2 {
                        self.register += n;
                        running_for = 0;
                        current_instruction = program.pop_front();
                    }
                }
                Some(Instruction::Noop) => {
                    running_for = 0;
                    current_instruction = program.pop_front();
                }
                None => {}
            }

            hook(self.register, self.clock);

            self.clock += 1;
            running_for += 1;
        }

        hook(self.register, self.clock);
    }
}

fn part1(input: &Input) -> Result<i32> {
    let mut cpu = Cpu::new();

    let result = Rc::new(RefCell::new(0));

    let clone = Rc::clone(&result);

    cpu.simulate_program_for(
        input.to_owned(),
        220,
        Box::new(move |register, clock| {
            if [20, 60, 100, 140, 180, 220].contains(&clock) {
                clone.borrow_mut().add_assign(clock as i32 * register);
            }
        }),
    );

    let b = result.borrow();
    Ok(b.to_owned())
}

fn part2(input: &Input) -> Result<i32> {
    let mut cpu = Cpu::new();

    print!("#");

    cpu.simulate_program_for(
        input.to_owned(),
        220,
        Box::new(move |register, clock| {
            if clock % 40 == 0 {
                println!("!");
            }

            if (register - 1..=register + 1).contains(&(clock as i32 % 40)) {
                print!("#");
            } else {
                print!(".");
            }
        }),
    );

    Ok(0)
}

#[test]
fn test_day10() {
    let input = include_str!("../../input/day10_test");

    let parsed = parse_input(input).unwrap();
    assert_eq!(part1(&parsed).unwrap(), 13140);
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
