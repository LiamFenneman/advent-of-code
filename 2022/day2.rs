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

fn calculate_round_score(opp: RPS, res: RPS) -> u32 {
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

fn part1(rounds: Vec<Round>) {
    // GOAL: What would your total score be if everything goes exactly according to your strategy guide?
    let score: u32 = rounds.iter()
        .map(|r| calculate_round_score(r.opp, r.res))
        .sum();
    println!("Total score: {}", score);
}

fn part2(_rounds: Vec<Round>) {

}

fn main() -> std::io::Result<()> {
    // get lines from stdin
    let lines: Vec<String> = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .collect();

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
    
    part1(rounds.clone());
    part2(rounds.clone());
    Ok(())
}
