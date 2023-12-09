// Advent of Code 2023
// Liam Fenneman

use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, Clone)]
struct Hand<C> {
    cards: Vec<C>,
    bid: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum CardPart1 {
    A,
    K,
    Q,
    J,
    T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum CardPart2 {
    A,
    K,
    Q,
    T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
    J,
}

impl Hand<CardPart1> {
    fn get_type(&self) -> HandType {
        let mut map: BTreeMap<CardPart1, u64> = BTreeMap::new();
        for card in &self.cards {
            if map.contains_key(card) {
                *map.get_mut(card).unwrap() += 1;
            } else {
                map.insert(*card, 1);
            }
        }

        if map.len() == 1 {
            return HandType::FiveOfAKind;
        }

        if map.iter().any(|(_, &v)| v == 4) {
            return HandType::FourOfAKind;
        }

        if map.iter().any(|(_, &v)| v == 3) && map.len() == 2 {
            return HandType::FullHouse;
        }

        if map.iter().any(|(_, &v)| v == 3) {
            return HandType::ThreeOfAKind;
        }

        if map.iter().filter(|(_, &v)| v == 2).count() == 2 {
            return HandType::TwoPair;
        }

        if map.iter().any(|(_, &v)| v == 2) {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

impl Hand<CardPart2> {
    fn get_type(&self) -> HandType {
        let mut map: BTreeMap<CardPart2, u64> = BTreeMap::new();
        for card in &self.cards {
            if map.contains_key(card) {
                *map.get_mut(card).unwrap() += 1;
            } else {
                map.insert(*card, 1);
            }
        }

        let n_joker = map.get(&CardPart2::J);
        let len = map.len();

        let any = |val| map.iter().any(|(_, &v)| v == val);
        let count = |val| map.iter().filter(|(_, &v)| v == val).count();

        let non_joker_any =
            |val| map.iter().any(|(&c, &v)| c != CardPart2::J && v == val);
        let joker_any =
            |val| any(val) || n_joker.is_some_and(|n| non_joker_any(val - n));

        if joker_any(5) {
            return HandType::FiveOfAKind;
        }

        if joker_any(4) {
            return HandType::FourOfAKind;
        }

        if joker_any(3) {
            if len == 2 || n_joker.is_some_and(|_| len == 3) {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        }

        if count(2) == 2 {
            return HandType::TwoPair;
        }

        if joker_any(2) {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

impl<C: Eq> PartialEq for Hand<C> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl<P: Eq> Eq for Hand<P> {}

impl PartialOrd for Hand<CardPart1> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<CardPart1> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        let cmp = self.get_type().cmp(&other.get_type());

        if cmp == Ordering::Equal {
            for (a, b) in self.cards.iter().zip(&other.cards) {
                let cmp = a.cmp(b);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
        }

        cmp
    }
}

impl PartialOrd for Hand<CardPart2> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<CardPart2> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        let cmp = self.get_type().cmp(&other.get_type());

        if cmp == Ordering::Equal {
            for (a, b) in self.cards.iter().zip(&other.cards) {
                let cmp = a.cmp(b);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
        }

        cmp
    }
}

impl<C> FromStr for Hand<C>
where
    C: TryFrom<char>,
    C: std::fmt::Debug,
    <C as std::convert::TryFrom<char>>::Error: std::fmt::Debug,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((cards, bid)) = s.split_once(' ') else {
            anyhow::bail!("failed to parse");
        };

        Ok(Hand {
            bid: bid.trim().parse()?,
            cards: cards.chars().map(|c| c.try_into().unwrap()).collect(),
        })
    }
}

impl TryFrom<char> for CardPart1 {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::_9,
            '8' => Self::_8,
            '7' => Self::_7,
            '6' => Self::_6,
            '5' => Self::_5,
            '4' => Self::_4,
            '3' => Self::_3,
            '2' => Self::_2,
            _ => anyhow::bail!("failed to parse card"),
        })
    }
}

impl TryFrom<char> for CardPart2 {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::_9,
            '8' => Self::_8,
            '7' => Self::_7,
            '6' => Self::_6,
            '5' => Self::_5,
            '4' => Self::_4,
            '3' => Self::_3,
            '2' => Self::_2,
            _ => anyhow::bail!("failed to parse card"),
        })
    }
}

fn part1(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| line.parse::<Hand<CardPart1>>().unwrap())
        .collect::<Vec<_>>();

    hands.sort();
    hands.reverse();

    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx as u64 + 1) * hand.bid)
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .inspect(|line| eprintln!("{:?}", line))
        .map(|line| line.parse::<Hand<CardPart2>>().unwrap())
        .collect::<Vec<_>>();

    hands.sort();
    hands.reverse();

    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx as u64 + 1) * hand.bid)
        .sum()
}

advent_of_code::setup! {
    "day7",
    Example: r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
    ",
    Part1: 6440,
    Part2: 5905,
}
