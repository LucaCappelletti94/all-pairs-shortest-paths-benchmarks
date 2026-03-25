use std::f64::consts::PI;
use std::hint::black_box;
use std::time::Duration;

use all_pairs_shortest_paths_benchmarks::{Graph, graph_label, random_weight, sampling_params};
use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
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

// All topology groups share the same set of topologies so radar charts have
// consistent axes. Generators that cannot hit the exact target V produce the
// closest possible value (e.g. grid 7x7 = 49 for V~50). The fixed-size
// Petersen graph is benchmarked separately in `extreme_cases.rs`.

fn grid_dims(target_vertices: usize) -> (usize, usize) {
    match target_vertices {
        50 => (7, 7),
        100 => (10, 10),
        200 => (14, 14),
        500 => (22, 23),
        _ => unreachable!(),
    }
}

fn hexagonal_lattice_dims(target_vertices: usize) -> (usize, usize) {
    match target_vertices {
        50 => (4, 4),
        100 => (6, 6),
        200 => (9, 9),
        500 => (15, 15),
        _ => unreachable!(),
    }
}

fn hypercube_dimension(target_vertices: usize) -> usize {
    match target_vertices {
        50 => 6,
        100 => 7,
        200 => 8,
        500 => 9,
        _ => unreachable!(),
    }
}

fn windmill_k4_cliques(target_vertices: usize) -> usize {
    match target_vertices {
        50 => 16,
        100 => 33,
        200 => 66,
        500 => 166,
        _ => unreachable!(),
    }
}

fn barbell_params(target_vertices: usize) -> (usize, usize) {
    match target_vertices {
        50 => (10, 30),
        100 => (20, 60),
        200 => (40, 120),
        500 => (100, 300),
        _ => unreachable!(),
    }
}

fn imbalanced_bipartite_parts(target_vertices: usize) -> (usize, usize) {
    let left = target_vertices / 5;
    (left, target_vertices - left)
}

fn sparse_bipartite_probability(target_vertices: usize) -> f64 {
    12.0 / target_vertices as f64
}

fn erdos_renyi_probabilities(target_vertices: usize) -> (f64, f64, f64) {
    match target_vertices {
        50 | 100 | 200 => (0.05, 0.2, 0.5),
        500 => (0.02, 0.1, 0.4),
        _ => unreachable!(),
    }
}

fn random_geometric_radius(target_vertices: usize) -> f64 {
    (6.0 / (PI * (target_vertices - 1) as f64)).sqrt()
}

fn topology_graphs(target_vertices: usize) -> Vec<(&'static str, Graph)> {
    let (grid_rows, grid_cols) = grid_dims(target_vertices);
    let (hex_rows, hex_cols) = hexagonal_lattice_dims(target_vertices);
    let (barbell_clique_size, barbell_path_len) = barbell_params(target_vertices);
    let (imbalanced_left, imbalanced_right) = imbalanced_bipartite_parts(target_vertices);
    let (er_sparse_p, er_medium_p, er_dense_p) = erdos_renyi_probabilities(target_vertices);
    let half = target_vertices / 2;

    vec![
        ("complete", complete_graph(target_vertices)),
        ("path", path_graph(target_vertices)),
        ("cycle", cycle_graph(target_vertices)),
        ("star", star_graph(target_vertices)),
        ("wheel", wheel_graph(target_vertices - 1)),
        ("grid", grid_graph(grid_rows, grid_cols)),
        ("torus", torus_graph(grid_rows, grid_cols)),
        (
            "hexagonal_lattice",
            hexagonal_lattice_graph(hex_rows, hex_cols),
        ),
        (
            "triangular_lattice",
            triangular_lattice_graph(grid_rows, grid_cols),
        ),
        (
            "hypercube",
            hypercube_graph(hypercube_dimension(target_vertices)),
        ),
        ("turan", turan_graph(target_vertices, 5)),
        ("friendship", friendship_graph((target_vertices - 1) / 2)),
        (
            "windmill_k4",
            windmill_graph(windmill_k4_cliques(target_vertices), 4),
        ),
        (
            "barbell",
            barbell_graph(barbell_clique_size, barbell_path_len),
        ),
        ("complete_bipartite", complete_bipartite_graph(half, half)),
        (
            "complete_bipartite_imbalanced",
            complete_bipartite_graph(imbalanced_left, imbalanced_right),
        ),
        ("crown", crown_graph(half)),
        (
            "random_sparse_bipartite",
            stochastic_block_model(
                42,
                &[half, half],
                0.0,
                sparse_bipartite_probability(target_vertices),
            ),
        ),
        (
            "er_sparse",
            erdos_renyi_gnp(42, target_vertices, er_sparse_p),
        ),
        (
            "er_medium",
            erdos_renyi_gnp(42, target_vertices, er_medium_p),
        ),
        ("er_dense", erdos_renyi_gnp(42, target_vertices, er_dense_p)),
        ("barabasi_albert", barabasi_albert(42, target_vertices, 3)),
        (
            "watts_strogatz",
            watts_strogatz(42, target_vertices, 6, 0.3),
        ),
        (
            "stochastic_block_model",
            stochastic_block_model(42, &[half, half], 0.3, 0.01),
        ),
        (
            "random_geometric",
            random_geometric_graph(
                42,
                target_vertices,
                random_geometric_radius(target_vertices),
            ),
        ),
        (
            "random_regular_k4",
            random_regular_graph(42, target_vertices, 4),
        ),
    ]
}

