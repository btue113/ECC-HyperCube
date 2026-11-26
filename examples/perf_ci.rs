//! Fixed decode workloads timed for per-commit performance tracking in CI.
//!
//! ```sh
//! cargo run --release --example perf_ci -- perf.json
//! ```
//!
//! Every workload decodes the same pre-generated frames (fixed seeds) on one thread. Each is
//! timed over several passes and reports the fastest pass in µs per word, which is the least
//! noisy statistic on shared CI runners. The word-error count is deterministic, so a change
//! in it means the decoder's output changed, not just its speed.
//!
//! A workload that panics is reported and skipped, so the rest still run. The `perf-group`
//! tags let `ci/perf_backfill.py` strip workloads whose API an older commit lacks.
//!
//! The JSON output is the `customSmallerIsBetter` format of
//! `benchmark-action/github-action-benchmark`. Workload names are the keys of the history
//! chart: renaming one starts a new chart.
use ecc_hypercube::CubeCode;
use ecc_hypercube::ReedMuller; // perf-group: rm
use ecc_hypercube::{GldpcRate, HypercubeGldpc}; // perf-group: gldpc
use std::hint::black_box;
use std::time::Instant;

const PASSES: usize = 5;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn gaussian(&mut self) -> f64 {
        let a = ((self.next() >> 11) as f64 + 0.5) / ((1u64 << 53) as f64);
        let b = ((self.next() >> 11) as f64 + 0.5) / ((1u64 << 53) as f64);
        (-2.0 * a.ln()).sqrt() * (2.0 * std::f64::consts::PI * b).cos()
    }
}

/// Received LLRs over BPSK/AWGN at `db` Eb/N0 and rate `k / n` (positive favors bit 1).
fn channel(codeword: &[bool], db: f64, k: usize, rng: &mut Rng) -> Vec<f64> {
    let n = codeword.len();
    let sigma = (1.0 / (2.0 * (k as f64 / n as f64) * 10f64.powf(db / 10.0))).sqrt();
    let scale = 2.0 / (sigma * sigma);
    codeword.iter().map(|&b| scale * (if b { 1.0 } else { -1.0 } + sigma * rng.gaussian())).collect()
}

struct Result {
    name: String,
    us_per_word: f64,
    errors: usize,
    frames: usize,
    detail: String,
}

/// Times `decode(frame) -> word error?` over all frames, `PASSES` times; keeps the fastest.
fn time_frames(frames: usize, mut decode: impl FnMut(usize) -> bool) -> (f64, usize) {
    let mut errors = 0;
    for i in 0..frames {
        errors += decode(i) as usize;
    }
    let mut best = f64::INFINITY;
    for _ in 0..PASSES {
        let t0 = Instant::now();
        for i in 0..frames {
            black_box(decode(i));
        }
        best = best.min(t0.elapsed().as_secs_f64() * 1e6 / frames as f64);
    }
    (best, errors)
}

fn hard(m: u32, p: u32, frames: usize) -> Result {
    let code = CubeCode::new(m, p).unwrap();
    let mut rng = Rng(0xC0DE_0000 ^ ((m as u64) << 8) ^ p as u64);
    let (mut msgs, mut words) = (Vec::new(), Vec::new());
    for _ in 0..frames {
        let msg: Vec<bool> = (0..code.data_len()).map(|_| rng.next() & 1 == 1).collect();
        let mut word = code.encode(&msg).unwrap();
        // The guaranteed number of flips, 2^p - 1, at random distinct places.
        let mut hit = vec![false; word.len()];
        let mut flipped = 0;
        while flipped < (1usize << p) - 1 {
            let pos = (rng.next() % word.len() as u64) as usize;
            if !hit[pos] {
                hit[pos] = true;
                word[pos] ^= true;
                flipped += 1;
            }
        }
        msgs.push(msg);
        words.push(word);
    }
    let (us, errors) = time_frames(frames, |i| code.decode(&words[i]).unwrap() != msgs[i]);
    let n = (1usize << m) - 1;
    Result {
        name: format!("hard decode ({n},{}) {} flips", code.data_len(), (1usize << p) - 1),
        us_per_word: us,
        errors,
        frames,
        detail: format!("CubeCode::new({m}, {p})"),
    }
}

// perf-group: rm
/// Fixed AWGN frames for RM(r, m): `(code, messages, received LLRs)`.
fn rm_frames(r: u32, m: u32, copies: usize, db: f64, frames: usize) -> (ReedMuller, Vec<Vec<bool>>, Vec<Vec<f64>>) {
    let rm = ReedMuller::new(r, m).unwrap();
    let k = rm.message_len();
    let mut rng = Rng(0x5EED_0000 ^ ((r as u64) << 16) ^ ((m as u64) << 8) ^ copies as u64);
    let (mut msgs, mut ys) = (Vec::new(), Vec::new());
    for _ in 0..frames {
        let msg: Vec<bool> = (0..k).map(|_| rng.next() & 1 == 1).collect();
        ys.push(channel(&rm.encode(&msg).unwrap(), db, k, &mut rng));
        msgs.push(msg);
    }
    (rm, msgs, ys)
}

