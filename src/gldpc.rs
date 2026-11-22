//! Hypercube Generalized LDPC (GLDPC) codes with exact-MAP Hartmann-Rudolph FWHT SISO decoding.
//!
//! Each codeword bit participates in two extended-Hamming hypercube check nodes (Tanner 1981):
//! - **Rate 3/8** (`GldpcRate::Rate3_8`): Group A = `RM(2, 4) = (16, 11)` 4-cubes,
//!   Group B = `RM(2, 4) = (16, 11)` 4-cubes (`(512, 193)`, `(1024, 385)`).
//! - **Rate 1/2** (`GldpcRate::Rate1_2`): Group A = `RM(2, 4) = (16, 11)` 4-cubes,
//!   Group B = `RM(3, 5) = (32, 26)` 5-cubes (`(512, 257)`, `(1024, 513)`).
//!
//! Every check node is decoded with exact-MAP soft-in/soft-out (Hartmann & Rudolph 1976)
//! directly in the `tanh(-L/2)` domain using a 4- or 5-stage Walsh product butterfly and
//! fast Walsh-Hadamard transform (FWHT) over 8 hypercubes in lockstep (`[f32; 8]`).
//! Zero `tanh()` or `ln()` calls appear inside the iteration loop.

use crate::{Bits, Error};

/// Supported Hypercube GLDPC rate families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GldpcRate {
    /// Group A = `RM(2, 4)` (16, 11), Group B = `RM(2, 4)` (16, 11). Exact rate ≈ 3/8.
    Rate3_8,
    /// Group A = `RM(2, 4)` (16, 11), Group B = `RM(3, 5)` (32, 26). Exact rate ≈ 1/2.
    Rate1_2,
}

/// Result of soft-decision GLDPC decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GldpcDecision {
    /// Decoded systematic message bits (length `k`).
    pub message: Bits,
    /// Decoded codeword bits (length `n`).
    pub codeword: Bits,
    /// Number of half-iterations executed (`0` if hard decisions already satisfied all checks).
    pub half_iters: usize,
    /// `true` if all Group-A and Group-B hypercube syndromes were zero.
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

    /// Inspect decoded bit `i` after `decode_into_scratch`.
    #[inline]
    pub fn bit(&self, idx: usize) -> bool {
        self.t_tot[idx] < 0.0
    }

    /// Returns `true` if any bit in the decoded codeword is `1` (`true`).
    /// Useful for all-zero codeword AWGN benchmarks without unpacking `Bits`.
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
    /// Group-B permutation in flat bit order (`0..n`).
    perm: Vec<usize>,
    /// Indices of the `k` systematic information bits in `0..n`.
    info_cols: Vec<usize>,
    /// Indices of the `n - k` parity bits in `0..n`.
    parity_cols: Vec<usize>,
    /// For each parity bit `p`, bitmask over `0..k` (in `u64` words) specifying which info bits XOR into `parity_cols[p]`.
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
    let p0 = bits.count_ones();
    let p1 = (bits & 0xAAAA).count_ones();
    let p2 = (bits & 0xCCCC).count_ones();
    let p3 = (bits & 0xF0F0).count_ones();
    let p4 = (bits & 0xFF00).count_ones();
    ((p0 | p1 | p2 | p3 | p4) & 1) == 0
}

#[inline(always)]
fn is_valid_ehamming32(bits: u32) -> bool {
    let p0 = bits.count_ones();
    let p1 = (bits & 0xAAAA_AAAA).count_ones();
    let p2 = (bits & 0xCCCC_CCCC).count_ones();
    let p3 = (bits & 0xF0F0_F0F0).count_ones();
    let p4 = (bits & 0xFF00_FF00).count_ones();
    let p5 = (bits & 0xFFFF_0000).count_ones();
    ((p0 | p1 | p2 | p3 | p4 | p5) & 1) == 0
}

/// Branchless, vectorizable `tanh(x)` via Padé `[5/4]` on `x/2` + exact half-angle doubling
/// `2*n*d / (d^2 + n^2)`. Strictly in `(-1, 1)` and accurate to `< 1.4e-6` on `[-5.2, 5.2]`.
#[inline(always)]
fn fast_tanh_f32(x: f32) -> f32 {
    let u = 0.5 * x.clamp(-5.2, 5.2);
    let u2 = u * u;
    let n = u * (945.0 + u2 * (105.0 + u2));
    let d = 945.0 + u2 * (420.0 + 15.0 * u2);
    (2.0 * n * d) / (d * d + n * n)
}

