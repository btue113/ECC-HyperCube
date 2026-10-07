//! The punctured hypercube code.
//!
//! Every non-zero vertex `c` of the `cw_dims`-cube owns one codeword bit. Bits
//! are ordered by level: level `i` holds the vertices with `cw_dims - i` ones,
//! in descending numeric order. The first `cw_dims - p_dims` levels carry the
//! message; the rest carry parity, where each parity bit makes the XOR over
//! its unit cube zero.
//!
//! Decoding recovers each data level, highest weight first, by majority vote
//! over `2^(p_dims+1) - 1` parallel cubes, then strips the decided value
//! out of the received word before moving to the next level.

use crate::hypercube::HyperCube;
use crate::{Bits, Error};
use std::cmp::Reverse;

/// Hard cap that keeps the vertex tables comfortably within memory.
pub const MAX_DIMS: u32 = 24;

#[derive(Debug)]
pub struct CubeCode {
    cw_dims: u32,
    p_dims: u32,
    cw_len: usize,
    data_len: usize,
    needed_votes: usize,
    bit_to_cube: Vec<u32>,
    cube_to_bit: Vec<usize>,
    cubes_in_level: Vec<usize>,
    /// Voting cubes for every data bit, indexed by bit.
    voters: Vec<Vec<HyperCube>>,
}

impl CubeCode {
    pub fn new(cw_dims: u32, p_dims: u32) -> Result<Self, Error> {
        if cw_dims == 0 || cw_dims > MAX_DIMS || p_dims >= cw_dims {
            return Err(Error::InvalidParams { cw_dims, p_dims });
        }
        let cw_len = (1usize << cw_dims) - 1;

        let mut bit_to_cube: Vec<u32> = (1..=cw_len as u32).collect();
        bit_to_cube.sort_unstable_by_key(|&c| Reverse((c.count_ones(), c)));

        let mut cube_to_bit = vec![0; cw_len + 1];
        for (bit, &cube) in bit_to_cube.iter().enumerate() {
            cube_to_bit[cube as usize] = bit;
        }

        let mut cubes_in_level = vec![0; cw_dims as usize + 1];
        for level in 0..cw_dims as usize {
            let ones = cw_dims as usize - level;
            let count = bit_to_cube.iter().filter(|c| c.count_ones() as usize == ones).count();
            cubes_in_level[level + 1] = cubes_in_level[level] + count;
        }

        let data_len = cubes_in_level[(cw_dims - p_dims) as usize];
        let votes = (1usize << (p_dims + 1)) - 1;
        let voters = bit_to_cube[..data_len]
            .iter()
            .map(|&cube| HyperCube::unit(cw_dims, cube).parallel_cubes(votes))
            .collect();

        Ok(Self {
            cw_dims,
            p_dims,
            cw_len,
            data_len,
            needed_votes: 1 << p_dims,
            bit_to_cube,
            cube_to_bit,
            cubes_in_level,
            voters,
        })
    }

    pub fn data_len(&self) -> usize {
        self.data_len
    }

    pub fn codeword_len(&self) -> usize {
        self.cw_len
    }

    pub fn bit_to_cube(&self) -> &[u32] {
        &self.bit_to_cube
    }

    pub fn cube_to_bit(&self) -> &[usize] {
        &self.cube_to_bit
    }

    pub fn cubes_in_level(&self) -> &[usize] {
        &self.cubes_in_level
    }

    pub fn voters(&self, bit: usize) -> &[HyperCube] {
        &self.voters[bit]
    }

    fn unit_cube(&self, bit: usize) -> HyperCube {
        HyperCube::unit(self.cw_dims, self.bit_to_cube[bit])
    }

    /// XOR of the codeword bits that sit on the vertices of `cube`.
    fn parity(&self, code: &[bool], cube: HyperCube) -> bool {
        cube.elements().fold(false, |acc, v| acc ^ code[self.cube_to_bit[v as usize]])
    }

    pub fn encode(&self, data: &[bool]) -> Result<Bits, Error> {
        check_len(self.data_len, data.len())?;
        let mut code = data.to_vec();
        code.resize(self.cw_len, false);
        for bit in self.data_len..self.cw_len {
            code[bit] = self.parity(&code, self.unit_cube(bit));
        }
        Ok(code)
    }

