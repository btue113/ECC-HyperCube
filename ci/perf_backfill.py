#!/usr/bin/env python3
"""Backfill the perf chart: run today's examples/perf_ci.rs against every past commit.

    # 1. Measure every first-parent commit of HEAD (oldest first) into backfill.json
    python3 ci/perf_backfill.py run --out backfill.json
    # 2. Replace the chart history (gh-pages dev/bench/data.js) with those results
    python3 ci/perf_backfill.py merge --backfill backfill.json --data path/to/data.js

`run` exports each commit into its own directory, drops in the current perf_ci.rs with the
workload groups that commit's API lacks stripped out (`// perf-group: <name>` tags), builds
it and runs it. If a build still fails (an API signature changed), it drops the newest
remaining group and retries. Commits where nothing builds are skipped and reported.

`merge` writes the entries in the format of benchmark-action/github-action-benchmark, so the
regular Perf workflow keeps appending to the same history afterwards. Entries already in
data.js for commits that are not in the backfill (pushed later) are kept after it.
"""
import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tarfile
import io
import time

BENCH_NAME = "ecc-hypercube decode time"
# Which public API each workload group needs; checked by text search in the commit's src/.
GROUP_NEEDS = {
    "scl": ["pub fn decode_soft_list"],
    "ensemble": ["pub fn decode_soft_ensemble_checked"],
    "gldpc": ["pub struct HypercubeGldpc", "pub fn decode_into_scratch", "Rate1_2"],
}
DROP_ORDER = ["gldpc", "ensemble", "scl"]  # newest API first


def git(*args, cwd=None, binary=False):
    out = subprocess.run(["git", *args], cwd=cwd, check=True, capture_output=True)
    return out.stdout if binary else out.stdout.decode()


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
    data = git("archive", sha, binary=True)
    with tarfile.open(fileobj=io.BytesIO(data)) as tar:
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


def run(args):
    template = open(args.template, encoding="utf-8").read()
    shas = git("rev-list", "--first-parent", "--reverse", args.rev).split()
    if args.limit:
        shas = shas[-args.limit:]
    os.makedirs(args.work, exist_ok=True)
    entries, report = [], []
    for i, sha in enumerate(shas, 1):
        short = sha[:7]
        root = os.path.join(args.work, "src-" + short)
        # A fresh target per commit: `git archive` sets file times to the commit time, so a
        # shared target dir can look up to date and link a previous commit's library.
        target = os.path.abspath(os.path.join(root, "target"))
        export_commit(sha, root)
        src = source_text(root)
        groups = [g for g, needs in GROUP_NEEDS.items() if all(n in src for n in needs)]
        bench = None
        while True:
            removed = [g for g in GROUP_NEEDS if g not in groups]
            os.makedirs(os.path.join(root, "examples"), exist_ok=True)
            with open(os.path.join(root, "examples", "perf_ci.rs"), "w", encoding="utf-8") as fh:
                fh.write(strip_groups(template, removed))
            env = dict(os.environ, CARGO_TARGET_DIR=target)
            build = subprocess.run(
                ["cargo", "build", "--release", "--example", "perf_ci", "-q"],
                cwd=root, env=env, capture_output=True, text=True,
            )
            if build.returncode == 0:
                out_json = os.path.abspath(os.path.join(args.work, f"perf-{short}.json"))
                res = subprocess.run(
                    [os.path.join(target, "release", "examples", "perf_ci"), out_json],
                    cwd=root, capture_output=True, text=True, timeout=1800,
                )
                if res.returncode == 0:
                    bench = json.load(open(out_json, encoding="utf-8"))
                else:
                    print(res.stderr[-2000:], file=sys.stderr)
                break
            first_error = next((l for l in build.stderr.splitlines() if l.startswith("error")), "?")
            drop = next((g for g in DROP_ORDER if g in groups), None)
            print(f"  {short}: build failed with groups {groups} ({first_error}); dropping {drop}", file=sys.stderr)
            if drop is None:
                break
            groups.remove(drop)
        status = f"{len(bench)} workloads, groups {groups}" if bench else "SKIPPED"
        print(f"[{i}/{len(shas)}] {short} {status}", flush=True)
        report.append({"commit": sha, "groups": groups, "workloads": len(bench) if bench else 0})
        if bench:
            entries.append({
                "commit": commit_meta(sha, args.repo_url, args.username),
                "date": int(time.time() * 1000),
                "tool": "customSmallerIsBetter",
                "benches": bench,
            })
        shutil.rmtree(root, ignore_errors=True)
    json.dump({"entries": entries, "report": report}, open(args.out, "w", encoding="utf-8"), indent=1)
    print(f"wrote {len(entries)} entries to {args.out}")


def merge(args):
    backfill = json.load(open(args.backfill, encoding="utf-8"))["entries"]
    text = open(args.data, encoding="utf-8").read() if os.path.exists(args.data) else ""
    if text:
        data = json.loads(text[text.index("{"):].rstrip().rstrip(";"))
    else:
        data = {"repoUrl": args.repo_url, "entries": {}}
    done = {e["commit"]["id"] for e in backfill}
    later = [e for e in data["entries"].get(BENCH_NAME, []) if e["commit"]["id"] not in done]
    data["entries"][BENCH_NAME] = backfill + later
    data["lastUpdate"] = int(time.time() * 1000)
    with open(args.data, "w", encoding="utf-8") as fh:
        fh.write("window.BENCHMARK_DATA = " + json.dumps(data, indent=2) + "\n")
    print(f"data.js: {len(backfill)} backfilled + {len(later)} later entries")


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest="cmd", required=True)
    r = sub.add_parser("run")
    r.add_argument("--rev", default="HEAD")
    r.add_argument("--template", default="examples/perf_ci.rs")
    r.add_argument("--work", default="perf-backfill")
    r.add_argument("--out", default="backfill.json")
    r.add_argument("--limit", type=int, default=0, help="only the newest N commits")
    r.add_argument("--repo-url", default="https://github.com/btue113/ECC-HyperCube")
    r.add_argument("--username", default="btue113")
    m = sub.add_parser("merge")
    m.add_argument("--backfill", required=True)
    m.add_argument("--data", required=True)
    m.add_argument("--repo-url", default="https://github.com/btue113/ECC-HyperCube")
    args = p.parse_args()
    run(args) if args.cmd == "run" else merge(args)


if __name__ == "__main__":
    main()
