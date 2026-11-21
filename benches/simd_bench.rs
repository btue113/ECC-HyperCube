use ecc_hypercube::ReedMuller;
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

fn bench_single_thread(r: u32, m: u32, db: f64, copies: usize, list_size: usize, frames: usize) {
    let rm = ReedMuller::new(r, m).unwrap();
    let (n, k) = (rm.block_len(), rm.message_len());
    let sigma = (1.0 / (2.0 * (k as f64 / n as f64) * 10f64.powf(db / 10.0))).sqrt();
    let scale = 2.0 / (sigma * sigma);
    let mut rng = Rng(0x1234_5678_9ABC_DEF1 ^ ((n as u64) << 16) ^ ((copies as u64) << 8) ^ list_size as u64);

    // Pre-generate frames so we time ONLY decoding with zero noise-gen overhead
    let mut msgs = Vec::with_capacity(frames);
    let mut ys = Vec::with_capacity(frames);
    for _ in 0..frames {
        let msg: Vec<bool> = (0..k).map(|_| rng.next() & 1 == 1).collect();
        let x = rm.encode(&msg).unwrap();
        let y: Vec<f64> = x
            .iter()
            .map(|&b| scale * ((if b { 1.0 } else { -1.0 }) + sigma * rng.gaussian()))
            .collect();
        msgs.push(msg);
        ys.push(y);
    }

    // Warmup
    for i in 0..frames.min(100) {
        if copies == 1 {
            let _ = rm.decode_soft_list(&ys[i], list_size).unwrap();
        } else {
            let _ = rm.decode_soft_ensemble_checked(&ys[i], list_size, copies).unwrap();
        }
    }

    let mut errs = 0usize;
    let mut cert_errs = 0usize;
    let mut uncert = 0usize;
    let t0 = Instant::now();
    for i in 0..frames {
        if copies == 1 {
            let got = rm.decode_soft_list(&ys[i], list_size).unwrap();
            if got != msgs[i] {
                errs += 1;
            }
        } else {
            let d = rm.decode_soft_ensemble_checked(&ys[i], list_size, copies).unwrap();
            if d.message != msgs[i] {
                errs += 1;
                if d.certified {
                    cert_errs += 1;
                }
            }
            if !d.certified {
                uncert += 1;
            }
        }
    }
    let elapsed_us = t0.elapsed().as_secs_f64() * 1e6;
    let us_per_word = elapsed_us / frames as f64;
    let mbps = (frames as f64 * k as f64) / elapsed_us; // bits / us = Mbps
    println!(
        "({n:>3},{k:>3}) {db:>3.1}dB {copies:>2}x{list_size:<2} | {frames:>6}w | errs={errs:>4} ({:.2e}) | uncert={:.2} | cert_err={cert_errs} | {us_per_word:>6.2} us/w | {mbps:>6.2} Mbps",
        errs as f64 / frames as f64,
        uncert as f64 / frames as f64,
    );
}

fn main() {
    println!("=== Single-Thread Decode Benchmark (pure decode time, pre-generated frames) ===");
    bench_single_thread(3, 7, 3.0, 1, 8, 20_000);
    bench_single_thread(3, 7, 3.0, 1, 32, 10_000);
    bench_single_thread(3, 7, 3.5, 8, 4, 20_000);
    bench_single_thread(3, 7, 3.5, 32, 8, 20_000);
    bench_single_thread(3, 8, 2.5, 1, 8, 10_000);
    bench_single_thread(3, 8, 2.5, 1, 32, 5_000);
    bench_single_thread(3, 8, 2.5, 8, 4, 5_000);
    bench_single_thread(3, 8, 2.5, 32, 8, 3_000);
    bench_single_thread(4, 8, 3.5, 8, 4, 10_000);
    bench_single_thread(4, 8, 3.5, 32, 8, 5_000);
    bench_single_thread(4, 9, 2.5, 16, 8, 1_000);
    bench_single_thread(4, 9, 3.0, 16, 8, 2_000);
}
