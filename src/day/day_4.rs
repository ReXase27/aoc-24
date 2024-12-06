use crate::day::{Day, Solution};

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (1, -1),
    (-1, 0),
    (0, -1),
    (-1, -1),
    (-1, 1),
];

const TARGET_WORD: [char; 4] = ['X', 'M', 'A', 'S'];

impl Solution for Day<4> {
    type Sample = u32;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_sample(input: &str) -> Self::Sample {
        Self::solve_part_one(input)
    }

    fn solve_part_one(input: &str) -> Self::PartOne {
        let matrix = parse_input_to_matrix(input);
        let num_rows = matrix.len();
        let num_cols = matrix[0].len();
        let mut acc = 0;

        for direction in &DIRECTIONS {
            for i in 0..num_rows {
                for j in 0..num_cols {
                    if check_word(&matrix, num_rows, num_cols, i, j, direction, &TARGET_WORD) {
                        acc += 1;
                    }
                }
            }
        }
        acc as u32
    }

    fn solve_part_two(input: &str) -> Self::PartTwo {
        let matrix = parse_input_to_matrix(input);
        let mut acc = 0;

        for r in 1..matrix.len() - 1 {
            for c in 1..matrix[0].len() - 1 {
                if matrix[r][c] != 'A' {
                    continue;
                }
                let corners = [
                    matrix[r - 1][c - 1],
                    matrix[r - 1][c + 1],
                    matrix[r + 1][c + 1],
                    matrix[r + 1][c - 1],
                ];
                let s = corners.iter().collect::<String>();
                if ["MMSS", "MSSM", "SSMM", "SMMS"].contains(&&**&s) {
                    acc += 1;
                }
            }
        }
        acc as u32
    }
}

fn parse_input_to_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn check_word(
    matrix: &Vec<Vec<char>>,
    num_rows: usize,
    num_cols: usize,
    start_row: usize,
    start_col: usize,
    direction: &(i32, i32),
    word: &[char],
) -> bool {
    let (dr, dc) = *direction;
    for k in 0..word.len() {
        let r = start_row as i32 + k as i32 * dr;
        let c = start_col as i32 + k as i32 * dc;
        if r < 0 || c < 0 || r >= num_rows as i32 || c >= num_cols as i32 {
            return false;
        }
        if matrix[r as usize][c as usize] != word[k] {
            return false;
        }
    }
    true
}
