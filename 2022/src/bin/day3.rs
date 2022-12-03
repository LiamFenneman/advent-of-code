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

fn main() {
    // get lines from stdin
    let lines: Vec<String> = io::stdin().lines().filter_map(|line| line.ok()).collect();

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
