use advent::char_map::CharMap;
use advent::day_06;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn day_06_block_guard(guard_map: &CharMap) {
    let mut guard_map = guard_map.clone();
    _ = day_06::block_guard(&mut guard_map);
}

fn day_06_block_guard_assign_map(guard_map: &CharMap) {
    let mut guard_map = guard_map.clone();
    _ = day_06::block_guard_assign_map(&mut guard_map);
}

fn day_06_benchmark(c: &mut Criterion) {
    let input = "inputs\\day_06.txt";
    let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_06.txt'");
    let guard_map = CharMap::new(input);

    let mut group = c.benchmark_group("day_06__block_guard");

    group.bench_function("assign_map", |b| b.iter(|| day_06_block_guard_assign_map(&guard_map)));

    group.bench_function("original", |b| b.iter(|| day_06_block_guard(&guard_map)));
}

criterion_group!(benches, day_06_benchmark);
criterion_main!(benches);
