//! Soft-decision successive-cancellation list (SCL) decoding on the hypercube.
//!
//! The decoder walks the same tree as the butterfly `superset_xor`: a node of dimension
//! `d` covers `2^d` consecutive vertices, its left half is decided first, then its right
//! half, and the two are combined. Every list path walks the tree in lockstep, so the
//! paths are stored side by side: each vertex holds one `[f32; L]` with a value per path.
//! Every step is then the same operation on all `L` values at once, which the compiler
//! turns into SIMD instructions without any platform-specific code.

use std::fmt::Debug;

/// The subtrees the decoder finishes in one step instead of recursing down to single
/// vertices. Dumer's recursive decoding of Reed–Muller codes ends at RM(0, d), RM(1, d),
/// and RM(d, d) (Green 1966; Dumer 2004; Dumer and Shabunov 2006).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Subtree {
    /// Every vertex is frozen: the only codeword is all zeros.
    Frozen,
    /// Only the subtree's last vertex is unfrozen: the codewords are all zeros and all
    /// ones, a repetition code RM(0, d). A single unfrozen vertex is the case d = 0.
    Repetition,
    /// First-order Reed–Muller code RM(1, d) (`d >= 2`): unfrozen vertices are exactly
    /// those of local weight `>= d - 1`, decoded in one step via Fast Walsh–Hadamard Transform.
    FirstOrder,
    /// Every vertex is unfrozen: every word is a codeword, RM(d, d).
    Free,
    /// Anything else: recurse into the two halves.
    Mixed,
}

/// Classify every subtree of the decoding tree, heap-indexed: node 1 is the whole cube,
/// node `k` has halves `2k` and `2k + 1`, and vertex `v` is leaf `n + v`.
pub(crate) fn classify_subtrees(is_unfrozen: &[bool]) -> Vec<Subtree> {
    let n = is_unfrozen.len();
    // Per node: number of unfrozen vertices, number of vertices, start vertex, and last unfrozen.
    let mut unfrozen = vec![0usize; 2 * n];
    let mut size = vec![1usize; 2 * n];
    let mut start = vec![0usize; 2 * n];
    let mut last_unfrozen = vec![false; 2 * n];
    for (v, &u) in is_unfrozen.iter().enumerate() {
        unfrozen[n + v] = usize::from(u);
        start[n + v] = v;
        last_unfrozen[n + v] = u;
    }
    for k in (1..n).rev() {
        unfrozen[k] = unfrozen[2 * k] + unfrozen[2 * k + 1];
        size[k] = 2 * size[2 * k];
        start[k] = start[2 * k];
        last_unfrozen[k] = last_unfrozen[2 * k + 1];
    }
    (0..2 * n)
        .map(|k| match unfrozen[k] {
            0 => Subtree::Frozen,
            1 if last_unfrozen[k] => Subtree::Repetition,
            u if u == size[k] => Subtree::Free,
            u if size[k] >= 4
                && u == size[k].trailing_zeros() as usize + 1
                && (0..size[k]).all(|x| {
                    is_unfrozen[start[k] + x] == (x.count_ones() >= size[k].trailing_zeros() - 1)
                }) =>
            {
                Subtree::FirstOrder
            }
            _ => Subtree::Mixed,
        })
        .collect()
}

/// A list decoder for one cube, with room for any list size up to its width.
pub(crate) trait ListDecode: Debug + Send {
    /// Decode `y` (one reading per vertex: `> 0` favors 1, `|y|` is the confidence)
    /// keeping at most `list_size` paths, which must not exceed the decoder's width.
    fn decode(&mut self, y: &[f64], list_size: usize, subtrees: &[Subtree]);
    /// Number of paths left in the list.
    fn paths(&self) -> usize;
    /// Metric of the `rank`-th cheapest path. A finished path's min-sum metric is its
    /// codeword's disagreement cost: the sum of `|y|` where the two disagree in sign.
    fn metric(&self, rank: usize) -> f64;
    /// Codeword bits of the `rank`-th cheapest path, one per vertex.
    fn word(&self, rank: usize, out: &mut Vec<bool>);
    /// Lowest metric among paths dropped from the list. Metrics only grow down the tree,
    /// so no codeword outside the final list costs less than this.
    fn pruned_min(&self) -> f64;
}

