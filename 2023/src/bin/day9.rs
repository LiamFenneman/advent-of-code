// Advent of Code 2023
// Liam Fenneman

use std::str::FromStr;

#[derive(Debug, Clone)]
struct History(Vec<i64>);

impl History {
    fn predict_end(&self) -> i64 {
        let diff_list = find_diff_list(self.0.clone(), vec![]);

        let index = diff_list.len() - 1;
        let incr = extrapolate_right(diff_list, index, 0);

        self.0.last().unwrap() + incr
    }

    fn predict_start(&self) -> i64 {
        let diff_list = find_diff_list(self.0.clone(), vec![]);

        let index = diff_list.len() - 1;
        let incr = extrapolate_left(diff_list, index, 0);

        self.0.first().unwrap() - incr
    }
}

fn find_diff_list(list: Vec<i64>, mut acc: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    if list.iter().all(|&n| n == 0) {
        return acc;
    }

    let mut diff = Vec::new();
    let mut prev = *list.first().unwrap();
    for n in list.into_iter().skip(1) {
        diff.push(n - prev);
        prev = n;
    }

    acc.push(diff.clone());

    find_diff_list(diff, acc)
}

fn extrapolate_right(diff_list: Vec<Vec<i64>>, index: usize, incr: i64) -> i64 {
    let list = diff_list.get(index).unwrap();

    let new_incr = list.last().unwrap() + incr;

    if index == 0 {
        return new_incr;
    }

    extrapolate_right(diff_list, index - 1, new_incr)
}

fn extrapolate_left(diff_list: Vec<Vec<i64>>, index: usize, incr: i64) -> i64 {
    let list = diff_list.get(index).unwrap();

    let new_incr = list.first().unwrap() - incr;

    if index == 0 {
        return new_incr;
    }

    extrapolate_left(diff_list, index - 1, new_incr)
}

impl FromStr for History {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(History(
            s.split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect(),
        ))
    }
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.parse::<History>().unwrap())
        .map(|h| h.predict_end())
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.parse::<History>().unwrap())
        .map(|h| h.predict_start())
        .sum()
}

advent_of_code::setup! {
    "day9",
    Example: r"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
    ",
    Part1: 114,
    Part2: 2,
}
