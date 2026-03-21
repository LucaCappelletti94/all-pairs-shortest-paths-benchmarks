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

macro_rules! bench_named {
    ($group:expr, $name:expr, $graph:expr) => {{
        let g: Graph = $graph;
        eprintln!("  {}...", $name);
        let lbl = format!("{}_{}", $name, graph_label(&g));
        bench_all_unweighted!($group, lbl, g);
    }};
}

macro_rules! bench_named_weighted {
    ($group:expr, $name:expr, $graph:expr) => {{
        let g: Graph = $graph;
        eprintln!("  {}...", $name);
        let lbl = format!("{}_{}", $name, graph_label(&g));
        bench_weighted!($group, lbl, g);
    }};
}

// All topology groups share the same set of topologies so radar charts have
// consistent axes. Generators that cannot hit the exact target V produce the
// closest possible value (e.g. grid 7x7 = 49 for V~50).

macro_rules! all_topologies {
    ($macro:ident, $group:expr, $v:expr) => {
        match $v {
            50 => {
                $macro!($group, "complete", complete_graph(50));
                $macro!($group, "cycle", cycle_graph(50));
                $macro!($group, "path", path_graph(50));
                $macro!($group, "star", star_graph(50));
                $macro!($group, "grid", grid_graph(7, 7));
                $macro!($group, "torus", torus_graph(7, 7));
                $macro!($group, "wheel", wheel_graph(49));
                $macro!($group, "crown", crown_graph(25));
                $macro!(
                    $group,
                    "complete_bipartite",
                    complete_bipartite_graph(25, 25)
                );
                $macro!($group, "turan", turan_graph(50, 5));
                $macro!($group, "friendship", friendship_graph(24));
                $macro!($group, "er_sparse", erdos_renyi_gnp(42, 50, 0.05));
                $macro!($group, "er_medium", erdos_renyi_gnp(42, 50, 0.2));
                $macro!($group, "er_dense", erdos_renyi_gnp(42, 50, 0.5));
                $macro!($group, "barabasi_albert", barabasi_albert(42, 50, 3));
                $macro!($group, "watts_strogatz", watts_strogatz(42, 50, 6, 0.3));
            }
            100 => {
                $macro!($group, "complete", complete_graph(100));
                $macro!($group, "cycle", cycle_graph(100));
                $macro!($group, "path", path_graph(100));
                $macro!($group, "star", star_graph(100));
                $macro!($group, "grid", grid_graph(10, 10));
                $macro!($group, "torus", torus_graph(10, 10));
                $macro!($group, "wheel", wheel_graph(99));
                $macro!($group, "crown", crown_graph(50));
                $macro!(
                    $group,
                    "complete_bipartite",
                    complete_bipartite_graph(50, 50)
                );
                $macro!($group, "turan", turan_graph(100, 5));
                $macro!($group, "friendship", friendship_graph(49));
                $macro!($group, "er_sparse", erdos_renyi_gnp(42, 100, 0.05));
                $macro!($group, "er_medium", erdos_renyi_gnp(42, 100, 0.2));
                $macro!($group, "er_dense", erdos_renyi_gnp(42, 100, 0.5));
                $macro!($group, "barabasi_albert", barabasi_albert(42, 100, 3));
                $macro!($group, "watts_strogatz", watts_strogatz(42, 100, 6, 0.3));
            }
            200 => {
                $macro!($group, "complete", complete_graph(200));
                $macro!($group, "cycle", cycle_graph(200));
                $macro!($group, "path", path_graph(200));
                $macro!($group, "star", star_graph(200));
                $macro!($group, "grid", grid_graph(14, 14));
                $macro!($group, "torus", torus_graph(14, 14));
                $macro!($group, "wheel", wheel_graph(199));
                $macro!($group, "crown", crown_graph(100));
                $macro!(
                    $group,
                    "complete_bipartite",
                    complete_bipartite_graph(100, 100)
                );
                $macro!($group, "turan", turan_graph(200, 5));
                $macro!($group, "friendship", friendship_graph(99));
                $macro!($group, "er_sparse", erdos_renyi_gnp(42, 200, 0.05));
                $macro!($group, "er_medium", erdos_renyi_gnp(42, 200, 0.2));
                $macro!($group, "er_dense", erdos_renyi_gnp(42, 200, 0.5));
                $macro!($group, "barabasi_albert", barabasi_albert(42, 200, 3));
                $macro!($group, "watts_strogatz", watts_strogatz(42, 200, 6, 0.3));
            }
            500 => {
                $macro!($group, "complete", complete_graph(500));
                $macro!($group, "cycle", cycle_graph(500));
                $macro!($group, "path", path_graph(500));
                $macro!($group, "star", star_graph(500));
                $macro!($group, "grid", grid_graph(22, 23));
                $macro!($group, "torus", torus_graph(22, 23));
                $macro!($group, "wheel", wheel_graph(499));
                $macro!($group, "crown", crown_graph(250));
                $macro!(
                    $group,
                    "complete_bipartite",
                    complete_bipartite_graph(250, 250)
                );
                $macro!($group, "turan", turan_graph(500, 5));
                $macro!($group, "friendship", friendship_graph(249));
                $macro!($group, "er_sparse", erdos_renyi_gnp(42, 500, 0.02));
                $macro!($group, "er_medium", erdos_renyi_gnp(42, 500, 0.1));
                $macro!($group, "er_dense", erdos_renyi_gnp(42, 500, 0.4));
                $macro!($group, "barabasi_albert", barabasi_albert(42, 500, 3));
                $macro!($group, "watts_strogatz", watts_strogatz(42, 500, 6, 0.3));
            }
            _ => unreachable!(),
        }
    };
}

