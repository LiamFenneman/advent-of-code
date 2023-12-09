// Advent of Code 2023
// Liam Fenneman

use std::str::FromStr;

struct CalibrationValue(u32);

impl From<(char, char)> for CalibrationValue {
    fn from(value: (char, char)) -> Self {
        // invariant: both chars are ascii digits
        // reason: will be concat into a string then parsed into a u32
        assert!(value.0.is_ascii_digit() && value.1.is_ascii_digit());

        let mut s = String::new();
        s.push(value.0);
        s.push(value.1);

        CalibrationValue(s.parse().expect("invariant means `s` is a valid u32"))
    }
}

impl FromStr for CalibrationValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for c in s.chars() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    // if `first` is None, then it *must* be the first digit
                    first = Some(c);
                }

                // we should always set the value of `last` since the
                // correct value will always be set last.
                last = Some(c);
            }
        }

        match (first, last) {
            (Some(f), Some(l)) => Ok((f, l).into()),
            _ => {
                dbg!(s);
                Err(anyhow::anyhow!("`first` and `last` are not set. Are there any numbers in the string?"))
            }
        }
    }
}

fn part1(input: &str) -> u32 {
    // parse each line into a calibration value then sum it
    input
        .lines()
        .map(|line| line.parse::<CalibrationValue>().unwrap().0)
        .sum()
}

fn part2(input: &str) -> u32 {
    // parse each line into a calibration value then sum it
    input
        .lines()
        // preprocess the line to replace spelled-out number (e.g. "one") with
        // the corresponding digit
        // NOTE: this is done left to right, **NOT** in numerical order
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let mut new_chars: Vec<char> = Vec::with_capacity(chars.len());

            let mut i = 0;
            while i < chars.len() {
                // match over different combinations of spelled out numbers
                match (
                    chars.get(i),
                    chars.get(i + 1),
                    chars.get(i + 2),
                    chars.get(i + 3),
                    chars.get(i + 4),
                ) {
                    (Some('o'), Some('n'), Some('e'), _, _) => {
                        new_chars.push('1');
                        i += 3;
                    }
                    (Some('t'), Some('w'), Some('o'), _, _) => {
                        new_chars.push('2');
                        i += 3;
                    }
                    (Some('t'), Some('h'), Some('r'), Some('e'), Some('e')) => {
                        new_chars.push('3');
                        i += 5;
                    }
                    (Some('f'), Some('o'), Some('u'), Some('r'), _) => {
                        new_chars.push('4');
                        i += 4;
                    }
                    (Some('f'), Some('i'), Some('v'), Some('e'), _) => {
                        new_chars.push('5');
                        i += 5;
                    }
                    (Some('s'), Some('i'), Some('x'), _, _) => {
                        new_chars.push('6');
                        i += 3;
                    }
                    (Some('s'), Some('e'), Some('v'), Some('e'), Some('n')) => {
                        new_chars.push('7');
                        i += 5;
                    }
                    (Some('e'), Some('i'), Some('g'), Some('h'), Some('t')) => {
                        new_chars.push('8');
                        i += 5;
                    }
                    (Some('n'), Some('i'), Some('n'), Some('e'), _) => {
                        new_chars.push('9');
                        i += 4;
                    }
                    (Some(&c), _, _, _, _) => {
                        new_chars.push(c);
                        i += 1;
                    }
                    _ => unreachable!("index out of bounds?"),
                }
            }

            new_chars.iter().collect::<String>()
        })
        .map(|line| line.parse::<CalibrationValue>().unwrap().0)
        .sum()
}

advent_of_code::setup! {
    "day1",
    Part1: r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
    " = 142,
    Part2: r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
    " = 281,
}
