#!/usr/bin/env python3
"""Generate radar/spider triptych SVG from Criterion benchmark results.

Parses JSON output under target/criterion/ and produces a multi-panel radar
chart comparing Floyd-Warshall, Pairwise BFS, and Pairwise Dijkstra across
topologies at different vertex counts.

Usage:
    python scripts/generate_radar.py [--output docs/radar_triptych.svg]
"""

import argparse
import json
import math
import os
import re
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.patches import FancyBboxPatch


def find_criterion_dir() -> Path:
    """Locate the Criterion output directory."""
    candidates = [
        Path("target/criterion"),
    ]
    for candidate in candidates:
        if candidate.is_dir():
            return candidate
    raise FileNotFoundError(
        "Cannot find target/criterion/. Run benchmarks first with `cargo bench`."
    )


def parse_estimates(criterion_dir: Path) -> dict[str, dict[str, float]]:
    """Parse Criterion JSON estimates.

    Returns a nested dict: {topology: {algorithm: mean_ns}}.
    Only considers topology_comparison benchmarks.
    """
    results: dict[str, dict[str, float]] = {}

    for bench_dir in criterion_dir.iterdir():
        if not bench_dir.is_dir():
            continue

        name = bench_dir.name
        if not name.startswith("topology/"):
            continue

        estimates_path = bench_dir / "new" / "estimates.json"
        if not estimates_path.exists():
            continue

        with open(estimates_path) as f:
            data = json.load(f)

        mean_ns = data["mean"]["point_estimate"]

        # Parse benchmark ID: "topology/V100/AlgorithmName/topo_V100_E200"
        parts = name.split("/")
        if len(parts) < 3:
            continue

        size_group = parts[1]  # e.g., "V100"
        algo = parts[2]  # e.g., "PairwiseBFS"

        # Extract topology name from the parameter part
        param = "/".join(parts[3:]) if len(parts) > 3 else ""
        topo_match = re.match(r"^(.+?)_V\d+", param)
        if topo_match:
            topology = topo_match.group(1)
        else:
            topology = param.split("_")[0] if param else "unknown"

        key = f"{size_group}/{topology}"
        if key not in results:
            results[key] = {}
        results[key][algo] = mean_ns

    return results


def group_by_size(
    results: dict[str, dict[str, float]],
) -> dict[str, dict[str, dict[str, float]]]:
    """Group results by vertex count.

    Returns {size_group: {topology: {algorithm: mean_ns}}}.
    """
    grouped: dict[str, dict[str, dict[str, float]]] = {}
    for key, algos in results.items():
        parts = key.split("/", 1)
        if len(parts) != 2:
            continue
        size_group, topology = parts
        if size_group not in grouped:
            grouped[size_group] = {}
        grouped[size_group][topology] = algos
    return grouped


ALGORITHMS = ["FloydWarshall", "PairwiseBFS", "PairwiseDijkstra"]
COLORS = {"FloydWarshall": "#e74c3c", "PairwiseBFS": "#3498db", "PairwiseDijkstra": "#2ecc71"}
LABELS = {"FloydWarshall": "Floyd-Warshall", "PairwiseBFS": "Pairwise BFS", "PairwiseDijkstra": "Pairwise Dijkstra"}


def draw_radar_panel(ax, title: str, topologies: list[str], data: dict[str, dict[str, float]]):
    """Draw a single radar panel on the given axes."""
    n = len(topologies)
    if n == 0:
        ax.set_visible(False)
        return

    angles = np.linspace(0, 2 * np.pi, n, endpoint=False).tolist()
    angles.append(angles[0])  # Close the polygon

    ax.set_theta_offset(np.pi / 2)
    ax.set_theta_direction(-1)
    ax.set_rlabel_position(0)

    ax.set_xticks(angles[:-1])
    ax.set_xticklabels(topologies, size=7)
    ax.set_title(title, size=12, fontweight="bold", pad=20)

    # Compute log-scale normalized values (larger = faster = better)
    # We invert: normalized = log(max_time / time) so faster algorithms score higher
    all_times = []
    for topo in topologies:
        for algo in ALGORITHMS:
            t = data.get(topo, {}).get(algo)
            if t is not None and t > 0:
                all_times.append(t)

    if not all_times:
        ax.set_visible(False)
        return

    max_time = max(all_times)

    for algo in ALGORITHMS:
        values = []
        for topo in topologies:
            t = data.get(topo, {}).get(algo)
            if t is not None and t > 0:
                values.append(math.log10(max_time / t) + 1)
            else:
                values.append(0)
        values.append(values[0])  # Close polygon

        ax.plot(angles, values, "o-", linewidth=1.5, label=LABELS[algo], color=COLORS[algo])
        ax.fill(angles, values, alpha=0.1, color=COLORS[algo])


def generate_triptych(results: dict[str, dict[str, float]], output_path: str):
    """Generate the multi-panel radar SVG."""
    grouped = group_by_size(results)

    # Sort size groups by vertex count
    size_order = sorted(grouped.keys(), key=lambda s: int(re.search(r"\d+", s).group()))

    n_panels = len(size_order)
    if n_panels == 0:
        print("No topology_comparison results found. Nothing to plot.")
        return

    fig, axes = plt.subplots(
        1,
        n_panels,
        figsize=(6 * n_panels, 6),
        subplot_kw={"projection": "polar"},
    )

    if n_panels == 1:
        axes = [axes]

    for ax, size_group in zip(axes, size_order):
        topo_data = grouped[size_group]
        topologies = sorted(topo_data.keys())
        draw_radar_panel(ax, size_group, topologies, topo_data)

    # Shared legend
    handles, labels = axes[0].get_legend_handles_labels()
    if handles:
        fig.legend(handles, labels, loc="lower center", ncol=3, fontsize=10)

    plt.tight_layout(rect=[0, 0.08, 1, 1])

    os.makedirs(os.path.dirname(output_path) or ".", exist_ok=True)
    fig.savefig(output_path, format="svg", bbox_inches="tight")
    print(f"Saved radar chart to {output_path}")
    plt.close(fig)


def main():
    parser = argparse.ArgumentParser(description="Generate APSP benchmark radar charts")
    parser.add_argument(
        "--output",
        default="docs/radar_triptych.svg",
        help="Output SVG path (default: docs/radar_triptych.svg)",
    )
    args = parser.parse_args()

    criterion_dir = find_criterion_dir()
    results = parse_estimates(criterion_dir)

    if not results:
        print("No benchmark results found. Run `cargo bench -- topology_comparison` first.")
        return

    generate_triptych(results, args.output)


if __name__ == "__main__":
    main()
