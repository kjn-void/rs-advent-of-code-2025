use aoc2025::days;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;

// Takes a day number, reads the matching cached input file, and returns its lines for benchmarking.
fn load_real_input(day: u32) -> Vec<String> {
    let path = PathBuf::from("input").join(format!("day{:02}.txt", day));
    let data = fs::read_to_string(path).expect("Missing input file (run with --fetch first)");
    data.lines().map(|s| s.to_string()).collect()
}

// Takes a Criterion runner and day, then benchmarks parsing, each part, and a full run for that solver.
fn bench_day(c: &mut Criterion, day: u32) {
    let lines = load_real_input(day);

    c.bench_function(&format!("day{:02}_set_input", day), |b| {
        b.iter(|| {
            let mut d = days::make_solver(day).unwrap();
            d.set_input(black_box(&lines));
        })
    });

    c.bench_function(&format!("day{:02}_part1", day), |b| {
        let mut d = days::make_solver(day).unwrap();
        d.set_input(&lines);
        b.iter(|| {
            black_box(d.part1());
        })
    });

    c.bench_function(&format!("day{:02}_part2", day), |b| {
        let mut d = days::make_solver(day).unwrap();
        d.set_input(&lines);
        b.iter(|| {
            black_box(d.part2());
        })
    });

    c.bench_function(&format!("day{:02}_full", day), |b| {
        b.iter(|| {
            let mut d = days::make_solver(day).unwrap();
            d.set_input(&lines);
            black_box(d.part1());
            black_box(d.part2());
        })
    });
}

// Registers benchmarks for every implemented day and delegates per-day setup to bench_day.
fn benches(c: &mut Criterion) {
    for day in days::implemented_days() {
        bench_day(c, day);
    }
}

criterion_group!(benches_all, benches);
criterion_main!(benches_all);
