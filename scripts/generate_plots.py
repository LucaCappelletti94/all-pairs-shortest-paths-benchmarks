#!/usr/bin/env python3
"""Generate comprehensive benchmark visualizations from Criterion results.

Parses JSON output under target/criterion/ and produces SVG charts in docs/.

Usage:
    uv run python scripts/generate_plots.py
"""

import json
import math
import os
import re
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
import numpy as np

CRITERION_DIR = Path("target/criterion")
DOCS_DIR = Path("docs")

ALGORITHMS = ["PairwiseBFS", "FloydWarshall", "PairwiseDijkstra"]
ALGO_LABELS = {
    "FloydWarshall": "Floyd-Warshall",
    "PairwiseBFS": "Pairwise BFS",
    "PairwiseDijkstra": "Pairwise Dijkstra",
}
ALGO_COLORS = {
    "FloydWarshall": "#e74c3c",
    "PairwiseBFS": "#3498db",
    "PairwiseDijkstra": "#2ecc71",
}
ALGO_MARKERS = {
    "FloydWarshall": "s",
    "PairwiseBFS": "o",
    "PairwiseDijkstra": "^",
}


def parse_all_results() -> list[dict]:
    """Parse all Criterion benchmark results into a flat list of records."""
    records = []
    for group_dir in sorted(CRITERION_DIR.iterdir()):
        if not group_dir.is_dir() or group_dir.name == "report":
            continue
        group_name = group_dir.name  # e.g., "topology_V200", "size_sparse_d6"
        for algo_dir in sorted(group_dir.iterdir()):
            if not algo_dir.is_dir():
                continue
            algo = algo_dir.name  # e.g., "PairwiseBFS", "FloydWarshall"
            for param_dir in sorted(algo_dir.iterdir()):
                if not param_dir.is_dir():
                    continue
                param = param_dir.name  # e.g., "complete_V200_E19900"
                est_path = param_dir / "new" / "estimates.json"
                if not est_path.exists():
                    continue
                with open(est_path) as f:
                    data = json.load(f)
                mean_ns = data["mean"]["point_estimate"]

                # Extract V and E from param string
                v_match = re.search(r"V(\d+)", param)
                e_match = re.search(r"E(\d+)", param)
                vertices = int(v_match.group(1)) if v_match else 0
                edges = int(e_match.group(1)) if e_match else 0

                records.append({
                    "group": group_name,
                    "algorithm": algo,
                    "param": param,
                    "vertices": vertices,
                    "edges": edges,
                    "mean_ns": mean_ns,
                    "mean_ms": mean_ns / 1e6,
                    "mean_s": mean_ns / 1e9,
                })
    return records


def filter_records(records, group_prefix=None, group_exact=None):
    """Filter records by group prefix or exact group name."""
    if group_exact:
        return [r for r in records if r["group"] == group_exact]
    if group_prefix:
        return [r for r in records if r["group"].startswith(group_prefix)]
    return records


def setup_style():
    """Configure matplotlib style."""
    plt.rcParams.update({
        "figure.facecolor": "white",
        "axes.facecolor": "#fafafa",
        "axes.grid": True,
        "grid.alpha": 0.3,
        "font.size": 10,
        "axes.titlesize": 13,
        "axes.labelsize": 11,
    })


def save_fig(fig, name):
    """Save figure to docs/ as SVG and PNG."""
    DOCS_DIR.mkdir(exist_ok=True)
    fig.savefig(DOCS_DIR / f"{name}.svg", format="svg", bbox_inches="tight", dpi=150)
    fig.savefig(DOCS_DIR / f"{name}.png", format="png", bbox_inches="tight", dpi=150)
    print(f"  Saved {name}.svg and {name}.png")
    plt.close(fig)


