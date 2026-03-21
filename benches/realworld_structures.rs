use std::hint::black_box;
use std::time::Duration;

use all_pairs_shortest_paths_benchmarks::{Graph, graph_label, random_weight, sampling_params};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use geometric_traits::{
    impls::GenericImplicitValuedMatrix2D,
    prelude::{randomized_graphs::*, *},
};

macro_rules! bench_all_unweighted {
    ($group:expr, $label:expr, $graph:expr) => {{
        let graph = &$graph;
        let valued = GenericImplicitValuedMatrix2D::new(graph.clone(), |_| 1usize);
        $group.bench_with_input(BenchmarkId::new("PairwiseBFS", &$label), graph, |b, g| {
            b.iter(|| black_box(g.pairwise_bfs()));
        });
        $group.bench_with_input(
            BenchmarkId::new("FloydWarshall", &$label),
            &valued,
            |b, v| {
                b.iter(|| black_box(v.floyd_warshall().unwrap()));
            },
        );
        $group.bench_with_input(
            BenchmarkId::new("PairwiseDijkstra", &$label),
            &valued,
            |b, v| {
                b.iter(|| black_box(v.pairwise_dijkstra().unwrap()));
            },
        );
    }};
}

macro_rules! bench_weighted {
    ($group:expr, $label:expr, $graph:expr) => {{
        let graph = &$graph;
        let weighted = GenericImplicitValuedMatrix2D::new(graph.clone(), random_weight(42));
        $group.bench_with_input(
            BenchmarkId::new("FloydWarshall", &$label),
            &weighted,
            |b, v| {
                b.iter(|| black_box(v.floyd_warshall().unwrap()));
            },
        );
        $group.bench_with_input(
            BenchmarkId::new("PairwiseDijkstra", &$label),
            &weighted,
            |b, v| {
                b.iter(|| black_box(v.pairwise_dijkstra().unwrap()));
            },
        );
    }};
}

fn bench_barabasi_albert_m2(c: &mut Criterion) {
    eprintln!("[1/14] Running realworld/barabasi_albert_m2 benchmarks...");
    let mut group = c.benchmark_group("realworld/barabasi_albert_m2");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating barabasi_albert(V={n}, m=2)...");
        let g: Graph = barabasi_albert(42, n, 2);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_barabasi_albert_m5(c: &mut Criterion) {
    eprintln!("[2/14] Running realworld/barabasi_albert_m5 benchmarks...");
    let mut group = c.benchmark_group("realworld/barabasi_albert_m5");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating barabasi_albert(V={n}, m=5)...");
        let g: Graph = barabasi_albert(42, n, 5);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_watts_strogatz_k6(c: &mut Criterion) {
    eprintln!("[3/14] Running realworld/watts_strogatz_k6 benchmarks...");
    let mut group = c.benchmark_group("realworld/watts_strogatz_k6");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating watts_strogatz(V={n}, k=6, beta=0.3)...");
        let g: Graph = watts_strogatz(42, n, 6, 0.3);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_watts_strogatz_k10(c: &mut Criterion) {
    eprintln!("[4/14] Running realworld/watts_strogatz_k10 benchmarks...");
    let mut group = c.benchmark_group("realworld/watts_strogatz_k10");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating watts_strogatz(V={n}, k=10, beta=0.5)...");
        let g: Graph = watts_strogatz(42, n, 10, 0.5);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_stochastic_block_model(c: &mut Criterion) {
    eprintln!("[5/14] Running realworld/stochastic_block_model benchmarks...");
    let mut group = c.benchmark_group("realworld/stochastic_block_model");

    for n in [50usize, 100, 200, 500] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating stochastic_block_model(V={n})...");
        let g: Graph = stochastic_block_model(42, &[n / 2, n / 2], 0.3, 0.01);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_random_geometric(c: &mut Criterion) {
    eprintln!("[6/14] Running realworld/random_geometric benchmarks...");
    let mut group = c.benchmark_group("realworld/random_geometric");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let radius = (6.0 / (std::f64::consts::PI * (n - 1) as f64)).sqrt();
        eprintln!("  Generating random_geometric_graph(V={n}, r={radius:.4})...");
        let g: Graph = random_geometric_graph(42, n, radius);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_random_regular_k4(c: &mut Criterion) {
    eprintln!("[7/14] Running realworld/random_regular_k4 benchmarks...");
    let mut group = c.benchmark_group("realworld/random_regular_k4");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating random_regular_graph(V={n}, k=4)...");
        let g: Graph = random_regular_graph(42, n, 4);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_barabasi_albert_m2_weighted(c: &mut Criterion) {
    eprintln!("[8/14] Running realworld/barabasi_albert_m2_weighted benchmarks...");
    let mut group = c.benchmark_group("realworld/barabasi_albert_m2_weighted");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let g: Graph = barabasi_albert(42, n, 2);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_barabasi_albert_m5_weighted(c: &mut Criterion) {
    eprintln!("[9/14] Running realworld/barabasi_albert_m5_weighted benchmarks...");
    let mut group = c.benchmark_group("realworld/barabasi_albert_m5_weighted");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let g: Graph = barabasi_albert(42, n, 5);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_watts_strogatz_k6_weighted(c: &mut Criterion) {
    eprintln!("[10/14] Running realworld/watts_strogatz_k6_weighted benchmarks...");
    let mut group = c.benchmark_group("realworld/watts_strogatz_k6_weighted");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let g: Graph = watts_strogatz(42, n, 6, 0.3);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_watts_strogatz_k10_weighted(c: &mut Criterion) {
    eprintln!("[11/14] Running realworld/watts_strogatz_k10_weighted benchmarks...");
    let mut group = c.benchmark_group("realworld/watts_strogatz_k10_weighted");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let g: Graph = watts_strogatz(42, n, 10, 0.5);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_stochastic_block_model_weighted(c: &mut Criterion) {
    eprintln!("[12/14] Running realworld/stochastic_block_model_weighted benchmarks...");
    let mut group = c.benchmark_group("realworld/stochastic_block_model_weighted");

    for n in [50usize, 100, 200, 500] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let g: Graph = stochastic_block_model(42, &[n / 2, n / 2], 0.3, 0.01);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_random_geometric_weighted(c: &mut Criterion) {
    eprintln!("[13/14] Running realworld/random_geometric_weighted benchmarks...");
    let mut group = c.benchmark_group("realworld/random_geometric_weighted");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let radius = (6.0 / (std::f64::consts::PI * (n - 1) as f64)).sqrt();
        let g: Graph = random_geometric_graph(42, n, radius);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_random_regular_k4_weighted(c: &mut Criterion) {
    eprintln!("[14/14] Running realworld/random_regular_k4_weighted benchmarks...");
    let mut group = c.benchmark_group("realworld/random_regular_k4_weighted");

    for n in [50usize, 100, 200, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let g: Graph = random_regular_graph(42, n, 4);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_barabasi_albert_m2,
    bench_barabasi_albert_m5,
    bench_watts_strogatz_k6,
    bench_watts_strogatz_k10,
    bench_stochastic_block_model,
    bench_random_geometric,
    bench_random_regular_k4,
    bench_barabasi_albert_m2_weighted,
    bench_barabasi_albert_m5_weighted,
    bench_watts_strogatz_k6_weighted,
    bench_watts_strogatz_k10_weighted,
    bench_stochastic_block_model_weighted,
    bench_random_geometric_weighted,
    bench_random_regular_k4_weighted,
);
criterion_main!(benches);
