use advent::day_03;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn day_06_benchmark_parse(c: &mut Criterion) {
    let input = "inputs\\day_03.txt";
    let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_03.txt'");

    let mut group = c.benchmark_group("day_03__parse");

    group.bench_function("original", |b| b.iter(|| day_03::sum_instructions(&day_03::parse_instructions(&input))));

    group.bench_function("sum", |b| b.iter(|| day_03::parse_instructions_and_sum(&input)));

    group.bench_function("no_regex", |b| b.iter(|| day_03::sum_instructions(&day_03::parse_instructions_no_regex(&input))));

    group.bench_function("no_regex_and_sum", |b| b.iter(|| day_03::parse_instructions_no_regex_and_sum(&input)));
}

fn day_06_benchmark_parse_with_do(c: &mut Criterion) {
    let input = "inputs\\day_03.txt";
    let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_03.txt'");

    let mut group = c.benchmark_group("day_03__parse_with_do");

    group.bench_function("original", |b| b.iter(|| day_03::sum_instructions(&day_03::parse_instructions_with_do(&input))));

    group.bench_function("sum", |b| b.iter(|| day_03::parse_instructions_with_do_and_sum(&input)));

    group.bench_function("no_regex", |b| b.iter(|| day_03::sum_instructions(&day_03::parse_instructions_with_do_no_regex(&input))));

    group.bench_function("no_regex_and_sum", |b| b.iter(|| day_03::parse_instructions_with_do_no_regex_and_sum(&input)));
}

criterion_group!(benches, day_06_benchmark_parse, day_06_benchmark_parse_with_do);
criterion_main!(benches);
