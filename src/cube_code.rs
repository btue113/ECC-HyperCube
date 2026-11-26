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
use crate::list_decoder::{classify_subtrees, new_list_decoder, ListDecode, Subtree, MAX_LIST_SIZE};
use crate::{Bits, Error};
use std::sync::{Mutex, OnceLock};

/// Hard cap that keeps the vertex tables comfortably within memory.
pub const MAX_DIMS: u32 = 24;
/// Plotkin splits the ensemble decoder takes before decoding flat. A second split cut word
/// errors at n = 1024 for about twice the time, still well short of 5G polar codes there.
const NEST_LEVELS: u32 = 1;

/// Early-stop heuristic for ensemble decoding (`list_size >= 4`): stop if the runner-up
/// codeword in the list costs more than `EARLY_STOP_COST_RATIO * best_cost`.
///
/// Scale-invariant (needs no SNR estimate). When a list lands on the true codeword, the
/// runner-up lies at distance >= d and typically costs ~1.8-2.5x as much; when the list
/// misses the true codeword, the surviving paths cluster at nearly equal high costs (~1.05x).
/// Chosen empirically over 1.25M frames across n = 128..1024: 1.5 and 1.65 caused false
/// stops at L = 4 (`(512, 256)` 3.0 dB 4x16: 2 -> 29 and 2 -> 5 errors), while 1.8 added
/// zero errors across all 10 configs (L = 4 and L = 8) for a 1.1-1.85x speedup.
const EARLY_STOP_COST_RATIO: f64 = 1.8;

#[derive(Debug)]
pub struct CubeCode {
    cw_dims: u32,
    p_dims: u32,
    cw_len: usize,
    data_len: usize,
    /// Lookup table indexed by vertex `0..=cw_len`: `true` for unfrozen vertices, `false` for frozen parity vertices.
    is_unfrozen: Vec<bool>,
    /// Kind of every subtree of the soft decoder's tree (see `classify_subtrees`).
    subtrees: Vec<Subtree>,
    needed_votes: usize,
    bit_to_cube: Vec<u32>,
    cube_to_bit: Vec<usize>,
    cubes_in_level: Vec<usize>,
    /// Compact voting assignments shared by vertices with the same weight.
    voter_complements: Vec<Vec<u32>>,
    /// Materialized only when a caller requests the public cube view.
    voters: Vec<OnceLock<Vec<HyperCube>>>,
    /// List decoders reused across calls by convenience methods that don't pass an [`RmScratch`].
    list_decoders: Mutex<RmScratch>,
    /// Lazily constructed Plotkin subcodes `(RM(r-1, m-1), RM(r, m-1))` for nested
    /// automorphism-ensemble decoding when `cw_dims >= 9`.
    nested_children: OnceLock<Box<(CubeCode, CubeCode)>>,
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

        let first_parity_level = (cw_dims - p_dims) as usize;
        let data_len = cubes_in_level[first_parity_level];
        let mut is_unfrozen = vec![false; cw_len + 1];
        for &v in &bit_to_cube[..data_len] {
            is_unfrozen[v as usize] = true;
        }

        let subtrees = classify_subtrees(&is_unfrozen);

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
            is_unfrozen,
            subtrees,
            needed_votes: 1 << p_dims,
            bit_to_cube,
            cube_to_bit,
            cubes_in_level,
            voter_complements,
            voters,
            list_decoders: Mutex::new(RmScratch::new()),
            nested_children: OnceLock::new(),
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
        if (1..=self.cw_len).all(|v| self.is_unfrozen[v] || !mask[v]) {
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
        Ok(self.bit_to_cube[..self.data_len]
            .iter()
            .map(|&v| coefficients[v as usize])
            .collect())
    }

    fn level(&self, level: usize) -> std::ops::Range<usize> {
        self.cubes_in_level[level]..self.cubes_in_level[level + 1]
    }

