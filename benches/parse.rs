use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hare::parser::parse;

fn parse_expr(expr: &str) {
    parse(expr).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_expr1", |b| {
        b.iter(|| parse_expr(black_box("1 + 2 * 3 / 4;")));
    });

    c.bench_function("parse_expr2", |b| {
        b.iter(|| parse_expr(black_box("1 + 2 * 3 / (4 - 5) * 6 + 7;")));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