fn bench_topology_v50(c: &mut Criterion) {
    eprintln!("[1/8] Running topology/V50 benchmarks...");
    let mut group = c.benchmark_group("topology/V50");
    let (samples, secs) = sampling_params(50);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    all_topologies!(bench_named, group, 50);
    group.finish();
}

fn bench_topology_v100(c: &mut Criterion) {
    eprintln!("[2/8] Running topology/V100 benchmarks...");
    let mut group = c.benchmark_group("topology/V100");
    let (samples, secs) = sampling_params(100);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    all_topologies!(bench_named, group, 100);
    group.finish();
}

fn bench_topology_v200(c: &mut Criterion) {
    eprintln!("[3/8] Running topology/V200 benchmarks...");
    let mut group = c.benchmark_group("topology/V200");
    let (samples, secs) = sampling_params(200);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    all_topologies!(bench_named, group, 200);
    group.finish();
}

fn bench_topology_v500(c: &mut Criterion) {
    eprintln!("[4/8] Running topology/V500 benchmarks...");
    let mut group = c.benchmark_group("topology/V500");
    let (samples, secs) = sampling_params(500);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    all_topologies!(bench_named, group, 500);
    group.finish();
}

fn bench_topology_v50_weighted(c: &mut Criterion) {
    eprintln!("[5/8] Running topology/V50_weighted benchmarks...");
    let mut group = c.benchmark_group("topology/V50_weighted");
    let (samples, secs) = sampling_params(50);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    all_topologies!(bench_named_weighted, group, 50);
    group.finish();
}

fn bench_topology_v100_weighted(c: &mut Criterion) {
    eprintln!("[6/8] Running topology/V100_weighted benchmarks...");
    let mut group = c.benchmark_group("topology/V100_weighted");
    let (samples, secs) = sampling_params(100);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    all_topologies!(bench_named_weighted, group, 100);
    group.finish();
}

fn bench_topology_v200_weighted(c: &mut Criterion) {
    eprintln!("[7/8] Running topology/V200_weighted benchmarks...");
    let mut group = c.benchmark_group("topology/V200_weighted");
    let (samples, secs) = sampling_params(200);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    all_topologies!(bench_named_weighted, group, 200);
    group.finish();
}

fn bench_topology_v500_weighted(c: &mut Criterion) {
    eprintln!("[8/8] Running topology/V500_weighted benchmarks...");
    let mut group = c.benchmark_group("topology/V500_weighted");
    let (samples, secs) = sampling_params(500);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    all_topologies!(bench_named_weighted, group, 500);
    group.finish();
}

criterion_group!(
    benches,
    bench_topology_v50,
    bench_topology_v100,
    bench_topology_v200,
    bench_topology_v500,
    bench_topology_v50_weighted,
    bench_topology_v100_weighted,
    bench_topology_v200_weighted,
    bench_topology_v500_weighted,
);
criterion_main!(benches);