// perf-group: scl
fn rm_list(r: u32, m: u32, list: usize, db: f64, frames: usize) -> Result {
    let (rm, msgs, ys) = rm_frames(r, m, 1, db, frames);
    let (n, k) = (rm.block_len(), rm.message_len());
    let (us, errors) = time_frames(frames, |i| rm.decode_soft_list(&ys[i], list).unwrap() != msgs[i]);
    Result {
        name: format!("RM({r},{m}) ({n},{k}) SCL-{list} {db:.1} dB"),
        us_per_word: us,
        errors,
        frames,
        detail: String::new(),
    }
}

// perf-group: ensemble
fn rm_ensemble(r: u32, m: u32, copies: usize, list: usize, db: f64, frames: usize) -> Result {
    let (rm, msgs, ys) = rm_frames(r, m, copies, db, frames);
    let (n, k) = (rm.block_len(), rm.message_len());
    let (us, errors) =
        time_frames(frames, |i| rm.decode_soft_ensemble_checked(&ys[i], list, copies).unwrap().message != msgs[i]);
    Result {
        name: format!("RM({r},{m}) ({n},{k}) ensemble {copies}x{list} {db:.1} dB"),
        us_per_word: us,
        errors,
        frames,
        detail: String::new(),
    }
}

// perf-group: gldpc
fn gldpc(n: usize, db: f64, frames: usize) -> Result {
    let code = HypercubeGldpc::new(n, GldpcRate::Rate1_2).unwrap();
    let (n, k) = (code.codeword_len(), code.message_len());
    let mut rng = Rng(0x61D9_0000 ^ n as u64);
    let (mut cws, mut ys) = (Vec::new(), Vec::new());
    for _ in 0..frames {
        let msg: Vec<bool> = (0..k).map(|_| rng.next() & 1 == 1).collect();
        let cw = code.encode(&msg).unwrap();
        ys.push(channel(&cw, db, k, &mut rng).iter().map(|&v| v as f32).collect::<Vec<f32>>());
        cws.push(cw);
    }
    let mut scratch = code.scratch();
    let (us, errors) = time_frames(frames, |i| {
        code.decode_into_scratch(&ys[i], 50, &mut scratch).unwrap();
        (0..n).any(|b| scratch.bit(b) != cws[i][b])
    });
    Result {
        name: format!("GLDPC ({n},{k}) {db:.1} dB"),
        us_per_word: us,
        errors,
        frames,
        detail: "50 iterations max".into(),
    }
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "perf.json".into());
    type Workload = Box<dyn Fn() -> Result>;
    let workloads: Vec<Workload> = vec![
        Box::new(|| hard(8, 3, 20_000)),
        Box::new(|| hard(12, 5, 500)),
        Box::new(|| rm_list(3, 7, 8, 3.0, 4_000)), // perf-group: scl
        Box::new(|| rm_ensemble(3, 7, 8, 8, 3.0, 4_000)), // perf-group: ensemble
        Box::new(|| rm_ensemble(4, 8, 8, 8, 3.5, 2_000)), // perf-group: ensemble
        Box::new(|| rm_ensemble(4, 9, 4, 16, 3.0, 500)), // perf-group: ensemble
        Box::new(|| gldpc(1024, 2.0, 500)), // perf-group: gldpc
    ];
    let mut results = Vec::new();
    for (i, w) in workloads.iter().enumerate() {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(w)) {
            Ok(r) => results.push(r),
            Err(_) => eprintln!("workload {i} panicked; skipped"),
        }
    }

    let mut json = String::from("[\n");
    for (i, r) in results.iter().enumerate() {
        println!("{:<45} {:>9.2} µs/word  {:>5} / {} word errors", r.name, r.us_per_word, r.errors, r.frames);
        let mut extra = format!("{} / {} word errors", r.errors, r.frames);
        if !r.detail.is_empty() {
            extra = format!("{extra}; {}", r.detail);
        }
        json.push_str(&format!(
            "  {{\"name\": \"{}\", \"unit\": \"us/word\", \"value\": {:.3}, \"extra\": \"{}\"}}{}\n",
            json_escape(&r.name),
            r.us_per_word,
            json_escape(&extra),
            if i + 1 < results.len() { "," } else { "" }
        ));
    }
    json.push(']');
    std::fs::write(&out, json + "\n").unwrap();
    println!("wrote {out}");
}
