#[macro_use]
extern crate criterion;
extern crate twentysixtyfour;

use criterion::Criterion;
use criterion::black_box;

use twentysixtyfour::{algorithm, simulate};

fn simulate_lookahead() -> simulate::BulkRunResult {
    simulate::bulk(|board| algorithm::naive_lookahead(board, 2, algorithm::ScoreFunction::FreeSpace), 1)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lookahead3", |b| b.iter(|| simulate_lookahead()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);