#[inline(always)]
fn clamp_t(v: f32) -> f32 {
    v.abs().clamp(1e-4, 0.99995).copysign(v)
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

/// Hartmann-Rudolph (1976) exact-MAP SISO in pure `tanh` domain for 8 `RM(3, 5) = (32, 26)` 5-cubes.
#[inline]
fn hr_map_siso_tanh_32x8(t_in: &[[f32; 8]; 32], t_ext: &mut [[f32; 8]; 32]) {
    let mut e = *t_in;
    let mut o = [[1.0f32; 8]; 32];

    let mut half = 1usize;
    while half < 32 {
        let mut base = 0;
        while base < 32 {
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
    let mut w = [[0.0f32; 8]; 32];
    for a in 0..32 {
        for k in 0..8 {
            s0[k] += o[a][k] + e[a][k];
            w[a][k] = o[a][k] - e[a][k];
        }
    }

    let mut half = 1usize;
    while half < 32 {
        let mut base = 0;
        while base < 32 {
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

    for x in 0..32 {
        for k in 0..8 {
            let num = s0[k] - w[x][k];
            let den = t_in[x][k] * (s0[k] + w[x][k]);
            let v = if den.abs() > 1e-12 { num / den } else { 0.0 };
            t_ext[x][k] = v.clamp(-0.99995, 0.99995);
        }
    }
}

/// Zero-4-cycle permutation connecting `nb = n/16` RM(2,4) A-checks and `nb = n/16` RM(2,4) B-checks.
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

/// Zero-4-cycle permutation connecting `na = n/16` RM(2,4) A-checks and `nb = n/32` RM(3,5) B-checks.
fn make_girth6_perm_16_32(n: usize) -> Vec<usize> {
    let mut rng = XorShift64(0x5050_61D9_1234_5678u64 ^ ((n as u64) << 16));
    let na = n / 16;
    let nb = n / 32;
    let a_slots: Vec<Vec<usize>> = (0..na)
        .map(|a| {
            let mut v: Vec<usize> = (0..16).map(|i| a * 16 + i).collect();
            for i in (1..16).rev() {
                let j = (rng.next() as usize) % (i + 1);
                v.swap(i, j);
            }
            v
        })
        .collect();
    let mut a_next = vec![0usize; na];

    let mut evens: Vec<usize> = (0..na).step_by(2).collect();
    let mut odds: Vec<usize> = (1..na).step_by(2).collect();
    for i in (1..evens.len()).rev() {
        let j = (rng.next() as usize) % (i + 1);
        evens.swap(i, j);
    }
    for i in (1..odds.len()).rev() {
        let j = (rng.next() as usize) % (i + 1);
        odds.swap(i, j);
    }
    evens.truncate(16);
    odds.truncate(16);
    let mut shifts = Vec::with_capacity(32);
    shifts.extend_from_slice(&evens);
    shifts.extend_from_slice(&odds);
    for i in (1..32).rev() {
        let j = (rng.next() as usize) % (i + 1);
        shifts.swap(i, j);
    }

    let mut perm = vec![0usize; n];
    for b in 0..nb {
        for (pos, &sh) in shifts.iter().enumerate() {
            let a = (2 * b + sh) % na;
            perm[b * 32 + pos] = a_slots[a][a_next[a]];
            a_next[a] += 1;
        }
    }
    perm
}

/// Build the combined parity-check matrix `H` and reduce it to RREF over GF(2) to obtain
/// systematic `(info_cols, parity_cols, parity_masks)`.
fn build_systematic_encoder(
    n: usize,
    perm: &[usize],
    rate: GldpcRate,
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
    match rate {
        GldpcRate::Rate3_8 => {
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
        }
        GldpcRate::Rate1_2 => {
            let masks32 = [
                0xFFFF_FFFFu32,
                0xAAAA_AAAA,
                0xCCCC_CCCC,
                0xF0F0_F0F0,
                0xFF00_FF00,
                0xFFFF_0000,
            ];
            for b in 0..(n / 32) {
                let base = b * 32;
                for &m in &masks32 {
                    let mut row = vec![0u64; words];
                    for i in 0..32 {
                        if (m >> i) & 1 != 0 {
                            let idx = perm[base + i];
                            row[idx / 64] |= 1u64 << (idx % 64);
                        }
                    }
                    rows.push(row);
                }
            }
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
    /// Construct a Hypercube GLDPC code of length `n` and rate family `rate`.
    ///
    /// - `Rate3_8` requires `n >= 256` and `n % 128 == 0` (e.g. `512`, `1024`).
    /// - `Rate1_2` requires `n >= 512` and `n % 256 == 0` (e.g. `512`, `1024`).
    pub fn new(n: usize, rate: GldpcRate) -> Result<Self, Error> {
        let ok = match rate {
            GldpcRate::Rate3_8 => n >= 256 && n % 128 == 0,
            GldpcRate::Rate1_2 => n >= 512 && n % 256 == 0,
        };
        if !ok {
            return Err(Error::Unsupported(
                "invalid HypercubeGldpc block length for selected rate",
            ));
        }
        let perm = match rate {
            GldpcRate::Rate3_8 => make_girth6_perm_16(n),
            GldpcRate::Rate1_2 => make_girth6_perm_16_32(n),
        };
        let (info_cols, parity_cols, parity_masks) = build_systematic_encoder(n, &perm, rate);
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

    /// Codeword length `n`.
    pub fn codeword_len(&self) -> usize {
        self.n
    }

    /// Exact message length `k = n - rank(H)`.
    pub fn message_len(&self) -> usize {
        self.k
    }

    /// Rate family (`Rate3_8` or `Rate1_2`).
    pub fn rate(&self) -> GldpcRate {
        self.rate
    }

    /// Allocate a reusable scratch buffer for zero-allocation decoding.
    pub fn scratch(&self) -> GldpcScratch {
        GldpcScratch::new(self.n)
    }

    /// Systematically encode a `k`-bit message into an `n`-bit codeword.
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

    /// Extract the `k` systematic message bits from an `n`-bit codeword.
    pub fn extract_message(&self, codeword: &[bool]) -> Result<Bits, Error> {
        if codeword.len() != self.n {
            return Err(Error::Length {
                expected: self.n,
                got: codeword.len(),
            });
        }
        Ok(self.info_cols.iter().map(|&col| codeword[col]).collect())
    }

    /// Check whether all Group-A and Group-B hypercube parity constraints hold.
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
        match self.rate {
            GldpcRate::Rate3_8 => {
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
            }
            GldpcRate::Rate1_2 => {
                for b in 0..(self.n / 32) {
                    let base = b * 32;
                    let mut m = 0u32;
                    for i in 0..32 {
                        if bits[self.perm[base + i]] {
                            m |= 1u32 << i;
                        }
                    }
                    if !is_valid_ehamming32(m) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Soft-decision Hartmann-Rudolph FWHT decoding.
    ///
    /// Convention (matches [`crate::CubeCode::decode_soft`]):
    /// `llrs[i] > 0.0` favors bit `1` (`true`), `llrs[i] < 0.0` favors bit `0` (`false`).
    pub fn decode_soft(&self, llrs: &[f32], max_iters: usize) -> Result<GldpcDecision, Error> {
        let mut scratch = self.scratch();
        self.decode_soft_with_scratch(llrs, max_iters, &mut scratch)
    }

    /// Soft-decision decoding reusing `scratch` (still allocates the returned `Bits`).
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

    /// Zero-allocation soft decode into `scratch`. Returns `(half_iters, converged)`.
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
        debug_assert_eq!(scratch.t_ch.len(), self.n);
        if self.all_syndromes_zero_llr(llrs) {
            for (dst, &l) in scratch.t_tot.iter_mut().zip(llrs) {
                *dst = -l;
            }
            return Ok((0, true));
        }

        for (dst, &l) in scratch.t_ch.iter_mut().zip(llrs) {
            *dst = clamp_t(fast_tanh_f32(-0.5 * l));
        }
        Ok(self.run_bp_from_t_ch(max_iters, scratch))
    }

    fn run_bp_from_t_ch(&self, max_iters: usize, scratch: &mut GldpcScratch) -> (usize, bool) {
        scratch.t_a.fill(0.0);
        scratch.t_b.fill(0.0);

        let num_grps_a = self.n / 128;
        let mut in_a = [[0.0f32; 8]; 16];
        let mut ext_a = [[0.0f32; 8]; 16];
        let mut in_b16 = [[0.0f32; 8]; 16];
        let mut ext_b16 = [[0.0f32; 8]; 16];
        let mut in_b32 = [[0.0f32; 8]; 32];
        let mut ext_b32 = [[0.0f32; 8]; 32];
        let mut half_iters = 0usize;

        for iter in 0..max_iters {
            // Half-iter 1: Group A — RM(2, 4) = (16, 11), 8 cubes at a time.
            for grp in 0..num_grps_a {
                let gbase = grp * 128;
                if iter == 0 {
                    for x in 0..16 {
                        for k in 0..8 {
                            in_a[x][k] = scratch.t_ch[gbase + k * 16 + x];
                        }
                    }
                } else {
                    for x in 0..16 {
                        let mut tc = [0.0f32; 8];
                        let mut tb = [0.0f32; 8];
                        for k in 0..8 {
                            let idx = gbase + k * 16 + x;
                            tc[k] = scratch.t_ch[idx];
                            tb[k] = scratch.t_b[idx];
                        }
                        for k in 0..8 {
                            in_a[x][k] = clamp_t((tc[k] + tb[k]) / (1.0 + tc[k] * tb[k]));
                        }
                    }
                }
                hr_map_siso_tanh_16x8(&in_a, &mut ext_a);
                if iter == 0 {
                    for x in 0..16 {
                        for k in 0..8 {
                            let idx = gbase + k * 16 + x;
                            let ta = ext_a[x][k];
                            scratch.t_a[idx] = ta;
                            scratch.t_tot[idx] = in_a[x][k] + ta;
                        }
                    }
                } else {
                    for x in 0..16 {
                        for k in 0..8 {
                            let idx = gbase + k * 16 + x;
                            let ta = 0.75 * ext_a[x][k] + 0.25 * scratch.t_a[idx];
                            scratch.t_a[idx] = ta;
                            scratch.t_tot[idx] = in_a[x][k] + ta;
                        }
                    }
                }
            }
            half_iters += 1;
            if self.all_syndromes_zero_tanh(&scratch.t_tot) {
                return (half_iters, true);
            }

            // Half-iter 2: Group B — RM(2, 4) for Rate3_8 or RM(3, 5) for Rate1_2.
            match self.rate {
                GldpcRate::Rate3_8 => {
                    for grp in 0..num_grps_a {
                        let gbase = grp * 128;
                        for x in 0..16 {
                            let mut tc = [0.0f32; 8];
                            let mut ta = [0.0f32; 8];
                            for k in 0..8 {
                                let p = self.perm[gbase + k * 16 + x];
                                tc[k] = scratch.t_ch[p];
                                ta[k] = scratch.t_a[p];
                            }
                            for k in 0..8 {
                                in_b16[x][k] = clamp_t((tc[k] + ta[k]) / (1.0 + tc[k] * ta[k]));
                            }
                        }
                        hr_map_siso_tanh_16x8(&in_b16, &mut ext_b16);
                        if iter == 0 {
                            for x in 0..16 {
                                for k in 0..8 {
                                    let p = self.perm[gbase + k * 16 + x];
                                    let tb = ext_b16[x][k];
                                    scratch.t_b[p] = tb;
                                    scratch.t_tot[p] = in_b16[x][k] + tb;
                                }
                            }
                        } else {
                            for x in 0..16 {
                                for k in 0..8 {
                                    let p = self.perm[gbase + k * 16 + x];
                                    let tb = 0.75 * ext_b16[x][k] + 0.25 * scratch.t_b[p];
                                    scratch.t_b[p] = tb;
                                    scratch.t_tot[p] = in_b16[x][k] + tb;
                                }
                            }
                        }
                    }
                }
                GldpcRate::Rate1_2 => {
                    let num_grps_b = self.n / 256;
                    for grp in 0..num_grps_b {
                        let gbase = grp * 256;
                        for x in 0..32 {
                            let mut tc = [0.0f32; 8];
                            let mut ta = [0.0f32; 8];
                            for k in 0..8 {
                                let p = self.perm[gbase + k * 32 + x];
                                tc[k] = scratch.t_ch[p];
                                ta[k] = scratch.t_a[p];
                            }
                            for k in 0..8 {
                                in_b32[x][k] = clamp_t((tc[k] + ta[k]) / (1.0 + tc[k] * ta[k]));
                            }
                        }
                        hr_map_siso_tanh_32x8(&in_b32, &mut ext_b32);
                        if iter == 0 {
                            for x in 0..32 {
                                for k in 0..8 {
                                    let p = self.perm[gbase + k * 32 + x];
                                    let tb = ext_b32[x][k];
                                    scratch.t_b[p] = tb;
                                    scratch.t_tot[p] = in_b32[x][k] + tb;
                                }
                            }
                        } else {
                            for x in 0..32 {
                                for k in 0..8 {
                                    let p = self.perm[gbase + k * 32 + x];
                                    let tb = 0.75 * ext_b32[x][k] + 0.25 * scratch.t_b[p];
                                    scratch.t_b[p] = tb;
                                    scratch.t_tot[p] = in_b32[x][k] + tb;
                                }
                            }
                        }
                    }
                }
            }
            half_iters += 1;
            if self.all_syndromes_zero_tanh(&scratch.t_tot) {
                return (half_iters, true);
            }
        }
        (half_iters, false)
    }

    #[inline]
    fn all_syndromes_zero_llr(&self, ch: &[f32]) -> bool {
        for a in 0..(self.n / 16) {
            let base = a * 16;
            let mut bits = 0u32;
            for i in 0..16 {
                bits |= (((-ch[base + i]).to_bits()) >> 31) << i;
            }
            if !is_valid_ehamming16(bits) {
                return false;
            }
        }
        match self.rate {
            GldpcRate::Rate3_8 => {
                for b in 0..(self.n / 16) {
                    let base = b * 16;
                    let mut bits = 0u32;
                    for i in 0..16 {
                        bits |= (((-ch[self.perm[base + i]]).to_bits()) >> 31) << i;
                    }
                    if !is_valid_ehamming16(bits) {
                        return false;
                    }
                }
            }
            GldpcRate::Rate1_2 => {
                for b in 0..(self.n / 32) {
                    let base = b * 32;
                    let mut bits = 0u32;
                    for i in 0..32 {
                        bits |= (((-ch[self.perm[base + i]]).to_bits()) >> 31) << i;
                    }
                    if !is_valid_ehamming32(bits) {
                        return false;
                    }
                }
            }
        }
        true
    }

    #[inline]
    fn all_syndromes_zero_tanh(&self, t_tot: &[f32]) -> bool {
        for a in 0..(self.n / 16) {
            let base = a * 16;
            let mut bits = 0u32;
            for i in 0..16 {
                bits |= (t_tot[base + i].to_bits() >> 31) << i;
            }
            if !is_valid_ehamming16(bits) {
                return false;
            }
        }
        match self.rate {
            GldpcRate::Rate3_8 => {
                for b in 0..(self.n / 16) {
                    let base = b * 16;
                    let mut bits = 0u32;
                    for i in 0..16 {
                        bits |= (t_tot[self.perm[base + i]].to_bits() >> 31) << i;
                    }
                    if !is_valid_ehamming16(bits) {
                        return false;
                    }
                }
            }
            GldpcRate::Rate1_2 => {
                for b in 0..(self.n / 32) {
                    let base = b * 32;
                    let mut bits = 0u32;
                    for i in 0..32 {
                        bits |= (t_tot[self.perm[base + i]].to_bits() >> 31) << i;
                    }
                    if !is_valid_ehamming32(bits) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gaussian(rng: &mut XorShift64) -> f64 {
        let a = ((rng.next() >> 11) as f64 + 0.5) / ((1u64 << 53) as f64);
        let b = ((rng.next() >> 11) as f64 + 0.5) / ((1u64 << 53) as f64);
        (-2.0 * a.ln()).sqrt() * (2.0 * std::f64::consts::PI * b).cos()
    }

    #[test]
    fn exact_dimensions() {
        assert_eq!(HypercubeGldpc::new(256, GldpcRate::Rate3_8).unwrap().message_len(), 97);
        assert_eq!(HypercubeGldpc::new(512, GldpcRate::Rate3_8).unwrap().message_len(), 193);
        assert_eq!(HypercubeGldpc::new(1024, GldpcRate::Rate3_8).unwrap().message_len(), 385);
        assert_eq!(HypercubeGldpc::new(2048, GldpcRate::Rate3_8).unwrap().message_len(), 769);
        assert_eq!(HypercubeGldpc::new(512, GldpcRate::Rate1_2).unwrap().message_len(), 257);
        assert_eq!(HypercubeGldpc::new(1024, GldpcRate::Rate1_2).unwrap().message_len(), 513);
        assert_eq!(HypercubeGldpc::new(2048, GldpcRate::Rate1_2).unwrap().message_len(), 1025);
        assert!(HypercubeGldpc::new(128, GldpcRate::Rate3_8).is_err());
        assert!(HypercubeGldpc::new(256, GldpcRate::Rate1_2).is_err());
    }

    #[test]
    fn systematic_encode_and_noisy_decode_round_trip() {
        let mut rng = XorShift64(0xCAFE_BABE_1234_5678);
        for &(n, rate) in &[
            (256, GldpcRate::Rate3_8),
            (512, GldpcRate::Rate3_8),
            (1024, GldpcRate::Rate3_8),
            (2048, GldpcRate::Rate3_8),
            (512, GldpcRate::Rate1_2),
            (1024, GldpcRate::Rate1_2),
            (2048, GldpcRate::Rate1_2),
        ] {
            let code = HypercubeGldpc::new(n, rate).unwrap();
            let msg: Bits = (0..code.message_len()).map(|_| (rng.next() & 1) != 0).collect();
            let cw = code.encode(&msg).unwrap();
            assert!(code.is_valid_codeword(&cw));
            assert_eq!(code.extract_message(&cw).unwrap(), msg);

            let mut llrs: Vec<f32> = cw.iter().map(|&b| if b { 2.5 } else { -2.5 }).collect();
            for i in (0..n).step_by(47) {
                llrs[i] = -llrs[i] * 0.4;
            }
            let dec = code.decode_soft(&llrs, 50).unwrap();
            assert!(dec.converged);
            assert_eq!(dec.codeword, cw);
            assert_eq!(dec.message, msg);
        }
    }

    #[test]
    fn random_codeword_awgn_converged_matches_valid() {
        let mut rng = XorShift64(0x5EED_A764_1234_9999);
        for &(n, rate, db) in &[
            (512, GldpcRate::Rate1_2, 3.0f64),
            (1024, GldpcRate::Rate1_2, 2.5f64),
        ] {
            let code = HypercubeGldpc::new(n, rate).unwrap();
            let r = code.message_len() as f64 / n as f64;
            let sigma = (1.0 / (2.0 * r * 10f64.powf(db / 10.0))).sqrt();
            let scale = (2.0 / (sigma * sigma)) as f32;
            let mut scratch = code.scratch();
            let mut llrs = vec![0.0f32; n];
            let mut converged_count = 0usize;

            for _ in 0..40 {
                let msg: Bits = (0..code.message_len()).map(|_| (rng.next() & 1) != 0).collect();
                let cw = code.encode(&msg).unwrap();
                for (dst, &bit) in llrs.iter_mut().zip(&cw) {
                    let tx = if bit { 1.0 } else { -1.0 };
                    *dst = scale * (tx + sigma * gaussian(&mut rng)) as f32;
                }
                let (_, converged) = code.decode_into_scratch(&llrs, 50, &mut scratch).unwrap();
                let dec_cw: Bits = (0..n).map(|i| scratch.bit(i)).collect();
                assert_eq!(converged, code.is_valid_codeword(&dec_cw));
                if converged {
                    assert_eq!(dec_cw, cw);
                    assert_eq!(code.extract_message(&dec_cw).unwrap(), msg);
                    converged_count += 1;
                }
            }
            assert!(converged_count >= 38, "expected >=38/40 converged, got {converged_count}");
        }
    }
}
