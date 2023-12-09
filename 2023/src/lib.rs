#[macro_export]
macro_rules! setup {
    ($lit:literal, Example: $example:literal, Part1: $p1:literal, Part2: $p2:literal,) => {
        advent_of_code::setup! {
            $lit,
            Part1: $example = $p1,
            Part2: $example = $p2,
        }
    };

    ($lit:literal, Part1: $p1example:literal = $p1:literal, Part2: $p2example:literal = $p2:literal,) => {
        fn main() -> anyhow::Result<()> {
            let file = include_str!(concat!("../../input/", $lit, ".txt"));

            println!("Part 1: {}", part1(file));
            println!("Part 2: {}", part2(file));

            Ok(())
        }

        #[test]
        fn example_part1() {
            assert_eq!($p1, part1($p1example.trim()));
        }

        #[test]
        fn example_part2() {
            assert_eq!($p2, part2($p2example.trim()));
        }
    };
}

