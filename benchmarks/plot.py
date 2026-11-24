# /// script
# requires-python = ">=3.10"
# dependencies = ["matplotlib>=3.8"]
# ///
"""Plot benchmark CSVs from the runner.

One figure per element type: a grid of small multiples, one panel per
distribution, x = input size, y = median ns / (n log2 n), one line per sorter.

    uv run benchmarks/plot.py results/u64.csv [more.csv ...] --out-dir results/
"""

import argparse
import csv
import math
from collections import defaultdict
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

# Reference palette, categorical slots in fixed order. Each sorter always gets
# the same slot, so its color never changes between charts or filters.
SORTERS = {
    # name: (label, color, linestyle)
    "ips4o": ("ips4o (Rust)", "#2a78d6", "-"),
    "cpp_ips4o": ("IPS4o (C++)", "#eb6834", "--"),
    "std_unstable": ("std sort_unstable", "#1baf7a", "-"),
    "cpp_std_sort": ("std::sort (C++)", "#eda100", "--"),
    "std_stable": ("std sort (stable)", "#e87ba4", "-"),
}
SURFACE = "#fcfcfb"
TEXT_PRIMARY = "#0b0b0b"
TEXT_SECONDARY = "#52514e"
MUTED = "#898781"
GRID = "#e1e0d9"
BASELINE = "#c3c2b7"

DISTRIBUTION_ORDER = [
    "uniform",
    "exponential",
    "almost_sorted",
    "root_dup",
    "two_dup",
    "eight_dup",
    "sorted",
    "reverse_sorted",
    "all_equal",
]


def load(paths):
    # data[type][distribution][sorter] = [(n, ns_per_n_log_n)]
    data = defaultdict(lambda: defaultdict(lambda: defaultdict(list)))
    for path in paths:
        with open(path, newline="") as f:
            for row in csv.DictReader(f):
                series = data[row["type"]][row["distribution"]][row["sorter"]]
                series.append((int(row["n"]), float(row["ns_per_n_log_n"])))
    return data


def size_label(n):
    exp = int(math.log2(n))
    return f"$2^{{{exp}}}$"


def plot_type(name, by_dist, out_dir):
    dists = [d for d in DISTRIBUTION_ORDER if d in by_dist] + sorted(set(by_dist) - set(DISTRIBUTION_ORDER))
    cols = 3
    rows = math.ceil(len(dists) / cols)
    fig, axes = plt.subplots(rows, cols, figsize=(4.2 * cols, 3.0 * rows), squeeze=False, facecolor=SURFACE)
    present = set()

    for ax, dist in zip(axes.flat, dists):
        ax.set_facecolor(SURFACE)
        for sorter, (label, color, style) in SORTERS.items():
            points = sorted(by_dist[dist].get(sorter, []))
            if not points:
                continue
            present.add(sorter)
            xs, ys = zip(*points)
            ax.plot(xs, ys, color=color, linestyle=style, linewidth=2, marker="o", markersize=5,
                    markeredgecolor=SURFACE, markeredgewidth=1.5, label=label, solid_capstyle="round")
        ax.set_xscale("log", base=2)
        ax.xaxis.set_major_formatter(matplotlib.ticker.FuncFormatter(lambda n, _: size_label(n)))
        ax.set_ylim(bottom=0)
        ax.set_title(dist.replace("_", " "), color=TEXT_PRIMARY, fontsize=11, loc="left")
        ax.grid(True, axis="y", color=GRID, linewidth=0.8)
        ax.set_axisbelow(True)
        for side in ("top", "right", "left"):
            ax.spines[side].set_visible(False)
        ax.spines["bottom"].set_color(BASELINE)
        ax.tick_params(colors=MUTED, labelsize=9, length=0)

    for ax in list(axes.flat)[len(dists):]:
        ax.set_visible(False)

    handles = [
        matplotlib.lines.Line2D([], [], color=color, linestyle=style, linewidth=2, marker="o", markersize=5,
                                markeredgecolor=SURFACE, label=label)
        for sorter, (label, color, style) in SORTERS.items()
        if sorter in present
    ]
    fig.legend(handles=handles, loc="upper center", ncol=len(handles), frameon=False,
               bbox_to_anchor=(0.5, 1.0), fontsize=10, labelcolor=TEXT_SECONDARY)
    fig.suptitle(f"{name}: median time per n log₂ n (ns, lower is better) vs input size",
                 x=0.01, y=1.035, ha="left", color=TEXT_PRIMARY, fontsize=13)
    fig.tight_layout(rect=(0, 0, 1, 0.975))

    out = out_dir / f"{name}.png"
    fig.savefig(out, dpi=150, bbox_inches="tight", facecolor=SURFACE)
    plt.close(fig)
    print(f"wrote {out}")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("csv", nargs="+", type=Path)
    parser.add_argument("--out-dir", type=Path, default=None, help="default: next to the first CSV")
    args = parser.parse_args()
    out_dir = args.out_dir or args.csv[0].parent
    out_dir.mkdir(parents=True, exist_ok=True)
    for name, by_dist in load(args.csv).items():
        plot_type(name, by_dist, out_dir)


if __name__ == "__main__":
    main()
