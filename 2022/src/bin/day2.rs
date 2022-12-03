// Advent of Code 2022
// Liam Fenneman
//
// Day 2
//
// https://adventofcode.com/2022/day/2

// Rock beats Scissors, Scissors beats Paper, Paper beats Rock
//
// Input:
// Opponent -> A = Rock, B = Paper, C = Scissors
// Response -> X = Rock, Y = Paper, Z = Scissors
//
// Total score = sum of scores for each round
// Score for a round = the *shape* + outcome
// Shape pts -> Rock = 1, Paper = 2, Scissors = 3
// Outcome pts -> Lose = 0, Draw = 3, Win = 6

#[derive(Debug, Clone, Copy)]
struct Round {
    opp: RPS,
    res: RPS,
}

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn calc_round_score(opp: RPS, res: RPS) -> u32 {
    use RPS::*;
    let mut score = 0;

    // add shape points
    score += match res {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    // add outcome points
    score += match (opp, res) {
        // loss
        (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 0,
        // draw
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        // win
        (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => 6,
    };

    score
}

fn calc_total_score(rounds: Vec<Round>) -> u32 {
    rounds.iter()
        .map(|r| calc_round_score(r.opp, r.res))
        .sum()
}

fn part1(lines: Vec<String>) {
    // each round is a single line
    // -> map each line to opponent RPS and response RPS
    let rounds: Vec<Round> = lines.iter()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|v| {
            let opp = match v[0] {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissors,
                _ => panic!("invalid input")
            };
            let res = match v[1] {
                "X" => RPS::Rock,
                "Y" => RPS::Paper,
                "Z" => RPS::Scissors,
                _ => panic!("invalid input")
            };
            Round { opp, res }
        })
        .collect();

    // GOAL: What would your total score be if everything goes exactly according to your strategy guide?
    let score = calc_total_score(rounds);
    println!("Part 1: {}", score);
}

fn part2(lines: Vec<String>) {
    use RPS::*;

    // each round is a single line
    // -> map each line to opponent RPS and response RPS
    let rounds: Vec<Round> = lines.iter()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|v| {
            let opp = match v[0] {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => panic!("invalid input")
            };
            // X = Lose, Y = Draw, Z = Win
            let res = match v[1] {
                "X" => match opp {
                    Rock => Scissors,
                    Paper => Rock,
                    Scissors => Paper,
                },
                "Y" => opp, // draw means that the two shapes are equal
                "Z" => match opp {
                    Rock => Paper,
                    Paper => Scissors,
                    Scissors => Rock,
                },
                _ => panic!("invalid input")
            };
            Round { opp, res }
        })
        .collect();

        // GOAL: Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
    let score = calc_total_score(rounds);
    println!("Part 2: {}", score);
}

fn main() -> std::io::Result<()> {
    // get lines from stdin
    let lines: Vec<String> = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .collect();
    
    part1(lines.clone());
    part2(lines.clone());
    Ok(())
}
