use criterion::{criterion_group, criterion_main, Criterion};
use day_02;
use std::fs::File;
use std::io;

fn day_02_benchmark(c: &mut Criterion) {
    let input = "inputs\\day_02.txt";
    let file = match File::open(&input) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: Failed to open file '{}': {}", input, e);
            return;
        }
    };

    let reader = io::BufReader::new(file);
    let reports = day_02::parse_reports(reader);

    c.bench_function("day_02__count_safe_reports_with_tolerance", |b| {
        b.iter(|| day_02::count_safe_reports_with_tolerance(&reports))
    });
}

criterion_group!(benches, day_02_benchmark);
criterion_main!(benches);
