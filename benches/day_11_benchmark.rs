use advent::day_11;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn day_11_part_one_benchmark(c: &mut Criterion) {
    let input = "inputs\\day_11.txt";
    let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_11.txt'");
    let stones = day_11::parse_stones(&input);

    let mut group = c.benchmark_group("day_11__part_one");

    group.bench_function("original", |b| b.iter(|| _ = day_11::calculate_number_of_stones_after_blinks(&stones, 25)));

    group.bench_function("cached", |b| b.iter(|| _ = day_11::calculate_number_of_stones_after_blinks_cached(&stones, 25)));
}

fn day_11_part_two_benchmark_part_two(c: &mut Criterion) {
    let input = "inputs\\day_11.txt";
    let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_11.txt'");
    let stones = day_11::parse_stones(&input);

    let mut group = c.benchmark_group("day_11__part_two");

    group.bench_function("cached", |b| b.iter(|| _ = day_11::calculate_number_of_stones_after_blinks_cached(&stones, 75)));
}

criterion_group!(benches, day_11_part_one_benchmark, day_11_part_two_benchmark_part_two);
criterion_main!(benches);
