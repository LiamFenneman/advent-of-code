// Advent of Code 2023
// Liam Fenneman

use std::str::FromStr;

use anyhow::Context;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

struct Races(Vec<Race>);

const ACCEL: u64 = 1;

impl FromStr for Races {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (times, distances) = s.trim().split_once('\n').context("")?;
        let times = times.split_ascii_whitespace();
        let distances = distances.split_ascii_whitespace();

        let races = times
            .zip(distances)
            .skip(1)
            .map(|(t, d)| {
                (t.parse::<u64>().unwrap(), d.parse::<u64>().unwrap())
            })
            .map(|(time, distance)| Race { time, distance })
            .collect::<Vec<_>>();

        Ok(Races(races))
    }
}

impl Race {
    fn count_wins(&self) -> u64 {
        let mut count = 0;

        for i in 0..=self.time {
            let time_accel = self.time - i;
            let distance = ACCEL * time_accel * i;
            if distance > self.distance {
                count += 1;
            }
        }

        count
    }
}

fn part1(input: &str) -> u64 {
    let races: Races = input.parse().unwrap();
    races.0.iter().map(|r| r.count_wins()).product()
}

fn part2(input: &str) -> u64 {
    let (times, distances) = input.trim().split_once('\n').context("").unwrap();
    let time = times
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = distances
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    Race { time, distance }.count_wins()
}

advent_of_code::setup! {
    "day6",
    Example: r"
Time:      7  15   30
Distance:  9  40  200
    ",
    Part1: 288,
    Part2: 71503,
}
