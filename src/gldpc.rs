//! Hypercube Generalized LDPC (GLDPC) codes with exact-MAP Hartmann-Rudolph FWHT SISO decoding.
//!
//! Each codeword bit participates in two extended-Hamming hypercube check nodes (Tanner 1981):
//! - **Rate 3/8** (`GldpcRate::Rate3_8`): Group A = `RM(2, 4) = (16, 11)` 4-cubes,
//!   Group B = `RM(2, 4) = (16, 11)` 4-cubes (`(512, 193)`, `(1024, 385)`).
//!
//! Every check node is decoded with exact-MAP soft-in/soft-out (Hartmann & Rudolph 1976)
//! directly in the `tanh(-L/2)` domain using a 4-stage Walsh product butterfly and
//! fast Walsh-Hadamard transform (FWHT) over 8 hypercubes in lockstep (`[f32; 8]`).
//! Zero `tanh()` or `ln()` calls appear inside the iteration loop.

use crate::{Bits, Error};

/// Supported Hypercube GLDPC rate families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GldpcRate {
    /// Group A = `RM(2, 4)` (16, 11), Group B = `RM(2, 4)` (16, 11). Exact rate ≈ 3/8.
    Rate3_8,
}

/// Result of soft-decision GLDPC decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GldpcDecision {
    pub message: Bits,
    pub codeword: Bits,
    pub half_iters: usize,
    pub converged: bool,
}

/// Reusable scratch buffers for zero-allocation GLDPC decoding.
#[derive(Debug, Clone)]
pub struct GldpcScratch {
    t_ch: Vec<f32>,
    t_tot: Vec<f32>,
    t_a: Vec<f32>,
    t_b: Vec<f32>,
}

impl GldpcScratch {
    pub fn new(n: usize) -> Self {
        Self {
            t_ch: vec![0.0; n],
            t_tot: vec![0.0; n],
            t_a: vec![0.0; n],
            t_b: vec![0.0; n],
        }
    }

    #[inline]
    pub fn bit(&self, flat_idx: usize) -> bool {
        self.t_tot[flat_idx] < 0.0
    }

    #[inline]
    pub fn any_bit_one(&self) -> bool {
        self.t_tot.iter().any(|&v| v < 0.0)
    }
}

/// Hypercube GLDPC code with systematic GF(2) encoding and Hartmann-Rudolph FWHT decoding.
#[derive(Debug, Clone)]
pub struct HypercubeGldpc {
    n: usize,
    k: usize,
    rate: GldpcRate,
    perm: Vec<usize>,
    info_cols: Vec<usize>,
    parity_cols: Vec<usize>,
    parity_masks: Vec<Vec<u64>>,
}

