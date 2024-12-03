use crate::day::{Day, Solution};
use std::collections::{HashMap, HashSet};

impl Solution for Day<1> {
    type Sample = u32;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_sample(input: &str) -> Self::Sample {
        Self::solve_part_one(input)
    }

    fn solve_part_one(input: &str) -> Self::PartOne {
        let sorted_pairs = create_sorted_pairs(input);

        sorted_pairs.iter().map(|(l, r)| l.abs_diff(*r)).sum()
    }

    fn solve_part_two(input: &str) -> Self::PartTwo {
        let pairs = create_pairs(input);
        let uniques = pairs.iter().map(|p| p.0).collect::<HashSet<u32>>();
        let mut appearances: HashMap<u32, u32> = HashMap::new();

        for unique in uniques {
            let count = pairs.iter().filter(|p| p.1 == unique).count();
            appearances.insert(unique, count as u32);
        }

        appearances.iter().map(|(k, v)| k * v).sum()
    }
}

fn create_sorted_pairs(input: &str) -> Vec<(u32, u32)> {
    let line_count = input.lines().count();

    let (mut left, mut right) = split_into_columns(input, line_count);

    left.sort();
    right.sort();

    let mut pairs: Vec<(u32, u32)> = Vec::with_capacity(left.len());
    while let Some(l) = left.pop() {
        pairs.push((l, right.pop().unwrap()));
    }

    pairs
}

fn create_pairs(input: &str) -> Vec<(u32, u32)> {
    let line_count = input.lines().count();

    let (mut left, mut right) = split_into_columns(input, line_count);

    let mut pairs: Vec<(u32, u32)> = Vec::with_capacity(left.len());
    while let Some(l) = left.pop() {
        pairs.push((l, right.pop().unwrap()));
    }

    pairs
}

fn split_into_columns(input: &str, line_count: usize) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::with_capacity(line_count);
    let mut right: Vec<u32> = Vec::with_capacity(line_count);

    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        let left_num = l.trim().parse::<u32>().unwrap();
        let right_num = r.trim().parse::<u32>().unwrap();
        left.push(left_num);
        right.push(right_num);
    }

    (left, right)
}
