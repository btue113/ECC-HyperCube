//! The punctured hypercube code.
//!
//! Every non-zero vertex `c` of the `cw_dims`-cube owns one codeword bit. Bits
//! are ordered by level: level `i` holds the vertices with `cw_dims - i` ones,
//! in descending numeric order. The first `cw_dims - p_dims` levels carry the
//! message; the rest carry parity, where each parity bit makes the XOR over
//! its unit cube zero.
//!
//! Decoding recovers each data level, lowest weight first (the all-ones vertex
//! last), by majority vote
//! over `2^(p_dims+1) - 1` parallel cubes, then strips the decided value
//! out of the received word before moving to the next level.

use crate::hypercube::HyperCube;
use crate::{Bits, Error};
use std::sync::{Mutex, OnceLock};

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
    /// Compact voting assignments shared by vertices with the same weight.
    voter_complements: Vec<Vec<u32>>,
    /// Materialized only when a caller requests the public cube view.
    voters: Vec<OnceLock<Vec<HyperCube>>>,
    /// Reusable workspace for soft SCL decoding across calls.
    scl_workspace: Mutex<Option<SclDecoder>>,
}

impl CubeCode {
    pub fn new(cw_dims: u32, p_dims: u32) -> Result<Self, Error> {
        if cw_dims == 0 || cw_dims > MAX_DIMS || p_dims >= cw_dims {
            return Err(Error::InvalidParams { cw_dims, p_dims });
        }
        let cw_len = (1usize << cw_dims) - 1;

        // Counting sort by weight, with descending vertices within each level.
        let mut cubes_in_level = vec![0; cw_dims as usize + 1];
        let mut count = 1;
        for level in 0..cw_dims as usize {
            cubes_in_level[level + 1] = cubes_in_level[level] + count;
            count = count * (cw_dims as usize - level) / (level + 1);
        }
        let mut next = cubes_in_level[..cw_dims as usize].to_vec();
        let mut bit_to_cube = vec![0; cw_len];
        let mut cube_to_bit = vec![0; cw_len + 1];
        for cube in (1..=cw_len as u32).rev() {
            let level = (cw_dims - cube.count_ones()) as usize;
            let bit = next[level];
            next[level] += 1;
            bit_to_cube[bit] = cube;
            cube_to_bit[cube as usize] = bit;
        }

        let data_len = cubes_in_level[(cw_dims - p_dims) as usize];
        let votes = (1usize << (p_dims + 1)) - 1;
        let mut voter_complements = vec![Vec::new(); cw_dims as usize + 1];
        // A compressed assignment depends only on the number of fixed bits.
        // Store zero positions: the first assignments have very few of them.
        for weight in p_dims + 1..=cw_dims {
            let fixed = (1u32 << weight) - 1;
            voter_complements[weight as usize] = HyperCube::unit(weight, fixed)
                .parallel_cubes(votes)
                .iter()
                .map(|cube| fixed ^ cube.fixed_value())
                .collect();
        }
        let voters = (0..data_len).map(|_| OnceLock::new()).collect();

        Ok(Self {
            cw_dims,
            p_dims,
            cw_len,
            data_len,
            needed_votes: 1 << p_dims,
            bit_to_cube,
            cube_to_bit,
            cubes_in_level,
            voter_complements,
            voters,
            scl_workspace: Mutex::new(None),
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
        self.voters[bit].get_or_init(|| {
            HyperCube::unit(self.cw_dims, self.bit_to_cube[bit])
                .parallel_cubes(2 * self.needed_votes - 1)
        })
    }

    pub fn encode(&self, data: &[bool]) -> Result<Bits, Error> {
        check_len(self.data_len, data.len())?;
        if self.p_dims == 0 {
            return Ok(data.to_vec());
        }
        // The superset XOR transform is its own inverse over GF(2).
        // Recover the data coefficients, zero the parity coefficients, then
        // transform back to obtain the systematic codeword.
        let mut values = self.to_vertices(data);
        superset_xor(&mut values);
        for &vertex in &self.bit_to_cube[self.data_len..] {
            values[vertex as usize] = false;
        }
        superset_xor(&mut values);
        Ok(self.bit_to_cube.iter().map(|&v| values[v as usize]).collect())
    }

    pub fn decode(&self, received: &[bool]) -> Result<Bits, Error> {
        check_len(self.cw_len, received.len())?;
        if self.p_dims == 0 {
            return Ok(received.to_vec());
        }
        let code = self.to_vertices(received);
        let mut mask = code.clone();
        superset_xor(&mut mask);
        if self.bit_to_cube[self.data_len..].iter().all(|&v| !mask[v as usize]) {
            return Ok(received[..self.data_len].to_vec());
        }

        // Consecutive vertices occupy lanes of a machine word. Each voting
        // cube can combine its lowest six dimensions with one masked popcount.
        let mut code = pack_vertices(&code);

        let mut coefficients = vec![false; self.cw_len + 1];
        let data_dims = (self.cw_dims - self.p_dims) as usize;
        for dim in (0..data_dims).rev() {
            mask.fill(false);
            let mut any = false;
            for bit in self.level(dim) {
                if self.vote(&code, bit) {
                    let vertex = self.bit_to_cube[bit] as usize;
                    mask[vertex] = true;
                    coefficients[vertex] = true;
                    any = true;
                }
            }
            if !any {
                continue;
            }
            // A level's coefficients contribute to all their subsets. Retain
            // the original decoder's cancellation window of p_dims levels,
            // including its behavior on words beyond the correction bound.
            if self.p_dims > 1 {
                superset_xor(&mut mask);
            }
            let end = self.cubes_in_level[dim + self.p_dims as usize];
            for &vertex in &self.bit_to_cube[self.cubes_in_level[dim]..end] {
                let vertex = vertex as usize;
                code[vertex >> 6] ^= u64::from(mask[vertex]) << (vertex & 63);
            }
        }
        superset_xor(&mut coefficients);
        Ok(self.bit_to_cube[..self.data_len].iter()
            .map(|&v| coefficients[v as usize]).collect())
    }

    fn level(&self, level: usize) -> std::ops::Range<usize> {
        self.cubes_in_level[level]..self.cubes_in_level[level + 1]
    }

    /// Majority vote using shared assignments and directly indexed vertices.
    fn vote(&self, code: &[u64], bit: usize) -> bool {
        let fixed = self.bit_to_cube[bit];
        let free = self.cw_len as u32 ^ fixed;
        let mut lanes = 1u64;
        let mut low_free = free & 63;
        while low_free != 0 {
            let shift = low_free & low_free.wrapping_neg();
            lanes |= lanes << shift;
            low_free ^= shift;
        }
        let word_free = free >> 6;
        let mut positions = [0u32; MAX_DIMS as usize];
        let mut remaining = fixed;
        let mut weight = 0;
        while remaining != 0 {
            let position = remaining & remaining.wrapping_neg();
            positions[weight] = position;
            weight += 1;
            remaining ^= position;
        }
        let mut votes = [0usize; 2];
        for &complement in &self.voter_complements[weight] {
            let mut base = fixed;
            let mut zeros = complement;
            while zeros != 0 {
                base ^= positions[zeros.trailing_zeros() as usize];
                zeros &= zeros - 1;
            }
            let mut parity_word = 0u64;
            let mut subset = 0;
            loop {
                parity_word ^= code[((base >> 6) | subset) as usize];
                subset = subset.wrapping_sub(word_free) & word_free;
                if subset == 0 {
                    break;
                }
            }
            let parity = (parity_word & (lanes << (base & 63))).count_ones() & 1 != 0;
            votes[parity as usize] += 1;
            if votes[parity as usize] == self.needed_votes {
                return parity;
            }
        }
        false
    }

    fn to_vertices(&self, bits: &[bool]) -> Bits {
        let mut values = vec![false; self.cw_len + 1];
        for (&bit, &vertex) in bits.iter().zip(&self.bit_to_cube) {
            values[vertex as usize] = bit;
        }
        values
    }

    /// Turns the decoded coefficients back into the original data bits.
    pub fn code_from_coefficients(&self, coefficients: &[bool]) -> Bits {
        // Data vertices only have data vertices as supersets.
        assert!(coefficients.len() >= self.data_len);
        let mut values = self.to_vertices(&coefficients[..self.data_len]);
        superset_xor(&mut values);
        self.bit_to_cube[..self.data_len].iter().map(|&v| values[v as usize]).collect()
    }

    /// Soft-decision Successive Cancellation List (SCL) decoder with default list size 8.
    ///
    /// Convention: `received[i] > 0.0` favors bit `1` (`true`), `received[i] < 0.0`
    /// favors bit `0` (`false`), and `|received[i]|` is the log-likelihood magnitude.
    pub fn decode_soft(&self, received: &[f64]) -> Result<Bits, Error> {
        self.decode_soft_list(received, 8)
    }

    /// Soft-decision Successive Cancellation List (SCL) decoder with `list_size` candidate paths.
    pub fn decode_soft_list(&self, received: &[f64], list_size: usize) -> Result<Bits, Error> {
        check_len(self.cw_len, received.len())?;
        self.decode_soft_list_with_zero(received, None, list_size)
    }

    pub(crate) fn decode_soft_list_with_zero(
        &self,
        received: &[f64],
        zero_vertex_llr: Option<f64>,
        list_size: usize,
    ) -> Result<Bits, Error> {
        if self.p_dims == 0 {
            return Ok(received.iter().map(|&y| y > 0.0).collect());
        }
        let list_size = list_size.max(1);

        // Fast path: if the hard-decision word already satisfies all parity checks, return it.
        let hard: Bits = received.iter().map(|&y| y > 0.0).collect();
        let mut mask = self.to_vertices(&hard);
        superset_xor(&mut mask);
        if self.bit_to_cube[self.data_len..].iter().all(|&v| !mask[v as usize])
            && zero_vertex_llr.map_or(true, |z| (z > 0.0) == mask[0])
        {
            return Ok(hard[..self.data_len].to_vec());
        }

        // Reuse the cached SclDecoder workspace across calls, growing it only when `list_size` increases.
        let mut guard = self.scl_workspace.lock().unwrap_or_else(|e| e.into_inner());
        let decoder = guard.get_or_insert_with(|| SclDecoder::new(self.cw_dims, self.p_dims + 1, list_size));
        decoder.prepare(list_size, &self.bit_to_cube, received, zero_vertex_llr);
        let values = decoder.decode();
        Ok(self.bit_to_cube[..self.data_len]
            .iter()
            .map(|&v| values[v as usize])
            .collect())
    }
}

#[derive(Debug)]
struct SclDecoder {
    dims: u32,
    min_data_weight: u32,
    list_size: usize,
    active: usize,
    metrics: Vec<f64>,
    /// Flat LLR workspace: `list_size` paths, each holding `2 * n` f64s (level `d` starts at `(1 << d) - 1`).
    llrs: Vec<f64>,
    /// Flat partial-sums workspace: `list_size` paths, each holding `n` bools.
    sums: Vec<bool>,
    llr_stride: usize,
    sum_stride: usize,
    candidates: Vec<(f64, usize, bool)>,
    parent_used: Vec<bool>,
    deferred: Vec<usize>,
}

impl SclDecoder {
    fn new(dims: u32, min_data_weight: u32, list_size: usize) -> Self {
        let n = 1usize << dims;
        let llr_stride = 2 * n;
        let sum_stride = n;
        Self {
            dims,
            min_data_weight,
            list_size,
            active: 1,
            metrics: vec![0.0f64; list_size],
            llrs: vec![0.0f64; list_size * llr_stride],
            sums: vec![false; list_size * sum_stride],
            llr_stride,
            sum_stride,
            candidates: Vec::with_capacity(list_size * 2),
            parent_used: vec![false; list_size],
            deferred: Vec::with_capacity(list_size),
        }
    }

    fn prepare(
        &mut self,
        list_size: usize,
        bit_to_cube: &[u32],
        received: &[f64],
        zero_vertex_llr: Option<f64>,
    ) {
        self.list_size = list_size;
        if self.metrics.len() < list_size {
            self.metrics.resize(list_size, 0.0);
            self.llrs.resize(list_size * self.llr_stride, 0.0);
            self.sums.resize(list_size * self.sum_stride, false);
            self.parent_used.resize(list_size, false);
        }
        self.active = 1;
        self.metrics[0] = 0.0;

        // Convert convention (`> 0` favors 1) to standard polar LLR (`> 0` favors 0): L = -y.
        // Vertex 0 is punctured in standalone `CubeCode` (erasure: LLR = 0.0), or supplied by `ReedMuller`.
        let root_off = Self::llr_offset(self.dims);
        self.llrs[root_off] = zero_vertex_llr.map_or(0.0, |z| -z);
        for (&y, &vertex) in received.iter().zip(bit_to_cube) {
            self.llrs[root_off + vertex as usize] = -y;
        }
    }

    #[inline]
    fn llr_offset(dim: u32) -> usize {
        (1usize << dim) - 1
    }

    fn decode(&mut self) -> &[bool] {
        self.step(self.dims, 0);
        let best_idx = (0..self.active)
            .min_by(|&a, &b| self.metrics[a].total_cmp(&self.metrics[b]))
            .unwrap();
        let start = best_idx * self.sum_stride;
        &self.sums[start..start + self.sum_stride]
    }

    fn step(&mut self, dim: u32, offset: u32) {
        if dim == 0 {
            let v = offset as usize;
            let is_data = offset.count_ones() >= self.min_data_weight;
            if !is_data {
                for p in 0..self.active {
                    let l = self.llrs[p * self.llr_stride];
                    if l < 0.0 {
                        self.metrics[p] -= l;
                    }
                    self.sums[p * self.sum_stride + v] = false;
                }
                return;
            }

            // Data vertex: fork active paths into 0 (`false`) and 1 (`true`).
            self.candidates.clear();
            for p in 0..self.active {
                let l = self.llrs[p * self.llr_stride];
                let m = self.metrics[p];
                let (m0, m1) = if l >= 0.0 { (m, m + l) } else { (m - l, m) };
                self.candidates.push((m0, p, false));
                self.candidates.push((m1, p, true));
            }
            if self.candidates.len() > self.list_size {
                self.candidates
                    .select_nth_unstable_by(self.list_size - 1, |a, b| a.0.total_cmp(&b.0));
                self.candidates.truncate(self.list_size);
            }

            // Keep the first child of each surviving parent in-place in slot `parent`,
            // and place second children into unused slots so we only copy on actual forks.
            let new_active = self.candidates.len();
            self.parent_used.fill(false);
            self.deferred.clear();

            for c_idx in 0..new_active {
                let (metric, parent, bit) = self.candidates[c_idx];
                if !self.parent_used[parent] {
                    self.parent_used[parent] = true;
                    self.metrics[parent] = metric;
                    self.sums[parent * self.sum_stride + v] = bit;
                } else {
                    self.deferred.push(c_idx);
                }
            }

            let mut free_slot = 0;
            for d in 0..self.deferred.len() {
                let (metric, parent, bit) = self.candidates[self.deferred[d]];
                while self.parent_used[free_slot] {
                    free_slot += 1;
                }
                self.parent_used[free_slot] = true;
                self.metrics[free_slot] = metric;
                // Copy the parent's full LLR tree (upper levels are still needed as recursion unwinds)
                // and the partial sums computed so far (`0..v`).
                let llr_src = parent * self.llr_stride;
                let llr_dst = free_slot * self.llr_stride;
                self.llrs.copy_within(llr_src..llr_src + self.llr_stride, llr_dst);

                let sum_src = parent * self.sum_stride;
                let sum_dst = free_slot * self.sum_stride;
                if v > 0 {
                    self.sums.copy_within(sum_src..sum_src + v, sum_dst);
                }
                self.sums[sum_dst + v] = bit;
            }
            self.active = new_active;
            return;
        }

        let half = 1usize << (dim - 1);
        let parent_off = Self::llr_offset(dim);
        let child_off = Self::llr_offset(dim - 1);
        let base_v = offset as usize;

        // 1. Left branch: min-sum Box-Plus f(l1, l2) = sign(l1)*sign(l2)*min(|l1|, |l2|).
        for p in 0..self.active {
            let p_base = p * self.llr_stride;
            for i in 0..half {
                let l1 = self.llrs[p_base + parent_off + i];
                let l2 = self.llrs[p_base + parent_off + half + i];
                self.llrs[p_base + child_off + i] = min_sum(l1, l2);
            }
        }

        self.step(dim - 1, offset);

        // 2. Right branch: g(l1, l2, u_lower) = l2 - l1 (if u_lower) else l2 + l1.
        for p in 0..self.active {
            let p_base = p * self.llr_stride;
            let s_base = p * self.sum_stride + base_v;
            for i in 0..half {
                let l1 = self.llrs[p_base + parent_off + i];
                let l2 = self.llrs[p_base + parent_off + half + i];
                let u = self.sums[s_base + i];
                self.llrs[p_base + child_off + i] = if u { l2 - l1 } else { l2 + l1 };
            }
        }

        self.step(dim - 1, offset | (half as u32));

        // 3. Butterfly combine in-place: u_lower[i] ^= u_upper[i].
        for p in 0..self.active {
            let s_base = p * self.sum_stride + base_v;
            let sub = &mut self.sums[s_base..s_base + 2 * half];
            let (u_lower, u_upper) = sub.split_at_mut(half);
            for (a, b) in u_lower.iter_mut().zip(u_upper) {
                *a ^= *b;
            }
        }
    }
}

#[inline(always)]
fn min_sum(l1: f64, l2: f64) -> f64 {
    let min_abs = l1.abs().min(l2.abs());
    if (l1.to_bits() ^ l2.to_bits()) >> 63 != 0 {
        -min_abs
    } else {
        min_abs
    }
}

fn pack_vertices(values: &[bool]) -> Vec<u64> {
    values.chunks(64).map(|chunk| {
        chunk.iter().enumerate().fold(0, |word, (i, &bit)| word | (u64::from(bit) << i))
    }).collect()
}

/// Replace each vertex's value by the XOR over all its supersets in O(m * 2^m).
/// Applying the butterfly twice restores the original values.
fn superset_xor(values: &mut [bool]) {
    let mut half = 1;
    while half < values.len() {
        for block in values.chunks_exact_mut(2 * half) {
            let (lower, upper) = block.split_at_mut(half);
            for (a, b) in lower.iter_mut().zip(upper) {
                *a ^= *b;
            }
        }
        half *= 2;
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
    fn initialization_preserves_weight_and_vertex_order() {
        for m in 1..=10 {
            let code = CubeCode::new(m, m / 2).unwrap();
            let mut expected: Vec<u32> = (1..1 << m).collect();
            expected.sort_unstable_by_key(|&v| std::cmp::Reverse((v.count_ones(), v)));
            assert_eq!(code.bit_to_cube(), expected);
            for (bit, &vertex) in expected.iter().enumerate() {
                assert_eq!(code.cube_to_bit()[vertex as usize], bit);
            }
            for level in 0..m as usize {
                let vertices = &expected[code.level(level)];
                assert!(vertices.iter().all(|v| v.count_ones() == m - level as u32));
            }
        }
    }

    #[test]
    fn coefficient_transform_matches_direct_cube_parities() {
        for m in 1..=8 {
            for p in 0..m {
                let code = CubeCode::new(m, p).unwrap();
                let coefficients: Vec<bool> = (0..code.codeword_len())
                    .map(|i| (i * 13 + i / 3) % 7 < 3).collect();
                let expected: Vec<bool> = code.bit_to_cube()[..code.data_len()].iter()
                    .map(|&v| HyperCube::unit(m, v).elements().fold(false, |acc, superset| {
                        acc ^ coefficients[code.cube_to_bit()[superset as usize]]
                    })).collect();
                assert_eq!(code.code_from_coefficients(&coefficients), expected);
                assert_eq!(code.code_from_coefficients(&coefficients[..code.data_len()]), expected);
            }
        }
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

    #[test]
    fn soft_decoding_3bit_and_7bit_examples() {
        // 1. 3-bit codeword (cw_dims = 2, p_dims = 1):
        // Sent [1, 1, 1], received [+0.95, -0.05, -0.08] -> two weak bit errors!
        // Hard decoding gets [1, 0, 0] and votes 0 (wrong); soft decoding recovers 1.
        let code3 = CubeCode::new(2, 1).unwrap();
        let soft3 = [0.95, -0.05, -0.08];
        let hard3: Bits = soft3.iter().map(|&y| y > 0.0).collect();
        assert_eq!(code3.decode(&hard3).unwrap(), vec![false]);
        assert_eq!(code3.decode_soft(&soft3).unwrap(), vec![true]);

        // 2. 7-bit codeword (cw_dims = 3, p_dims = 1):
        // Sent all-zeros [0, 0, 0, 0] -> codeword [0; 7].
        // Order of vertices in bit_to_cube: [111, 110, 101, 011, 100, 010, 001].
        // Flip vertex 110 (bit 1) strongly (+0.80) and vertex 010 (bit 5) weakly (+0.05).
        // Hard decoding fails (2 errors > tolerance 1); soft decoding recovers [0, 0, 0, 0].
        let code7 = CubeCode::new(3, 1).unwrap();
        let soft7 = [-0.90, 0.80, -0.90, -0.90, -0.90, 0.05, -0.90];
        let hard7: Bits = soft7.iter().map(|&y| y > 0.0).collect();
        assert_ne!(code7.decode(&hard7).unwrap(), vec![false; 4]);
        assert_eq!(code7.decode_soft(&soft7).unwrap(), vec![false; 4]);
    }
}
