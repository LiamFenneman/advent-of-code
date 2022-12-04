// Advent of Code 2022
// Liam Fenneman
//
// Day 4
//
// https://adventofcode.com/2022/day/4
#![allow(unused)]

type Range = std::ops::RangeInclusive<u32>;

/// Does range A fully contain range B.
fn range_contains(a: &Range, b: &Range) -> bool {
    // a range full contains another when it contains
    // both the start and end of the other range
    a.contains(&b.start()) && a.contains(&b.end())
}

/// Does range A overlap range B at all?
fn range_overlap(a: &Range, b: &Range) -> bool {
    // overlap when any value within range A is contained in range B
    for i in a.clone().into_iter() {
        if b.contains(&i) {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone)]
struct Pair(Range, Range);

impl Pair {
    /// Does one range (in the pair) overlap the entire other range?
    fn does_fully_contain(&self) -> bool {
        range_contains(&self.0, &self.1) || range_contains(&self.1, &self.0)
    }

    /// Do the two ranges overlap at all?
    fn does_overlap(&self) -> bool {
        range_overlap(&self.0, &self.1)
    }
}

fn part1(lines: Vec<String>) -> usize {
    lines.iter()
        // we have: "#-#,#-#" -- we want: ["#-#", "#-#"]
        .map(|line| line.split(",")
             .map(|s| s.to_owned())
             .collect::<Vec<String>>())
        // we have: ["#-#", "#-#"] -- we want ("#-#", "#-#")
        .map(|v| (v[0].clone(), v[1].clone()))
        // we have: ("#-#", "#-#") -- we want: ((#, #), (#, #))
        .map(|(a, b)| {
            let spt_a = a.split("-").collect::<Vec<&str>>();
            let spt_b = b.split("-").collect::<Vec<&str>>();

            let a_btm = spt_a[0].parse::<u32>().unwrap();
            let a_top = spt_a[1].parse::<u32>().unwrap();
            let b_btm = spt_b[0].parse::<u32>().unwrap();
            let b_top = spt_b[1].parse::<u32>().unwrap();

            ((a_btm, a_top), (b_btm, b_top))
        })
        // we have: ((#, #), (#, #)) -- we want: Pair(#..=#, #..=#)
        .map(|(a, b)| Pair(a.0..=a.1, b.0..=b.1))
        // filter the pairs that don't have on range fully contained
        .filter(|p| p.does_fully_contain())
        .count()
}

fn part2(lines: Vec<String>) -> usize {
    lines.iter()
        // we have: "#-#,#-#" -- we want: ["#-#", "#-#"]
        .map(|line| line.split(",")
             .map(|s| s.to_owned())
             .collect::<Vec<String>>())
        // we have: ["#-#", "#-#"] -- we want ("#-#", "#-#")
        .map(|v| (v[0].clone(), v[1].clone()))
        // we have: ("#-#", "#-#") -- we want: ((#, #), (#, #))
        .map(|(a, b)| {
            let spt_a = a.split("-").collect::<Vec<&str>>();
            let spt_b = b.split("-").collect::<Vec<&str>>();

            let a_btm = spt_a[0].parse::<u32>().unwrap();
            let a_top = spt_a[1].parse::<u32>().unwrap();
            let b_btm = spt_b[0].parse::<u32>().unwrap();
            let b_top = spt_b[1].parse::<u32>().unwrap();

            ((a_btm, a_top), (b_btm, b_top))
        })
        // we have: ((#, #), (#, #)) -- we want: Pair(#..=#, #..=#)
        .map(|(a, b)| Pair(a.0..=a.1, b.0..=b.1))
        // filter the pairs that don't have on range fully contained
        .filter(|p| p.does_overlap())
        .count()
}

fn main() {
    let lines = std::io::stdin().lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>();

    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}