/// A decoder for `list_size` paths on the `dims`-cube, `list_size` rounded up to a
/// power of two (at most 256).
pub(crate) fn new_list_decoder(dims: u32, list_size: usize) -> Box<dyn ListDecode> {
    match list_size.max(1).next_power_of_two() {
        1 => Box::new(ListDecoder::<1>::new(dims)),
        2 => Box::new(ListDecoder::<2>::new(dims)),
        4 => Box::new(ListDecoder::<4>::new(dims)),
        8 => Box::new(ListDecoder::<8>::new(dims)),
        16 => Box::new(ListDecoder::<16>::new(dims)),
        32 => Box::new(ListDecoder::<32>::new(dims)),
        64 => Box::new(ListDecoder::<64>::new(dims)),
        128 => Box::new(ListDecoder::<128>::new(dims)),
        _ => Box::new(ListDecoder::<256>::new(dims)),
    }
}

/// Largest list size `new_list_decoder` supports.
pub(crate) const MAX_LIST_SIZE: usize = 256;

#[derive(Debug)]
struct ListDecoder<const L: usize> {
    dims: u32,
    list_size: usize,
    /// Paths in use: lanes `0..active` of every `[_; L]`.
    active: usize,
    metrics: [f64; L],
    /// LLRs, one `[f32; L]` per vertex and level: level `d` starts at `(1 << d) - 1`
    /// and holds `2^d` entries; level `dims` is the received word. Polar convention:
    /// `> 0` favors 0. `f32` fits twice as many values in each SIMD register as `f64`.
    llrs: Vec<[f32; L]>,
    /// Decided bits, one `[bool; L]` per vertex.
    sums: Vec<[bool; L]>,
    /// Paths sorted by metric, cheapest first (filled when decoding ends).
    order: Vec<usize>,
    pruned_min: f64,
    /// Reused candidate list for repetition forks: (metric, parent lane, bit).
    candidates: Vec<(f64, usize, bool)>,
    /// Reused workspace for RM(1, d) Fast Walsh–Hadamard Transform.
    fwht: Vec<[f32; L]>,
    /// Reused compact candidate list for RM(1, d) forks: (metric, parent lane, (a << 1) | b).
    fwht_cands: Vec<(f32, u16, u32)>,
}

impl<const L: usize> ListDecoder<L> {
    fn new(dims: u32) -> Self {
        let n = 1usize << dims;
        Self {
            dims,
            list_size: L,
            active: 1,
            metrics: [0.0; L],
            llrs: vec![[0.0; L]; 2 * n - 1],
            sums: vec![[false; L]; n],
            order: Vec::with_capacity(L),
            pruned_min: f64::INFINITY,
            candidates: Vec::with_capacity(2 * L),
            fwht: vec![[0.0; L]; n],
            fwht_cands: Vec::with_capacity(L * 16),
        }
    }

    #[inline]
    fn level(dim: u32) -> usize {
        (1usize << dim) - 1
    }

    /// Decode the subtree `node`, which covers vertices `offset..offset + 2^dim` and whose
    /// LLRs sit at level `dim`. On return, `sums[offset..offset + 2^dim]` holds each
    /// path's codeword bits there.
    fn step(&mut self, dim: u32, offset: usize, node: usize, subtrees: &[Subtree]) {
        let len = 1usize << dim;
        let level = Self::level(dim);
        match subtrees[node] {
            Subtree::Frozen => {
                // The only codeword is all zeros; each path pays for every LLR that favors 1.
                let cost = lane_costs(&self.llrs[level..level + len], |l| (-l).max(0.0));
                for (m, c) in self.metrics.iter_mut().zip(cost) {
                    *m += f64::from(c);
                }
                self.sums[offset..offset + len].fill([false; L]);
                return;
            }
            Subtree::Repetition => {
                self.fork_repetition(dim, offset);
                return;
            }
            Subtree::FirstOrder => {
                self.fork_first_order(dim, offset);
                return;
            }
            Subtree::Free if self.list_size == 1 => {
                // Greedy: every word is a codeword, so take the hard decisions. The cheapest
                // word dropped flips the least reliable one.
                let mut weakest = f32::INFINITY;
                let llrs = &self.llrs[level..level + len];
                for (bits, l) in self.sums[offset..offset + len].iter_mut().zip(llrs) {
                    bits[0] = l[0] < 0.0;
                    weakest = weakest.min(l[0].abs());
                }
                self.pruned_min = self.pruned_min.min(self.metrics[0] + f64::from(weakest));
                return;
            }
            _ => {}
        }

        let half = len / 2;
        let child = Self::level(dim - 1);
        // Level `dim - 1` ends exactly where level `dim` starts, so the two never overlap.
        let (lower, upper) = self.llrs.split_at_mut(level);
        let children = &mut lower[child..];
        let (l1, l2) = upper[..len].split_at(half);

        // 1. Left half: min-sum box-plus f(l1, l2) = sign(l1) sign(l2) min(|l1|, |l2|).
        for ((out, a), b) in children.iter_mut().zip(l1).zip(l2) {
            for k in 0..L {
                out[k] = min_sum(a[k], b[k]);
            }
        }

        self.step(dim - 1, offset, 2 * node, subtrees);

        // 2. Right half: g(l1, l2, u) = l2 - l1 if the left half's bit u is 1, else l2 + l1.
        let (lower, upper) = self.llrs.split_at_mut(level);
        let children = &mut lower[child..];
        let (l1, l2) = upper[..len].split_at(half);
        let u_left = &self.sums[offset..offset + half];
        for (((out, a), b), u) in children.iter_mut().zip(l1).zip(l2).zip(u_left) {
            for k in 0..L {
                out[k] = if u[k] { b[k] - a[k] } else { b[k] + a[k] };
            }
        }

        self.step(dim - 1, offset + half, 2 * node + 1, subtrees);

        // 3. Combine: the left half's codeword bits are u_left ^ u_right.
        let (u_left, u_right) = self.sums[offset..offset + len].split_at_mut(half);
        for (a, b) in u_left.iter_mut().zip(u_right.iter()) {
            for k in 0..L {
                a[k] ^= b[k];
            }
        }
    }