# ---------------------------------------------------------------------------
# 1. Scaling with size - log-log line plots
# ---------------------------------------------------------------------------
def plot_scaling_with_size(records):
    """One subplot per size group showing time vs V for each algorithm."""
    groups = ["size_sparse_d6", "size_medium_d20", "size_dense_half", "size_complete", "size_grid"]
    titles = ["Sparse (d~6)", "Medium (d~20)", "Dense (E=V^2/4)", "Complete", "Grid"]

    fig, axes = plt.subplots(1, 5, figsize=(24, 5), sharey=False)
    fig.suptitle("Scaling with Size (Unweighted)", fontsize=16, fontweight="bold", y=1.02)

    for ax, group, title in zip(axes, groups, titles):
        subset = filter_records(records, group_exact=group)
        for algo in ALGORITHMS:
            pts = sorted(
                [(r["vertices"], r["mean_ms"]) for r in subset if r["algorithm"] == algo],
                key=lambda x: x[0],
            )
            if pts:
                xs, ys = zip(*pts)
                ax.plot(xs, ys, marker=ALGO_MARKERS[algo], color=ALGO_COLORS[algo],
                        label=ALGO_LABELS[algo], linewidth=1.5, markersize=5)
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_title(title)
        ax.set_xlabel("Vertices")
        ax.set_ylabel("Time (ms)")
        ax.xaxis.set_major_formatter(ticker.ScalarFormatter())

    axes[0].legend(fontsize=8, loc="upper left")
    fig.tight_layout()
    save_fig(fig, "scaling_with_size")


def plot_scaling_with_size_weighted(records):
    """Weighted variant: FW vs Dijkstra."""
    groups = ["size_sparse_d6_weighted", "size_medium_d20_weighted",
              "size_dense_half_weighted", "size_complete_weighted", "size_grid_weighted"]
    titles = ["Sparse (d~6)", "Medium (d~20)", "Dense (E=V^2/4)", "Complete", "Grid"]
    algos = ["FloydWarshall", "PairwiseDijkstra"]

    fig, axes = plt.subplots(1, 5, figsize=(24, 5), sharey=False)
    fig.suptitle("Scaling with Size (Weighted: FW vs Dijkstra)", fontsize=16, fontweight="bold", y=1.02)

    for ax, group, title in zip(axes, groups, titles):
        subset = filter_records(records, group_exact=group)
        for algo in algos:
            pts = sorted(
                [(r["vertices"], r["mean_ms"]) for r in subset if r["algorithm"] == algo],
                key=lambda x: x[0],
            )
            if pts:
                xs, ys = zip(*pts)
                ax.plot(xs, ys, marker=ALGO_MARKERS[algo], color=ALGO_COLORS[algo],
                        label=ALGO_LABELS[algo], linewidth=1.5, markersize=5)
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_title(title)
        ax.set_xlabel("Vertices")
        ax.set_ylabel("Time (ms)")
        ax.xaxis.set_major_formatter(ticker.ScalarFormatter())

    axes[0].legend(fontsize=8, loc="upper left")
    fig.tight_layout()
    save_fig(fig, "scaling_with_size_weighted")


# ---------------------------------------------------------------------------
# 2. Scaling with density
# ---------------------------------------------------------------------------
def plot_scaling_with_density(records):
    """Time vs edge probability for each vertex count."""
    groups = [("density_V50", "V=50"), ("density_V100", "V=100"),
              ("density_V200", "V=200"), ("density_V500", "V=500")]

    fig, axes = plt.subplots(1, 4, figsize=(20, 5), sharey=False)
    fig.suptitle("Scaling with Density (Unweighted)", fontsize=16, fontweight="bold", y=1.02)

    for ax, (group, title) in zip(axes, groups):
        subset = filter_records(records, group_exact=group)
        for algo in ALGORITHMS:
            algo_pts = [r for r in subset if r["algorithm"] == algo]
            # Extract probability from param: "p0.05_V50_E61"
            pts = []
            for r in algo_pts:
                p_match = re.match(r"p([\d.]+)_", r["param"])
                if p_match:
                    pts.append((float(p_match.group(1)), r["mean_ms"]))
            pts.sort()
            if pts:
                xs, ys = zip(*pts)
                ax.plot(xs, ys, marker=ALGO_MARKERS[algo], color=ALGO_COLORS[algo],
                        label=ALGO_LABELS[algo], linewidth=1.5, markersize=5)
        ax.set_yscale("log")
        ax.set_title(title)
        ax.set_xlabel("Edge probability (p)")
        ax.set_ylabel("Time (ms)")

    axes[0].legend(fontsize=8, loc="upper left")
    fig.tight_layout()
    save_fig(fig, "scaling_with_density")


