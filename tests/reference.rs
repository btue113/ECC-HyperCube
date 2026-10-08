//! Checks the transform-based codec against a direct port of the original C++
//! algorithm on arbitrary words, including ones far beyond the error bound,
//! where only an exact reimplementation gives the same answer.

use ecc_hypercube::{CubeCode, HyperCube};

/// The original algorithm: explicit cube element enumeration and voting.
struct Reference {
    m: u32,
    p: u32,
    cw_len: usize,
    data_len: usize,
    bit_to_cube: Vec<u32>,
    cube_to_bit: Vec<usize>,
    levels: Vec<usize>,
}

impl Reference {
    fn new(code: &CubeCode, m: u32, p: u32) -> Self {
        Self {
            m,
            p,
            cw_len: code.codeword_len(),
            data_len: code.data_len(),
            bit_to_cube: code.bit_to_cube().to_vec(),
            cube_to_bit: code.cube_to_bit().to_vec(),
            levels: code.cubes_in_level().to_vec(),
        }
    }

    fn unit(&self, bit: usize) -> HyperCube {
        HyperCube::unit(self.m, self.bit_to_cube[bit])
    }

    fn parity(&self, code: &[bool], cube: HyperCube) -> bool {
        cube.elements().fold(false, |acc, v| acc ^ code[self.cube_to_bit[v as usize]])
    }

    fn encode(&self, data: &[bool]) -> Vec<bool> {
        let mut code = data.to_vec();
        code.resize(self.cw_len, false);
        for bit in self.data_len..self.cw_len {
            code[bit] = self.parity(&code, self.unit(bit));
        }
        code
    }

    fn decode(&self, received: &[bool]) -> Vec<bool> {
        if (self.data_len..self.cw_len).all(|b| !self.parity(received, self.unit(b))) {
            return received[..self.data_len].to_vec();
        }
        let votes = (1usize << (self.p + 1)) - 1;
        let needed = 1usize << self.p;
        let mut code = received.to_vec();
        let mut coefficients = vec![false; self.cw_len];
        for dim in (0..(self.m - self.p) as usize).rev() {
            let mut mask = vec![false; self.cw_len];
            for bit in self.levels[dim]..self.levels[dim + 1] {
                let mut tally = [0usize; 2];
                for cube in self.unit(bit).parallel_cubes(votes) {
                    let v = self.parity(&code, cube);
                    tally[v as usize] += 1;
                    if tally[v as usize] == needed {
                        mask[bit] = v;
                        break;
                    }
                }
                coefficients[bit] |= mask[bit];
            }
            for upper in dim + 1..dim + self.p as usize {
                for bit in self.levels[upper]..self.levels[upper + 1] {
                    mask[bit] = self.parity(&mask, self.unit(bit));
                }
            }
            for (c, m) in code.iter_mut().zip(&mask) {
                *c ^= m;
            }
        }
        (0..self.data_len).map(|bit| self.parity(&coefficients, self.unit(bit))).collect()
    }
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Bits that are 1 with probability 1/2^sparsity (sparsity >= 1).
    fn bits(&mut self, len: usize, sparsity: u32) -> Vec<bool> {
        (0..len).map(|_| self.next() >> (64 - sparsity) == 0).collect()
    }
}

#[test]
fn matches_original_algorithm() {
    let mut rng = Rng(0xDEC0DE);
    for m in 1..=10 {
        for p in 0..m {
            let code = CubeCode::new(m, p).unwrap();
            let reference = Reference::new(&code, m, p);
            for trial in 0..12 {
                let data = rng.bits(code.data_len(), 1);
                assert_eq!(code.encode(&data).unwrap(), reference.encode(&data), "encode m={m} p={p}");

                // Trials 0-5: a codeword with no noise up to half its bits
                // flipped; 6-11: a uniformly random word.
                let received = if trial < 6 {
                    let noise = if trial == 0 { vec![false; code.codeword_len()] } else { rng.bits(code.codeword_len(), 6 - trial) };
                    reference.encode(&data).iter().zip(noise).map(|(c, n)| c ^ n).collect()
                } else {
                    rng.bits(code.codeword_len(), 1)
                };
                assert_eq!(code.decode(&received).unwrap(), reference.decode(&received), "decode m={m} p={p} trial={trial}");
            }
        }
    }
}
