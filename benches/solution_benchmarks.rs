use aoc_24::Day;
use aoc_24::Solution;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("d1-p1", |b| b.iter(|| black_box(Day::<1>::solve_part_one)));

    c.bench_function("d1-p2", |b| b.iter(|| black_box(Day::<1>::solve_part_two)));
}

fn day2_benchmark(c: &mut Criterion) {
    c.bench_function("d2-p1", |b| b.iter(|| black_box(Day::<2>::solve_part_one)));

    c.bench_function("d2-p2", |b| b.iter(|| black_box(Day::<2>::solve_part_two)));
}

criterion_group!(benches, day1_benchmark, day2_benchmark);
criterion_main!(benches);
