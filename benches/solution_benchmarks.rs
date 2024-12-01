use aoc_24::Day;
use aoc_24::Solution;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("day 1 part 1 solution", |b| {
        b.iter(|| black_box(Day::<1>::solve_part_one))
    });

    c.bench_function("day 1 part 2 solution", |b| {
        b.iter(|| black_box(Day::<1>::solve_part_two))
    });
}

criterion_group!(benches, day1_benchmark);
criterion_main!(benches);