    /// Finish a repetition subtree at level `dim` covering `offset..offset + 2^dim`:
    /// every path forks into all zeros and all ones, and the cheapest `list_size`
    /// children survive.
    fn fork_repetition(&mut self, dim: u32, offset: usize) {
        let len = 1usize << dim;
        let level = Self::level(dim);
        let llrs = &self.llrs[level..level + len];
        let zeros = lane_costs(llrs, |l| (-l).max(0.0));
        let ones = lane_costs(llrs, |l| l.max(0.0));

        self.candidates.clear();
        for p in 0..self.active {
            let m = self.metrics[p];
            self.candidates.push((m + f64::from(zeros[p]), p, false));
            self.candidates.push((m + f64::from(ones[p]), p, true));
        }
        if self.candidates.len() > self.list_size {
            self.candidates
                .select_nth_unstable_by(self.list_size - 1, |a, b| a.0.total_cmp(&b.0));
            let dropped = &self.candidates[self.list_size..];
            let lowest = dropped.iter().map(|c| c.0).fold(f64::INFINITY, f64::min);
            self.pruned_min = self.pruned_min.min(lowest);
            self.candidates.truncate(self.list_size);
        }

        // Each surviving child takes a lane. A parent's first child keeps the parent's lane;
        // a second child moves into a lane whose path died and copies its parent's history.
        // Those lanes are never a parent of a surviving child, so copying in place is safe.
        let mut kept = [false; L];
        let mut bits = [false; L];
        for &(metric, parent, bit) in &self.candidates {
            if !kept[parent] {
                kept[parent] = true;
                self.metrics[parent] = metric;
                bits[parent] = bit;
            }
        }
        let mut free = (0..L).filter(|&lane| !kept[lane]);
        let mut first = [true; L];
        let mut moves = [(0usize, 0usize); L];
        let mut n_moves = 0;
        for &(metric, parent, bit) in &self.candidates {
            if first[parent] {
                first[parent] = false;
                continue;
            }
            let lane = free.next().expect("at most L children survive");
            self.metrics[lane] = metric;
            bits[lane] = bit;
            moves[n_moves] = (lane, parent);
            n_moves += 1;
        }
        let moves = &moves[..n_moves];

        if !moves.is_empty() {
            // The history to copy: the bits decided so far, and the LLR levels still to be
            // read. An ancestor at level `d` reads its LLRs again only if we are in its left
            // half (bit `d - 1` of `offset` is 0). The received word (level `dims`) is the
            // same in every lane, so it never needs copying.
            for entry in &mut self.sums[..offset] {
                for &(lane, parent) in moves {
                    entry[lane] = entry[parent];
                }
            }
            for d in dim + 1..self.dims {
                if offset & (1 << (d - 1)) == 0 {
                    let level = Self::level(d);
                    for entry in &mut self.llrs[level..level + (1 << d)] {
                        for &(lane, parent) in moves {
                            entry[lane] = entry[parent];
                        }
                    }
                }
            }
        }
        self.sums[offset..offset + len].fill(bits);
        self.active = self.candidates.len();
    }

