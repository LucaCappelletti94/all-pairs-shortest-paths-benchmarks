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

fn bench_sparse_d6(c: &mut Criterion) {
    eprintln!("[1/10] Running size/sparse_d6 benchmarks...");
    let mut group = c.benchmark_group("size/sparse_d6");

    for n in [10usize, 20, 50, 100, 200, 300, 500, 750, 1000] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating erdos_renyi_gnm(V={n}, E={})...", n * 3);
        let g: Graph = erdos_renyi_gnm(42, n, n * 3);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_medium_d20(c: &mut Criterion) {
    eprintln!("[2/10] Running size/medium_d20 benchmarks...");
    let mut group = c.benchmark_group("size/medium_d20");

    for n in [50usize, 100, 200, 300, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating erdos_renyi_gnm(V={n}, E={})...", n * 10);
        let g: Graph = erdos_renyi_gnm(42, n, n * 10);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_dense_half(c: &mut Criterion) {
    eprintln!("[3/10] Running size/dense_half benchmarks...");
    let mut group = c.benchmark_group("size/dense_half");

    for n in [20usize, 50, 100, 200, 300, 500] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let m = n * n / 4;
        eprintln!("  Generating erdos_renyi_gnm(V={n}, E={m})...");
        let g: Graph = erdos_renyi_gnm(42, n, m);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_complete(c: &mut Criterion) {
    eprintln!("[4/10] Running size/complete benchmarks...");
    let mut group = c.benchmark_group("size/complete");

    for n in [10usize, 20, 50, 100, 150, 200, 300] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating complete_graph(V={n})...");
        let g: Graph = complete_graph(n);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_grid(c: &mut Criterion) {
    eprintln!("[5/10] Running size/grid benchmarks...");
    let mut group = c.benchmark_group("size/grid");

    for k in [3usize, 5, 7, 10, 15, 20, 25] {
        let v = k * k;
        let (samples, secs) = sampling_params(v);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating grid_graph({k}, {k}), V={v}...");
        let g: Graph = grid_graph(k, k);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_sparse_d6_weighted(c: &mut Criterion) {
    eprintln!("[6/10] Running size/sparse_d6_weighted benchmarks...");
    let mut group = c.benchmark_group("size/sparse_d6_weighted");

    for n in [10usize, 20, 50, 100, 200, 300, 500, 750, 1000] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let g: Graph = erdos_renyi_gnm(42, n, n * 3);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_medium_d20_weighted(c: &mut Criterion) {
    eprintln!("[7/10] Running size/medium_d20_weighted benchmarks...");
    let mut group = c.benchmark_group("size/medium_d20_weighted");

    for n in [50usize, 100, 200, 300, 500, 750] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let g: Graph = erdos_renyi_gnm(42, n, n * 10);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_dense_half_weighted(c: &mut Criterion) {
    eprintln!("[8/10] Running size/dense_half_weighted benchmarks...");
    let mut group = c.benchmark_group("size/dense_half_weighted");

    for n in [20usize, 50, 100, 200, 300, 500] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let m = n * n / 4;
        let g: Graph = erdos_renyi_gnm(42, n, m);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_complete_weighted(c: &mut Criterion) {
    eprintln!("[9/10] Running size/complete_weighted benchmarks...");
    let mut group = c.benchmark_group("size/complete_weighted");

    for n in [10usize, 20, 50, 100, 150, 200, 300] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let g: Graph = complete_graph(n);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_grid_weighted(c: &mut Criterion) {
    eprintln!("[10/10] Running size/grid_weighted benchmarks...");
    let mut group = c.benchmark_group("size/grid_weighted");

    for k in [3usize, 5, 7, 10, 15, 20, 25] {
        let v = k * k;
        let (samples, secs) = sampling_params(v);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        let g: Graph = grid_graph(k, k);
        let lbl = graph_label(&g);
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_sparse_d6,
    bench_medium_d20,
    bench_dense_half,
    bench_complete,
    bench_grid,
    bench_sparse_d6_weighted,
    bench_medium_d20_weighted,
    bench_dense_half_weighted,
    bench_complete_weighted,
    bench_grid_weighted,
);
criterion_main!(benches);
