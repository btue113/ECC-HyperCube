#!/usr/bin/env python3
"""Per-commit decode-time tracking, normalized to reference µs/word against a fixed baseline.

GitHub's `ubuntu-latest` runners come in several CPU speeds (1.3-1.6x apart), so raw
timings jump whenever a job lands on different hardware. Every measurement here therefore
builds two programs from today's examples/perf_ci.rs - one against the commit under test, one
against a pinned baseline commit - runs them in paired alternating rounds pinned to one core
(`taskset -c 0`), takes the median per-round ratio `t_rev / t_base`, and scales it by the
baseline's reference `us/word` (`REF_BASE_US`). The chart stays in readable `us/word` while
runner speed cancels out.

    # One commit (the Perf workflow, on every push)
    python3 ci/perf.py measure --rev HEAD --base <sha> --out perf.json
    # Every first-parent commit of HEAD, oldest first (the Perf backfill workflow)
    python3 ci/perf.py backfill --base <sha> --out backfill.json
    # Replace the chart history (gh-pages dev/bench/data.js) with a backfill
    python3 ci/perf.py merge --backfill backfill.json --data path/to/data.js

For each commit, the workload groups whose API that commit lacks are stripped from the template
(`// perf-group: <name>` tags); if a build still fails (a signature changed), the newest
remaining group is dropped and the build retried. Output is the `customSmallerIsBetter` JSON of
benchmark-action/github-action-benchmark: `value` is normalized `us/word`, and `extra` carries
the relative %, raw runner timings, word-error count, CPU model, and rustc version.
"""
import argparse
import io
import json
import os
import platform
import re
import shutil
import statistics
import subprocess
import sys
import tarfile
import time

BENCH_NAME = "ecc-hypercube decode time"
# Which public API each workload group needs; checked by text search in the commit's src/.
GROUP_NEEDS = {
    "scl": ["pub fn decode_soft_list"],
    "ensemble": ["pub fn decode_soft_ensemble_checked"],
    "gldpc": ["pub struct HypercubeGldpc", "pub fn decode_into_scratch", "Rate1_2"],
}
DROP_ORDER = ["gldpc", "ensemble", "scl"]  # newest API first
ROUNDS = 5  # paired alternating commit/baseline rounds; median of per-round ratios is kept

# Median us/word of PERF_BASELINE (179e6b2) on AMD EPYC 9V45, used to scale the same-runner
# rev/base ratio back into readable us/word without runner-CPU jumps.
REF_BASE_US = {
    "hard decode (255,163) 7 flips": 9.55,
    "hard decode (4095,2510) 31 flips": 598.01,
    "RM(3,7) (128,64) SCL-8 3.0 dB": 16.16,
    "RM(3,7) (128,64) ensemble 8x8 3.0 dB": 10.16,
    "RM(4,8) (256,163) ensemble 8x8 3.5 dB": 32.17,
    "RM(4,9) (512,256) ensemble 4x16 3.0 dB": 193.68,
    "GLDPC (1024,513) 2.0 dB": 22.16,
}


def git(*args, binary=False):
    out = subprocess.run(["git", *args], check=True, capture_output=True)
    return out.stdout if binary else out.stdout.decode()


def cpu_model():
    try:
        with open("/proc/cpuinfo", encoding="utf-8") as fh:
            for line in fh:
                if line.startswith("model name"):
                    return line.split(":", 1)[1].strip()
    except OSError:
        pass
    try:
        return subprocess.run(["sysctl", "-n", "machdep.cpu.brand_string"], capture_output=True,
                              text=True, check=True).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return platform.processor() or "unknown CPU"


def strip_groups(template, removed):
    """Drops tagged lines and tagged top-level items (tag line up to the next column-0 `}`)."""
    if "scl" in removed and "ensemble" in removed:
        removed = set(removed) | {"rm"}
    out, skipping = [], False
    for line in template.splitlines(keepends=True):
        if skipping:
            if line.rstrip("\n") == "}":
                skipping = False
            continue
        m = re.match(r"^// perf-group: (\w+)\s*$", line)
        if m:
            skipping = m.group(1) in removed
            continue
        m = re.search(r"// perf-group: (\w+)\s*$", line)
        if m and m.group(1) in removed:
            continue
        out.append(line)
    return "".join(out)


def export_commit(sha, dest):
    if os.path.exists(dest):
        shutil.rmtree(dest)
    os.makedirs(dest)
    with tarfile.open(fileobj=io.BytesIO(git("archive", sha, binary=True))) as tar:
        if hasattr(tarfile, "data_filter"):
            tar.extractall(dest, filter="data")
        else:
            tar.extractall(dest)