    pub fn decode(&self, received: &[bool]) -> Result<Bits, Error> {
        check_len(self.cw_len, received.len())?;
        let clean = (self.data_len..self.cw_len)
            .all(|bit| !self.parity(received, self.unit_cube(bit)));
        if clean {
            return Ok(received[..self.data_len].to_vec());
        }

        let mut code = received.to_vec();
        let mut coefficients = vec![false; self.cw_len];
        let data_dims = (self.cw_dims - self.p_dims) as usize;
        for dim in (0..data_dims).rev() {
            let mut mask = vec![false; self.cw_len];
            for bit in self.level(dim) {
                mask[bit] = self.vote(&code, bit);
                coefficients[bit] |= mask[bit];
            }
            // Spread the decided coefficients over the next p_dims - 1 levels
            // so they can be cancelled out of the received word.
            for upper in dim + 1..dim + self.p_dims as usize {
                for bit in self.level(upper) {
                    if self.parity(&mask, self.unit_cube(bit)) {
                        mask[bit] = true;
                    }
                }
            }
            for (c, m) in code.iter_mut().zip(&mask) {
                *c ^= m;
            }
        }
        Ok(self.code_from_coefficients(&coefficients))
    }

    fn level(&self, level: usize) -> std::ops::Range<usize> {
        self.cubes_in_level[level]..self.cubes_in_level[level + 1]
    }

    /// Majority vote over the bit's parallel cubes.
    fn vote(&self, code: &[bool], bit: usize) -> bool {
        let mut votes = [0usize; 2];
        for &cube in &self.voters[bit] {
            let v = self.parity(code, cube);
            votes[v as usize] += 1;
            if votes[v as usize] == self.needed_votes {
                return v;
            }
        }
        false
    }

    /// Turns the decoded coefficients back into the original data bits.
    pub fn code_from_coefficients(&self, coefficients: &[bool]) -> Bits {
        (0..self.data_len)
            .map(|bit| self.parity(coefficients, self.unit_cube(bit)))
            .collect()
    }
}

fn check_len(expected: usize, got: usize) -> Result<(), Error> {
    if expected == got {
        Ok(())
    } else {
        Err(Error::Length { expected, got })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{format_bits, parse_bits};

    fn bits(s: &str) -> Bits {
        parse_bits(s).unwrap()
    }

    #[test]
    fn initialization() {
        let code = CubeCode::new(4, 1).unwrap();
        assert_eq!(code.bit_to_cube(), &[15, 14, 13, 11, 7, 12, 10, 9, 6, 5, 3, 8, 4, 2, 1]);
        assert_eq!(code.cube_to_bit(), &[0, 14, 13, 10, 12, 9, 8, 4, 11, 7, 6, 3, 5, 2, 1, 0]);
        assert_eq!(code.cubes_in_level(), &[0, 1, 5, 11, 15]);
        assert_eq!(code.data_len(), 11);
        // bit 4 is cube 7; its first voter is the all-ones assignment
        assert_eq!(code.voters(4)[0], HyperCube::unit(4, 7));
        assert_eq!(code.voters(4).len(), 3);
    }

    #[test]
    fn rejects_bad_params() {
        assert!(CubeCode::new(4, 4).is_err());
        assert!(CubeCode::new(0, 0).is_err());
        assert!(CubeCode::new(MAX_DIMS + 1, 1).is_err());
    }

    #[test]
    fn code_from_coefficients() {
        let code = CubeCode::new(4, 1).unwrap();
        let actual = code.code_from_coefficients(&bits("111011110000000"));
        // the C++ test only derived the first 7 data bits
        assert_eq!(format_bits(&actual[..7]), "1001001");
    }

    #[test]
    fn encoding() {
        let cases = [(1, "10010010010", "100100100101011"), (2, "10110", "101100011000110"), (3, "1", "111111111111111")];
        for (p_dims, data, expected) in cases {
            let code = CubeCode::new(4, p_dims).unwrap();
            assert_eq!(format_bits(&code.encode(&bits(data)).unwrap()), expected);
        }
    }

    #[test]
    fn encode_rejects_wrong_length() {
        let code = CubeCode::new(4, 1).unwrap();
        assert_eq!(code.encode(&bits("101")), Err(Error::Length { expected: 11, got: 3 }));
    }

    #[test]
    fn decoding() {
        let cases = [
            (1, "100100100101011", "10010010010"), // no error
            (1, "100100110101011", "10010010010"), // 1 bit error
            (2, "101110001000010", "10110"),       // 3 bit errors
            (2, "101100011101111", "10110"),
            (2, "011000011000110", "10110"),
        ];
        for (p_dims, received, expected) in cases {
            let code = CubeCode::new(4, p_dims).unwrap();
            assert_eq!(format_bits(&code.decode(&bits(received)).unwrap()), expected, "{received}");
        }
    }
}
