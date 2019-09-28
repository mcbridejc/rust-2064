#[macro_use]
extern crate criterion;
extern crate flame;
extern crate twentysixtyfour;

use criterion::Criterion;
use criterion::profiler::Profiler;

use twentysixtyfour::{algorithm, simulate};
use std::path::Path;
use std::fs::File;

pub struct FlameProfiler;
impl Profiler for FlameProfiler {
    fn start_profiling(&mut self, benchmark_id: &str, _benchmark_dir: &Path) {
        println!("Starting profile for {}", benchmark_id);
        flame::start(benchmark_id.to_string());
    }
    fn stop_profiling(&mut self, benchmark_id: &str, _benchmark_dir: &Path) {
        flame::end(benchmark_id.to_string());
    }
}

fn profiled() -> Criterion {
    Criterion::default()
    //.with_profiler(FlameProfiler)
    .sample_size(20)
}

fn simulate_lookahead() -> simulate::BulkRunResult {
    simulate::bulk(|mut player, board| algorithm::naive_lookahead(&mut player, board, 2, algorithm::ScoreFunction::FreeSpaceWithSortedness), 1)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lookahead3", |b| b.iter(|| simulate_lookahead()));
     // Dump the profile report to disk
    //flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}

criterion_group!{
    name = benches;
    config = profiled(); 
    targets = criterion_benchmark
}

criterion_main!(benches);