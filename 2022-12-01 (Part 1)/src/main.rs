// Advent of Code 2022
// Liam Fenneman
//
// Day 1 -- Part 1
//
// https://adventofcode.com/2022/day/1

#[derive(Debug)]
struct Elf {
    id: u32,
    calories: u32,
}

fn main() -> std::io::Result<()> {
    // get lines from stdin
    let lines: Vec<String> = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    // parse lines into numbers
    let mut elves: Vec<Elf> = Vec::new();

    let mut elf = Elf { id: 0, calories: 0 };
    // loop over all lines
    for line in lines {
        // if the line is not empty, parse the line for the current elf
        if let Ok(calories) = line.parse::<u32>() {
            elf.calories += calories;
        } else {
            // otherwise, add the elf to the list and start a new elf
            let i = elf.id;
            elves.push(elf);
            elf = Elf {
                id: i + 1,
                calories: 0,
            };
        }
    }

    // GOAL: Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
    let Some(elf) = elves.iter().max_by(|a, b| a.calories.cmp(&b.calories)) else {
        println!("No elves found!");
        return Ok(());
    };

    println!("Elf {} has the most calories: {}", elf.id, elf.calories);
    Ok(())
}