def source_text(root):
    text = []
    for dirpath, _, files in os.walk(os.path.join(root, "src")):
        for f in files:
            if f.endswith(".rs"):
                with open(os.path.join(dirpath, f), encoding="utf-8") as fh:
                    text.append(fh.read())
    return "\n".join(text)


def build(sha, template, work):
    """Builds today's perf_ci.rs against `sha`. Returns (binary path or None, groups)."""
    root = os.path.abspath(os.path.join(work, "src-" + sha[:12]))
    export_commit(sha, root)
    groups = [g for g, needs in GROUP_NEEDS.items() if all(n in source_text(root) for n in needs)]
    # A fresh target per commit: `git archive` sets file times to the commit time, so a shared
    # target dir can look up to date and link a previous commit's library.
    target = os.path.join(root, "target")
    while True:
        removed = [g for g in GROUP_NEEDS if g not in groups]
        os.makedirs(os.path.join(root, "examples"), exist_ok=True)
        with open(os.path.join(root, "examples", "perf_ci.rs"), "w", encoding="utf-8") as fh:
            fh.write(strip_groups(template, removed))
        res = subprocess.run(["cargo", "build", "--release", "--example", "perf_ci", "-q"], cwd=root,
                             env=dict(os.environ, CARGO_TARGET_DIR=target), capture_output=True, text=True)
        if res.returncode == 0:
            return os.path.join(target, "release", "examples", "perf_ci"), groups
        first_error = next((l for l in res.stderr.splitlines() if l.startswith("error")), "?")
        drop = next((g for g in DROP_ORDER if g in groups), None)
        print(f"  {sha[:7]}: build failed with groups {groups} ({first_error}); dropping {drop}", file=sys.stderr)
        if drop is None:
            return None, groups
        groups.remove(drop)


def rustc_version():
    try:
        return subprocess.run(["rustc", "--version"], capture_output=True, text=True, check=True).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return "rustc unknown"


def run_bin(binary, work):
    out = os.path.join(work, "perf-run.json")
    cmd = [binary, out]
    if shutil.which("taskset"):
        cmd = ["taskset", "-c", "0", *cmd]
    res = subprocess.run(cmd, capture_output=True, text=True, timeout=1800)
    if res.returncode != 0:
        print(res.stderr[-2000:], file=sys.stderr)
        return None
    return {b["name"]: b for b in json.load(open(out, encoding="utf-8"))}


def relative(rev_bin, base_bin, work, cpu, rustc):
    """Runs both binaries in paired alternating rounds; returns normalized us/word."""
    ratios, raw_rev, raw_base, errors = {}, {}, {}, {}
    order = []
    for rnd in range(ROUNDS):
        if rnd % 2 == 0:
            r_res = run_bin(rev_bin, work)
            b_res = run_bin(base_bin, work) if r_res is not None else None
        else:
            b_res = run_bin(base_bin, work)
            r_res = run_bin(rev_bin, work) if b_res is not None else None
        if r_res is None or b_res is None:
            return None
        for name, r in r_res.items():
            b = b_res.get(name)
            if b is None or b["value"] <= 0:
                continue
            if name not in ratios:
                order.append(name)
                ratios[name], raw_rev[name], raw_base[name] = [], [], []
                errors[name] = r["extra"].split(";")[0]
            ratios[name].append(r["value"] / b["value"])
            raw_rev[name].append(r["value"])
            raw_base[name].append(b["value"])
    benches = []
    for name in order:
        ratio = statistics.median(ratios[name])
        med_rev = statistics.median(raw_rev[name])
        med_base = statistics.median(raw_base[name])
        ref = REF_BASE_US.get(name)
        if ref is None:
            print(f"  warning: {name!r} not in REF_BASE_US; using raw us/word", file=sys.stderr)
            value = round(med_rev, 3)
        else:
            value = round(ref * ratio, 3)
        benches.append({
            "name": name,
            "unit": "us/word",
            "value": value,
            "extra": f"{100 * ratio:.1f}% of baseline (raw {med_rev:.2f} vs {med_base:.2f} us/word); {errors[name]}; {cpu}; {rustc}",
        })
    return benches


def commit_meta(sha, repo_url, username):
    fmt = "%H%x00%T%x00%an%x00%ae%x00%cn%x00%ce%x00%cI%x00%B"
    h, tree, an, ae, cn, ce, ts, msg = git("show", "-s", f"--format={fmt}", sha).split("\x00", 7)
    return {
        "author": {"email": ae, "name": an, "username": username},
        "committer": {"email": ce, "name": cn, "username": username},
        "distinct": True,
        "id": h,
        "message": msg.strip(),
        "timestamp": ts,
        "tree_id": tree,
        "url": f"{repo_url}/commit/{h}",
    }


