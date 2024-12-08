use advent::day_07;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io;

fn day_07_part_one_benchmark(c: &mut Criterion) {
    let input = "inputs\\day_07.txt";
    let file = match File::open(&input) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: Failed to open file '{}': {}", input, e);
            return;
        }
    };

    let reader = io::BufReader::new(file);
    let equations = day_07::parse_equations(reader);

    let mut group = c.benchmark_group("day_07__part_one");

    group.bench_function("original", |b| b.iter(|| _ = day_07::sum_can_calibrate_values(&equations)));
}

fn day_07_part_two_benchmark_part_two(c: &mut Criterion) {
    let input = "inputs\\day_07.txt";
    let file = match File::open(&input) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: Failed to open file '{}': {}", input, e);
            return;
        }
    };

    let reader = io::BufReader::new(file);
    let equations = day_07::parse_equations(reader);

    let mut group = c.benchmark_group("day_07__part_two");

    group.bench_function("original", |b| b.iter(|| _ = day_07::sum_can_calibrate_values_concat(&equations)));
}

criterion_group!(benches, day_07_part_one_benchmark, day_07_part_two_benchmark_part_two);
criterion_main!(benches);
