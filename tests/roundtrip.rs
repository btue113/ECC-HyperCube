//! Encode random messages, flip up to the guaranteed number of bits, decode.

use ecc_hypercube::ReedMuller;

/// xorshift64*, enough for test data without pulling in a dependency.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    fn gaussian(&mut self) -> f64 {
        let u1 = ((self.next() >> 11) as f64 + 0.5) / ((1u64 << 53) as f64);
        let u2 = ((self.next() >> 11) as f64 + 0.5) / ((1u64 << 53) as f64);
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

fn check(r: u32, m: u32, trials: usize, rng: &mut Rng) {
    let rm = ReedMuller::new(r, m).unwrap();
    for _ in 0..trials {
        let message: Vec<bool> = (0..rm.message_len()).map(|_| rng.next() & 1 == 1).collect();
        let mut received = rm.encode(&message).unwrap();
        assert_eq!(received.len(), rm.block_len());

        let errors = rng.below(rm.error_tolerance() + 1);
        let mut positions: Vec<usize> = (0..received.len()).collect();
        for i in 0..errors {
            let j = i + rng.below(positions.len() - i);
            positions.swap(i, j);
            received[positions[i]] ^= true;
        }
        assert_eq!(rm.decode(&received).unwrap(), message, "RM({r},{m}) with {errors} errors");
    }
}

#[test]
fn small_codes() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for m in 2..=8 {
        for r in 0..m {
            check(r, m, 50, &mut rng);
        }
    }
}

#[test]
fn large_code() {
    let mut rng = Rng(42);
    check(7, 15, 2, &mut rng);
}

#[test]
fn scl_benchmark_3db() {
    // Word error rate at Eb/N0 = 3 dB (2000 words per cell) matching the user's table.
    let mut rng = Rng(0x1234_5678_9ABC_DEF0);
    let eb_n0_linear = 10.0f64.powf(3.0 / 10.0); // 3 dB
    let trials = 2000;

    println!("\n=== SCL Worktree (Eb/N0 = 3 dB, {trials} words) ===");
    println!("{:<12} | {:<14} | {:<14} | {:<14}", "Code (n,k)", "RM hard", "SCL (L=8)", "SCL (L=32)");
    println!("{:-<12}-+-{:-<14}-+-{:-<14}-+-{:-<14}", "", "", "", "");

    for (r, m) in [(1, 3), (1, 4), (2, 5), (3, 5), (3, 6), (3, 7)] {
        let rm = ReedMuller::new(r, m).unwrap();
        let n = rm.block_len();
        let k = rm.message_len();
        let rate = rm.rate();
        let sigma = (1.0 / (2.0 * rate * eb_n0_linear)).sqrt();
        let scale = 2.0 / (sigma * sigma);

        let mut hard_err = 0;
        let mut scl8_err = 0;
        let mut scl32_err = 0;

        for _ in 0..trials {
            let message: Vec<bool> = (0..k).map(|_| rng.next() & 1 == 1).collect();
            let encoded = rm.encode(&message).unwrap();
            let llrs: Vec<f64> = encoded
                .iter()
                .map(|&bit| {
                    let tx = if bit { 1.0 } else { -1.0 };
                    let rx = tx + sigma * rng.gaussian();
                    scale * rx
                })
                .collect();
            let hard_rx: Vec<bool> = llrs.iter().map(|&y| y > 0.0).collect();

            if rm.decode(&hard_rx).unwrap() != message {
                hard_err += 1;
            }
            if rm.decode_soft_list(&llrs, 8).unwrap() != message {
                scl8_err += 1;
            }
            if rm.decode_soft_list(&llrs, 32).unwrap() != message {
                scl32_err += 1;
            }
        }
        println!(
            "({n},{k}){:<6} | {:.3}          | {:.3}          | {:.3}",
            "",
            hard_err as f64 / trials as f64,
            scl8_err as f64 / trials as f64,
            scl32_err as f64 / trials as f64,
        );
        assert!(scl8_err < hard_err);
    }
}

/// Sum of |llr| over the positions where `word` disagrees with the hard decision.
fn disagreement(word: &[bool], llrs: &[f64]) -> f64 {
    word.iter().zip(llrs).filter(|&(&c, &l)| c != (l > 0.0)).map(|(_, &l)| l.abs()).sum()
}

