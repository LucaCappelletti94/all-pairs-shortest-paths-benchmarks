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

fn bench_barbell(c: &mut Criterion) {
    eprintln!("[1/9] Running extreme/barbell benchmarks...");
    let mut group = c.benchmark_group("extreme/barbell");

    for (k, p) in [
        (10usize, 0usize),
        (20, 0),
        (50, 0),
        (100, 0),
        (10, 50),
        (20, 20),
        (50, 10),
    ] {
        let v = 2 * k + p;
        let (samples, secs) = sampling_params(v);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating barbell_graph(k={k}, p={p})...");
        let g: Graph = barbell_graph(k, p);
        let lbl = format!("k{k}_p{p}_{}", graph_label(&g));
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_hypercube(c: &mut Criterion) {
    eprintln!("[2/9] Running extreme/hypercube benchmarks...");
    let mut group = c.benchmark_group("extreme/hypercube");

    for d in [4usize, 6, 8] {
        let v = 1usize << d;
        let (samples, secs) = sampling_params(v);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating hypercube_graph(d={d}), V={v}...");
        let g: Graph = hypercube_graph(d);
        let lbl = format!("d{d}_{}", graph_label(&g));
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_star(c: &mut Criterion) {
    eprintln!("[3/9] Running extreme/star benchmarks...");
    let mut group = c.benchmark_group("extreme/star");

    for n in [50usize, 100, 200, 500, 750, 1000] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating star_graph(V={n})...");
        let g: Graph = star_graph(n);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_path(c: &mut Criterion) {
    eprintln!("[4/9] Running extreme/path benchmarks...");
    let mut group = c.benchmark_group("extreme/path");

    for n in [50usize, 100, 200, 500, 750, 1000] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating path_graph(V={n})...");
        let g: Graph = path_graph(n);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_cycle(c: &mut Criterion) {
    eprintln!("[5/9] Running extreme/cycle benchmarks...");
    let mut group = c.benchmark_group("extreme/cycle");

    for n in [50usize, 100, 200, 500, 750, 1000] {
        let (samples, secs) = sampling_params(n);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating cycle_graph(V={n})...");
        let g: Graph = cycle_graph(n);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_crown(c: &mut Criterion) {
    eprintln!("[6/9] Running extreme/crown benchmarks...");
    let mut group = c.benchmark_group("extreme/crown");

    for n in [10usize, 25, 50, 75, 100] {
        let v = 2 * n;
        let (samples, secs) = sampling_params(v);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating crown_graph(n={n}), V={v}...");
        let g: Graph = crown_graph(n);
        let lbl = graph_label(&g);
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_complete_bipartite(c: &mut Criterion) {
    eprintln!("[7/9] Running extreme/complete_bipartite benchmarks...");
    let mut group = c.benchmark_group("extreme/complete_bipartite");

    for (m, n) in [
        (10usize, 10usize),
        (25, 25),
        (50, 50),
        (100, 100),
        (10, 100),
        (50, 200),
    ] {
        let v = m + n;
        let (samples, secs) = sampling_params(v);
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(secs));

        eprintln!("  Generating complete_bipartite_graph({m}, {n})...");
        let g: Graph = complete_bipartite_graph(m, n);
        let lbl = format!("{m}x{n}_{}", graph_label(&g));
        bench_all_unweighted!(group, lbl, g);
    }

    group.finish();
}

fn bench_petersen(c: &mut Criterion) {
    eprintln!("[8/9] Running extreme/petersen benchmarks...");
    let mut group = c.benchmark_group("extreme/petersen");
    let (samples, secs) = sampling_params(10);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));

    eprintln!("  Generating petersen_graph()...");
    let g: Graph = petersen_graph();
    let lbl = graph_label(&g);
    bench_all_unweighted!(group, lbl, g);

    group.finish();
}

fn bench_petersen_weighted(c: &mut Criterion) {
    eprintln!("[9/9] Running extreme/petersen_weighted benchmarks...");
    let mut group = c.benchmark_group("extreme/petersen_weighted");
    let (samples, secs) = sampling_params(10);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));

    eprintln!("  Generating petersen_graph()...");
    let g: Graph = petersen_graph();
    let lbl = graph_label(&g);
    bench_weighted!(group, lbl, g);

    group.finish();
}

criterion_group!(
    benches,
    bench_barbell,
    bench_hypercube,
    bench_star,
    bench_path,
    bench_cycle,
    bench_crown,
    bench_complete_bipartite,
    bench_petersen,
    bench_petersen_weighted,
);
criterion_main!(benches);
