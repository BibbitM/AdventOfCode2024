use advent::char_map::CharMap;
use advent::day_08;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn day_08_part_one_benchmark(c: &mut Criterion) {
    let input = "inputs\\day_08.txt";
    let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_08.txt'");

    let antennas_map = CharMap::new(&input);
    let antennas = day_08::gather_antennas(&antennas_map);

    let mut group = c.benchmark_group("day_08__part_one");

    group.bench_function("original", |b| b.iter(|| _ = day_08::find_antinodes(&antennas, &antennas_map)));

    group.bench_function("sort_dedup", |b| b.iter(|| _ = day_08::find_antinodes_sort_dedup(&antennas, &antennas_map)));

    group.bench_function("hash_set", |b| b.iter(|| _ = day_08::find_antinodes_hash_set(&antennas, &antennas_map)));
}

fn day_08_part_two_benchmark_part_two(c: &mut Criterion) {
    let input = "inputs\\day_08.txt";
    let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_08.txt'");

    let antennas_map = CharMap::new(&input);
    let antennas = day_08::gather_antennas(&antennas_map);

    let mut group = c.benchmark_group("day_08__part_two");

    group.bench_function("original", |b| b.iter(|| _ = day_08::find_antinodes_in_line(&antennas, &antennas_map)));

    group.bench_function("sort_dedup", |b| b.iter(|| _ = day_08::find_antinodes_in_line_sort_dedup(&antennas, &antennas_map)));

    group.bench_function("hash_set", |b| b.iter(|| _ = day_08::find_antinodes_in_line_hash_set(&antennas, &antennas_map)));
}

criterion_group!(benches, day_08_part_one_benchmark, day_08_part_two_benchmark_part_two);
criterion_main!(benches);