    /// Finish a first-order Reed–Muller subtree RM(1, d) at level `dim` covering
    /// `offset..offset + 2^dim` via Fast Walsh–Hadamard Transform (Green 1966; Dumer 2004).
    ///
    /// For each codeword `c_{a,b}(x) = popcount(a & x) ^ b`, the exact disagreement cost is
    /// `0.5 * (sum(|L|) - (-1)^b * H[a])`, where `H` is the FWHT of the subtree's LLRs.
    fn fork_first_order(&mut self, dim: u32, offset: usize) {
        let len = 1usize << dim;
        let level = Self::level(dim);

        // 1. Copy LLRs into `self.fwht[..len]` and accumulate `abs_sum = sum(|L|)` per lane.
        let mut abs_sum = [0.0f32; L];
        for (dst, src) in self.fwht[..len].iter_mut().zip(&self.llrs[level..level + len]) {
            *dst = *src;
            for k in 0..L {
                abs_sum[k] += src[k].abs();
            }
        }

        // 2. In-place Fast Walsh–Hadamard Transform across all L lanes simultaneously.
        let mut half = 1;
        while half < len {
            for block in self.fwht[..len].chunks_exact_mut(2 * half) {
                let (lo, hi) = block.split_at_mut(half);
                for (a, b) in lo.iter_mut().zip(hi.iter_mut()) {
                    for k in 0..L {
                        let x = a[k];
                        let y = b[k];
                        a[k] = x + y;
                        b[k] = x - y;
                    }
                }
            }
            half *= 2;
        }

        // Fast path for greedy L = 1: pick the single largest |H[a]| and track the runner-up.
        if self.list_size == 1 {
            let mut best_a = 0usize;
            let mut best_abs = -1.0f32;
            let mut best_h = 0.0f32;
            let mut second_abs = -1.0f32;
            for a in 0..len {
                let h = self.fwht[a][0];
                let ha = h.abs();
                if ha > best_abs {
                    second_abs = best_abs;
                    best_abs = ha;
                    best_h = h;
                    best_a = a;
                } else if ha > second_abs {
                    second_abs = ha;
                }
            }
            let s = f64::from(abs_sum[0]);
            let m = self.metrics[0];
            let best_cost = (0.5 * (s - f64::from(best_abs))).max(0.0);
            let drop_cost = (0.5 * (s - f64::from(second_abs)))
                .min(0.5 * (s + f64::from(best_abs)))
                .max(0.0);
            self.metrics[0] = m + best_cost;
            self.pruned_min = self.pruned_min.min(m + drop_cost);
            let b = best_h < 0.0;
            for x in 0..len {
                self.sums[offset + x][0] = ((best_a & x).count_ones() & 1 != 0) ^ b;
            }
            return;
        }

        // 3. Collect compact candidates `(metric, parent, (a << 1) | b)` across active paths.
        //    For any `a`, the better sign has cost `(s - |H[a]|) / 2 <= s / 2` while the
        //    opposite sign has cost `(s + |H[a]|) / 2 >= s / 2`. When `len >= list_size`,
        //    no opposite-sign codeword can beat the `len` better-sign codewords of the same
        //    parent, so we only push better-sign candidates and track the cheapest opposite-sign
        //    one for `pruned_min`.
        self.fwht_cands.clear();
        for p in 0..self.active {
            let m = self.metrics[p] as f32;
            let s = abs_sum[p];
            let start_idx = self.fwht_cands.len();
            if len >= self.list_size {
                let mut min_abs_h = f32::INFINITY;
                for a in 0..len {
                    let h = self.fwht[a][p];
                    let ha = h.abs();
                    if ha < min_abs_h {
                        min_abs_h = ha;
                    }
                    let c = (0.5 * (s - ha)).max(0.0);
                    let code = ((a as u32) << 1) | u32::from(h < 0.0);
                    self.fwht_cands.push((m + c, p as u16, code));
                }
                let opp_min = m + 0.5 * (s + min_abs_h);
                self.pruned_min = self.pruned_min.min(f64::from(opp_min));
            } else {
                for a in 0..len {
                    let h = self.fwht[a][p];
                    let c0 = (0.5 * (s - h)).max(0.0);
                    let c1 = (0.5 * (s + h)).max(0.0);
                    let a2 = (a as u32) << 1;
                    self.fwht_cands.push((m + c0, p as u16, a2));
                    self.fwht_cands.push((m + c1, p as u16, a2 | 1));
                }
            }
            let count = self.fwht_cands.len() - start_idx;
            if count > 2 * self.list_size {
                let slice = &mut self.fwht_cands[start_idx..];
                slice.select_nth_unstable_by(self.list_size - 1, |a, b| a.0.total_cmp(&b.0));
                let lowest = slice[self.list_size..]
                    .iter()
                    .map(|c| c.0)
                    .fold(f32::INFINITY, f32::min);
                self.pruned_min = self.pruned_min.min(f64::from(lowest));
                self.fwht_cands.truncate(start_idx + self.list_size);
            }
        }

        if self.fwht_cands.len() > self.list_size {
            self.fwht_cands
                .select_nth_unstable_by(self.list_size - 1, |a, b| a.0.total_cmp(&b.0));
            let dropped = &self.fwht_cands[self.list_size..];
            let lowest = dropped.iter().map(|c| c.0).fold(f32::INFINITY, f32::min);
            self.pruned_min = self.pruned_min.min(f64::from(lowest));
            self.fwht_cands.truncate(self.list_size);
        }

        // 4. Assign lanes: first surviving child of each parent keeps the parent's lane;
        //    all additional surviving children move into dead lanes and copy history in place.
        let mut kept = [false; L];
        let mut lane_code = [0u32; L];
        for &(metric, parent_u16, code) in &self.fwht_cands {
            let parent = parent_u16 as usize;
            if !kept[parent] {
                kept[parent] = true;
                self.metrics[parent] = f64::from(metric);
                lane_code[parent] = code;
            }
        }
        let mut free = (0..L).filter(|&lane| !kept[lane]);
        let mut first = [true; L];
        let mut moves = [(0usize, 0usize); L];
        let mut n_moves = 0;
        for &(metric, parent_u16, code) in &self.fwht_cands {
            let parent = parent_u16 as usize;
            if first[parent] {
                first[parent] = false;
                continue;
            }
            let lane = free.next().expect("at most L children survive");
            self.metrics[lane] = f64::from(metric);
            lane_code[lane] = code;
            moves[n_moves] = (lane, parent);
            n_moves += 1;
        }
        let moves = &moves[..n_moves];

        if !moves.is_empty() {
            for entry in &mut self.sums[..offset] {
                for &(lane, parent) in moves {
                    entry[lane] = entry[parent];
                }
            }
            for d in dim + 1..self.dims {
                if offset & (1 << (d - 1)) == 0 {
                    let level = Self::level(d);
                    for entry in &mut self.llrs[level..level + (1 << d)] {
                        for &(lane, parent) in moves {
                            entry[lane] = entry[parent];
                        }
                    }
                }
            }
        }

        let new_active = self.fwht_cands.len();
        for x in 0..len {
            let mut bits = [false; L];
            for k in 0..new_active {
                let code = lane_code[k];
                let a = (code >> 1) as usize;
                let b = (code & 1) != 0;
                bits[k] = ((a & x).count_ones() & 1 != 0) ^ b;
            }
            self.sums[offset + x] = bits;
        }
        self.active = new_active;
    }
}