    /// Majority vote for the coefficient at the vertex of data bit `bit`.
    ///
    /// The vertex's unit cube is every superset of it: its 1-digits are
    /// fixed and its 0-digits are free. The cubes that vote are the copies of
    /// that cube with some fixed digits switched to 0 (the unit cube itself is
    /// one of them). Each copy votes with the XOR of the received bits on its
    /// corners, and the first side to reach a majority wins.
    ///
    /// `code` holds vertex `v` at bit `v & 63` of word `v >> 6`. Moving along
    /// digit `d` is then a fixed jump of `2^d` bits, so a voting cube is a
    /// fixed bit pattern: a mask over its free low digits (0..6) inside each
    /// word, repeated across the words picked by its free high digits.
    fn vote(&self, code: &[u64], bit: usize) -> bool {
        let fixed = self.bit_to_cube[bit];
        let free = self.cw_len as u32 ^ fixed;
        // In-word mask of a cube based at 0: one bit per combination of the
        // free low digits. Each free digit doubles the pattern, e.g. free
        // digits {0, 2} give lanes = 0b110011 (bits 0, 1, 4, 5).
        let mut lanes = 1u64;
        let mut low_free = free & 63;
        while low_free != 0 {
            let shift = low_free & low_free.wrapping_neg();
            lanes |= lanes << shift;
            low_free ^= shift;
        }
        // Free digits 6 and up select which words the cube spans.
        let word_free = free >> 6;
        // positions[j] is the j-th lowest 1-digit of the vertex, as a bit.
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
        // Every vertex of the same weight votes with the same list of cubes,
        // up to relabelling digits. So the list is stored once per weight:
        // bit j of `complement` says "switch the vertex's j-th 1-digit to 0".
        for &complement in &self.voter_complements[weight] {
            // The voting cube's lowest corner: the vertex with those 1-digits
            // cleared. Its free digits are all 0, so adding any combination
            // of them (`base | s`) reaches every corner of the cube.
            let mut base = fixed;
            let mut zeros = complement;
            while zeros != 0 {
                base ^= positions[zeros.trailing_zeros() as usize];
                zeros &= zeros - 1;
            }
            // XOR together every word the cube touches. `subset` walks all
            // combinations of the free high digits (Knuth's
            // `(s - mask) & mask` step), starting and ending at 0.
            let mut parity_word = 0u64;
            let mut subset = 0;
            loop {
                parity_word ^= code[((base >> 6) | subset) as usize];
                subset = subset.wrapping_sub(word_free) & word_free;
                if subset == 0 {
                    break;
                }
            }
            // Keep only this cube's lanes and count them: odd votes 1.
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

    /// Soft-decision Successive Cancellation List (SCL) decoder with `list_size` candidate paths
    /// (at most 256).
    pub fn decode_soft_list(&self, received: &[f64], list_size: usize) -> Result<Bits, Error> {
        check_len(self.cw_len, received.len())?;
        self.decode_soft_list_with_zero(received, None, list_size)
    }

    /// [`CubeCode::decode_soft_list`] using a caller-owned [`RmScratch`] instead of the
    /// internal `Mutex`, so threads can share one code.
    pub fn decode_soft_list_with_scratch(
        &self,
        received: &[f64],
        list_size: usize,
        scratch: &mut RmScratch,
    ) -> Result<Bits, Error> {
        check_len(self.cw_len, received.len())?;
        self.decode_soft_list_with_zero_scratch(received, None, list_size, scratch)
    }

    pub(crate) fn decode_soft_list_with_zero(
        &self,
        received: &[f64],
        zero_vertex_llr: Option<f64>,
        list_size: usize,
    ) -> Result<Bits, Error> {
        let mut guard = self.list_decoders.lock().unwrap_or_else(|e| e.into_inner());
        self.decode_soft_list_with_zero_scratch(received, zero_vertex_llr, list_size, &mut guard)
    }

    pub(crate) fn decode_soft_list_with_zero_scratch(
        &self,
        received: &[f64],
        zero_vertex_llr: Option<f64>,
        list_size: usize,
        scratch: &mut RmScratch,
    ) -> Result<Bits, Error> {
        if self.p_dims == 0 {
            return Ok(received.iter().map(|&y| y > 0.0).collect());
        }
        let list_size = list_size.clamp(1, MAX_LIST_SIZE);

        // Fast path: if the hard-decision word already satisfies all parity checks, return it.
        let hard: Bits = received.iter().map(|&y| y > 0.0).collect();
        let code = self.to_vertices(&hard);
        let mut mask = code.clone();
        superset_xor(&mut mask);
        if (1..=self.cw_len).all(|v| self.is_unfrozen[v] || !mask[v])
            && zero_vertex_llr.map_or(true, |z| (z > 0.0) == mask[0])
        {
            return Ok(hard[..self.data_len].to_vec());
        }

        let y = self.vertex_readings(received, zero_vertex_llr.unwrap_or(0.0));
        let decoder = scratch.decoder(self.cw_dims, list_size);
        decoder.decode(&y, list_size, &self.subtrees);
        let mut word = Vec::new();
        decoder.word(0, &mut word);
        Ok(self.bit_to_cube[..self.data_len].iter().map(|&v| word[v as usize]).collect())
    }

    /// Automorphism-ensemble SCL: decode up to `ensemble_size` relabelled copies of the
    /// cube and keep the codeword that disagrees least with the received word.
    ///
    /// RM codes are unchanged by any affine relabelling of the corners' digits,
    /// `x -> A x ^ b`, but the decoder's fixed sweep order is not. Each relabelling
    /// shows the decoder the same word in a different order, so its mistakes differ.
    /// The first copy is the identity, so the result is never worse than plain SCL.
    ///
    /// For large cubes (`cw_dims >= 9`, `n >= 512`) with `2 <= r <= m - 2`, the copies after
    /// the first split the cube once by Plotkin's decomposition `RM(r, m) = (u, u ^ v)` with
    /// `u in RM(r-1, m-1)` and `v in RM(r, m-1)`, running an ensemble on each half inside
    /// each outer relabelling.
    ///
    /// Decoding stops at the first copy whose best word is proven maximum likelihood.
    /// The best codeword is the same in every copy, so one copy's proof covers them all.
    pub(crate) fn decode_soft_ensemble_with_zero(
        &self,
        received: &[f64],
        zero_vertex_llr: f64,
        list_size: usize,
        ensemble_size: usize,
    ) -> Result<SoftDecision, Error> {
        let mut guard = self.list_decoders.lock().unwrap_or_else(|e| e.into_inner());
        self.decode_soft_ensemble_with_zero_scratch(
            received,
            zero_vertex_llr,
            list_size,
            ensemble_size,
            &mut guard,
        )
    }

    pub(crate) fn decode_soft_ensemble_with_zero_scratch(
        &self,
        received: &[f64],
        zero_vertex_llr: f64,
        list_size: usize,
        ensemble_size: usize,
        scratch: &mut RmScratch,
    ) -> Result<SoftDecision, Error> {
        if self.p_dims == 0 {
            return Err(Error::Unsupported("ensemble decoding needs p_dims > 0"));
        }
        let list_size = list_size.clamp(1, MAX_LIST_SIZE);
        let ensemble_size = ensemble_size.max(1);
        let y = self.vertex_readings(received, zero_vertex_llr);
        let mut best = vec![false; self.cw_len + 1];
        let (cost, runner_up_cost, pruned_bound, distance_certified) = self
            .decode_vertices_ensemble(&y, list_size, ensemble_size, NEST_LEVELS, &mut best, scratch);
        let message =
            self.bit_to_cube[..self.data_len].iter().map(|&v| best[v as usize]).collect();
        Ok(SoftDecision {
            message,
            cost,
            runner_up_cost,
            pruned_bound,
            distance_certified,
            certified: distance_certified || cost <= pruned_bound,
        })
    }

    /// Vertex-domain ensemble decoder shared by top-level and recursive Plotkin calls.
    /// Writes the winning codeword (in vertex order `0..2^cw_dims`) into `best` and returns
    /// `(cost, runner_up_cost, pruned_bound, distance_certified)`. `nest_levels` caps how
    /// many Plotkin splits may still be taken on the way down.
    fn decode_vertices_ensemble(
        &self,
        y: &[f64],
        list_size: usize,
        ensemble_size: usize,
        nest_levels: u32,
        best: &mut [bool],
        scratch: &mut RmScratch,
    ) -> (f64, f64, f64, bool) {
        let n = self.cw_len + 1;

        // Fast path: the hard-decision word is already a codeword, at cost 0.
        let mut mask: Bits = y.iter().map(|&l| l > 0.0).collect();
        let hard = mask.clone();
        superset_xor(&mut mask);
        if (0..n).all(|v| self.is_unfrozen[v] || !mask[v]) {
            best.copy_from_slice(&hard);
            return (0.0, f64::INFINITY, f64::INFINITY, true);
        }

        // Cheap first pass: try a small list (L = 1, then L = 4 for lists of 16 or more) on
        // the identity cube and exit if the decision clears the same bar as copy 0 of the
        // full ensemble. Two exits prove maximum likelihood (distance test, cost <= pruned
        // bound); the other two are the ensemble's heuristics (fewer than d/2 flips, and the
        // runner-up ratio at L >= 4), now applied to the smaller list. An L = 1 exit has no
        // runner-up, so it reports `runner_up_cost = INFINITY`.
        if list_size > 1 && ensemble_size > 1 {
            let min_distance = 1usize << (self.p_dims + 1);
            let mut word = Vec::with_capacity(n);
            for &probe in &[1usize, 4usize] {
                if probe >= list_size || (probe == 4 && list_size < 16) {
                    continue;
                }
                let decoder = scratch.decoder(self.cw_dims, probe);
                decoder.decode(y, probe, &self.subtrees);
                let pruned = decoder.pruned_min();
                let runner_up = if decoder.paths() > 1 { decoder.metric(1) } else { f64::INFINITY };
                decoder.word(0, &mut word);
                let mut cost = 0.0;
                let mut flips = 0usize;
                for (&c, &l) in word.iter().zip(y) {
                    if c != (l > 0.0) {
                        cost += l.abs();
                        flips += 1;
                    }
                }
                let dist_cert = is_ml_certified(&word, y, min_distance);
                if dist_cert
                    || cost <= pruned
                    || flips < self.needed_votes
                    || (probe >= 4 && runner_up > EARLY_STOP_COST_RATIO * cost)
                {
                    best.copy_from_slice(&word);
                    return (cost, runner_up, pruned, dist_cert);
                }
            }
        }

        // For n >= 512 and 2 <= r <= m - 2, recurse via Plotkin's decomposition
        // RM(r, m) = {(c1 ^ u, c1) : u in RM(r-1, m-1), c1 in RM(r, m-1)}. At r = 1 the
        // `u` half is a repetition code and the flat ensemble makes fewer errors.
        if nest_levels > 0
            && self.cw_dims >= 9
            && ensemble_size > 1
            && self.p_dims >= 1
            && self.p_dims + 2 < self.cw_dims
        {
            return self.decode_vertices_nested_ensemble(
                y,
                list_size,
                ensemble_size,
                nest_levels,
                best,
                scratch,
            );
        }
        self.decode_vertices_flat(y, list_size, ensemble_size, best, scratch)
    }

    /// Automorphism-ensemble SCL on the whole cube: copy 0 is the identity, the rest are
    /// random affine relabellings.
    fn decode_vertices_flat(
        &self,
        y: &[f64],
        list_size: usize,
        ensemble_size: usize,
        best: &mut [bool],
        scratch: &mut RmScratch,
    ) -> (f64, f64, f64, bool) {
        let n = self.cw_len + 1;
        let min_distance = 1usize << (self.p_dims + 1);
        let decoder = scratch.decoder(self.cw_dims, list_size);
        let mut word = Vec::with_capacity(n);
        // A fixed seed keeps decoding deterministic.
        let mut seed = 0x9E37_79B9_7F4A_7C15u64;
        let mut perm = vec![0u32; n];
        let mut y_perm = vec![0.0; n];
        let mut best_cost = f64::INFINITY;
        let mut best_flips = usize::MAX;
        let mut distance_certified = false;
        let mut runner_up_cost = f64::INFINITY;
        let mut agreements = 0usize;
        // Each copy proves that no codeword outside its final list costs less than its
        // `pruned_min`, so the best word is maximum likelihood once it costs no more than
        // the largest of these bounds.
        let mut pruned_bound = 0.0f64;
        for copy in 0..ensemble_size {
            if copy == 0 {
                for x in 0..n {
                    perm[x] = x as u32;
                }
            } else {
                let columns = random_invertible(self.cw_dims, &mut seed);
                perm[0] = (splitmix64(&mut seed) as u32) & (self.cw_len as u32);
                // perm[x] = A x ^ b, built from x with its lowest 1-digit cleared.
                for x in 1..n {
                    perm[x] = perm[x & (x - 1)] ^ columns[x.trailing_zeros() as usize];
                }
            }
            for (yp, &p) in y_perm.iter_mut().zip(&perm) {
                *yp = y[p as usize];
            }
            decoder.decode(&y_perm, list_size, &self.subtrees);
            pruned_bound = pruned_bound.max(decoder.pruned_min());
            if decoder.paths() > 1 {
                runner_up_cost = runner_up_cost.min(decoder.metric(1));
            }
            decoder.word(0, &mut word);
            let mut cost = 0.0;
            let mut flips = 0usize;
            for (&c, &l) in word.iter().zip(&y_perm) {
                if c != (l > 0.0) {
                    cost += l.abs();
                    flips += 1;
                }
            }
            if cost < best_cost - 1e-9 {
                runner_up_cost = runner_up_cost.min(best_cost);
                best_cost = cost;
                best_flips = flips;
                for (&c, &p) in word.iter().zip(&perm) {
                    best[p as usize] = c;
                }
                distance_certified = is_ml_certified(best, y, min_distance);
                agreements = 1;
            } else if cost <= best_cost + 1e-9
                && word.iter().zip(&perm).all(|(&c, &p)| c == best[p as usize])
            {
                agreements += 1;
            } else if cost > best_cost + 1e-9
                || word.iter().zip(&perm).any(|(&c, &p)| c != best[p as usize])
            {
                runner_up_cost = runner_up_cost.min(cost);
            }
            // Stop when the best word is proven maximum likelihood. Two cheap heuristics
            // (not proofs) also stop: the best word is within d/2 - 1 flips of the hard
            // decision, where it is the unique nearest codeword in Hamming distance; or three
            // copies with lists agree on it (greedy L = 1 copies agree on wrong words too often).
            // Three, not two, also inside the Plotkin decoder's `u` step: two was free at
            // (512, 256) but gave 18% more word errors at (512, 130), where `u` is the weaker
            // RM(2, 8).
            if distance_certified
                || best_cost <= pruned_bound
                || best_flips < self.needed_votes
                || (list_size >= 4
                    && (agreements >= 3 || runner_up_cost > EARLY_STOP_COST_RATIO * best_cost))
            {
                break;
            }
        }
        (best_cost, runner_up_cost, pruned_bound, distance_certified)
    }

    /// Recursive Plotkin automorphism-ensemble decoder for `cw_dims >= 9` (`n >= 512`).
    ///
    /// Copy 0 is plain SCL on the whole cube, so the result is never worse than SCL and
    /// keeps the list's pruned bound as an ML certificate. Each further copy is an outer
    /// `GA(m, 2)` relabelling (first cycling through all `m` coordinate axes for distinct
    /// top splits, then random affine maps):
    /// 1. Fold the top axis via min-sum to get `y_u` for `u = c0 ^ c1 in RM(r-1, m-1)`.
    /// 2. Decode `u` with an inner ensemble on `RM(r-1, m-1)`.
    /// 3. Combine `y_v = g(y0, y1, u)` (+3 dB SNR) and decode `c1 in RM(r, m-1)`.
    /// 4. Reconstruct `c = (c1 ^ u, c1)` and check the Taipale–Pursley ML certificate.
    fn decode_vertices_nested_ensemble(
        &self,
        y: &[f64],
        list_size: usize,
        ensemble_size: usize,
        nest_levels: u32,
        best: &mut [bool],
        scratch: &mut RmScratch,
    ) -> (f64, f64, f64, bool) {
        let (mut best_cost, mut runner_up_cost, pruned_bound, mut distance_certified) =
            self.decode_vertices_flat(y, list_size, 1, best, scratch);
        let mut best_flips = best.iter().zip(y).filter(|&(&c, &l)| c != (l > 0.0)).count();
        if distance_certified
            || best_cost <= pruned_bound
            || best_flips < self.needed_votes
            || (list_size >= 4 && runner_up_cost > EARLY_STOP_COST_RATIO * best_cost)
        {
            return (best_cost, runner_up_cost, pruned_bound, distance_certified);
        }
        let mut agreements = 1usize;

        let (u_code, v_code) = self
            .nested_children
            .get_or_init(|| {
                let u = CubeCode::new(self.cw_dims - 1, self.p_dims)
                    .expect("valid RM(r-1, m-1) subcode");
                let v = CubeCode::new(self.cw_dims - 1, self.p_dims - 1)
                    .expect("valid RM(r, m-1) subcode");
                Box::new((u, v))
            })
            .as_ref();

        let n = self.cw_len + 1;
        let half = n / 2;
        let min_distance = 1usize << (self.p_dims + 1);
        let u_copies = ensemble_size.clamp(4, 16);
        let u_list = list_size;
        let v_copies = (ensemble_size / 4).clamp(2, 4);
        let v_list = list_size.min(4);

        let mut seed = 0xD1B5_4A32_D192_ED03u64 ^ ((self.cw_dims as u64) << 32);
        let mut perm = vec![0u32; n];
        let mut y_u = vec![0.0; half];
        let mut y_v = vec![0.0; half];
        let mut u_word = vec![false; half];
        let mut c1_word = vec![false; half];
        let mut cand = vec![false; n];

        for copy in 0..ensemble_size - 1 {
            if copy == 0 {
                for x in 0..n {
                    perm[x] = x as u32;
                }
            } else if copy < self.cw_dims as usize {
                // Swap the top axis with axis `cw_dims - 1 - copy` so the first `m`
                // outer copies try all `m` distinct coordinate hyperplane splits.
                let top = self.cw_dims - 1;
                let other = self.cw_dims - 1 - copy as u32;
                let mut columns = [0u32; 32];
                for b in 0..self.cw_dims as usize {
                    columns[b] = 1u32 << b;
                }
                columns.swap(top as usize, other as usize);
                perm[0] = 0;
                for x in 1..n {
                    perm[x] = perm[x & (x - 1)] ^ columns[x.trailing_zeros() as usize];
                }
            } else {
                let columns = random_invertible(self.cw_dims, &mut seed);
                perm[0] = (splitmix64(&mut seed) as u32) & (self.cw_len as u32);
                for x in 1..n {
                    perm[x] = perm[x & (x - 1)] ^ columns[x.trailing_zeros() as usize];
                }
            }

            // 1. Top-axis min-sum fold (in `> 0` favors 1 convention: u = c0 ^ c1).
            for i in 0..half {
                let a = y[perm[i] as usize];
                let b = y[perm[half + i] as usize];
                let s = if (a >= 0.0) == (b >= 0.0) { -1.0 } else { 1.0 };
                y_u[i] = s * a.abs().min(b.abs());
            }

            // 2. Decode u in RM(r-1, m-1) via inner ensemble.
            u_code.decode_vertices_ensemble(&y_u, u_list, u_copies, nest_levels - 1, &mut u_word, scratch);

            // 3. Combine g(y0, y1, u) and decode c1 in RM(r, m-1).
            for i in 0..half {
                let a = y[perm[i] as usize];
                let b = y[perm[half + i] as usize];
                y_v[i] = if u_word[i] { b - a } else { b + a };
            }
            v_code.decode_vertices_ensemble(&y_v, v_list, v_copies, nest_levels - 1, &mut c1_word, scratch);

            // 4. Reconstruct c = (c1 ^ u, c1) in unpermuted vertex order and score against y.
            let mut cost = 0.0;
            let mut flips = 0usize;
            for i in 0..half {
                let b0 = c1_word[i] ^ u_word[i];
                let b1 = c1_word[i];
                let p0 = perm[i] as usize;
                let p1 = perm[half + i] as usize;
                cand[p0] = b0;
                cand[p1] = b1;
                if b0 != (y[p0] > 0.0) {
                    cost += y[p0].abs();
                    flips += 1;
                }
                if b1 != (y[p1] > 0.0) {
                    cost += y[p1].abs();
                    flips += 1;
                }
            }

            if cost < best_cost - 1e-9 {
                runner_up_cost = runner_up_cost.min(best_cost);
                best_cost = cost;
                best_flips = flips;
                best.copy_from_slice(&cand);
                distance_certified = is_ml_certified(best, y, min_distance);
                agreements = 1;
            } else if cost <= best_cost + 1e-9 && cand == best {
                agreements += 1;
            } else if cost > best_cost + 1e-9 || cand != best {
                runner_up_cost = runner_up_cost.min(cost);
            }

            if distance_certified
                || best_cost <= pruned_bound
                || best_flips < self.needed_votes
                || (list_size >= 4 && agreements >= 3)
            {
                break;
            }
        }
        (best_cost, runner_up_cost, pruned_bound, distance_certified)
    }

    /// Soft SCL decoding that also reports how trustworthy the answer is.
    ///
    /// `zero_vertex_llr` is vertex 0's reading (`None` when punctured, read as an erasure).
    /// The certificate checks the extended code, whose minimum distance is `2^(p_dims + 1)`.
    pub(crate) fn decode_soft_checked_with_zero(
        &self,
        received: &[f64],
        zero_vertex_llr: Option<f64>,
        list_size: usize,
    ) -> Result<SoftDecision, Error> {
        let mut guard = self.list_decoders.lock().unwrap_or_else(|e| e.into_inner());
        self.decode_soft_checked_with_zero_scratch(received, zero_vertex_llr, list_size, &mut guard)
    }

    pub(crate) fn decode_soft_checked_with_zero_scratch(
        &self,
        received: &[f64],
        zero_vertex_llr: Option<f64>,
        list_size: usize,
        scratch: &mut RmScratch,
    ) -> Result<SoftDecision, Error> {
        if self.p_dims == 0 {
            return Err(Error::Unsupported("checked decoding needs p_dims > 0"));
        }
        let list_size = list_size.clamp(1, MAX_LIST_SIZE);
        let min_distance = 1usize << (self.p_dims + 1);
        let y = self.vertex_readings(received, zero_vertex_llr.unwrap_or(0.0));
        let message = |word: &[bool]| -> Bits {
            self.bit_to_cube[..self.data_len].iter().map(|&v| word[v as usize]).collect()
        };

        // Fast path: the hard-decision word is already a codeword, at cost 0.
        let hard: Bits = y.iter().map(|&l| l > 0.0).collect();
        let mut mask = hard.clone();
        superset_xor(&mut mask);
        if (0..=self.cw_len).all(|v| self.is_unfrozen[v] || !mask[v]) {
            return Ok(SoftDecision {
                message: message(&hard),
                cost: 0.0,
                runner_up_cost: f64::INFINITY,
                pruned_bound: f64::INFINITY,
                distance_certified: true,
                certified: true,
            });
        }

        let decoder = scratch.decoder(self.cw_dims, list_size);
        decoder.decode(&y, list_size, &self.subtrees);
        let mut word = Vec::new();
        decoder.word(0, &mut word);
        // A finished path's min-sum metric is its codeword's disagreement cost.
        let runner_up_cost = if decoder.paths() > 1 { decoder.metric(1) } else { f64::INFINITY };
        let pruned_bound = decoder.pruned_min();
        let cost = disagreement_cost(&word, &y);
        let distance_certified = is_ml_certified(&word, &y, min_distance);
        Ok(SoftDecision {
            message: message(&word),
            cost,
            runner_up_cost,
            pruned_bound,
            distance_certified,
            certified: distance_certified || cost <= pruned_bound,
        })
    }

    /// One reading per vertex, `y[v]` for vertex `v`: the received bits, plus vertex 0's
    /// reading (0.0, an erasure, when it was not sent).
    fn vertex_readings(&self, received: &[f64], zero_vertex_llr: f64) -> Vec<f64> {
        let mut y = vec![0.0; self.cw_len + 1];
        y[0] = zero_vertex_llr;
        for (&r, &vertex) in received.iter().zip(&self.bit_to_cube) {
            y[vertex as usize] = r;
        }
        y
    }
}

/// Reusable per-thread workspace for lock-free soft-decision RM decoding.
///
/// Create one `RmScratch::new()` per worker thread and pass it to the `*_with_scratch`
/// methods on [`crate::ReedMuller`] (list, checked and ensemble decoding) or to
/// [`CubeCode::decode_soft_list_with_scratch`], so threads can share a single code object
/// without contending on its internal `Mutex`. One scratch serves any code and list size:
/// list decoders are created on first use and kept for reuse.
#[derive(Debug, Default)]
pub struct RmScratch {
    decoders: Vec<Option<Box<dyn ListDecode>>>,
}

impl RmScratch {
    /// An empty workspace; list decoders are allocated on first use.
    pub fn new() -> Self {
        Self::default()
    }

    /// Cached list decoder for `(dims, list_size)`, created on first use.
    fn decoder(&mut self, dims: u32, list_size: usize) -> &mut dyn ListDecode {
        // MAX_LIST_SIZE = 256 => log2(list_size) is in 0..=8 (9 slots per dimension).
        let slot = (dims as usize) * 9 + list_size.max(1).next_power_of_two().trailing_zeros() as usize;
        if self.decoders.len() <= slot {
            self.decoders.resize_with(slot + 1, || None);
        }
        self.decoders[slot]
            .get_or_insert_with(|| new_list_decoder(dims, list_size))
            .as_mut()
    }
}

/// A random invertible `dims x dims` matrix over GF(2), as its columns.
fn random_invertible(dims: u32, seed: &mut u64) -> Vec<u32> {
    let mask = ((1u64 << dims) - 1) as u32;
    loop {
        let columns: Vec<u32> = (0..dims).map(|_| splitmix64(seed) as u32 & mask).collect();
        // Gaussian elimination on a copy: invertible iff every column adds a new pivot.
        let mut basis = [0u32; 32];
        let independent = columns.iter().all(|&c| {
            let mut c = c;
            while c != 0 {
                let top = 31 - c.leading_zeros() as usize;
                if basis[top] == 0 {
                    basis[top] = c;
                    return true;
                }
                c ^= basis[top];
            }
            false
        });
        if independent {
            return columns;
        }
    }
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)

}

/// A soft decision with evidence about whether to trust it.
#[derive(Debug, Clone, PartialEq)]
pub struct SoftDecision {
    pub message: Bits,
    /// Sum of `|llr|` over the bits where the decoded codeword disagrees with the received signs.
    pub cost: f64,
    /// Cost of the next-best codeword left in the list (`INFINITY` when there is none).
    pub runner_up_cost: f64,
    /// No codeword outside the final list costs less than this (`INFINITY` if nothing was pruned).
    pub pruned_bound: f64,
    /// `true` when the minimum-distance test alone proves maximum likelihood (`is_ml_certified`).
    pub distance_certified: bool,
    /// `true` when the codeword is provably maximum likelihood: by the distance test, or
    /// because it costs no more than `pruned_bound`.
    pub certified: bool,
}

fn disagreement_cost(word: &[bool], llrs: &[f64]) -> f64 {
    word.iter().zip(llrs).filter(|&(&c, &l)| c != (l > 0.0)).map(|(_, &l)| l.abs()).sum()
}

/// Taipale–Pursley sufficient condition for maximum likelihood (1991).
///
/// Any other codeword differs from `word` in at least `min_distance` places. At most
/// `|D|` of those can be places where `word` disagrees with the received signs (set `D`),
/// so at least `min_distance - |D|` are places where `word` agrees, and the other codeword
/// pays for each of them. If even the cheapest such places cost more than `word` does,
/// no codeword is cheaper.
fn is_ml_certified(word: &[bool], llrs: &[f64], min_distance: usize) -> bool {
    let mut cost = 0.0;
    let mut disagreements = 0;
    let mut agreeing: Vec<f64> = Vec::with_capacity(llrs.len());
    for (&c, &l) in word.iter().zip(llrs) {
        if c != (l > 0.0) {
            cost += l.abs();
            disagreements += 1;
        } else {
            agreeing.push(l.abs());
        }
    }
    if disagreements >= min_distance {
        return false;
    }
    let needed = min_distance - disagreements;
    agreeing.select_nth_unstable_by(needed - 1, f64::total_cmp);
    cost < agreeing[..needed].iter().sum::<f64>()
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

    /// Decodes the same AWGN frames with the ensemble (cheap first pass included) and with
    /// the flat ensemble alone (no first pass). Returns
    /// `(frames, first-pass word errors, flat word errors, frames where they differ)` and
    /// asserts that every certified first-pass decision costs no more than the flat one.
    fn first_pass_vs_flat(m: u32, p: u32, db: f64, list: usize, copies: usize, frames: usize) -> (usize, usize, usize, usize) {
        let code = CubeCode::new(m, p).unwrap();
        assert!(code.cw_len + 1 < 512, "the flat decoder is the no-first-pass reference only below n = 512");
        let n = code.cw_len + 1;
        let k = code.data_len();
        let sigma = (1.0 / (2.0 * (k as f64 / n as f64) * 10f64.powf(db / 10.0))).sqrt();
        let mut seed = 0x5EED_0000_0000_0000u64 ^ ((m as u64) << 8) ^ p as u64;
        let gaussian = |seed: &mut u64| {
            let u1 = ((splitmix64(seed) >> 11) as f64 + 1.0) / (1u64 << 53) as f64;
            let u2 = (splitmix64(seed) >> 11) as f64 / (1u64 << 53) as f64;
            (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
        };
        let mut scratch = RmScratch::new();
        let (mut fast_errs, mut flat_errs, mut differ) = (0, 0, 0);
        let (mut fast_word, mut flat_word) = (vec![false; n], vec![false; n]);
        for _ in 0..frames {
            let data: Bits = (0..k).map(|_| splitmix64(&mut seed) & 1 == 1).collect();
            let cw = code.encode(&data).unwrap();
            let mut truth = vec![false; n];
            truth[0] = cw.iter().filter(|&&b| b).count() % 2 == 1;
            for (&b, &v) in cw.iter().zip(&code.bit_to_cube) {
                truth[v as usize] = b;
            }
            let y: Vec<f64> = truth
                .iter()
                .map(|&b| 2.0 / (sigma * sigma) * (if b { 1.0 } else { -1.0 } + sigma * gaussian(&mut seed)))
                .collect();
            let (fast_cost, _, fast_pruned, fast_dist) =
                code.decode_vertices_ensemble(&y, list, copies, NEST_LEVELS, &mut fast_word, &mut scratch);
            let (flat_cost, _, _, _) = code.decode_vertices_flat(&y, list, copies, &mut flat_word, &mut scratch);
            if fast_dist || fast_cost <= fast_pruned {
                assert!(fast_cost <= flat_cost + 1e-9, "certified first-pass word costs more than the flat ensemble's");
            }
            fast_errs += (fast_word != truth) as usize;
            flat_errs += (flat_word != truth) as usize;
            differ += (fast_word != flat_word) as usize;
        }
        (frames, fast_errs, flat_errs, differ)
    }

    #[test]
    fn ensemble_first_pass_matches_flat_ensemble() {
        // RM(3, 7) and RM(4, 8) at 2.5 dB, where word errors are frequent enough to compare.
        for &(m, p) in &[(7u32, 3u32), (8, 3)] {
            let (frames, fast, flat, differ) = first_pass_vs_flat(m, p, 2.5, 8, 8, 1000);
            assert!(fast <= flat + frames / 200, "m = {m}: first pass {fast} vs flat {flat} errors in {frames} frames");
            assert!(differ <= frames / 100, "m = {m}: {differ} of {frames} decisions differ");
        }
    }

    /// Paired WER comparison over many frames; run with
    /// `cargo test --release first_pass_paired -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn first_pass_paired() {
        for &(m, p, db) in &[(7u32, 3u32, 2.5), (7, 3, 3.0), (7, 3, 3.5), (8, 3, 3.0), (8, 3, 3.5), (8, 5, 2.0), (8, 5, 2.5)] {
            let (frames, fast, flat, differ) = first_pass_vs_flat(m, p, db, 8, 8, 200_000);
            println!("m = {m}, p = {p}, {db} dB: first pass {fast}, flat {flat} word errors, {differ} differ, of {frames}");
        }
    }
}
