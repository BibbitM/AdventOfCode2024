use advent::day_09;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn day_09_part_one_benchmark(c: &mut Criterion) {
    let input = "inputs\\day_09.txt";
    let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_09.txt'");
    let input = input.chars().collect();

    let mut group = c.benchmark_group("day_09__part_one");

    group.bench_function("original", |b| b.iter(|| _ = day_09::calculate_filesystem_checksum(&input)));
}

fn day_09_part_two_benchmark_part_two(c: &mut Criterion) {
    let input = "inputs\\day_09.txt";
    let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_09.txt'");
    let input = input.chars().collect();

    let mut group = c.benchmark_group("day_09__part_two");

    group.bench_function("original", |b| b.iter(|| _ = day_09::calculate_filesystem_checksum_v2(&input)));

    group.bench_function("optimized", |b| b.iter(|| _ = day_09::calculate_filesystem_checksum_v2_optimized(&input)));
}

criterion_group!(benches, day_09_part_one_benchmark, day_09_part_two_benchmark_part_two);
criterion_main!(benches);
