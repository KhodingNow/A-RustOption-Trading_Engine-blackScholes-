use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_options_engine::model::black_scholes::call_price;
use rust_options_engine::types::*;

fn bs_call_benchmark(c: &mut Criterion) {
    c.bench_function("black_scholes_call", |b| {
        b.iter(|| {
            call_price(
                black_box(Spot(100.0)),
                black_box(Strike(100.0)),
                black_box(Rate(0.05)),
                balck_box(Volatility(0.2)),
                balck_box(TimeToMaturity(1.0)),
            )
        })
    });
}

criterion_group!(benches, bs_call_benchmark);
criterion_main!(benches);