fn bench_topologies_unweighted(group: &mut BenchmarkGroup<'_, WallTime>, target_vertices: usize) {
    for (name, graph) in topology_graphs(target_vertices) {
        eprintln!("  {}...", name);
        let lbl = format!("{}_{}", name, graph_label(&graph));
        bench_all_unweighted!(group, lbl, graph);
    }
}

fn bench_topologies_weighted(group: &mut BenchmarkGroup<'_, WallTime>, target_vertices: usize) {
    for (name, graph) in topology_graphs(target_vertices) {
        eprintln!("  {}...", name);
        let lbl = format!("{}_{}", name, graph_label(&graph));
        bench_weighted!(group, lbl, graph);
    }
}

fn bench_topology_v50(c: &mut Criterion) {
    eprintln!("[1/8] Running topology/V50 benchmarks...");
    let mut group = c.benchmark_group("topology/V50");
    let (samples, secs) = sampling_params(50);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    bench_topologies_unweighted(&mut group, 50);
    group.finish();
}

fn bench_topology_v100(c: &mut Criterion) {
    eprintln!("[2/8] Running topology/V100 benchmarks...");
    let mut group = c.benchmark_group("topology/V100");
    let (samples, secs) = sampling_params(100);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    bench_topologies_unweighted(&mut group, 100);
    group.finish();
}

fn bench_topology_v200(c: &mut Criterion) {
    eprintln!("[3/8] Running topology/V200 benchmarks...");
    let mut group = c.benchmark_group("topology/V200");
    let (samples, secs) = sampling_params(200);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    bench_topologies_unweighted(&mut group, 200);
    group.finish();
}

fn bench_topology_v500(c: &mut Criterion) {
    eprintln!("[4/8] Running topology/V500 benchmarks...");
    let mut group = c.benchmark_group("topology/V500");
    let (samples, secs) = sampling_params(500);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    bench_topologies_unweighted(&mut group, 500);
    group.finish();
}

fn bench_topology_v50_weighted(c: &mut Criterion) {
    eprintln!("[5/8] Running topology/V50_weighted benchmarks...");
    let mut group = c.benchmark_group("topology/V50_weighted");
    let (samples, secs) = sampling_params(50);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    bench_topologies_weighted(&mut group, 50);
    group.finish();
}

fn bench_topology_v100_weighted(c: &mut Criterion) {
    eprintln!("[6/8] Running topology/V100_weighted benchmarks...");
    let mut group = c.benchmark_group("topology/V100_weighted");
    let (samples, secs) = sampling_params(100);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    bench_topologies_weighted(&mut group, 100);
    group.finish();
}

fn bench_topology_v200_weighted(c: &mut Criterion) {
    eprintln!("[7/8] Running topology/V200_weighted benchmarks...");
    let mut group = c.benchmark_group("topology/V200_weighted");
    let (samples, secs) = sampling_params(200);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    bench_topologies_weighted(&mut group, 200);
    group.finish();
}

fn bench_topology_v500_weighted(c: &mut Criterion) {
    eprintln!("[8/8] Running topology/V500_weighted benchmarks...");
    let mut group = c.benchmark_group("topology/V500_weighted");
    let (samples, secs) = sampling_params(500);
    group
        .sample_size(samples)
        .measurement_time(Duration::from_secs(secs));
    bench_topologies_weighted(&mut group, 500);
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
