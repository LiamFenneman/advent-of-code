// Advent of Code 2023
// Liam Fenneman

#[derive(Debug, Clone)]
struct Number {
    starts_at: usize,
    ends_at: usize,
    line_no: usize,
    value: u32,
}

impl Number {
    fn new(digit: char, pos: (usize, usize)) -> Self {
        Number {
            starts_at: pos.1,
            ends_at: pos.1,
            line_no: pos.0,
            value: digit.to_string().parse().unwrap(),
        }
    }

    fn push_digit(&mut self, digit: char, pos: usize) {
        assert!(digit.is_ascii_digit());

        self.ends_at = pos;

        let mut current = self.value.to_string();
        current.push(digit);
        self.value = current.parse().unwrap();
    }

    #[allow(clippy::ptr_arg)]
    fn is_part_number(&self, char_field: &Vec<Vec<char>>) -> bool {
        // check left
        if self.starts_at > 0 {
            if let Some(&x) = char_field[self.line_no].get(self.starts_at - 1) {
                if !x.is_ascii_digit() && x != '.' {
                    return true;
                }
            }
        }

        // check right
        if let Some(&x) = char_field[self.line_no].get(self.ends_at + 1) {
            if !x.is_ascii_digit() && x != '.' {
                return true;
            }
        }

        // check above
        if self.line_no > 0 && self.check(&char_field[self.line_no - 1]) {
            return true;
        }

        // check below
        if self.line_no + 1 < char_field.len()
            && self.check(&char_field[self.line_no + 1])
        {
            return true;
        }

        // otherwise false
        false
    }

    fn check(&self, line: &[char]) -> bool {
        let srt = self.starts_at.saturating_sub(1);
        let end = usize::min(self.ends_at.saturating_add(1), line.len() - 1);
        line[srt..=end]
            .iter()
            .any(|&ch| !ch.is_ascii_digit() && ch != '.')
    }

    /// is the given index within the bounds of this number
    fn check_collision(&self, line_no: usize, index: usize) -> bool {
        self.line_no == line_no
            && self.starts_at <= index
            && self.ends_at >= index
    }

    fn is_adjcacent_to(&self, line_no: usize, index: usize) -> bool {
        self.check_collision(line_no - 1, index - 1)
            || self.check_collision(line_no - 1, index)
            || self.check_collision(line_no - 1, index + 1)
            || self.check_collision(line_no, index - 1)
            || self.check_collision(line_no, index + 1)
            || self.check_collision(line_no + 1, index - 1)
            || self.check_collision(line_no + 1, index)
            || self.check_collision(line_no + 1, index + 1)
    }
}

fn find_numbers(input: &[&str]) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    for (l, _) in input.iter().enumerate() {
        let mut cur_num: Option<Number> = None;
        let chars = input[l].chars().collect::<Vec<_>>();
        for (c, &ch) in chars.iter().enumerate() {
            match ch {
                _ if ch.is_ascii_digit() => match cur_num {
                    None => cur_num = Some(Number::new(ch, (l, c))),
                    Some(ref mut n) => n.push_digit(ch, c),
                },
                _ => {
                    if let Some(ref n) = cur_num {
                        numbers.push(n.clone());
                        cur_num = None;
                    }
                }
            }
        }
        if let Some(ref n) = cur_num {
            numbers.push(n.clone());
            cur_num = None;
        }
        assert!(cur_num.is_none());
    }

    numbers
}

#[derive(Debug, Clone)]
struct Gear {
    line_no: usize,
    index: usize,
}

impl Gear {
    fn get_adjacent(&self, numbers: &[Number]) -> Vec<Number> {
        numbers
            .iter()
            .filter(|n| n.is_adjcacent_to(self.line_no, self.index))
            .cloned()
            .collect()
    }

    fn get_ratio(&self, numbers: &[Number]) -> u32 {
        let adjacent = self.get_adjacent(numbers);

        if adjacent.len() <= 1 {
            0
        } else {
            adjacent.iter().map(|n| n.value).product()
        }
    }
}

fn part1(input: &str) -> u32 {
    let input = input.lines().collect::<Vec<_>>();

    let numbers = find_numbers(&input);

    let char_field = input
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    numbers
        .iter()
        .filter(|n| n.is_part_number(&char_field))
        .map(|n| n.value)
        .sum()
}

fn part2(input: &str) -> u32 {
    let input = input.lines().collect::<Vec<_>>();

    // find all (potential) gears
    let mut gears: Vec<Gear> = Vec::new();
    for (l, s) in input.iter().enumerate() {
        let chars = s.chars().collect::<Vec<_>>();
        for (i, &ch) in chars.iter().enumerate() {
            if ch == '*' {
                gears.push(Gear {
                    line_no: l,
                    index: i,
                })
            }
        }
    }

    let numbers = find_numbers(&input);
    gears.iter().map(|g| g.get_ratio(&numbers)).sum()
}

advent_of_code::setup! {
    "day3",
    Example: r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
    ",
    Part1: 4361,
    Part2: 467835,
}
