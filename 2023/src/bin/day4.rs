// Advent of Code 2023
// Liam Fenneman

use std::str::FromStr;

#[derive(Debug, Clone)]
struct Card {
    instances: u32,
    winning_set: CardSet,
    our_set: CardSet,
}

#[derive(Debug, Clone)]
struct CardSet(Vec<u32>);

impl Card {
    fn calc_points(&self) -> u32 {
        let count = self.count_matches();

        if count > 0 {
            2u32.pow(count - 1)
        } else {
            0
        }
    }

    fn count_matches(&self) -> u32 {
        self.winning_set
            .0
            .iter()
            .filter(|&&n| self.our_set.0.iter().any(|&o| n == o))
            .count() as u32
    }

    fn increment_instances(&mut self, by: u32) {
        self.instances += by;
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((_, rest)) = s.split_once(':') else {
            anyhow::bail!("couldn't split on :");
        };
        let Some((winning, ours)) = rest.split_once('|') else {
            anyhow::bail!("couldn't split on |")
        };

        Ok(Card {
            instances: 1,
            winning_set: winning.parse()?,
            our_set: ours.parse()?,
        })
    }
}

impl FromStr for CardSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CardSet(
            s.split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect(),
        ))
    }
}

fn part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| line.parse::<Card>().unwrap())
        .map(|c| c.calc_points())
        .sum()
}

fn part2(input: &[&str]) -> u32 {
    let mut cards: Vec<Card> = input
        .iter()
        .map(|line| line.parse::<Card>().unwrap())
        .collect();

    let mut total_cards = 0;

    for i in 0..cards.len() {
        let card = cards[i].clone();
        let count = cards[i].count_matches();
        for next in (i + 1)..=(i + count as usize) {
            if let Some(next_card) = cards.get_mut(next) {
                next_card.increment_instances(card.instances);
            }
        }
        total_cards += card.instances;
    }

    total_cards
}

fn main() -> anyhow::Result<()> {
    let file = std::fs::read_to_string("input/day4.txt")?;
    let input = file
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[allow(dead_code)]
const FILE_EXAMPLE: &str = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

#[test]
fn example_part1() {
    let input = FILE_EXAMPLE
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    assert_eq!(13, part1(&input));
}

#[test]
fn example_part2() {
    let input = FILE_EXAMPLE
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    assert_eq!(30, part2(&input));
}