#[test]
fn ensemble_never_worse_than_plain_scl() {
    let mut rng = Rng(0xAE5C);
    for (r, m, db) in [(1, 4, 1.0), (2, 5, 2.0), (3, 7, 2.5), (3, 8, 2.0)] {
        let rm = ReedMuller::new(r, m).unwrap();
        let sigma = (1.0 / (2.0 * rm.rate() * 10f64.powf(db / 10.0))).sqrt();
        let mut ensemble_err = 0;
        for _ in 0..200 {
            let message: Vec<bool> = (0..rm.message_len()).map(|_| rng.next() & 1 == 1).collect();
            let encoded = rm.encode(&message).unwrap();
            let llrs: Vec<f64> = encoded.iter()
                .map(|&b| (if b { 1.0 } else { -1.0 }) + sigma * rng.gaussian())
                .collect();
            let plain = rm.decode_soft_list(&llrs, 4).unwrap();
            assert_eq!(rm.decode_soft_ensemble(&llrs, 4, 1).unwrap(), plain);
            let ensemble = rm.decode_soft_ensemble(&llrs, 4, 8).unwrap();
            let cost = |msg: &[bool]| disagreement(&rm.encode(msg).unwrap(), &llrs);
            assert!(cost(&ensemble) <= cost(&plain) + 1e-9, "RM({r},{m})");
            ensemble_err += usize::from(ensemble != message);
        }
        assert!(ensemble_err < 200, "RM({r},{m}) ensemble never decoded");
    }
    // RM(4, 5) has no parity shell (p_dims = 0), so there is nothing to relabel.
    assert!(ReedMuller::new(4, 5).unwrap().decode_soft_ensemble(&vec![1.0; 32], 8, 4).is_err());
}

#[test]
fn certified_decisions_are_maximum_likelihood() {
    let mut rng = Rng(0xCE27);
    for (r, m, db) in [(1, 4, 0.0), (2, 4, 1.0), (1, 5, 0.0), (2, 5, 2.0)] {
        let rm = ReedMuller::new(r, m).unwrap();
        let k = rm.message_len();
        let codewords: Vec<(Vec<bool>, Vec<bool>)> = (0..1usize << k)
            .map(|i| {
                let msg: Vec<bool> = (0..k).map(|b| i >> b & 1 == 1).collect();
                (rm.encode(&msg).unwrap(), msg)
            })
            .collect();
        let sigma = (1.0 / (2.0 * rm.rate() * 10f64.powf(db / 10.0))).sqrt();
        let (mut certified, trials) = (0, 300);
        for _ in 0..trials {
            let sent = &codewords[rng.below(codewords.len())].0;
            let llrs: Vec<f64> = sent.iter()
                .map(|&b| (if b { 1.0 } else { -1.0 }) + sigma * rng.gaussian())
                .collect();
            let cost = |word: &[bool]| -> f64 {
                word.iter().zip(&llrs).filter(|&(&c, &l)| c != (l > 0.0)).map(|(_, &l)| l.abs()).sum()
            };
            let (ml_word, ml_msg) = codewords.iter()
                .min_by(|a, b| cost(&a.0).total_cmp(&cost(&b.0))).unwrap();
            let decisions = [
                rm.decode_soft_checked(&llrs, 4).unwrap(),
                rm.decode_soft_ensemble_checked(&llrs, 1, 8).unwrap(),
                rm.decode_soft_ensemble_checked(&llrs, 4, 4).unwrap(),
            ];
            for d in decisions {
                assert!((d.cost - cost(&rm.encode(&d.message).unwrap())).abs() < 1e-9);
                assert!(d.runner_up_cost >= d.cost - 1e-9);
                if d.certified {
                    certified += 1;
                    assert_eq!(&d.message, ml_msg, "RM({r},{m}) certified a non-ML word");
                    assert!(d.cost <= cost(ml_word) + 1e-9);
                }
            }
        }
        assert!(certified > 0, "RM({r},{m}) never certified");
    }
    assert!(ReedMuller::new(4, 5).unwrap().decode_soft_checked(&vec![1.0; 32], 8).is_err());
}
