#!/usr/bin/env python3
"""WER curves for the README: measure, then plot.

    # 1. Measure (slow: about an hour on an 8-core laptop). Needs the `wer` example and AFF3CT.
    cargo build --release --example wer
    python3 scripts/wer_curves.py measure --aff3ct /path/to/aff3ct/build/bin/aff3ct
    # 2. Plot docs/wer/*.svg from docs/wer/wer.csv (needs matplotlib)
    python3 scripts/wer_curves.py plot

Every point is BPSK/AWGN with Eb/N0 on the payload rate k/n, run to 100 word errors or the
word cap, 8 threads. ecc-hypercube points come from `examples/wer.rs`; polar points from
AFF3CT v3.0.2 with the 5G construction and the same payload k plus an 11-bit CRC, as in the
README tables. A series stops once a point falls below WER 1e-4.
"""
import argparse
import csv
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
OUT_DIR = os.path.join(ROOT, "docs", "wer")
CSV_PATH = os.path.join(OUT_DIR, "wer.csv")

# Panels: (title, n, k, Eb/N0 grid, series). Series: (label, kind, args).
PANELS = [
    ("(128, 64)", 128, 64, [2.0, 2.5, 3.0, 3.5, 4.0, 4.5], [
        ("RM ensemble 32×8", "rm", (3, 7, 32, 8)),
        ("RM ensemble 8×4", "rm", (3, 7, 8, 4)),
        ("Polar SCL8 + CRC-11", "polar", ("SCL", 8)),
        ("Polar ASCL128 + CRC-11", "polar", ("ASCL", 128)),
    ]),
    ("(256, 163)", 256, 163, [2.5, 3.0, 3.5, 4.0, 4.5], [
        ("RM ensemble 32×8", "rm", (4, 8, 32, 8)),
        ("RM ensemble 8×4", "rm", (4, 8, 8, 4)),
        ("Polar SCL8 + CRC-11", "polar", ("SCL", 8)),
        ("Polar ASCL128 + CRC-11", "polar", ("ASCL", 128)),
    ]),
    ("(512, 256)", 512, 256, [1.5, 2.0, 2.5, 3.0, 3.5], [
        ("RM ensemble 32×8", "rm", (4, 9, 32, 8)),
        ("RM ensemble 16×4", "rm", (4, 9, 16, 4)),
        ("GLDPC (512, 257)", "gldpc", (512, "1/2")),
        ("Polar SCL8 + CRC-11", "polar", ("SCL", 8)),
        ("Polar ASCL128 + CRC-11", "polar", ("ASCL", 128)),
    ]),
    ("(1024, 513)", 1024, 513, [1.5, 1.75, 2.0, 2.25, 2.5, 2.75], [
        ("GLDPC (1024, 513)", "gldpc", (1024, "1/2")),
        ("Polar SCL8 + CRC-11", "polar", ("SCL", 8)),
        ("Polar ASCL128 + CRC-11", "polar", ("ASCL", 128)),
    ]),
]

# Fixed colour per decoder family across every panel (validated categorical palette).
STYLE = {
    "RM ensemble 32×8": ("#2a78d6", "-", "o"),
    "RM ensemble 8×4": ("#1baf7a", "-", "s"),
    "RM ensemble 16×4": ("#1baf7a", "-", "s"),
    "GLDPC": ("#6250d6", "-", "D"),
    "Polar SCL8 + CRC-11": ("#eb6834", "--", "^"),
    "Polar ASCL128 + CRC-11": ("#e87ba4", "--", "v"),
}
STOP_BELOW = 1e-4


# One legend entry per decoder family, shared by all panels.
FAMILY = {
    "RM ensemble 32×8": "RM ensemble 32×8",
    "RM ensemble 8×4": "RM ensemble, fast (8×4; 16×4 at n = 512)",
    "RM ensemble 16×4": "RM ensemble, fast (8×4; 16×4 at n = 512)",
    "Polar SCL8 + CRC-11": "Polar SCL8 + CRC-11 (AFF3CT)",
    "Polar ASCL128 + CRC-11": "Polar ASCL128 + CRC-11 (AFF3CT)",
}


def style_for(label):
    return STYLE["GLDPC"] if label.startswith("GLDPC") else STYLE[label]


def family(label):
    return "Hypercube GLDPC (k = 257 at n = 512)" if label.startswith("GLDPC") else FAMILY[label]


def run_ours(kind, args, db, wer_bin):
    if kind == "rm":
        r, m, copies, lst = args
        cmd = [wer_bin, "rm", str(r), str(m), str(copies), str(lst), f"{db}"]
    else:
        n, rate = args
        cmd = [wer_bin, "gldpc", str(n), rate, f"{db}"]
    out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
    m = re.search(r"errors\s+(\d+)\s*/\s*(\d+) words", out)
    return int(m.group(1)), int(m.group(2))


def run_polar(n, k, args, db, aff3ct):
    dec, lst = args
    cmd = [aff3ct, "-C", "POLAR", "-N", str(n), "-K", str(k + 11), "--enc-fb-gen-method", "5G",
           "--crc-type", "11-5G", "-D", dec, "-L", str(lst), "-m", f"{db}", "-M", f"{db}",
           "-e", "100", "-n", "1000000", "-t", "8", "--ter-freq", "0"]
    out = subprocess.run(cmd, capture_output=True, text=True).stdout
    # Result row: Es/N0 | Eb/N0 | FRA | BE | FE | BER | FER | ...
    for line in out.splitlines():
        cols = [c.strip() for c in re.split(r"\|+", line)]
        if len(cols) > 6 and re.fullmatch(r"-?\d+\.\d+", cols[0] or "x"):
            fra, fe = cols[2], cols[4]
            return int(float(fe)), int(float(fra))
    raise RuntimeError("could not parse AFF3CT output:\n" + out[-1500:])