struct XorShift64(u64);
impl XorShift64 {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

#[inline(always)]
fn is_valid_ehamming16(bits: u32) -> bool {
    if bits.count_ones() & 1 != 0 {
        return false;
    }
    let mut syn = 0usize;
    let mut b = bits;
    while b != 0 {
        syn ^= b.trailing_zeros() as usize;
        b &= b - 1;
    }
    syn == 0
}

#[inline(always)]
fn clamp_t(v: f32) -> f32 {
    let v = v.clamp(-0.99995, 0.99995);
    if v.abs() < 1e-4 {
        if v < 0.0 { -1e-4 } else { 1e-4 }
    } else {
        v
    }
}

/// Hartmann-Rudolph (1976) exact-MAP SISO in pure `tanh` domain for 8 `RM(2, 4) = (16, 11)` 4-cubes.
#[inline]
fn hr_map_siso_tanh_16x8(t_in: &[[f32; 8]; 16], t_ext: &mut [[f32; 8]; 16]) {
    let mut e = *t_in;
    let mut o = [[1.0f32; 8]; 16];

    let mut half = 1usize;
    while half < 16 {
        let mut base = 0;
        while base < 16 {
            for i in 0..half {
                let i0 = base + i;
                let i1 = base + half + i;
                for k in 0..8 {
                    let e0 = e[i0][k];
                    let o0 = o[i0][k];
                    let e1 = e[i1][k];
                    let o1 = o[i1][k];
                    e[i0][k] = e0 * e1;
                    o[i0][k] = o0 * o1;
                    e[i1][k] = e0 * o1;
                    o[i1][k] = o0 * e1;
                }
            }
            base += 2 * half;
        }
        half *= 2;
    }

    let mut s0 = [0.0f32; 8];
    let mut w = [[0.0f32; 8]; 16];
    for a in 0..16 {
        for k in 0..8 {
            s0[k] += o[a][k] + e[a][k];
            w[a][k] = o[a][k] - e[a][k];
        }
    }

    let mut half = 1usize;
    while half < 16 {
        let mut base = 0;
        while base < 16 {
            for i in 0..half {
                let i0 = base + i;
                let i1 = base + half + i;
                for k in 0..8 {
                    let u = w[i0][k];
                    let v = w[i1][k];
                    w[i0][k] = u + v;
                    w[i1][k] = u - v;
                }
            }
            base += 2 * half;
        }
        half *= 2;
    }

    for x in 0..16 {
        for k in 0..8 {
            let num = s0[k] - w[x][k];
            let den = t_in[x][k] * (s0[k] + w[x][k]);
            let v = if den.abs() > 1e-12 { num / den } else { 0.0 };
            t_ext[x][k] = v.clamp(-0.99995, 0.99995);
        }
    }
}

fn make_girth6_perm_16(n: usize) -> Vec<usize> {
    let mut rng = XorShift64(0x61D9_C0DE_1234_5678u64 ^ ((n as u64) << 16));
    let nb = n / 16;
    let a_slots: Vec<Vec<usize>> = (0..nb)
        .map(|a| {
            let mut v: Vec<usize> = (0..16).map(|i| a * 16 + i).collect();
            for i in (1..16).rev() {
                let j = (rng.next() as usize) % (i + 1);
                v.swap(i, j);
            }
            v
        })
        .collect();
    let mut a_next = vec![0usize; nb];
    let mut shifts: Vec<usize> = (0..nb).collect();
    for i in (1..nb).rev() {
        let j = (rng.next() as usize) % (i + 1);
        shifts.swap(i, j);
    }
    shifts.truncate(16);
    let mut perm = vec![0usize; n];
    for b in 0..nb {
        for (pos, &sh) in shifts.iter().enumerate() {
            let a = (b + sh) % nb;
            perm[b * 16 + pos] = a_slots[a][a_next[a]];
            a_next[a] += 1;
        }
    }
    perm
}

fn build_systematic_encoder(
    n: usize,
    perm: &[usize],
) -> (Vec<usize>, Vec<usize>, Vec<Vec<u64>>) {
    let words = n.div_ceil(64);
    let mut rows: Vec<Vec<u64>> = Vec::new();
    let masks16 = [0xFFFFu32, 0xAAAA, 0xCCCC, 0xF0F0, 0xFF00];
    for a in 0..(n / 16) {
        let base = a * 16;
        for &m in &masks16 {
            let mut row = vec![0u64; words];
            for i in 0..16 {
                if (m >> i) & 1 != 0 {
                    let idx = base + i;
                    row[idx / 64] |= 1u64 << (idx % 64);
                }
            }
            rows.push(row);
        }
    }
    for b in 0..(n / 16) {
        let base = b * 16;
        for &m in &masks16 {
            let mut row = vec![0u64; words];
            for i in 0..16 {
                if (m >> i) & 1 != 0 {
                    let idx = perm[base + i];
                    row[idx / 64] |= 1u64 << (idx % 64);
                }
            }
            rows.push(row);
        }
    }

    let mut parity_cols = Vec::new();
    let mut info_cols = Vec::new();
    let mut rank = 0usize;
    for col in 0..n {
        let w = col / 64;
        let bit = 1u64 << (col % 64);
        if let Some(piv) = (rank..rows.len()).find(|&r| (rows[r][w] & bit) != 0) {
            rows.swap(rank, piv);
            let piv_row = rows[rank].clone();
            for (r, row) in rows.iter_mut().enumerate() {
                if r != rank && (row[w] & bit) != 0 {
                    for j in 0..words {
                        row[j] ^= piv_row[j];
                    }
                }
            }
            parity_cols.push(col);
            rank += 1;
        } else {
            info_cols.push(col);
        }
    }

    let k = info_cols.len();
    let k_words = k.div_ceil(64);
    let mut parity_masks = Vec::with_capacity(rank);
    for row in rows.iter().take(rank) {
        let mut mask = vec![0u64; k_words];
        for (j, &col) in info_cols.iter().enumerate() {
            if (row[col / 64] >> (col % 64)) & 1 != 0 {
                mask[j / 64] |= 1u64 << (j % 64);
            }
        }
        parity_masks.push(mask);
    }

    (info_cols, parity_cols, parity_masks)
}

impl HypercubeGldpc {
    pub fn new(n: usize, rate: GldpcRate) -> Result<Self, Error> {
        let ok = (256..=65536).contains(&n) && n % 128 == 0;
        if !ok {
            return Err(Error::Unsupported(
                "invalid HypercubeGldpc block length for selected rate",
            ));
        }
        let perm = make_girth6_perm_16(n);
        let (info_cols, parity_cols, parity_masks) = build_systematic_encoder(n, &perm);
        let k = info_cols.len();
        Ok(Self {
            n,
            k,
            rate,
            perm,
            info_cols,
            parity_cols,
            parity_masks,
        })
    }

