use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decrypt::iter_product::*;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn run_benchmark(count: usize, letters: usize) -> usize {
    let mut val = 0;
    for v in (0..count).fixed_product(letters) {
        val += v.len();
    }
    return val;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("iter 14**7", |b| {
        b.iter(|| run_benchmark(black_box(14), black_box(3)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
