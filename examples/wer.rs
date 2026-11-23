//! Word error rate and decode time over BPSK/AWGN, using only the public API and std.
//!
//! ```sh
//! cargo run --release --example wer -- rm <r> <m> <copies> <list> <dbs> [max_words] [max_errs]
//! cargo run --release --example wer -- gldpc <n> <3/8|1/2> <dbs> [max_words] [max_errs]
//! cargo run --release --example wer -- gldpc-noise <n> <3/8|1/2> <trials> [db]
//! ```
//!
//! `dbs` is a comma-separated list of Eb/N0 values in dB, with the rate taken as k/n. Each point
//! sends random messages until `max_errs` word errors (default 100) or `max_words` words
//! (default 1,000,000). Time is the decode call only, in µs per word per thread.
//! `gldpc-noise` decodes pure noise and counts how often the decoder claims a valid codeword.
use ecc_hypercube::{GldpcRate, HypercubeGldpc, ReedMuller};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

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

/// Noise standard deviation for unit-energy BPSK at `db` Eb/N0 and rate `k / n`.
fn sigma(db: f64, n: usize, k: usize) -> f64 {
    (1.0 / (2.0 * (k as f64 / n as f64) * 10f64.powf(db / 10.0))).sqrt()
}

/// Received LLRs for `codeword` (positive favors bit 1, the crate's convention).
fn channel(codeword: &[bool], sigma: f64, rng: &mut Rng) -> Vec<f64> {
    let scale = 2.0 / (sigma * sigma);
    codeword
        .iter()
        .map(|&b| scale * (if b { 1.0 } else { -1.0 } + sigma * rng.gaussian()))
        .collect()
}

/// Runs `trial` on every thread until the shared error or word budget is spent. Each thread
/// gets its own decoder state from `init`: a `ReedMuller` locks its list decoder while decoding,
/// so threads must not share one. `trial` returns (word error, decode nanoseconds).
/// Prints errors, words, WER and µs per word per thread.
fn simulate<S, I, F>(label: &str, max_words: u64, max_errs: u64, init: I, trial: F)
where
    I: Fn() -> S + Sync,
    F: Fn(&mut S, &mut Rng) -> (bool, u128) + Sync,
{
    let threads = std::thread::available_parallelism().map_or(1, |t| t.get()).min(8) as u64;
    let (words, errs, nanos) = (AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0));
    std::thread::scope(|s| {
        for t in 0..threads {
            let (words, errs, nanos, init, trial) = (&words, &errs, &nanos, &init, &trial);
            s.spawn(move || {
                let mut state = init();
                let mut rng = Rng(0x9E37_79B9_7F4A_7C15u64.wrapping_mul(t + 1) | 1);
                while errs.load(Ordering::Relaxed) < max_errs
                    && words.fetch_add(1, Ordering::Relaxed) < max_words
                {
                    let (err, ns) = trial(&mut state, &mut rng);
                    errs.fetch_add(u64::from(err), Ordering::Relaxed);
                    nanos.fetch_add(ns as u64, Ordering::Relaxed);
                }
            });
        }
    });
    // The loop condition over-counts one word per thread that stopped on the word budget.
    let w = words.load(Ordering::Relaxed).min(max_words);
    let e = errs.load(Ordering::Relaxed);
    let us = nanos.load(Ordering::Relaxed) as f64 / 1000.0 / w as f64;
    println!("{label} | errors {e:>4} / {w:>8} words | WER {:.2e} | {us:>8.1} µs/word/thread ({threads} threads)", e as f64 / w as f64);
}

fn parse_rate(s: &str) -> GldpcRate {
    match s {
        "3/8" => GldpcRate::Rate3_8,
        "1/2" => GldpcRate::Rate1_2,
        _ => panic!("rate must be 3/8 or 1/2, got {s}"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg = |i: usize| args.get(i).unwrap_or_else(|| panic!("missing argument {i}; see examples/wer.rs"));
    let num = |i: usize, default: u64| args.get(i).map_or(default, |s| s.parse().unwrap());
    let dbs = |i: usize| -> Vec<f64> { arg(i).split(',').map(|s| s.parse().unwrap()).collect() };

    match arg(0).as_str() {
        "rm" => {
            let (r, m, copies, list) = (num(1, 0) as u32, num(2, 0) as u32, num(3, 1) as usize, num(4, 8) as usize);
            let probe = ReedMuller::new(r, m).unwrap();
            let (n, k) = (probe.block_len(), probe.message_len());
            for db in dbs(5) {
                let sigma = sigma(db, n, k);
                let init = || ReedMuller::new(r, m).unwrap();
                simulate(&format!("RM({r},{m}) ({n},{k}) {copies}x{list} {db:.2} dB"), num(6, 1_000_000), num(7, 100), init, |rm, rng| {
                    let msg: Vec<bool> = (0..k).map(|_| rng.next() & 1 == 1).collect();
                    let y = channel(&rm.encode(&msg).unwrap(), sigma, rng);
                    let t0 = Instant::now();
                    let d = rm.decode_soft_ensemble_checked(&y, list, copies).unwrap();
                    (d.message != msg, t0.elapsed().as_nanos())
                });
            }
        }
        "gldpc" => {
            let code = HypercubeGldpc::new(num(1, 0) as usize, parse_rate(arg(2))).unwrap();
            let (n, k) = (code.codeword_len(), code.message_len());
            for db in dbs(3) {
                let sigma = sigma(db, n, k);
                simulate(&format!("GLDPC ({n},{k}) {db:.2} dB"), num(4, 1_000_000), num(5, 100), || code.scratch(), |scratch, rng| {
                    let msg: Vec<bool> = (0..k).map(|_| rng.next() & 1 == 1).collect();
                    let cw = code.encode(&msg).unwrap();
                    let y: Vec<f32> = channel(&cw, sigma, rng).iter().map(|&v| v as f32).collect();
                    let t0 = Instant::now();
                    code.decode_into_scratch(&y, 50, scratch).unwrap();
                    let ns = t0.elapsed().as_nanos();
                    ((0..n).any(|i| scratch.bit(i) != cw[i]), ns)
                });
            }
        }
        "gldpc-noise" => {
            let code = HypercubeGldpc::new(num(1, 0) as usize, parse_rate(arg(2))).unwrap();
            let (n, k) = (code.codeword_len(), code.message_len());
            let (trials, db) = (num(3, 1_000_000), args.get(4).map_or(2.5, |s| s.parse().unwrap()));
            let sigma = sigma(db, n, k);
            // Here an "error" is a false alarm: pure noise decoded to a valid codeword.
            simulate(&format!("GLDPC ({n},{k}) noise only, LLR scale of {db:.2} dB"), trials, u64::MAX, || code.scratch(), |scratch, rng| {
                let y: Vec<f32> = channel(&vec![false; n], sigma, rng)
                    .iter()
                    .map(|&v| (v + 2.0 / (sigma * sigma)) as f32)
                    .collect();
                let t0 = Instant::now();
                let (_, converged) = code.decode_into_scratch(&y, 50, scratch).unwrap();
                (converged, t0.elapsed().as_nanos())
            });
        }
        other => panic!("unknown mode {other}; use rm, gldpc or gldpc-noise"),
    }
}
