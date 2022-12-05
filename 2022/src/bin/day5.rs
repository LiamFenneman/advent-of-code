// Advent of Code 2022
// Liam Fenneman
//
// Day 5
//
// https://adventofcode.com/2022/day/5
#![allow(unused)]

use std::{str::FromStr, collections::VecDeque};
use anyhow::bail;

/// Converts the column number to a index within a row of characters.
fn column_to_index(c: usize) -> usize {
    4 * (c - 1) + 1
}

type Crate = char;
type Stack = Vec<Crate>;
type Platform = Vec<Stack>;

#[derive(Debug, Clone, Copy)]
struct Movement {
    count: usize,
    from: usize,
    to: usize,
}

impl Movement {
    /// Execute the movement on the given platform.
    pub fn execute(&self, platform: &mut Platform) {
        for i in 0..self.count {
            let c = platform[self.from-1].pop().unwrap();
            platform[self.to-1].push(c);
        }
    }

    /// Execute the movement on the given platform.
    /// -- Alternate: move multiple crates at once
    pub fn execute_alt(&self, platform: &mut Platform) {
        // this is achieved by popping all then pushing
        // in reverse order
        let mut crane: Vec<Crate> = Vec::with_capacity(self.count);
        for i in 0..self.count {
            let c = platform[self.from-1].pop().unwrap();
            crane.push(c);
        }

        for &c in crane.iter().rev() {
            platform[self.to-1].push(c);
        }
    }
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // we only care about numbers
        // the order of which make up the movement
        let r = s.split(" ").collect::<Vec<_>>().iter()
            .filter_map(|c| c.parse::<usize>().ok())
            .collect::<Vec<_>>();

        if r.len() != 3 { bail!("no enough nums"); }
        Ok(Movement { count: r[0], from: r[1], to: r[2] })
    }
}

fn main() -> anyhow::Result<()> {
    // read lines from stdin
    let lines = std::io::stdin()
        .lines()
        .flat_map(|l| l.ok())
        .collect::<Vec<_>>();

    // the input has two sections:
    // 1. initial state
    // 2. movement command
    // -- they are separated by an empty line
    let mut init_state: Vec<String> = Vec::new();
    let mut movements: Vec<String> = Vec::new();
    let mut has_parsed_init = false;
    for l in lines {
        if l.is_empty() {
            has_parsed_init = true;
            continue;
        }

        match has_parsed_init {
            false => init_state.push(l.clone()),
            true => movements.push(l.clone()),
        }
    }

    // init state needs to be read column by column
    let chars = init_state.iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
   
    let num_stacks = (chars[0].len()+1) / 4;
    let mut platform = Vec::with_capacity(num_stacks);
    // loop over the number of stacks, creating a stack and filling it
    for i in 1..=num_stacks {
        let ind = column_to_index(i);
        let mut stack = Vec::new();
        for line in &chars {
            stack.push(line[ind]);
        }
        // stack currently includes spaces and the column number
        // -- this needs to be removed before adding to the platform
        // stack is also in the wrong order
        let stack = stack.iter()
            .filter(|&&c| c != ' ' && !c.is_numeric())
            .map(|c| c.to_owned())
            .rev()
            .collect::<Vec<_>>();
        platform.push(stack);
    }
    
    // movements...
    let movements = movements.iter()
        .filter_map(|s| s.parse::<Movement>().ok())
        .collect::<VecDeque<_>>();

    // loop through all movements, executing them
    for m in movements {
        //m.execute(&mut platform);
        m.execute_alt(&mut platform);
    }
    
    // what crate ends up on top of each stack
    let mut result = String::new();
    for mut stack in platform {
        result = format!("{}{}", result, stack.pop().unwrap());
    }
    println!("Result: {}", result);

    Ok(())
}