def plot_scaling_with_density_weighted(records):
    """Weighted density: FW vs Dijkstra."""
    groups = [("density_V50_weighted", "V=50"), ("density_V100_weighted", "V=100"),
              ("density_V200_weighted", "V=200"), ("density_V500_weighted", "V=500")]
    algos = ["FloydWarshall", "PairwiseDijkstra"]

    fig, axes = plt.subplots(1, 4, figsize=(20, 5), sharey=False)
    fig.suptitle("Scaling with Density (Weighted: FW vs Dijkstra)", fontsize=16, fontweight="bold", y=1.02)

    for ax, (group, title) in zip(axes, groups):
        subset = filter_records(records, group_exact=group)
        for algo in algos:
            algo_pts = [r for r in subset if r["algorithm"] == algo]
            pts = []
            for r in algo_pts:
                p_match = re.match(r"p([\d.]+)_", r["param"])
                if p_match:
                    pts.append((float(p_match.group(1)), r["mean_ms"]))
            pts.sort()
            if pts:
                xs, ys = zip(*pts)
                ax.plot(xs, ys, marker=ALGO_MARKERS[algo], color=ALGO_COLORS[algo],
                        label=ALGO_LABELS[algo], linewidth=1.5, markersize=5)
        ax.set_yscale("log")
        ax.set_title(title)
        ax.set_xlabel("Edge probability (p)")
        ax.set_ylabel("Time (ms)")

    axes[0].legend(fontsize=8, loc="upper left")
    fig.tight_layout()
    save_fig(fig, "scaling_with_density_weighted")


# ---------------------------------------------------------------------------
# 3. Topology comparison - grouped bar charts
# ---------------------------------------------------------------------------
def plot_topology_comparison(records):
    """Grouped bar chart per vertex count."""
    groups = [("topology_V50", "V~50"), ("topology_V100", "V~100"),
              ("topology_V200", "V~200"), ("topology_V500", "V~500")]

    for group_name, title in groups:
        subset = filter_records(records, group_exact=group_name)
        if not subset:
            continue

        # Gather topologies
        topo_map: dict[str, dict[str, float]] = {}
        for r in subset:
            # Extract topology name from param (everything before _V\d+)
            t_match = re.match(r"^(.+?)_V\d+", r["param"])
            topo = t_match.group(1) if t_match else r["param"]
            if topo not in topo_map:
                topo_map[topo] = {}
            topo_map[topo][r["algorithm"]] = r["mean_ms"]

        topologies = sorted(topo_map.keys())
        n_topo = len(topologies)
        x = np.arange(n_topo)
        width = 0.25

        fig, ax = plt.subplots(figsize=(max(10, n_topo * 1.2), 6))
        for i, algo in enumerate(ALGORITHMS):
            vals = [topo_map[t].get(algo, 0) for t in topologies]
            ax.bar(x + i * width, vals, width, label=ALGO_LABELS[algo],
                   color=ALGO_COLORS[algo], alpha=0.85)

        ax.set_yscale("log")
        ax.set_ylabel("Time (ms)")
        ax.set_title(f"Topology Comparison ({title})", fontsize=14, fontweight="bold")
        ax.set_xticks(x + width)
        ax.set_xticklabels(topologies, rotation=45, ha="right", fontsize=9)
        ax.legend(fontsize=9)
        fig.tight_layout()
        save_fig(fig, f"topology_{group_name}")


