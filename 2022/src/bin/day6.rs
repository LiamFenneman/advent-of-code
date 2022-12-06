// Advent of Code 2022
// Liam Fenneman
//
// Day 6
//
// https://adventofcode.com/2022/day/6
#![allow(unused)]

fn any_chars_equal(window: Vec<char>) -> bool {
    for (ai, a) in window.iter().enumerate() {
        for (bi, b) in window.iter().enumerate() {
            // dont compare the same index of the window
            if ai == bi { continue; }
            if a == b { return true; }
        }
    }

    false
}

fn part1(chars: Vec<char>) -> usize {
    // to process we start at index 4 since this is the first time that
    // we are able to have a marker (marker is 4 unqiue chars in a row)

    for (i, &c) in chars.iter().enumerate().skip(3) {
        let window = vec![chars[i-3], chars[i-2], chars[i-1], c];

        // are all chars unique?
        // no? continue looping
        if any_chars_equal(window) {
            continue;
        }

        // otherwise
        return i+1;
    }

    usize::MAX
}

fn part2(chars: Vec<char>) -> usize {
    // now the window must be of size 14
    for (i, &c) in chars.iter().enumerate().skip(13) {
        let window = &chars[(i-13)..=i];

        // are all chars unique?
        // no? continue looping
        if any_chars_equal(window.to_vec()) {
            continue;
        }

        // otherwise
        return i+1;
    }

    usize::MAX
}

fn main() {
    let chars = std::io::stdin()
        .lines()
        .filter_map(|l| l.ok())
        .flat_map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ret = part1(chars.clone());
    println!("Part 1: {}", ret);

    let ret = part2(chars.clone());
    println!("Part 2: {}", ret);
}
