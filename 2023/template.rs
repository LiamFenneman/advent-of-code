// Advent of Code 2023
// Liam Fenneman

fn part1(input: &str) -> u64 {
    todo!()
}

fn part2(input: &str) -> u64 {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input/dayN.txt");

    println!("Part 1: {}", part1(file));
    println!("Part 2: {}", part2(file));

    Ok(())
}

#[allow(dead_code)]
const FILE_EXAMPLE: &str = r"
";

#[test]
fn example_part1() {
    assert_eq!(0, part1(FILE_EXAMPLE));
}

#[test]
fn example_part2() {
    assert_eq!(0, part2(FILE_EXAMPLE));
}
