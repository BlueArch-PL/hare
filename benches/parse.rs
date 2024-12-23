use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hare::parser::parse;

/// 解析表达式并检查是否成功
/// 
/// # 参数
/// 
/// * `expr` - 要解析的表达式字符串
fn parse_expr(expr: &str) {
    parse(expr).unwrap();
}

/// 使用 criterion 库进行基准测试
/// 
/// # 参数
/// 
/// * `c` - 用于配置和运行基准测试的 Criterion 对象
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_expr1", |b| {
        b.iter(|| parse_expr(black_box("1 + 2 * 3 / 4;")));
    });

    c.bench_function("parse_expr2", |b| {
        b.iter(|| parse_expr(black_box("1 + 2 * 3 / (4 - 5) * 6 + 7;")));
    });

    c.bench_function("parse_assign_with_type_annotation", |b| {
        b.iter(|| parse_expr(black_box("let x: int = 1 + 2 * 3 / 4;")));
        b.iter(|| parse_expr(black_box("let z: bool = true")));
        b.iter(|| parse_expr(black_box("let a: str = \"a\";")));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