def measure(args):
    os.makedirs(OUT_DIR, exist_ok=True)
    done = set()
    if os.path.exists(CSV_PATH):
        for row in csv.DictReader(open(CSV_PATH)):
            done.add((row["panel"], row["series"], float(row["ebn0"])))
    new = not os.path.exists(CSV_PATH)
    with open(CSV_PATH, "a", newline="") as fh:
        w = csv.writer(fh)
        if new:
            w.writerow(["panel", "series", "n", "k", "ebn0", "errors", "words", "wer"])
        for title, n, k, grid, series in PANELS:
            for label, kind, sargs in series:
                for db in grid:
                    if (title, label, db) in done:
                        continue
                    if kind == "polar":
                        errs, words = run_polar(n, k, sargs, db, args.aff3ct)
                    else:
                        errs, words = run_ours(kind, sargs, db, args.wer)
                    wer = errs / words
                    w.writerow([title, label, n, k, db, errs, words, f"{wer:.3e}"])
                    fh.flush()
                    print(f"{title:<12} {label:<24} {db:5.2f} dB  WER {wer:.2e} ({errs}/{words})", flush=True)
                    if wer < STOP_BELOW:
                        break


def plot(args):
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    rows = list(csv.DictReader(open(CSV_PATH)))
    ink, muted, grid_c = "#0b0b0b", "#52514e", "#e2e1dc"
    plt.rcParams.update({
        "font.family": "sans-serif", "font.size": 10, "axes.edgecolor": muted, "axes.labelcolor": ink,
        "xtick.color": muted, "ytick.color": muted, "svg.fonttype": "none", "mathtext.fontset": "dejavusans",
    })
    fig, axes = plt.subplots(2, 2, figsize=(10, 7.0), sharey=True)
    handles = {}
    for ax, (title, n, k, grid, series) in zip(axes.flat, PANELS):
        xs = []
        for label, _, _ in series:
            pts = sorted((float(r["ebn0"]), int(r["errors"]), int(r["words"]))
                         for r in rows if r["panel"] == title and r["series"] == label)
            pts = [(x, e / w) for x, e, w in pts if e > 0]
            if not pts:
                continue
            xs += [p[0] for p in pts]
            color, ls, marker = style_for(label)
            (line,) = ax.plot([p[0] for p in pts], [p[1] for p in pts], ls=ls, color=color, lw=2,
                              marker=marker, ms=6, mec="white", mew=1.2)
            handles.setdefault(family(label), line)
        ax.set_yscale("log")
        ax.set_ylim(1e-5, 1)
        step = min(b - a for a, b in zip(grid, grid[1:]))
        ax.set_xlim(min(xs) - step / 4, max(xs) + step / 4)
        ax.set_title(f"n, k = {title}", loc="left", fontsize=11, color=ink, fontweight="bold")
        ax.grid(True, which="major", color=grid_c, lw=0.8)
        ax.grid(True, which="minor", axis="y", color=grid_c, lw=0.4, alpha=0.6)
        ax.axhline(1e-4, color=muted, lw=0.8, ls=":")
        for side in ("top", "right"):
            ax.spines[side].set_visible(False)
    for ax in axes[1]:
        ax.set_xlabel("Eb/N0 (dB)")
    for ax in axes[:, 0]:
        ax.set_ylabel("Word error rate")
    fig.suptitle("ecc-hypercube vs. 5G polar codes, BPSK/AWGN", x=0.065, y=0.985, ha="left",
                 fontsize=13, color=ink, fontweight="bold")
    fig.text(0.065, 0.945, "Solid: ecc-hypercube. Dashed: polar with the same payload k plus CRC-11. "
             "Dotted line: WER 1e-4, where the README tables compare.", fontsize=9, color=muted)
    order = [f for f in ["RM ensemble 32×8", "RM ensemble, fast (8×4; 16×4 at n = 512)",
                         "Hypercube GLDPC (k = 257 at n = 512)", "Polar SCL8 + CRC-11 (AFF3CT)",
                         "Polar ASCL128 + CRC-11 (AFF3CT)"] if f in handles]
    fig.legend([handles[f] for f in order], order, loc="upper left", bbox_to_anchor=(0.058, 0.93),
               ncol=3, frameon=False, fontsize=9, handlelength=2.6, columnspacing=1.6)
    fig.tight_layout(rect=(0, 0, 1, 0.885))
    path = os.path.join(OUT_DIR, "wer_curves.svg")
    fig.savefig(path, format="svg", facecolor="white")
    print(f"wrote {path}")


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest="cmd", required=True)
    m = sub.add_parser("measure")
    m.add_argument("--wer", default=os.path.join(ROOT, "target", "release", "examples", "wer"))
    m.add_argument("--aff3ct", required=True)
    sub.add_parser("plot")
    args = p.parse_args()
    measure(args) if args.cmd == "measure" else plot(args)


if __name__ == "__main__":
    sys.exit(main())