# ---------------------------------------------------------------------------
# 4. Radar / spider chart
# ---------------------------------------------------------------------------
def plot_radar(records):
    """Radar chart per vertex count from topology_comparison, 2x2 grid."""
    groups = [("topology_V50", "V~50"), ("topology_V100", "V~100"),
              ("topology_V200", "V~200"), ("topology_V500", "V~500")]

    fig, axes = plt.subplots(2, 2, figsize=(14, 14), subplot_kw={"projection": "polar"})
    axes = axes.flatten()
    fig.suptitle("Algorithm Performance Radar (larger polygon = faster)", fontsize=18,
                 fontweight="bold", y=0.98)

    for ax, (group_name, title) in zip(axes, groups):
        subset = filter_records(records, group_exact=group_name)
        if not subset:
            ax.set_visible(False)
            continue

        topo_map: dict[str, dict[str, float]] = {}
        for r in subset:
            t_match = re.match(r"^(.+?)_V\d+", r["param"])
            topo = t_match.group(1) if t_match else r["param"]
            if topo not in topo_map:
                topo_map[topo] = {}
            topo_map[topo][r["algorithm"]] = r["mean_ns"]

        topologies = sorted(topo_map.keys())
        # Replace underscores with newlines for multi-line labels
        display_labels = [t.replace("_", "\n") for t in topologies]
        n = len(topologies)
        angles = np.linspace(0, 2 * np.pi, n, endpoint=False).tolist()
        angles.append(angles[0])

        ax.set_theta_offset(np.pi / 2)
        ax.set_theta_direction(-1)
        ax.set_xticks(angles[:-1])
        ax.set_xticklabels(display_labels, size=10, linespacing=0.9)
        # Push labels outward so they don't overlap the outer circle
        ax.tick_params(axis="x", pad=18)
        ax.set_title(title, size=14, fontweight="bold", pad=25)
        ax.set_yticklabels([])

        # Normalize: score = log10(slowest / this_time) + 1
        all_times = [topo_map[t].get(a, 0) for t in topologies for a in ALGORITHMS if topo_map[t].get(a, 0) > 0]
        if not all_times:
            continue
        max_time = max(all_times)

        for algo in ALGORITHMS:
            values = []
            for t in topologies:
                v = topo_map[t].get(algo)
                if v and v > 0:
                    values.append(math.log10(max_time / v) + 1)
                else:
                    values.append(0)
            values.append(values[0])
            ax.plot(angles, values, "o-", linewidth=1.5, label=ALGO_LABELS[algo],
                    color=ALGO_COLORS[algo], markersize=4)
            ax.fill(angles, values, alpha=0.08, color=ALGO_COLORS[algo])

    handles, labels = axes[0].get_legend_handles_labels()
    if handles:
        fig.legend(handles, labels, loc="lower center", ncol=3, fontsize=12)
    fig.tight_layout(rect=[0, 0.04, 1, 0.96])
    save_fig(fig, "radar_triptych")


def plot_radar_weighted(records):
    """Radar chart for weighted benchmarks (FW vs Dijkstra), 2x2 grid."""
    groups = [("topology_V50_weighted", "V~50"), ("topology_V100_weighted", "V~100"),
              ("topology_V200_weighted", "V~200"), ("topology_V500_weighted", "V~500")]
    algos_weighted = ["FloydWarshall", "PairwiseDijkstra"]

    fig, axes = plt.subplots(2, 2, figsize=(14, 14), subplot_kw={"projection": "polar"})
    axes = axes.flatten()
    fig.suptitle("Weighted Algorithm Radar: FW vs Dijkstra (larger polygon = faster)",
                 fontsize=16, fontweight="bold", y=0.98)

    for ax, (group_name, title) in zip(axes, groups):
        subset = filter_records(records, group_exact=group_name)
        if not subset:
            ax.set_visible(False)
            continue

        topo_map: dict[str, dict[str, float]] = {}
        for r in subset:
            t_match = re.match(r"^(.+?)_V\d+", r["param"])
            topo = t_match.group(1) if t_match else r["param"]
            if topo not in topo_map:
                topo_map[topo] = {}
            topo_map[topo][r["algorithm"]] = r["mean_ns"]

        topologies = sorted(topo_map.keys())
        display_labels = [t.replace("_", "\n") for t in topologies]
        n = len(topologies)
        angles = np.linspace(0, 2 * np.pi, n, endpoint=False).tolist()
        angles.append(angles[0])

        ax.set_theta_offset(np.pi / 2)
        ax.set_theta_direction(-1)
        ax.set_xticks(angles[:-1])
        ax.set_xticklabels(display_labels, size=10, linespacing=0.9)
        ax.tick_params(axis="x", pad=18)
        ax.set_title(title, size=14, fontweight="bold", pad=25)
        ax.set_yticklabels([])

        all_times = [topo_map[t].get(a, 0) for t in topologies for a in algos_weighted if topo_map[t].get(a, 0) > 0]
        if not all_times:
            continue
        max_time = max(all_times)

        for algo in algos_weighted:
            values = []
            for t in topologies:
                v = topo_map[t].get(algo)
                if v and v > 0:
                    values.append(math.log10(max_time / v) + 1)
                else:
                    values.append(0)
            values.append(values[0])
            ax.plot(angles, values, "o-", linewidth=1.5, label=ALGO_LABELS[algo],
                    color=ALGO_COLORS[algo], markersize=4)
            ax.fill(angles, values, alpha=0.08, color=ALGO_COLORS[algo])

    handles, labels = axes[0].get_legend_handles_labels()
    if handles:
        fig.legend(handles, labels, loc="lower center", ncol=2, fontsize=12)
    fig.tight_layout(rect=[0, 0.04, 1, 0.96])
    save_fig(fig, "radar_weighted")


