use crate::{Day, Solution};
use std::cmp::Ordering;
use std::ops::RangeInclusive;

impl Solution for Day<5> {
    type Sample = u32;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_sample(input: &str) -> Self::Sample {
        Self::solve_part_one(input)
    }

    fn solve_part_one(input: &str) -> Self::PartOne {
        let (rules, updates) = parse_input(input);

        updates
            .into_iter()
            .filter(|update| {
                update
                    .windows(2)
                    .all(|window| rules.iter().any(|rule| *rule == (window[0]..=window[1])))
            })
            .map(|update| update[update.len() / 2])
            .sum()
    }

    fn solve_part_two(input: &str) -> Self::PartTwo {
        let (rules, updates) = parse_input(input);

        let mut filtered_updates: Vec<_> = updates
            .into_iter()
            .filter(|update| {
                update
                    .windows(2)
                    .any(|window| rules.iter().all(|rule| *rule != (window[0]..=window[1])))
            })
            .collect();

        filtered_updates.iter_mut().for_each(|update| {
            update.sort_by(|&a, &b| compare(a, b, &rules).unwrap_or(Ordering::Equal));
        });

        filtered_updates
            .into_iter()
            .map(|update| update[update.len() / 2])
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u32>>, Vec<Vec<u32>>) {
    let (rules_section, updates_section) = input.split_once("\n\r").unwrap();
    let rules = parse_rules(rules_section);
    let updates = parse_updates(updates_section);
    (rules, updates)
}

fn parse_rules(input: &str) -> Vec<RangeInclusive<u32>> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('|').unwrap();
            let start = start.trim().parse().unwrap();
            let end = end.trim().parse().unwrap();
            start..=end
        })
        .collect()
}

fn parse_updates(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

fn compare(a: u32, b: u32, rules: &[RangeInclusive<u32>]) -> Option<Ordering> {
    if rules.iter().any(|rule| rule == &(a..=b)) {
        Some(Ordering::Equal)
    } else {
        Some(Ordering::Less)
    }
}