impl<const L: usize> ListDecode for ListDecoder<L> {
    fn decode(&mut self, y: &[f64], list_size: usize, subtrees: &[Subtree]) {
        assert!((1..=L).contains(&list_size), "list size {list_size} needs a wider decoder");
        self.list_size = list_size;
        self.active = 1;
        self.metrics = [0.0; L];
        self.pruned_min = f64::INFINITY;
        // Convert convention (`> 0` favors 1) to polar LLRs (`> 0` favors 0): L = -y.
        let root = Self::level(self.dims);
        for (entry, &r) in self.llrs[root..].iter_mut().zip(y) {
            *entry = [-r as f32; L];
        }
        self.step(self.dims, 0, 1, subtrees);

        self.order.clear();
        self.order.extend(0..self.active);
        let metrics = &self.metrics;
        self.order.sort_unstable_by(|&a, &b| metrics[a].total_cmp(&metrics[b]));
    }

    fn paths(&self) -> usize {
        self.active
    }

    fn metric(&self, rank: usize) -> f64 {
        self.metrics[self.order[rank]]
    }

    fn word(&self, rank: usize, out: &mut Vec<bool>) {
        let lane = self.order[rank];
        out.clear();
        out.extend(self.sums.iter().map(|bits| bits[lane]));
    }

    fn pruned_min(&self) -> f64 {
        self.pruned_min
    }
}

#[inline(always)]
fn min_sum(l1: f32, l2: f32) -> f32 {
    let sign = (l1.to_bits() ^ l2.to_bits()) & 0x8000_0000;
    f32::from_bits(l1.abs().min(l2.abs()).to_bits() | sign)
}

/// Per path, the sum of `cost(l)` over the given vertices.
#[inline(always)]
fn lane_costs<const L: usize>(llrs: &[[f32; L]], cost: impl Fn(f32) -> f32) -> [f32; L] {
    let mut total = [0.0f32; L];
    for l in llrs {
        for k in 0..L {
            total[k] += cost(l[k]);
        }
    }
    total
}
