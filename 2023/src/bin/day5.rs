// Advent of Code 2023
// Liam Fenneman

use std::str::FromStr;

use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug, Clone)]
struct Map(Vec<PartialRange>);

#[derive(Debug, Clone)]
struct PartialRange {
    dest: u64,
    src: u64,
    len: u64,
}

impl Almanac {
    #[inline]
    fn eval(&self, seed: u64) -> u64 {
        let mut seed = seed;

        for map in &self.maps {
            if let Some(range) = map.0.iter().find(|r| r.is_within(seed)) {
                seed = range.map(seed);
            }
        }

        seed
    }
}

impl PartialRange {
    #[inline]
    fn is_within(&self, seed: u64) -> bool {
        seed >= self.src && seed < (self.src + self.len)
    }

    #[inline]
    fn map(&self, seed: u64) -> u64 {
        seed - self.src + self.dest
    }
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections: Vec<_> = s.split("\n\n").collect();

        let seeds = sections[0]
            .split(' ')
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();

        let maps = sections[1..]
            .iter()
            .map(|s| s.parse::<Map>().unwrap())
            .collect();

        Ok(Almanac { seeds, maps })
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect()))
    }
}

impl FromStr for PartialRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((dest, rest)) = s.split_once(' ') else {
            anyhow::bail!("partial range dest parse failed");
        };
        let Some((src, len)) = rest.split_once(' ') else {
            anyhow::bail!("partial range dest parse failed");
        };
        Ok(PartialRange {
            dest: dest.parse()?,
            src: src.parse()?,
            len: len.parse()?,
        })
    }
}

fn part1(input: &str) -> u64 {
    let almanac: Almanac = input.parse().unwrap();

    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.eval(seed))
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let almanac: Almanac = input.parse().unwrap();

    assert!(almanac.seeds.len() % 2 == 0);

    let mut pairs = Vec::new();
    let mut i = 0;
    while i < almanac.seeds.len() {
        pairs.push((almanac.seeds[i], almanac.seeds[i + 1]));
        i += 2;
    }

    pairs
        .par_iter()
        .flat_map(|&(start, len)| start..(start + len))
        .map(|seed| almanac.eval(seed))
        .min()
        .unwrap()
}

advent_of_code::setup! {
    "day5",
    Example: r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
    ",
    Part1: 35,
    Part2: 46,
}
