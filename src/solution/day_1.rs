use crate::solution::{Day, Solution};
use std::collections::{HashMap, HashSet};

impl Solution for Day<1> {
    type Sample = u32;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_sample(input: &str) -> Self::Sample {
        let pairs = create_pairs(input);

        let mut acc: u32 = 0;

        for pair in pairs.iter() {
            if pair.0 <= pair.1 {
                acc += pair.1 - pair.0;
            } else {
                acc += pair.0 + pair.1;
            }
        }

        acc
    }

    fn solve_part_one(input: &str) -> Self::PartOne {
        let pairs = create_pairs(input);

        let mut acc: u32 = 0;

        for pair in pairs.iter() {
            let (m, mx) = { (pair.0.min(pair.1), pair.0.max(pair.1)) };
            if m == mx {
                continue;
            }
            acc += mx - m;
        }

        acc
    }

    fn solve_part_two(input: &str) -> Self::PartTwo {
        let pairs = create_pairs_unsorted(input);
        let uniques = pairs.iter().map(|p| p.0).collect::<HashSet<u32>>();
        let mut appearances: HashMap<u32, u32> = HashMap::new();

        for unique in uniques {
            let count = pairs.iter().filter(|p| p.1 == unique).count();
            appearances.insert(unique, count as u32);
        }

        appearances.iter().map(|(k, v)| k * v).sum()
    }
}

fn create_pairs(input: &str) -> Vec<(u32, u32)> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    populate_arrays(input, &mut left, &mut right);

    left.sort();
    right.sort();

    let mut pairs: Vec<(u32, u32)> = Vec::with_capacity(left.len());
    while let Some(l) = left.pop() {
        pairs.push((l, right.pop().unwrap()));
    }

    pairs
}

fn create_pairs_unsorted(input: &str) -> Vec<(u32, u32)> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    populate_arrays(input, &mut left, &mut right);

    let mut pairs: Vec<(u32, u32)> = Vec::with_capacity(left.len());
    while let Some(l) = left.pop() {
        pairs.push((l, right.pop().unwrap()));
    }

    pairs
}

fn populate_arrays(input: &str, left: &mut Vec<u32>, right: &mut Vec<u32>) {
    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        let left_num = l.trim().parse::<u32>().unwrap();
        let right_num = r.trim().parse::<u32>().unwrap();
        left.push(left_num);
        right.push(right_num);
    }
}
