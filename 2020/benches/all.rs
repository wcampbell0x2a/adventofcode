use advent_of_code2020::part_11::Part;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench_11(c: &mut Criterion) {
    c.bench_function("11_part1", |b| {
        b.iter(|| {
            let input = include_str!("../data/11.in");
            let seats: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

            let _part1 = Part::One.steps(seats);
        })
    });
    c.bench_function("11_part2", |b| {
        b.iter(|| {
            let input = include_str!("../data/11.in");
            let seats: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

            let _part1 = Part::Two.steps(seats);
        })
    });
}

criterion_group!(benches, bench_11);
criterion_main!(benches);
