use crate::solution::Day;
use crate::solution::Solution;

mod solution;

macro_rules! solve_for_days {
    ($day:literal) => {
        Day::<$day>::solve();
    };
    ($day:literal, $($days:literal),+) => {
        Day::<$day>::solve();
        solve_for_days![$($days),+];
    }
}

fn main() {
    solve_for_days![1];
}
