use std::{
    collections::{HashMap, VecDeque},
    io,
    ops::DivAssign,
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1, space0},
    combinator::map_res,
    error::{convert_error, VerboseError},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
};

type Input = Vec<Monkey>;

pub(crate) type IResult<'a, T> = nom::IResult<&'a str, T, VerboseError<&'a str>>;

type Int = i64;
type MonkeyId = Int;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Int>,
    operation: Assignment,
    test_divisible_by: Int,
    forward: (MonkeyId, MonkeyId),
}

fn number(input: &str) -> IResult<Int> {
    map_res(digit1, str::parse)(input)
}

#[derive(Debug, Clone)]
enum Ident {
    Variable(String),
    Const(Int),
}

impl FromStr for Ident {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<Int>() {
            Ok(n) => Ident::Const(n),
            Err(_) => Ident::Variable(s.to_string()),
        })
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Add(Ident, Ident),
    Multiply(Ident, Ident),
}

#[derive(Debug, Clone)]
struct Assignment {
    lhs: Ident,
    rhs: Expr,
}

impl Expr {
    fn eval(&self, locals: &HashMap<String, Int>) -> Int {
        match self {
            Expr::Add(i1, i2) => {
                let i1 = match i1 {
                    Ident::Variable(v) => *locals.get(v).unwrap(),
                    Ident::Const(v) => *v,
                };
                let i2 = match i2 {
                    Ident::Variable(v) => *locals.get(v).unwrap(),
                    Ident::Const(v) => *v,
                };

                i1 + i2
            }
            Expr::Multiply(i1, i2) => {
                let i1 = match i1 {
                    Ident::Variable(v) => *locals.get(v).unwrap(),
                    Ident::Const(v) => *v,
                };
                let i2 = match i2 {
                    Ident::Variable(v) => *locals.get(v).unwrap(),
                    Ident::Const(v) => *v,
                };

                i1 * i2
            }
        }
    }
}

fn word<'a>(w: &str, s: &'a str) -> IResult<'a, &'a str> {
    tag(w)(s)
}

fn ident<'a>(s: &'a str) -> IResult<Ident> {
    delimited(space0, map_res(alphanumeric1, Ident::from_str), space0)(s)
}

fn expr<'a>(s: &'a str) -> IResult<Expr> {
    let (rest, lhs) = ident(s)?;
    let (rest, op) = delimited(space0, alt((char('+'), char('*'))), space0)(rest)?;
    let (rest, rhs) = ident(rest)?;

    Ok((
        rest,
        match op {
            '+' => Expr::Add(lhs, rhs),
            '*' => Expr::Multiply(lhs, rhs),
            _ => unreachable!(),
        },
    ))
}

fn operation<'a>(s: &'a str) -> IResult<Assignment> {
    let (_, (lhs, rhs)) = separated_pair(ident, tag("="), expr)(s)?;

    Ok(("", Assignment { lhs, rhs }))
}

fn numbers(s: &str) -> IResult<Vec<Int>> {
    separated_list1(tag(", "), map_res(digit1, str::parse))(s)
}

fn parse_input(input: &str) -> Result<Input> {
    let mut monkeys = vec![];
    let lines = input.lines().filter(|l| !l.is_empty()).collect_vec();

    for chunk in lines.chunks_exact(6) {
        let mut lines = chunk.iter();
        let line = lines.next().unwrap().trim();

        let _ = pair(tag("Monkey "), number)(line).unwrap();

        let line = lines.next().unwrap().trim();
        let (rest, _) = word("Starting items: ", line).unwrap();

        let (_, items) = numbers(rest).unwrap();
        let line = lines.next().unwrap().trim_start();
        let (rest, _) = word("Operation: ", line).unwrap();

        let op = match operation(rest) {
            Ok(op) => op.1,
            Err(nom::Err::Error(e)) => bail!(convert_error(rest, e)),
            Err(e) => bail!("Another error occured `{}`", e),
        };

        let line = lines.next().unwrap().trim_start();
        let (_, (_, divisible_by)) = pair(tag("Test: divisible by "), number)(line).unwrap();

        let line = lines.next().unwrap().trim_start();
        let (_, (_, monkey_id_true)) =
            pair(tag("If true: throw to monkey "), number)(line).unwrap();

        let line = lines.next().unwrap().trim_start();
        let (_, (_, monkey_id_false)) =
            pair(tag("If false: throw to monkey "), number)(line).unwrap();

        monkeys.push(Monkey {
            items,
            operation: op,
            test_divisible_by: divisible_by,
            forward: (monkey_id_true, monkey_id_false),
        });
    }

    Ok(monkeys)
}

