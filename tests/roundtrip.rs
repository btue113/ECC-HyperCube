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
