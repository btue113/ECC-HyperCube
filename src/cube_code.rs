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
    /// List decoders reused across calls, one per width (`list_decoder` picks the slot).
    list_decoders: Mutex<Vec<Option<Box<dyn ListDecode>>>>,
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
            list_decoders: Mutex::new(Vec::new()),
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

    pub(crate) fn decode_soft_list_with_zero(
        &self,
        received: &[f64],
        zero_vertex_llr: Option<f64>,
        list_size: usize,
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
        let mut guard = self.list_decoders.lock().unwrap_or_else(|e| e.into_inner());
        let decoder = list_decoder(&mut guard, self.cw_dims, list_size);
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
    /// Decoding stops at the first copy whose best word is proven maximum likelihood.
    /// The best codeword is the same in every copy, so one copy's proof covers them all.
    pub(crate) fn decode_soft_ensemble_with_zero(
        &self,
        received: &[f64],
        zero_vertex_llr: f64,
        list_size: usize,
        ensemble_size: usize,
    ) -> Result<SoftDecision, Error> {
        if self.p_dims == 0 {
            return Err(Error::Unsupported("ensemble decoding needs p_dims > 0"));
        }
        let list_size = list_size.clamp(1, MAX_LIST_SIZE);
        let min_distance = 1usize << (self.p_dims + 1);
        let n = self.cw_len + 1;
        let y = self.vertex_readings(received, zero_vertex_llr);
        let message = |word: &[bool]| -> Bits {
            self.bit_to_cube[..self.data_len].iter().map(|&v| word[v as usize]).collect()
        };

        // Fast path: the hard-decision word is already a codeword, at cost 0.
        let mut mask: Bits = y.iter().map(|&l| l > 0.0).collect();
        let hard = mask.clone();
        superset_xor(&mut mask);
        if (0..n).all(|v| self.is_unfrozen[v] || !mask[v]) {
            return Ok(SoftDecision {
                message: message(&hard),
                cost: 0.0,
                runner_up_cost: f64::INFINITY,
                pruned_bound: f64::INFINITY,
                distance_certified: true,
                certified: true,
            });
        }

        let mut guard = self.list_decoders.lock().unwrap_or_else(|e| e.into_inner());
        let decoder = list_decoder(&mut guard, self.cw_dims, list_size);
        let mut word = Vec::with_capacity(n);
        // A fixed seed keeps decoding deterministic.
        let mut seed = 0x9E37_79B9_7F4A_7C15u64;
        let mut perm = vec![0u32; n];
        let mut y_perm = vec![0.0; n];
        let mut best = vec![false; n];
        let mut best_cost = f64::INFINITY;
        let mut best_flips = usize::MAX;
        let mut distance_certified = false;
        let mut runner_up_cost = f64::INFINITY;
        let mut agreements = 0usize;
        // Each copy proves that no codeword outside its final list costs less than its
        // `pruned_min`, so the best word is maximum likelihood once it costs no more than
        // the largest of these bounds.
        let mut pruned_bound = 0.0f64;
        for copy in 0..ensemble_size.max(1) {
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
                distance_certified = is_ml_certified(&best, &y, min_distance);
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
            if distance_certified
                || best_cost <= pruned_bound
                || best_flips < self.needed_votes
                || (list_size >= 4 && agreements >= 3)
            {
                break;
            }
        }
        Ok(SoftDecision {
            message: message(&best),
            cost: best_cost,
            runner_up_cost,
            pruned_bound,
            distance_certified,
            certified: distance_certified || best_cost <= pruned_bound,
        })
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

        let mut guard = self.list_decoders.lock().unwrap_or_else(|e| e.into_inner());
        let decoder = list_decoder(&mut guard, self.cw_dims, list_size);
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

/// The cached list decoder wide enough for `list_size` paths, created on first use.
fn list_decoder(
    cache: &mut Vec<Option<Box<dyn ListDecode>>>,
    dims: u32,
    list_size: usize,
) -> &mut dyn ListDecode {
    let slot = list_size.next_power_of_two().trailing_zeros() as usize;
    if cache.len() <= slot {
        cache.resize_with(slot + 1, || None);
    }
    cache[slot].get_or_insert_with(|| new_list_decoder(dims, list_size)).as_mut()
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
}