# ---------------------------------------------------------------------------
# 5. Extreme cases
# ---------------------------------------------------------------------------
def plot_extreme_cases(records):
    """Line plots for pathological graph structures."""
    groups = [
        ("extreme_barbell", "Barbell"),
        ("extreme_hypercube", "Hypercube"),
        ("extreme_star", "Star"),
        ("extreme_path", "Path"),
        ("extreme_cycle", "Cycle"),
        ("extreme_crown", "Crown"),
        ("extreme_complete_bipartite", "Complete Bipartite"),
        ("extreme_petersen", "Petersen"),
    ]

    fig, axes = plt.subplots(2, 4, figsize=(22, 10), sharey=False)
    axes = axes.flatten()
    fig.suptitle("Extreme / Pathological Structures", fontsize=16, fontweight="bold", y=1.02)

    for ax, (group, title) in zip(axes, groups):
        subset = filter_records(records, group_exact=group)
        for algo in ALGORITHMS:
            pts = sorted(
                [(r["vertices"], r["mean_ms"]) for r in subset if r["algorithm"] == algo],
                key=lambda x: x[0],
            )
            if pts:
                xs, ys = zip(*pts)
                ax.plot(xs, ys, marker=ALGO_MARKERS[algo], color=ALGO_COLORS[algo],
                        label=ALGO_LABELS[algo], linewidth=1.5, markersize=5)
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_title(title)
        ax.set_xlabel("Vertices")
        ax.set_ylabel("Time (ms)")
        ax.xaxis.set_major_formatter(ticker.ScalarFormatter())

    axes[0].legend(fontsize=8, loc="upper left")
    fig.tight_layout()
    save_fig(fig, "extreme_cases")


# ---------------------------------------------------------------------------
# 6. Extreme cases (weighted)
# ---------------------------------------------------------------------------
def plot_extreme_cases_weighted(records):
    """Weighted bar chart for fixed-size extreme cases."""
    group = "extreme_petersen_weighted"
    algos = ["FloydWarshall", "PairwiseDijkstra"]
    subset = filter_records(records, group_exact=group)
    if not subset:
        return

    algo_map = {r["algorithm"]: r["mean_ms"] for r in subset}
    labels = [ALGO_LABELS[algo] for algo in algos if algo in algo_map]
    values = [algo_map[algo] for algo in algos if algo in algo_map]
    colors = [ALGO_COLORS[algo] for algo in algos if algo in algo_map]

    fig, ax = plt.subplots(figsize=(7, 5))
    ax.bar(labels, values, color=colors, alpha=0.85)
    ax.set_yscale("log")
    ax.set_ylabel("Time (ms)")
    ax.set_title("Weighted Petersen Graph", fontsize=14, fontweight="bold")

    for idx, value in enumerate(values):
        ax.text(idx, value, f"{value:.3f} ms", ha="center", va="bottom", fontsize=9)

    fig.tight_layout()
    save_fig(fig, "extreme_cases_weighted")


