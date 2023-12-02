// Advent of Code 2023
// Liam Fenneman

use std::{collections::HashSet, str::FromStr};

struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

struct CubeSet {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl Game {
    /// is the game possible to occur if there are 12 reds, 13 greens, and 14
    /// blues?
    pub fn is_possible(&self) -> bool {
        !self.sets.iter().any(|set| {
            set.red.is_some_and(|n| n > 12)
                || set.green.is_some_and(|n| n > 13)
                || set.blue.is_some_and(|n| n > 14)
        })
    }

    /// fewest number of cubes of each color that could have been in the bag to
    /// make the game possible. The power of a set of cubes is equal to the
    /// numbers of red, green, and blue cubes multiplied together
    pub fn power(&self) -> u32 {
        // find max shown red, green, blue
        let mut max_red: Option<u32> = None;
        let mut max_green: Option<u32> = None;
        let mut max_blue: Option<u32> = None;

        for set in &self.sets {
            match max_red {
                Some(r) if set.red.is_some_and(|n| n > r) => {
                    max_red = set.red;
                }
                None => {
                    // set max red to whatever the current set has since if the
                    // value is Some it is correct otherwise it will remain None
                    max_red = set.red
                }
                // do nothing otherwise
                _ => {}
            }
            match max_green {
                Some(g) if set.green.is_some_and(|n| n > g) => {
                    max_green = set.green;
                }
                None => {
                    // set max green to whatever the current set has since if the
                    // value is Some it is correct otherwise it will remain None
                    max_green = set.green
                }
                // do nothing otherwise
                _ => {}
            }
            match max_blue {
                Some(b) if set.blue.is_some_and(|n| n > b) => {
                    max_blue = set.blue;
                }
                None => {
                    // set max blue to whatever the current set has since if the
                    // value is Some it is correct otherwise it will remain None
                    max_blue = set.blue
                }
                // do nothing otherwise
                _ => {}
            }
        }

        max_red.unwrap() * max_green.unwrap() * max_blue.unwrap()
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((game, sets)) = s.split_once(':') else {
            anyhow::bail!("could not find game");
        };

        // get the id from the "Game <number>" part of the `game` string
        let id = game.split_at(4).1.trim().parse::<u32>().unwrap();

        // split with ';' as delimiter and parse into a cube set
        let sets = sets
            .split(';')
            .map(|s| s.parse::<CubeSet>().unwrap())
            .collect::<Vec<_>>();

        Ok(Game { id, sets })
    }
}

impl FromStr for CubeSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = CubeSet {
            red: None,
            green: None,
            blue: None,
        };

        s.split(',').for_each(|s| {
            let (count, colour) = s.trim().split_once(' ').unwrap();
            let count = count.parse::<u32>().unwrap();
            match colour {
                "red" => set.red = Some(count),
                "green" => set.green = Some(count),
                "blue" => set.blue = Some(count),
                _ => {}
            }
        });

        Ok(set)
    }
}

fn part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| line.parse::<Game>().unwrap())
        .filter(|g| g.is_possible())
        .map(|g| g.id)
        .sum()
}

fn part2(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| line.parse::<Game>().unwrap())
        .map(|g| g.power())
        .sum()
}

fn main() -> anyhow::Result<()> {
    let file = std::fs::read_to_string("input/day2.txt")?;
    let input = file
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
