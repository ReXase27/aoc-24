use crate::solution::{Day, Solution};

#[derive(Debug)]
struct Level {
    reports: Vec<u32>,
}

impl Level {
    pub fn is_safe(&self, min: u32, max: u32) -> bool {
        let is_increasing = { self.reports.first().unwrap() < self.reports.get(1).unwrap() };

        self.reports
            .windows(2)
            .all(|w| (min..=max).contains(&w[0].abs_diff(w[1])) && (w[0] < w[1]) == is_increasing)
    }

    pub fn is_partially_safe(&self, min: u32, max: u32) -> bool {
        let is_increasing = { self.reports.first().unwrap() < self.reports.get(1).unwrap() };

        self.reports
            .windows(2)
            .filter(|w| {
                !((min..=max).contains(&w[0].abs_diff(w[1])) && (w[0] < w[1]) == is_increasing)
            })
            .count()
            <= 1
    }
}

impl From<&str> for Level {
    fn from(input: &str) -> Self {
        let reports = input
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();

        Level { reports }
    }
}

impl Solution for Day<2> {
    type Sample = u32;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_sample(input: &str) -> Self::Sample {
        let levels: Vec<Level> = input.lines().map(Level::from).collect();

        levels.iter().filter(|l| l.is_safe(1, 3)).count() as u32
    }

    fn solve_part_one(input: &str) -> Self::PartOne {
        let levels: Vec<Level> = input.lines().map(Level::from).collect();

        levels.iter().filter(|l| l.is_safe(1, 3)).count() as u32
    }

    fn solve_part_two(input: &str) -> Self::PartTwo {
        let levels: Vec<Level> = input.lines().map(Level::from).collect();

        levels.iter().filter(|l| l.is_partially_safe(1, 3)).count() as u32
    }
}