fn part1(input: &Input) -> Result<Int> {
    let monkeys = input.to_vec();
    let mut items = HashMap::new();

    for (i, m) in monkeys.iter().enumerate() {
        for item in m.items.iter().cloned() {
            items.entry(i).or_insert_with(VecDeque::new).push_back(item)
        }
    }

    let mut inspected_count = HashMap::new();

    for _ in 0..20 {
        let m = *items.keys().max().unwrap();

        for monkey_id in 0..=m {
            let q1 = items.get_mut(&monkey_id).unwrap().clone();

            for item in q1 {
                let mut m = HashMap::new();
                m.insert("old".to_string(), item);
                *inspected_count.entry(monkey_id).or_insert(0) += 1;

                let monkey = &monkeys[monkey_id];
                let (ift, iff) = monkey.forward;

                let mut new = monkey.operation.rhs.eval(&m);
                new.div_assign(3);

                if (new % monkey.test_divisible_by) == 0 {
                    items.get_mut(&(ift as usize)).unwrap().push_front(new)
                } else {
                    items.get_mut(&(iff as usize)).unwrap().push_front(new)
                }
            }

            items.get_mut(&monkey_id).unwrap().clear();
        }
    }

    let (top1, top2) = inspected_count
        .values()
        .sorted()
        .rev()
        .take(2)
        .collect_tuple()
        .unwrap();

    Ok(top1 * top2)
}

fn part2(input: &Input) -> Result<Int> {
    let monkeys = input.to_vec();
    let mut items = HashMap::new();

    for (i, m) in monkeys.iter().enumerate() {
        for item in m.items.iter().cloned() {
            items.entry(i).or_insert_with(VecDeque::new).push_back(item)
        }
    }

    let mut inspected_count = HashMap::new();

    let mod_all: Int = monkeys.iter().map(|m| m.test_divisible_by).product();

    for _ in 0..10_000 {
        let m = *items.keys().max().unwrap();

        for monkey_id in 0..=m {
            let q1 = items.get_mut(&monkey_id).unwrap().clone();

            for item in q1 {
                let mut m = HashMap::new();
                m.insert("old".to_string(), item);
                *inspected_count.entry(monkey_id).or_insert(0) += 1;

                let monkey = &monkeys[monkey_id];
                let (ift, iff) = monkey.forward;

                let mut new = monkey.operation.rhs.eval(&m);
                new %= mod_all;

                if (new % monkey.test_divisible_by) == 0 {
                    items.get_mut(&(ift as usize)).unwrap().push_front(new)
                } else {
                    items.get_mut(&(iff as usize)).unwrap().push_front(new)
                }
            }

            items.get_mut(&monkey_id).unwrap().clear();
        }
    }

    let (top1, top2) = inspected_count
        .values()
        .sorted()
        .rev()
        .take(2)
        .collect_tuple()
        .unwrap();

    Ok(top1 * top2)
}

#[test]
fn test_day11() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    ";

    let parsed = parse_input(input).unwrap();

    assert_eq!(part1(&parsed).unwrap(), 10605);
    assert_eq!(part2(&parsed).unwrap(), 2713310158);
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
