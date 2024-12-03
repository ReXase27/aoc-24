use crate::day::{Day, Solution};
use regex::Regex;

const FULL_EXPR_REGEX: &str = r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)";
const MUL_EXPR_REGEX: &str = r"mul\((\d+),(\d+)\)";

impl Solution for Day<3> {
    type Sample = u32;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_sample(input: &str) -> Self::Sample {
        Self::solve_part_one(input)
    }

    fn solve_part_one(input: &str) -> Self::PartOne {
        let accumulator = do_mul(input);
        accumulator
    }

    fn solve_part_two(input: &str) -> Self::PartTwo {
        let full_expr = Regex::new(FULL_EXPR_REGEX).unwrap();
        let mul_expr = Regex::new(MUL_EXPR_REGEX).unwrap();
        let mut accumulator = 0;
        let mut is_enabled = true;

        for match_obj in full_expr.find_iter(input) {
            match match_obj.as_str() {
                "do()" => is_enabled = true,
                "don't()" => is_enabled = false,
                _ if is_enabled => {
                    mul_and_acc(&match_obj, &mul_expr, &mut accumulator);
                }
                _ => {}
            }
        }
        accumulator
    }
}

fn do_mul(input: &str) -> u32 {
    let mul_expr = Regex::new(MUL_EXPR_REGEX).unwrap();
    let mut acc = 0;
    for (_, [left, right]) in mul_expr.captures_iter(input).map(|c| c.extract()) {
        acc += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
    }
    acc
}

fn mul_and_acc(re_match: &regex::Match, mul_expr: &Regex, acc: &mut u32) {
    for (_, [left, right]) in mul_expr
        .captures_iter(re_match.as_str())
        .map(|c| c.extract())
    {
        *acc += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
    }
}
