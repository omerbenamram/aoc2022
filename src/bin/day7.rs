use std::{
    collections::{BTreeMap, HashMap},
    io,
};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

#[derive(Debug)]
enum Entry {
    Dir,
    File(usize),
}

type Input = FileSystem;

#[derive(Debug)]
struct FileSystem(BTreeMap<String, Entry>);

impl FileSystem {
    pub fn du(&self) -> HashMap<String, usize> {
        let mut du = HashMap::new();

        for (path, file_size) in self.0.iter() {
            if let Entry::File(sz) = file_size {
                *du.entry("/".to_string()).or_insert(0) += sz;

                let dirs = path.split('/').collect_vec();

                let n = dirs.len();

                for i in 0..n {
                    let d = dirs[0..i].join("/");

                    if d.is_empty() {
                        continue;
                    }

                    *du.entry(d).or_insert(0) += sz;
                }
            }
        }
        du
    }
}

fn mkpath(stack: &[&str], f: Option<&str>) -> String {
    let mut path = String::new();
    path.push('/');
    if !stack.is_empty() {
        path.push_str(&stack.join("/"));
        path.push('/');
    }

    if let Some(f) = f {
        path.push_str(f);
    }

    path
}

fn parse_input(input: &str) -> Result<Input> {
    let mut cwd = Vec::new();
    let mut fs = BTreeMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let words = line.split(' ').collect_vec();

        if line.starts_with('$') {
            match words[1] {
                "cd" => {
                    let path = words[2];
                    if path == ".." {
                        cwd.pop();
                    } else if path == "/" {
                        continue;
                    } else {
                        cwd.push(path)
                    }
                }
                "ls" => {
                    continue;
                }
                _ => bail!("unknown shell command"),
            }
        } else {
            match words[0] {
                "dir" => {
                    let path = mkpath(&cwd, Some(words[1]));
                    fs.insert(path, Entry::Dir);
                }
                sz => {
                    let n = sz
                        .parse::<usize>()
                        .context("expected size to be a number")?;

                    let path = mkpath(&cwd, Some(words[1]));

                    fs.insert(path, Entry::File(n));
                }
            }
        }
    }

    Ok(FileSystem(fs))
}

fn part1(input: &Input) -> Result<i32> {
    Ok(input
        .du()
        .values()
        .filter_map(|&v| if v <= 100000 { Some(v as i32) } else { None })
        .sum())
}

fn part2(input: &Input) -> Result<i32> {
    let du = input.du();

    let total_space = 70000000;
    let needed_space = 30000000;
    let unused_space = total_space - du["/"];

    Ok(*du
        .values()
        .sorted()
        .find_or_first(|v| unused_space + *v >= needed_space)
        .context("impossible to free space")? as i32)
}

#[test]
fn test() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    let parsed = dbg!(parse_input(input).unwrap());

    assert_eq!(part1(&parsed).unwrap(), 95437);
    assert_eq!(part2(&parsed).unwrap(), 24933642);
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