def setup(args):
    os.makedirs(args.work, exist_ok=True)
    template = open(args.template, encoding="utf-8").read()
    base = git("rev-parse", args.base).strip()
    base_bin, base_groups = build(base, template, args.work)
    if base_bin is None or set(base_groups) != set(GROUP_NEEDS):
        sys.exit(f"baseline {base[:7]} must build every workload group (got {base_groups})")
    cpu = cpu_model()
    rustc = rustc_version()
    print(f"baseline {base[:7]}, runner CPU: {cpu}, {rustc}", flush=True)
    return template, base, base_bin, cpu, rustc


def measure(args):
    template, base, base_bin, cpu, rustc = setup(args)
    rev = git("rev-parse", args.rev).strip()
    rev_bin, groups = build(rev, template, args.work)
    benches = rev_bin and relative(rev_bin, base_bin, args.work, cpu, rustc)
    if not benches:
        sys.exit(f"could not measure {rev[:7]}")
    json.dump(benches, open(args.out, "w", encoding="utf-8"), indent=1)
    for b in benches:
        print(f"{b['name']:<45} {b['value']:9.2f} us/word  ({b['extra']})")


def backfill(args):
    template, base, base_bin, cpu, rustc = setup(args)
    shas = git("rev-list", "--first-parent", "--reverse", args.rev).split()
    if args.limit:
        shas = shas[-args.limit:]
    entries = []
    for i, sha in enumerate(shas, 1):
        rev_bin, groups = build(sha, template, args.work)
        benches = rev_bin and relative(rev_bin, base_bin, args.work, cpu, rustc)
        print(f"[{i}/{len(shas)}] {sha[:7]} " + (f"{len(benches)} workloads, groups {groups}" if benches else "SKIPPED"),
              flush=True)
        if benches:
            entries.append({
                "commit": commit_meta(sha, args.repo_url, args.username),
                "date": int(time.time() * 1000),
                "tool": "customSmallerIsBetter",
                "benches": benches,
            })
        if sha != base:
            shutil.rmtree(os.path.join(args.work, "src-" + sha[:12]), ignore_errors=True)
    json.dump({"baseline": base, "cpu": cpu, "rustc": rustc, "entries": entries}, open(args.out, "w", encoding="utf-8"), indent=1)
    print(f"wrote {len(entries)} entries to {args.out}")


def merge(args):
    entries = json.load(open(args.backfill, encoding="utf-8"))["entries"]
    text = open(args.data, encoding="utf-8").read() if os.path.exists(args.data) else ""
    data = json.loads(text[text.index("{"):].rstrip().rstrip(";")) if text else {"repoUrl": args.repo_url, "entries": {}}
    done = {e["commit"]["id"] for e in entries}
    # Keep later entries only if they are already in the new unit (pushed after the backfill began).
    later = [e for e in data["entries"].get(BENCH_NAME, [])
             if e["commit"]["id"] not in done and all(b.get("unit") == "us/word" for b in e["benches"])]
    data["entries"][BENCH_NAME] = entries + later
    data["lastUpdate"] = int(time.time() * 1000)
    with open(args.data, "w", encoding="utf-8") as fh:
        fh.write("window.BENCHMARK_DATA = " + json.dumps(data, indent=2) + "\n")
    print(f"data.js: {len(entries)} backfilled + {len(later)} later entries")


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest="cmd", required=True)
    for name in ("measure", "backfill"):
        s = sub.add_parser(name)
        s.add_argument("--base", required=True, help="baseline commit (must have every workload)")
        s.add_argument("--rev", default="HEAD")
        s.add_argument("--template", default="examples/perf_ci.rs")
        s.add_argument("--work", default="perf-work")
        s.add_argument("--out", required=True)
        if name == "backfill":
            s.add_argument("--limit", type=int, default=0, help="only the newest N commits")
            s.add_argument("--repo-url", default="https://github.com/btue113/ECC-HyperCube")
            s.add_argument("--username", default="btue113")
    m = sub.add_parser("merge")
    m.add_argument("--backfill", required=True)
    m.add_argument("--data", required=True)
    m.add_argument("--repo-url", default="https://github.com/btue113/ECC-HyperCube")
    args = p.parse_args()
    {"measure": measure, "backfill": backfill, "merge": merge}[args.cmd](args)


if __name__ == "__main__":
    main()
