// Advent of Code 2023
// Liam Fenneman

use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
struct Node {
    label: String,
    left: String,
    right: String,
}

struct NodeMap(HashMap<String, Node>);

#[derive(Debug, Clone)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Instructions {
    inner: Vec<Instruction>,
    index: usize,
}

impl FromStr for NodeMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NodeMap(
            s.lines()
                .map(|line| line.parse::<Node>().unwrap())
                .map(|n| (n.label.clone(), n.clone()))
                .collect(),
        ))
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, tuple) = s.trim().split_once(" = ").unwrap();

        let (left, right) =
            tuple[1..tuple.len() - 1].trim().split_once(", ").unwrap();

        Ok(Node {
            label: label.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        })
    }
}

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instructions {
            inner: s
                .chars()
                .map(|ch| match ch {
                    'L' => Instruction::Left,
                    'R' => Instruction::Right,
                    _ => panic!("invalid input"),
                })
                .collect(),
            index: 0,
        })
    }
}

impl Iterator for Instructions {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.inner.get((self.index - 1) % self.inner.len()).cloned()
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut h = std::cmp::max(a, b);
    let mut l = std::cmp::min(a, b);

    while l != 0 {
        let temp = l;
        l = h % l;
        h = temp;
    }

    h
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn part1(input: &str) -> u64 {
    let (instructions, nodes) = input.trim().split_once("\n\n").unwrap();

    let nodes: NodeMap = nodes.parse().unwrap();

    let instructions = instructions.parse::<Instructions>().unwrap();

    let mut count = 0;
    let mut current_node = nodes.0.get("AAA");

    for ins in instructions {
        if current_node.is_some_and(|n| n.label == "ZZZ")
            || current_node.is_none()
        {
            break;
        }

        let cn = current_node.unwrap();

        match ins {
            Instruction::Left => current_node = nodes.0.get(&cn.left),
            Instruction::Right => current_node = nodes.0.get(&cn.right),
        }

        count += 1;
    }

    count
}

fn part2(input: &str) -> u64 {
    let (instructions, nodes) = input.trim().split_once("\n\n").unwrap();
    let map: NodeMap = nodes.parse().unwrap();

    let start_nodes = map
        .0
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect::<Vec<String>>();
    let mut counts = Vec::new();

    for node in start_nodes.iter() {
        let mut current = node.clone();
        let mut idx = 0;
        let mut current_ins = instructions.chars().nth(idx).unwrap();
        let mut count = 0;
        while !current.ends_with('Z') {
            current = match current_ins {
                'R' => map.0.get(&current).unwrap().right.clone(),
                'L' => map.0.get(&current).unwrap().left.clone(),
                _ => panic!("Invalid direction"),
            };
            idx = (idx + 1) % instructions.len();
            current_ins = instructions.chars().nth(idx).unwrap();
            count += 1;
        }
        counts.push(count);
    }
    let count = counts.iter().fold(1, |acc, x| lcm(acc, *x as u64));

    count
}

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input/day8.txt");

    println!("Part 1: {}", part1(file));
    println!("Part 2: {}", part2(file));

    Ok(())
}

#[allow(dead_code)]
const FILE_EXAMPLE_P1: &str = r"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

#[allow(dead_code)]
const FILE_EXAMPLE_P2: &str = r"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

#[test]
fn example_part1() {
    assert_eq!(6, part1(FILE_EXAMPLE_P1));
}

#[test]
fn example_part2() {
    assert_eq!(6, part2(FILE_EXAMPLE_P2));
}