    pub fn codeword_len(&self) -> usize {
        self.n
    }

    pub fn message_len(&self) -> usize {
        self.k
    }

    pub fn rate(&self) -> GldpcRate {
        self.rate
    }

    pub fn scratch(&self) -> GldpcScratch {
        GldpcScratch::new(self.n)
    }

    pub fn encode(&self, msg: &[bool]) -> Result<Bits, Error> {
        if msg.len() != self.k {
            return Err(Error::Length {
                expected: self.k,
                got: msg.len(),
            });
        }
        let k_words = self.k.div_ceil(64);
        let mut msg_packed = vec![0u64; k_words];
        let mut cw = vec![false; self.n];
        for (j, &bit) in msg.iter().enumerate() {
            if bit {
                msg_packed[j / 64] |= 1u64 << (j % 64);
                cw[self.info_cols[j]] = true;
            }
        }
        for (&pcol, mask) in self.parity_cols.iter().zip(&self.parity_masks) {
            let mut parity = 0u32;
            for (&mw, &bw) in mask.iter().zip(&msg_packed) {
                parity ^= (mw & bw).count_ones();
            }
            cw[pcol] = (parity & 1) != 0;
        }
        Ok(cw)
    }

    pub fn extract_message(&self, codeword: &[bool]) -> Result<Bits, Error> {
        if codeword.len() != self.n {
            return Err(Error::Length {
                expected: self.n,
                got: codeword.len(),
            });
        }
        Ok(self.info_cols.iter().map(|&col| codeword[col]).collect())
    }

    pub fn is_valid_codeword(&self, bits: &[bool]) -> bool {
        if bits.len() != self.n {
            return false;
        }
        for a in 0..(self.n / 16) {
            let base = a * 16;
            let mut m = 0u32;
            for i in 0..16 {
                if bits[base + i] {
                    m |= 1u32 << i;
                }
            }
            if !is_valid_ehamming16(m) {
                return false;
            }
        }
        for b in 0..(self.n / 16) {
            let base = b * 16;
            let mut m = 0u32;
            for i in 0..16 {
                if bits[self.perm[base + i]] {
                    m |= 1u32 << i;
                }
            }
            if !is_valid_ehamming16(m) {
                return false;
            }
        }
        true
    }

    pub fn decode_soft(&self, llrs: &[f32], max_iters: usize) -> Result<GldpcDecision, Error> {
        let mut scratch = self.scratch();
        self.decode_soft_with_scratch(llrs, max_iters, &mut scratch)
    }

    pub fn decode_soft_with_scratch(
        &self,
        llrs: &[f32],
        max_iters: usize,
        scratch: &mut GldpcScratch,
    ) -> Result<GldpcDecision, Error> {
        let (half_iters, converged) = self.decode_into_scratch(llrs, max_iters, scratch)?;
        let codeword: Bits = (0..self.n).map(|i| scratch.bit(i)).collect();
        let message: Bits = self.info_cols.iter().map(|&col| codeword[col]).collect();
        Ok(GldpcDecision {
            message,
            codeword,
            half_iters,
            converged,
        })
    }

