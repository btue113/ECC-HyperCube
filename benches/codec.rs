//! Run with `cargo bench --bench codec`. Times exclude formatting and I/O.
use ecc_hypercube::CubeCode;
use std::hint::black_box;
use std::time::{Duration, Instant};

fn measure(mut f: impl FnMut()) -> f64 {
    f();
    let start = Instant::now();
    let mut runs = 0;
    while start.elapsed() < Duration::from_millis(150) {
        f();
        runs += 1;
    }
    start.elapsed().as_secs_f64() * 1e6 / runs as f64
}

fn main() {
    println!("m p init_us encode_us clean_decode_us noisy_decode_us");
    for (m, p) in [(4, 1), (8, 3), (12, 5), (15, 7), (12, 0), (12, 11)] {
        let init = measure(|| { black_box(CubeCode::new(m, p).unwrap()); });
        let code = CubeCode::new(m, p).unwrap();
        let mut rng = 42u64;
        let message: Vec<bool> = (0..code.data_len()).map(|_| {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng & 1 != 0
        }).collect();
        let clean = code.encode(&message).unwrap();
        let mut noisy = clean.clone();
        // Distinct, reproducible errors within the correction bound.
        for i in 0..(1usize << p) - 1 {
            let pos = (i * 17 + 3) % noisy.len();
            noisy[pos] ^= true;
        }
        assert_eq!(code.decode(&noisy).unwrap(), message);
        let encode = measure(|| { black_box(code.encode(black_box(&message)).unwrap()); });
        let clean_decode = measure(|| { black_box(code.decode(black_box(&clean)).unwrap()); });
        let noisy_decode = measure(|| { black_box(code.decode(black_box(&noisy)).unwrap()); });
        println!("{m} {p} {init:.3} {encode:.3} {clean_decode:.3} {noisy_decode:.3}");
    }
}
