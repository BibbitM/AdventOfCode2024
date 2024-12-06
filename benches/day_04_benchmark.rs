use advent::day_04;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn day_06_benchmark_parse(c: &mut Criterion) {
    let input = "inputs\\day_04.txt";
    let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_04.txt'");

    let word_search = day_04::WordSearch::new(input);

    let mut group = c.benchmark_group("day_04__xmas");

    group.bench_function("original", |b| b.iter(|| _ = word_search.count_xmas()));

    group.bench_function("check_xs", |b| b.iter(|| _ = word_search.count_xmas_check_xs()));
}

fn day_06_benchmark_parse_with_do(c: &mut Criterion) {
    let input = "inputs\\day_04.txt";
    let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_04.txt'");

    let word_search = day_04::WordSearch::new(input);

    let mut group = c.benchmark_group("day_04__mas_diagonal");

    group.bench_function("original", |b| b.iter(|| _ = word_search.count_mas_diagonal()));

    group.bench_function("check_a", |b| b.iter(|| _ = word_search.count_mas_diagonal_check_a()));

    group.bench_function("check_mask", |b| b.iter(|| _ = word_search.count_mas_diagonal_check_mask()));

    group.bench_function("check_if", |b| b.iter(|| _ = word_search.count_mas_diagonal_check_if()));
}

criterion_group!(benches, day_06_benchmark_parse, day_06_benchmark_parse_with_do);
criterion_main!(benches);