# ---------------------------------------------------------------------------
# 7. Realworld models
# ---------------------------------------------------------------------------
def plot_realworld(records):
    """Scaling for real-world graph models."""
    groups = [
        ("realworld_barabasi_albert_m2", "BA (m=2)"),
        ("realworld_barabasi_albert_m5", "BA (m=5)"),
        ("realworld_watts_strogatz_k6", "WS (k=6)"),
        ("realworld_watts_strogatz_k10", "WS (k=10)"),
        ("realworld_stochastic_block_model", "SBM"),
        ("realworld_random_geometric", "RGG"),
        ("realworld_random_regular_k4", "RR (k=4)"),
    ]

    fig, axes = plt.subplots(2, 4, figsize=(22, 10), sharey=False)
    fig.suptitle("Real-World Graph Models (Unweighted)", fontsize=16, fontweight="bold", y=1.01)

    for idx, (group, title) in enumerate(groups):
        ax = axes[idx // 4][idx % 4]
        subset = filter_records(records, group_exact=group)
        for algo in ALGORITHMS:
            pts = sorted(
                [(r["vertices"], r["mean_ms"]) for r in subset if r["algorithm"] == algo],
                key=lambda x: x[0],
            )
            if pts:
                xs, ys = zip(*pts)
                ax.plot(xs, ys, marker=ALGO_MARKERS[algo], color=ALGO_COLORS[algo],
                        label=ALGO_LABELS[algo], linewidth=1.5, markersize=5)
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_title(title)
        ax.set_xlabel("Vertices")
        ax.set_ylabel("Time (ms)")
        ax.xaxis.set_major_formatter(ticker.ScalarFormatter())

    # Hide unused subplot
    axes[1][3].set_visible(False)
    axes[0][0].legend(fontsize=8, loc="upper left")
    fig.tight_layout()
    save_fig(fig, "realworld_structures")


# ---------------------------------------------------------------------------
# 8. Realworld models (weighted)
# ---------------------------------------------------------------------------
def plot_realworld_weighted(records):
    """Scaling for real-world graph models, weighted variant."""
    groups = [
        ("realworld_barabasi_albert_m2_weighted", "BA (m=2)"),
        ("realworld_barabasi_albert_m5_weighted", "BA (m=5)"),
        ("realworld_watts_strogatz_k6_weighted", "WS (k=6)"),
        ("realworld_watts_strogatz_k10_weighted", "WS (k=10)"),
        ("realworld_stochastic_block_model_weighted", "SBM"),
        ("realworld_random_geometric_weighted", "RGG"),
        ("realworld_random_regular_k4_weighted", "RR (k=4)"),
    ]
    algos = ["FloydWarshall", "PairwiseDijkstra"]

    fig, axes = plt.subplots(2, 4, figsize=(22, 10), sharey=False)
    fig.suptitle(
        "Real-World Graph Models (Weighted: FW vs Dijkstra)",
        fontsize=16,
        fontweight="bold",
        y=1.01,
    )

    for idx, (group, title) in enumerate(groups):
        ax = axes[idx // 4][idx % 4]
        subset = filter_records(records, group_exact=group)
        for algo in algos:
            pts = sorted(
                [(r["vertices"], r["mean_ms"]) for r in subset if r["algorithm"] == algo],
                key=lambda x: x[0],
            )
            if pts:
                xs, ys = zip(*pts)
                ax.plot(
                    xs,
                    ys,
                    marker=ALGO_MARKERS[algo],
                    color=ALGO_COLORS[algo],
                    label=ALGO_LABELS[algo],
                    linewidth=1.5,
                    markersize=5,
                )
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_title(title)
        ax.set_xlabel("Vertices")
        ax.set_ylabel("Time (ms)")
        ax.xaxis.set_major_formatter(ticker.ScalarFormatter())

    axes[1][3].set_visible(False)
    axes[0][0].legend(fontsize=8, loc="upper left")
    fig.tight_layout()
    save_fig(fig, "realworld_structures_weighted")


# ---------------------------------------------------------------------------
# 9. Speedup heatmap: BFS / FW ratio
# ---------------------------------------------------------------------------
def plot_speedup_heatmap(records):
    """Heatmap showing BFS speedup over FW across topologies and sizes."""
    topo_groups = ["topology_V50", "topology_V100", "topology_V200", "topology_V500"]
    size_labels = ["V~50", "V~100", "V~200", "V~500"]

    # Collect all topologies across all groups
    all_topos = set()
    data_map: dict[str, dict[str, dict[str, float]]] = {}
    for group_name in topo_groups:
        subset = filter_records(records, group_exact=group_name)
        data_map[group_name] = {}
        for r in subset:
            t_match = re.match(r"^(.+?)_V\d+", r["param"])
            topo = t_match.group(1) if t_match else r["param"]
            all_topos.add(topo)
            if topo not in data_map[group_name]:
                data_map[group_name][topo] = {}
            data_map[group_name][topo][r["algorithm"]] = r["mean_ns"]

    topologies = sorted(all_topos)
    n_topos = len(topologies)
    n_sizes = len(topo_groups)

    matrix = np.full((n_topos, n_sizes), np.nan)
    for j, group_name in enumerate(topo_groups):
        for i, topo in enumerate(topologies):
            entry = data_map.get(group_name, {}).get(topo, {})
            bfs = entry.get("PairwiseBFS")
            fw = entry.get("FloydWarshall")
            if bfs and fw and bfs > 0:
                matrix[i, j] = fw / bfs

    fig, ax = plt.subplots(figsize=(8, max(6, n_topos * 0.45)))
    im = ax.imshow(matrix, aspect="auto", cmap="RdYlGn", interpolation="nearest")

    # Annotate cells
    for i in range(n_topos):
        for j in range(n_sizes):
            val = matrix[i, j]
            if not np.isnan(val):
                ax.text(j, i, f"{val:.1f}x", ha="center", va="center",
                        fontsize=8, color="black" if 2 < val < 50 else "white")

    ax.set_xticks(range(n_sizes))
    ax.set_xticklabels(size_labels)
    ax.set_yticks(range(n_topos))
    ax.set_yticklabels(topologies, fontsize=9)
    ax.set_title("Floyd-Warshall / BFS Slowdown Factor\n(higher = BFS is faster)",
                 fontsize=13, fontweight="bold")
    fig.colorbar(im, ax=ax, label="FW / BFS ratio", shrink=0.8)
    fig.tight_layout()
    save_fig(fig, "speedup_heatmap")


# ---------------------------------------------------------------------------
# 10. Summary table
# ---------------------------------------------------------------------------
def print_summary(records):
    """Print a text summary of key findings."""
    print("\n" + "=" * 70)
    print("BENCHMARK SUMMARY")
    print("=" * 70)

    # Find crossover points in density benchmarks
    for v_size in [50, 100, 200, 500]:
        group = f"density_V{v_size}"
        subset = filter_records(records, group_exact=group)
        bfs_pts = {}
        fw_pts = {}
        for r in subset:
            p_match = re.match(r"p([\d.]+)_", r["param"])
            if not p_match:
                continue
            p = float(p_match.group(1))
            if r["algorithm"] == "PairwiseBFS":
                bfs_pts[p] = r["mean_ms"]
            elif r["algorithm"] == "FloydWarshall":
                fw_pts[p] = r["mean_ms"]

        crossover = None
        for p in sorted(bfs_pts.keys()):
            if p in fw_pts and bfs_pts[p] > fw_pts[p]:
                crossover = p
                break

        if crossover:
            print(f"  V={v_size}: FW beats BFS at p={crossover} "
                  f"(BFS={bfs_pts[crossover]:.2f}ms, FW={fw_pts[crossover]:.2f}ms)")
        else:
            max_p = max(bfs_pts.keys()) if bfs_pts else 0
            if max_p and max_p in fw_pts:
                ratio = fw_pts[max_p] / bfs_pts[max_p]
                print(f"  V={v_size}: BFS still faster at p={max_p} (FW/BFS = {ratio:.1f}x)")

    # Fastest per topology at V=500
    print(f"\n  Fastest algorithm per topology at V~500:")
    subset = filter_records(records, group_exact="topology_V500")
    topo_map: dict[str, dict[str, float]] = {}
    for r in subset:
        t_match = re.match(r"^(.+?)_V\d+", r["param"])
        topo = t_match.group(1) if t_match else r["param"]
        if topo not in topo_map:
            topo_map[topo] = {}
        topo_map[topo][r["algorithm"]] = r["mean_ms"]

    for topo in sorted(topo_map.keys()):
        best_algo = min(topo_map[topo], key=topo_map[topo].get)
        best_time = topo_map[topo][best_algo]
        print(f"    {topo:25s} -> {ALGO_LABELS[best_algo]:20s} ({best_time:.2f} ms)")

    print("=" * 70)


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
def main():
    setup_style()
    print("Parsing benchmark results...")
    records = parse_all_results()
    print(f"  Found {len(records)} data points across {len(set(r['group'] for r in records))} groups")

    print("\nGenerating plots:")
    plot_scaling_with_size(records)
    plot_scaling_with_size_weighted(records)
    plot_scaling_with_density(records)
    plot_scaling_with_density_weighted(records)
    plot_topology_comparison(records)
    plot_radar(records)
    plot_radar_weighted(records)
    plot_extreme_cases(records)
    plot_extreme_cases_weighted(records)
    plot_realworld(records)
    plot_realworld_weighted(records)
    plot_speedup_heatmap(records)

    print_summary(records)
    print(f"\nAll plots saved to {DOCS_DIR}/")


if __name__ == "__main__":
    main()