    pub fn decode_into_scratch(
        &self,
        llrs: &[f32],
        max_iters: usize,
        scratch: &mut GldpcScratch,
    ) -> Result<(usize, bool), Error> {
        if llrs.len() != self.n {
            return Err(Error::Length {
                expected: self.n,
                got: llrs.len(),
            });
        }
        let num_blocks = self.n / 16;
        for (dst, &l) in scratch.t_tot.iter_mut().zip(llrs) {
            *dst = -l;
        }
        if self.all_syndromes_zero_tanh(&scratch.t_tot) {
            return Ok((0, true));
        }
        for (dst, &l) in scratch.t_ch.iter_mut().zip(llrs) {
            *dst = clamp_t((-0.5 * l).tanh());
        }
        scratch.t_a.fill(0.0);
        scratch.t_b.fill(0.0);

        let mut in_buf = [[0.0f32; 8]; 16];
        let mut ext_buf = [[0.0f32; 8]; 16];
        let mut half_iters = 0usize;

        for iter in 0..max_iters {
            for grp in 0..(num_blocks / 8) {
                for k in 0..8 {
                    let base = (grp * 8 + k) * 16;
                    for x in 0..16 {
                        let tc = scratch.t_ch[base + x];
                        let tb = scratch.t_b[base + x];
                        in_buf[x][k] = clamp_t((tc + tb) / (1.0 + tc * tb));
                    }
                }
                hr_map_siso_tanh_16x8(&in_buf, &mut ext_buf);
                for k in 0..8 {
                    let base = (grp * 8 + k) * 16;
                    for x in 0..16 {
                        let ta = if iter == 0 {
                            ext_buf[x][k]
                        } else {
                            0.75 * ext_buf[x][k] + 0.25 * scratch.t_a[base + x]
                        };
                        scratch.t_a[base + x] = ta;
                        scratch.t_tot[base + x] = in_buf[x][k] + ta;
                    }
                }
            }
            half_iters += 1;
            if self.all_syndromes_zero_tanh(&scratch.t_tot) {
                return Ok((half_iters, true));
            }

            for grp in 0..(num_blocks / 8) {
                for k in 0..8 {
                    let base = (grp * 8 + k) * 16;
                    for x in 0..16 {
                        let p = self.perm[base + x];
                        let tc = scratch.t_ch[p];
                        let ta = scratch.t_a[p];
                        in_buf[x][k] = clamp_t((tc + ta) / (1.0 + tc * ta));
                    }
                }
                hr_map_siso_tanh_16x8(&in_buf, &mut ext_buf);
                for k in 0..8 {
                    let base = (grp * 8 + k) * 16;
                    for x in 0..16 {
                        let p = self.perm[base + x];
                        let tb = if iter == 0 {
                            ext_buf[x][k]
                        } else {
                            0.75 * ext_buf[x][k] + 0.25 * scratch.t_b[p]
                        };
                        scratch.t_b[p] = tb;
                        scratch.t_tot[p] = in_buf[x][k] + tb;
                    }
                }
            }
            half_iters += 1;
            if self.all_syndromes_zero_tanh(&scratch.t_tot) {
                return Ok((half_iters, true));
            }
        }
        Ok((half_iters, false))
    }

    #[inline]
    fn all_syndromes_zero_tanh(&self, t_tot: &[f32]) -> bool {
        for a in 0..(self.n / 16) {
            let base = a * 16;
            let mut bits = 0u32;
            for i in 0..16 {
                if t_tot[base + i] < 0.0 {
                    bits |= 1u32 << i;
                }
            }
            if !is_valid_ehamming16(bits) {
                return false;
            }
        }
        for b in 0..(self.n / 16) {
            let base = b * 16;
            let mut bits = 0u32;
            for i in 0..16 {
                if t_tot[self.perm[base + i]] < 0.0 {
                    bits |= 1u32 << i;
                }
            }
            if !is_valid_ehamming16(bits) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_dimensions() {
        assert_eq!(HypercubeGldpc::new(512, GldpcRate::Rate3_8).unwrap().message_len(), 193);
        assert_eq!(HypercubeGldpc::new(1024, GldpcRate::Rate3_8).unwrap().message_len(), 385);
    }

    #[test]
    fn systematic_encode_and_noisy_decode_round_trip() {
        let mut rng = XorShift64(0xCAFE_BABE_1234_5678);
        for &(n, rate) in &[(512, GldpcRate::Rate3_8), (1024, GldpcRate::Rate3_8)] {
            let code = HypercubeGldpc::new(n, rate).unwrap();
            let msg: Bits = (0..code.message_len()).map(|_| (rng.next() & 1) != 0).collect();
            let cw = code.encode(&msg).unwrap();
            assert!(code.is_valid_codeword(&cw));
            assert_eq!(code.extract_message(&cw).unwrap(), msg);

            let mut llrs: Vec<f32> = cw.iter().map(|&b| if b { 2.5 } else { -2.5 }).collect();
            for i in (0..n).step_by(47) {
                llrs[i] = -llrs[i] * 0.4;
            }
            let dec = code.decode_soft(&llrs, 12).unwrap();
            assert!(dec.converged);
            assert_eq!(dec.codeword, cw);
            assert_eq!(dec.message, msg);
        }
    }
}
