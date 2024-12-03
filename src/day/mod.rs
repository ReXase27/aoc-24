mod day_1;
mod day_2;
mod day_3;

use std::fmt::Display;
use std::fs;

pub struct Day<const DAY_NUMBER: u8>;

pub trait AdventDay {
    const DAY_NUMBER: u8;

    fn day_name() -> String;
}

impl<const DAY_NUMBER: u8> AdventDay for Day<DAY_NUMBER> {
    const DAY_NUMBER: u8 = DAY_NUMBER;

    fn day_name() -> String {
        format!("day-{}", DAY_NUMBER)
    }
}

pub trait Solution: AdventDay {
    type Sample: Display;
    type PartOne: Display;
    type PartTwo: Display;

    fn solve_sample(input: &str) -> Self::Sample;
    fn solve_part_one(input: &str) -> Self::PartOne;
    fn solve_part_two(input: &str) -> Self::PartTwo;

    fn solve() {
        let sample_path = format!("days/{}/sample.txt", Self::day_name());
        let input_path = format!("days/{}/input.txt", Self::day_name());
        let sample_data = fs::read_to_string(&sample_path).unwrap_or_else(|_| {
            panic!(
                "there was no sample file for day {} with path {}",
                Self::day_name(),
                sample_path
            )
        });
        let data = fs::read_to_string(&input_path).unwrap_or_else(|_| {
            panic!(
                "there was no input file for day {} with path {}",
                Self::day_name(),
                input_path
            )
        });

        println!("Day {}: ", Self::DAY_NUMBER);
        println!("Sample day: {}", Self::solve_sample(&sample_data));
        println!("Part one day: {}", Self::solve_part_one(&data));
        println!("Part two day: {}", Self::solve_part_two(&data));
        println!();
    }
}
