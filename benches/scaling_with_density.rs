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

fn bench_density_v50(c: &mut Criterion) {
    eprintln!("[1/8] Running density/V50 benchmarks...");
    let mut group = c.benchmark_group("density/V50");
    let (samples, secs) = sampling_params(50);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));

    for p in [0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 0.9] {
        eprintln!("  Generating erdos_renyi_gnp(V=50, p={p})...");
        let g: Graph = erdos_renyi_gnp(42, 50, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_density_v100(c: &mut Criterion) {
    eprintln!("[2/8] Running density/V100 benchmarks...");
    let mut group = c.benchmark_group("density/V100");
    let (samples, secs) = sampling_params(100);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));

    for p in [0.02, 0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 0.9] {
        eprintln!("  Generating erdos_renyi_gnp(V=100, p={p})...");
        let g: Graph = erdos_renyi_gnp(42, 100, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_density_v200(c: &mut Criterion) {
    eprintln!("[3/8] Running density/V200 benchmarks...");
    let mut group = c.benchmark_group("density/V200");
    let (samples, secs) = sampling_params(200);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));

    for p in [0.02, 0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 0.9] {
        eprintln!("  Generating erdos_renyi_gnp(V=200, p={p})...");
        let g: Graph = erdos_renyi_gnp(42, 200, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_density_v500(c: &mut Criterion) {
    eprintln!("[4/8] Running density/V500 benchmarks...");
    let mut group = c.benchmark_group("density/V500");
    let (samples, secs) = sampling_params(500);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));

    for p in [0.01, 0.02, 0.05, 0.1, 0.2, 0.3, 0.5] {
        eprintln!("  Generating erdos_renyi_gnp(V=500, p={p})...");
        let g: Graph = erdos_renyi_gnp(42, 500, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_density_v50_weighted(c: &mut Criterion) {
    eprintln!("[5/8] Running density/V50_weighted benchmarks...");
    let mut group = c.benchmark_group("density/V50_weighted");
    let (samples, secs) = sampling_params(50);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));

    for p in [0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 0.9] {
        let g: Graph = erdos_renyi_gnp(42, 50, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_density_v100_weighted(c: &mut Criterion) {
    eprintln!("[6/8] Running density/V100_weighted benchmarks...");
    let mut group = c.benchmark_group("density/V100_weighted");
    let (samples, secs) = sampling_params(100);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));

    for p in [0.02, 0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 0.9] {
        let g: Graph = erdos_renyi_gnp(42, 100, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_density_v200_weighted(c: &mut Criterion) {
    eprintln!("[7/8] Running density/V200_weighted benchmarks...");
    let mut group = c.benchmark_group("density/V200_weighted");
    let (samples, secs) = sampling_params(200);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));

    for p in [0.02, 0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 0.9] {
        let g: Graph = erdos_renyi_gnp(42, 200, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_density_v500_weighted(c: &mut Criterion) {
    eprintln!("[8/8] Running density/V500_weighted benchmarks...");
    let mut group = c.benchmark_group("density/V500_weighted");
    let (samples, secs) = sampling_params(500);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));

    for p in [0.01, 0.02, 0.05, 0.1, 0.2, 0.3, 0.5] {
        let g: Graph = erdos_renyi_gnp(42, 500, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_weighted!(group, lbl, g);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_density_v50,
    bench_density_v100,
    bench_density_v200,
    bench_density_v500,
    bench_density_v50_weighted,
    bench_density_v100_weighted,
    bench_density_v200_weighted,
    bench_density_v500_weighted,
);
criterion_main!(benches);
