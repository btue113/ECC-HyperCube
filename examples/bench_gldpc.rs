use ecc_hypercube::{GldpcRate, HypercubeGldpc};
use std::sync::atomic::{AtomicUsize, Ordering};
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

fn bench_point(
    code: &HypercubeGldpc,
    db: f64,
    max_iters: usize,
    max_words: usize,
    max_errs: usize,
    threads: usize,
) {
    let n = code.codeword_len();
    let k = code.message_len();
    let rate = k as f64 / n as f64;
    let sigma = (1.0 / (2.0 * rate * 10f64.powf(db / 10.0))).sqrt();
    let scale = (2.0 / (sigma * sigma)) as f32;

    let total_words = AtomicUsize::new(0);
    let total_errs = AtomicUsize::new(0);
    let total_undet = AtomicUsize::new(0);
    let total_hi = AtomicUsize::new(0);
    let total_dec_ns = AtomicUsize::new(0);

    std::thread::scope(|s| {
        for t in 0..threads {
            let total_words = &total_words;
            let total_errs = &total_errs;
            let total_undet = &total_undet;
            let total_hi = &total_hi;
            let total_dec_ns = &total_dec_ns;
            s.spawn(move || {
                let mut rng = Rng(
                    0x61D9_1976u64
                        ^ ((t as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15))
                        ^ ((n as u64) << 16)
                        ^ ((k as u64) << 4)
                        ^ db.to_bits(),
                );
                let mut ch = vec![0.0f32; n];
                let mut scratch = code.scratch();
                let mut local_ns = 0u64;
                loop {
                    let w = total_words.fetch_add(1, Ordering::Relaxed);
                    if w >= max_words || total_errs.load(Ordering::Relaxed) >= max_errs {
                        total_words.fetch_sub(1, Ordering::Relaxed);
                        break;
                    }
                    for dst in &mut ch {
                        *dst = scale * (-1.0 + sigma * rng.gaussian()) as f32;
                    }
                    let td = Instant::now();
                    let (hi, converged) = code
                        .decode_into_scratch(&ch, max_iters, &mut scratch)
                        .unwrap();
                    local_ns += td.elapsed().as_nanos() as u64;
                    total_hi.fetch_add(hi, Ordering::Relaxed);
                    if scratch.any_bit_one() {
                        total_errs.fetch_add(1, Ordering::Relaxed);
                        if converged {
                            total_undet.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
                total_dec_ns.fetch_add(local_ns as usize, Ordering::Relaxed);
            });
        }
    });

    let words = total_words.load(Ordering::Relaxed);
    let errs = total_errs.load(Ordering::Relaxed);
    let undet = total_undet.load(Ordering::Relaxed);
    let avg_hi = total_hi.load(Ordering::Relaxed) as f64 / words as f64;
    let us_per_word_per_thread =
        (total_dec_ns.load(Ordering::Relaxed) as f64) / 1000.0 / (words as f64);
    println!(
        "GLDPC ({n:>4},{k:>3}) R={rate:.4} @ {db:.1} dB | errs={errs:>4} (undet={undet:>2}) / {words:<7} | WER={:.2e} | avg_hi={avg_hi:.2} | {us_per_word_per_thread:>5.1} us/w/t",
        errs as f64 / words as f64
    );
}

fn main() {
    let threads = 8usize;
    println!("=== Hypercube GLDPC HR-FWHT Benchmark ({threads}T, pure decode us/w/t) ===");

    let c512_r38 = HypercubeGldpc::new(512, GldpcRate::Rate3_8).unwrap();
    let c1024_r38 = HypercubeGldpc::new(1024, GldpcRate::Rate3_8).unwrap();

    bench_point(&c512_r38, 2.5, 24, 200_000, 100, threads);
    bench_point(&c512_r38, 3.0, 24, 200_000, 100, threads);
    bench_point(&c1024_r38, 2.5, 24, 200_000, 100, threads);
    bench_point(&c1024_r38, 3.0, 24, 300_000, 100, threads);
}
