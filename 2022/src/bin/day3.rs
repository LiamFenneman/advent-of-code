// Advent of Code 2022
// Liam Fenneman
//
// Day 3
//
// https://adventofcode.com/2022/day/3

// Both compartments have the SAME number of items

use std::io;

// VERY UGLY!
fn char_to_prio(c: char) -> u32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("invalid char"),
    }
}

#[derive(Debug, Clone)]
struct Rucksack {
    left: Vec<char>,
    right: Vec<char>,
}

fn find_dupe(rs: &Rucksack) -> char {
    for c in &rs.left {
        for d in &rs.right {
            if c == d {
                return *c;
            }
        }
    }

    panic!("no dupe found");
}

fn part1(lines: Vec<String>) {
    // create a list of rucksacks
    // each rusksack has two compartments
    // compartments are a half of the full string
    let sacks: Vec<Rucksack> = lines
        .iter()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let left = a.chars().collect();
            let right = b.chars().collect();
            Rucksack { left, right }
        })
    .collect();

    // sum the priorities of the duplicated items
    // (items that are in both compartments of a rucksack)
    let prio_sum: u32 = sacks.iter()
        .map(|rs| find_dupe(&rs))
        .map(|ch| char_to_prio(ch))
        .sum();
    println!("Part 1: {}", prio_sum);
}

type Group = [Rucksack; 3];

fn find_dupe_multi(g: &Group) -> char {
    // combine the sacks' compartments since they don't matter for P2
    let mut groups: Vec<Vec<char>> = Vec::new();
    for rs in g {
        let mut v = Vec::new();
        v.append(&mut rs.left.clone());
        v.append(&mut rs.right.clone());
        groups.push(v);
    }

    // find the common letter between all sacks
    for a in &groups[0] {
        for b in &groups[1] {
            for c in &groups[2] {
                if a == b && b == c {
                    return *a;
                }
            }
        }
    }

    panic!("no dupe found");
}

fn part2(lines: Vec<String>) {
    // create a list of rucksacks
    // each rusksack has two compartments
    // compartments are a half of the full string
    let sacks: Vec<Rucksack> = lines
        .iter()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let left = a.chars().collect();
            let right = b.chars().collect();
            Rucksack { left, right }
        })
    .collect();
    let mut groups: Vec<Group> = Vec::new();
    {
        let mut i = 0;
        while i < (sacks.len() - 2) {
            groups.push([sacks[i].clone(), sacks[i+1].clone(), sacks[i+2].clone()]);
            i += 3;
        }
    }
    
    // sum the priorities of the duplicated items
    // (items that are in both compartments of a rucksack)
    let prio_sum: u32 = groups.iter()
        .map(|rs| find_dupe_multi(rs))
        .map(|ch| char_to_prio(ch))
        .sum();
    println!("Part 2: {}", prio_sum);
}

fn main() {
    // get lines from stdin
    let lines: Vec<String> = io::stdin().lines().filter_map(|line| line.ok()).collect();

    part1(lines.clone());
    part2(lines.clone());
}
