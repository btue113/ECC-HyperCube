//! Compare `ecc-hypercube` Reed-Muller soft SCL decoding against 5G NR Polar codes
//! across three benchmark modes on AWGN with BPSK:
//!
//! 1. `5g`: Pure `ecc_hypercube::ReedMuller` (no CRC) vs. full 3GPP TS 38.212 5G NR
//!    uplink control polar code (CRC6 + 3 PC bits for 12 <= A <= 19, CRC11 for A >= 20)
//!    at equal user payload A = K_RM.
//! 2. `noCRC`: Bare Reed-Muller vs. Bare Polar (5G reliability order, no CRC, no PC)
//!    at equal (n, K).
//! 3. `sameCRC`: CA-SCL Reed-Muller + CRC vs. CA-SCL Polar + CRC (same K, same payload
//!    A = K - crc_bits, same CRC6 at n=32 / CRC11 at n>=64).
//!
//! Further modes: `ae` / `ae512` (automorphism ensemble), `cert` (ML certificate),
//! `rmcrc` (RM + CRC on the parity shell) and `pac` (PAC with the RM rate profile).
//!
//! The CRC-on-parity-shell and PAC constructions (`PolarCode5G::rm_crc`, `PolarCode5G::rm_pac`)
//! live only in this benchmark, as comparison baselines. They rely on techniques covered by
//! active patents (polarization-weight ranking, convolutional precoding, CRC-aided list
//! decoding), so they are deliberately kept out of the `ecc-hypercube` library.
//!
//! Run a quick verification across all modes:
//!   cargo bench --bench polar_5g
//!
//! Run a specific mode (`5g`, `nocrc`, `samecrc`, `ae`, `cert`, `rmcrc`, `pac`, or `all`)
//! with custom duration per row:
//!   BENCH_MODE=all SECS_PER_ROW=6.0 cargo bench --bench polar_5g

use ecc_hypercube::ReedMuller;
use std::time::Instant;

/// 3GPP TS 38.212 Table 5.3.1.2-1 polar sequence Q_0^{N_max - 1} (N_max = 1024),
/// ordered from least reliable to most reliable.
const Q_NMAX: [usize; 1024] = [
    0, 1, 2, 4, 8, 16, 32, 3, 5, 64, 9, 6, 17, 10, 18, 128, 12, 33, 65, 20, 256, 34, 24, 36, 7,
    129, 66, 512, 11, 40, 68, 130, 19, 13, 48, 14, 72, 257, 21, 132, 35, 258, 26, 513, 80, 37, 25,
    22, 136, 260, 264, 38, 514, 96, 67, 41, 144, 28, 69, 42, 516, 49, 74, 272, 160, 520, 288, 528,
    192, 544, 70, 44, 131, 81, 50, 73, 15, 320, 133, 52, 23, 134, 384, 76, 137, 82, 56, 27, 97,
    39, 259, 84, 138, 145, 261, 29, 43, 98, 515, 88, 140, 30, 146, 71, 262, 265, 161, 576, 45,
    100, 640, 51, 148, 46, 75, 266, 273, 517, 104, 162, 53, 193, 152, 77, 164, 768, 268, 274, 518,
    54, 83, 57, 521, 112, 135, 78, 289, 194, 85, 276, 522, 58, 168, 139, 99, 86, 60, 280, 89, 290,
    529, 524, 196, 141, 101, 147, 176, 142, 530, 321, 31, 200, 90, 545, 292, 322, 532, 263, 149,
    102, 105, 304, 296, 163, 92, 47, 267, 385, 546, 324, 208, 386, 150, 153, 165, 106, 55, 328,
    536, 577, 548, 113, 154, 79, 269, 108, 578, 224, 166, 519, 552, 195, 270, 641, 523, 275, 580,
    291, 59, 169, 560, 114, 277, 156, 87, 197, 116, 170, 61, 531, 525, 642, 281, 278, 526, 177,
    293, 388, 91, 584, 769, 198, 172, 120, 201, 336, 62, 282, 143, 103, 178, 294, 93, 644, 202,
    592, 323, 392, 297,770, 107, 180, 151, 209, 284, 648, 94, 204, 298, 400, 608, 352, 325, 533,
    155, 210, 305, 547, 300, 109, 184, 534, 537, 115, 167, 225, 326, 306, 772, 157, 656, 329, 110,
    117, 212, 171, 776, 330, 226, 549, 538, 387, 308, 216, 416, 271, 279, 158, 337, 550, 672, 118,
    332, 579, 540, 389, 173, 121, 553, 199, 784, 179, 228, 338, 312, 704, 390, 174, 554, 581, 393,
    283, 122, 448, 353, 561, 203, 63, 340, 394, 527, 582, 556, 181, 295, 285, 232, 124, 205, 182,
    643, 562, 286, 585, 299, 354, 211, 401, 185, 396, 344, 586, 645, 593, 535, 240, 206, 95, 327,
    564, 800, 402, 356, 307, 301, 417, 213, 568, 832, 588, 186, 646, 404, 227, 896, 594, 418, 302,
    649, 771, 360, 539, 111, 331, 214, 309, 188, 449, 217, 408, 609, 596, 551, 650, 229, 159, 420,
    310, 541, 773, 610, 657, 333, 119, 600, 339, 218, 368, 652, 230, 391, 313, 450, 542, 334, 233,
    555, 774, 175, 123, 658, 612, 341, 777, 220, 314, 424, 395, 673, 583, 355, 287, 183, 234, 125,
    557, 660, 616, 342, 316, 241, 778, 563, 345, 452, 397, 403, 207, 674, 558, 785, 432, 357, 187,
    236, 664, 624, 587, 780, 705, 126, 242, 565, 398, 346, 456, 358, 405, 303, 569, 244, 595, 189,
    566, 676, 361, 706, 589, 215, 786, 647, 348, 419, 406, 464, 680, 801, 362, 590, 409, 570, 788,
    597, 572, 219, 311, 708, 598, 601, 651, 421, 792, 802, 611, 602, 410, 231, 688, 653, 248, 369,
    190, 364, 654, 659, 335, 480, 315, 221, 370, 613, 422, 425, 451, 614, 543, 235, 412, 343, 372,
    775, 317, 222, 426, 453, 237, 559, 833, 804, 712, 834, 661, 808, 779, 617, 604, 433, 720, 816,
    836, 347, 897, 243, 662, 454, 318, 675, 618, 898, 781, 376, 428, 665, 736, 567, 840, 625, 238,
    359, 457, 399, 787, 591, 678, 434, 677, 349, 245, 458, 666, 620, 363, 127, 191, 782, 407, 436,
    626, 571, 465, 681, 246, 707, 350, 599, 668, 790, 460, 249, 682, 573, 411, 803, 789, 709, 365,
    440, 628, 689, 374, 423, 466, 793, 250, 371, 481, 574, 413, 603, 366, 468, 655, 900, 805, 615,
    684, 710, 429, 794, 252, 373, 605, 848, 690, 713, 632, 482, 806, 427, 904, 414, 223, 663, 692,
    835, 619, 472, 455, 796, 809, 714, 721, 837, 716, 864, 810, 606, 912, 722, 696, 377, 435, 817,
    319, 621, 812, 484, 430, 838, 667, 488, 239, 378, 459, 622, 627, 437, 380, 818, 461, 496, 669,
    679, 724, 841, 629, 351, 467, 438, 737, 251, 462, 442, 441, 469, 247, 683, 842, 738, 899, 670,
    783, 849, 820, 728, 928, 791, 367, 901, 630, 685, 844, 633, 711, 253, 691, 824, 902, 686, 740,
    850, 375, 444, 470, 483, 415, 485, 905, 795, 473, 634, 744, 852, 960, 865, 693, 797, 906, 715,
    807, 474, 636, 694, 254, 717, 575, 913, 798, 811, 379, 697, 431, 607, 489, 866, 723, 486, 908,
    718, 813, 476, 856, 839, 725, 698, 914, 752, 868, 819, 814, 439, 929, 490, 623, 671, 739, 916,
    463, 843, 381, 497, 930, 821, 726, 961, 872, 492, 631, 729, 700, 443, 741, 845, 920, 382, 822,
    851, 730, 498, 880, 742, 445, 471, 635, 932, 687, 903, 825, 500, 846, 745, 826, 732, 446, 962,
    936, 475, 853, 867, 637, 907, 487, 695, 746, 828, 753, 854, 857, 504, 799, 255, 964, 909, 719,
    477, 915, 638, 748, 944, 869, 491, 699, 754, 858, 478, 968, 383, 910, 815, 976, 870, 917, 727,
    493, 873, 701, 931, 756, 860, 499, 731, 823, 922, 874, 918, 502, 933, 743, 760, 881, 494, 702,
    921, 501, 876, 847, 992, 447, 733, 827, 934, 882, 937, 963, 747, 505, 855, 924, 734, 829, 965,
    938, 884, 506, 749, 945, 966, 755, 859, 940, 830, 911, 871, 639, 888, 479, 946, 750, 969, 508,
    861, 757, 970, 919, 875, 862, 758, 948, 977, 923, 972, 761, 877, 952, 495, 703, 935, 978, 883,
    762, 503, 925, 878, 735, 993, 885, 939, 994, 980, 926, 764, 941, 967, 886, 831, 947, 507, 889,
    984, 751, 942, 996, 971, 890, 509, 949, 973, 1000, 892, 950, 863, 759, 1008, 510, 979, 953,
    763, 974, 954, 879, 981, 982, 927, 995, 765, 956, 887, 985, 997, 986, 943, 891, 998, 766, 511,
    988, 1001, 951, 1002, 893, 975, 894, 1009, 955, 1004, 1010, 957, 983, 958, 987, 1012, 999,
    1016, 767, 989, 1003, 990, 1005, 959, 1011, 1013, 895, 1006, 1014, 1017, 1018, 991, 1020, 1007,
    1015, 1019, 1021, 1022, 1023,
];

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn gaussian(&mut self) -> f64 {
        let a = ((self.next() >> 11) as f64 + 0.5) / ((1u64 << 53) as f64);
        let b = ((self.next() >> 11) as f64 + 0.5) / ((1u64 << 53) as f64);
        (-2.0 * a.ln()).sqrt() * (2.0 * std::f64::consts::PI * b).cos()
    }
}

fn butterfly(v: &mut [bool]) {
    let mut h = 1;
    while h < v.len() {
        for b in v.chunks_exact_mut(2 * h) {
            let (l, u) = b.split_at_mut(h);
            for (a, c) in l.iter_mut().zip(u) {
                *a ^= *c;
            }
        }
        h *= 2;
    }
}

fn crc(bits: &[bool], poly: u32, width: u32) -> Vec<bool> {
    if width == 0 {
        return Vec::new();
    }
    let mut r = 0u32;
    let mask = (1u32 << width) - 1;
    for &b in bits {
        let top = ((r >> (width - 1)) & 1 == 1) ^ b;
        r = (r << 1) & mask;
        if top {
            r ^= poly;
        }
    }
    (0..width).rev().map(|i| (r >> i) & 1 == 1).collect()
}

#[derive(Clone)]
struct PolarCode5G {
    n: usize,
    payload_len: usize,
    /// 0 = information/CRC bit, 1 = frozen bit, 2 = parity-check (PC) bit.
    kind: Vec<u8>,
    crc_width: u32,
    crc_poly: u32,
    has_pc: bool,
    info_positions: Vec<usize>,
    /// PAC convolutional generator (bit `j` = tap `g_j`, `g_0 = 1`); `0` disables precoding.
    pac_poly: u64,
}

/// Arıkan's 7-tap PAC generator `g = [1, 0, 1, 1, 0, 1, 1]` (octal `0o133`), `g_j` at bit `j`.
const DEFAULT_PAC_POLY: u64 = 0b110_1101;

/// Polarization weight (β-expansion, β = 2^(1/4)): `W(v) = Σ_{bit j of v set} β^j`.
fn beta_weight(mut v: u32) -> f64 {
    const BETA: f64 = 1.189207115002721;
    let (mut w, mut p) = (0.0, 1.0);
    while v != 0 {
        if v & 1 != 0 {
            w += p;
        }
        v >>= 1;
        p *= BETA;
    }
    w
}

#[inline]
fn parity(x: u64) -> bool {
    x.count_ones() & 1 == 1
}

impl PolarCode5G {
    /// TS 38.212 uplink control: CRC6 + 3 PC bits for 12 <= A <= 19, CRC11 for A >= 20.
    fn new(n: usize, payload_len: usize) -> Self {
        let q: Vec<usize> = Q_NMAX.iter().copied().filter(|&x| x < n).collect();
        let (crc_width, crc_poly, num_pc) = if payload_len <= 19 {
            (6u32, 0b100001u32, 3usize)
        } else {
            (11u32, 0b11000100001u32, 0usize)
        };
        let k = payload_len + crc_width as usize;
        let mut kind = vec![1u8; n];
        let active = &q[n - k - num_pc..];
        for &i in active {
            kind[i] = 0;
        }
        for &i in &active[..num_pc] {
            kind[i] = 2;
        }
        let info_positions = (0..n).filter(|&i| kind[i] == 0).collect();
        Self {
            n,
            payload_len,
            kind,
            crc_width,
            crc_poly,
            has_pc: num_pc > 0,
            info_positions,
            pac_poly: 0,
        }
    }

    /// Construct a K-dimensional subcode of F^{\otimes m} with `payload_len = k - crc_width`
    /// payload bits and `crc_width` CRC bits (no PC bits):
    /// - If `rm_min_weight = Some(m - r)`: selects the K Reed-Muller rows (popcount >= m - r).
    /// - If `rm_min_weight = None`: selects the top K rows from the 5G reliability sequence Q.
    fn new_subcode(
        n: usize,
        k: usize,
        payload_len: usize,
        crc_width: u32,
        crc_poly: u32,
        rm_min_weight: Option<u32>,
    ) -> Self {
        let mut kind = vec![1u8; n];
        if let Some(min_w) = rm_min_weight {
            for i in 0..n {
                if (i as u32).count_ones() >= min_w {
                    kind[i] = 0;
                }
            }
        } else {
            let q: Vec<usize> = Q_NMAX.iter().copied().filter(|&x| x < n).collect();
            for &i in &q[n - k..] {
                kind[i] = 0;
            }
        }
        let info_positions: Vec<usize> = (0..n).filter(|&i| kind[i] == 0).collect();
        assert_eq!(info_positions.len(), k);
        Self {
            n,
            payload_len,
            kind,
            crc_width,
            crc_poly,
            has_pc: false,
            info_positions,
            pac_poly: 0,
        }
    }

    /// RM(r, m) keeping its full payload k, plus a `crc_width`-bit CRC on the most reliable
    /// weight-(m - r - 1) rows, ranked by polarization weight. The CRC occupies the last
    /// `crc_width` information rows in index order.
    fn rm_crc(m: u32, r: u32, crc_width: u32, crc_poly: u32) -> Self {
        let n = 1usize << m;
        let mut kind: Vec<u8> =
            (0..n).map(|i| u8::from((i as u32).count_ones() < m - r)).collect();
        let payload_len = kind.iter().filter(|&&k| k == 0).count();
        let mut shell: Vec<usize> =
            (0..n).filter(|&i| (i as u32).count_ones() == m - r - 1).collect();
        shell.sort_by(|&a, &b| {
            beta_weight(b as u32).total_cmp(&beta_weight(a as u32)).then_with(|| b.cmp(&a))
        });
        for &i in &shell[..crc_width as usize] {
            kind[i] = 0;
        }
        let info_positions = (0..n).filter(|&i| kind[i] == 0).collect();
        Self { n, payload_len, kind, crc_width, crc_poly, has_pc: false, info_positions, pac_poly: 0 }
    }

    /// PAC code with the RM(r, m) rate profile: data `d` passes through the convolutional
    /// precoder `pac_poly` over all rows in index order, so frozen rows carry `u = conv(d)`.
    fn rm_pac(m: u32, r: u32, pac_poly: u64) -> Self {
        let n = 1usize << m;
        let kind: Vec<u8> = (0..n).map(|i| u8::from((i as u32).count_ones() < m - r)).collect();
        let info_positions: Vec<usize> = (0..n).filter(|&i| kind[i] == 0).collect();
        let payload_len = info_positions.len();
        Self { n, payload_len, kind, crc_width: 0, crc_poly: 0, has_pc: false, info_positions, pac_poly }
    }

    /// The PAC data bits behind leaf decisions `u` (`None` if a frozen row breaks the recurrence).
    fn pac_data(&self, u: &[bool], strict: bool) -> Option<Vec<bool>> {
        let taps = self.pac_poly & !1;
        let mut state = 0u64;
        let mut d = Vec::with_capacity(self.payload_len);
        for i in 0..self.n {
            state <<= 1;
            let mem = parity(state & taps);
            if self.kind[i] == 0 {
                let bit = u[i] ^ mem;
                state |= u64::from(bit);
                d.push(bit);
            } else if strict && u[i] != mem {
                return None;
            }
        }
        Some(d)
    }

    fn encode(&self, msg: &[bool]) -> Vec<bool> {
        if self.pac_poly != 0 {
            let taps = self.pac_poly & !1;
            let (mut state, mut j) = (0u64, 0);
            let mut u = vec![false; self.n];
            for i in 0..self.n {
                state <<= 1;
                let mem = parity(state & taps);
                if self.kind[i] == 0 {
                    state |= u64::from(msg[j]);
                    u[i] = msg[j] ^ mem;
                    j += 1;
                } else {
                    u[i] = mem;
                }
            }
            butterfly(&mut u);
            return u;
        }
        let mut b = msg.to_vec();
        b.extend(crc(msg, self.crc_poly, self.crc_width));
        let mut u = vec![false; self.n];
        let mut y = [false; 5];
        let mut j = 0;
        for i in 0..self.n {
            y.rotate_left(1);
            match self.kind[i] {
                0 => {
                    u[i] = b[j];
                    j += 1;
                    y[0] ^= u[i];
                }
                2 => {
                    u[i] = y[0];
                    y[0] ^= u[i];
                }
                _ => {}
            }
        }
        butterfly(&mut u);
        u
    }

    /// Returns `Some(payload)` if all frozen, PC, and CRC constraints hold.
    fn extract_checked(&self, u: &[bool]) -> Option<Vec<bool>> {
        if self.pac_poly != 0 {
            return self.pac_data(u, true);
        }
        let b: Vec<bool> = self.info_positions.iter().map(|&i| u[i]).collect();
        let msg = &b[..self.payload_len];
        if crc(msg, self.crc_poly, self.crc_width)[..] != b[self.payload_len..] {
            return None;
        }
        let mut y = [false; 5];
        for i in 0..self.n {
            y.rotate_left(1);
            match self.kind[i] {
                0 => y[0] ^= u[i],
                2 => {
                    if u[i] != y[0] {
                        return None;
                    }
                    y[0] ^= u[i];
                }
                _ => {
                    if u[i] {
                        return None;
                    }
                }
            }
        }
        Some(msg.to_vec())
    }
}

#[inline(always)]
fn min_sum(a: f64, b: f64) -> f64 {
    let m = a.abs().min(b.abs());
    if (a.to_bits() ^ b.to_bits()) >> 63 != 0 {
        -m
    } else {
        m
    }
}

struct PolarSclDecoder {
    dims: u32,
    code: PolarCode5G,
    list_size: usize,
    active: usize,
    metrics: Vec<f64>,
    llrs: Vec<f64>,
    sums: Vec<bool>,
    llr_stride: usize,
    sum_stride: usize,
    candidates: Vec<(f64, usize, bool)>,
    used: Vec<bool>,
    deferred: Vec<usize>,
    pc_reg: Vec<[bool; 5]>,
    order: Vec<usize>,
    tmp: Vec<bool>,
    /// Leaf decisions `u` per path (needed to undo the PAC precoder).
    uleaf: Vec<bool>,
    pac_state: Vec<u64>,
    /// Lowest metric among paths dropped from the list (no codeword outside it costs less).
    pruned_min: f64,
}

impl PolarSclDecoder {
    fn new(code: PolarCode5G, list_size: usize) -> Self {
        let n = code.n;
        Self {
            dims: n.trailing_zeros(),
            code,
            list_size,
            active: 1,
            metrics: vec![0.0; list_size],
            llrs: vec![0.0; list_size * 2 * n],
            sums: vec![false; list_size * n],
            llr_stride: 2 * n,
            sum_stride: n,
            candidates: Vec::with_capacity(2 * list_size),
            used: vec![false; list_size],
            deferred: Vec::with_capacity(list_size),
            pc_reg: vec![[false; 5]; list_size],
            order: Vec::with_capacity(list_size),
            tmp: vec![false; n],
            uleaf: vec![false; list_size * n],
            pac_state: vec![0; list_size],
            pruned_min: f64::INFINITY,
        }
    }

    fn step(&mut self, d: u32, off: usize) {
        if d == 0 {
            let v = off;
            let has_pc = self.code.has_pc;
            let taps = self.code.pac_poly & !1;
            let kind = self.code.kind[v];
            if has_pc {
                for p in 0..self.active {
                    self.pc_reg[p].rotate_left(1);
                }
            }
            if taps != 0 {
                for p in 0..self.active {
                    self.pac_state[p] <<= 1;
                }
            }
            if kind != 0 {
                for p in 0..self.active {
                    let l = self.llrs[p * self.llr_stride];
                    let want = if taps != 0 {
                        parity(self.pac_state[p] & taps)
                    } else {
                        kind == 2 && self.pc_reg[p][0]
                    };
                    let cost = if want {
                        if l > 0.0 { l } else { 0.0 }
                    } else if l < 0.0 {
                        -l
                    } else {
                        0.0
                    };
                    self.metrics[p] += cost;
                    self.sums[p * self.sum_stride + v] = want;
                    self.uleaf[p * self.sum_stride + v] = want;
                }
                return;
            }

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
                let lowest = self.candidates[self.list_size..]
                    .iter()
                    .map(|c| c.0)
                    .fold(f64::INFINITY, f64::min);
                self.pruned_min = self.pruned_min.min(lowest);
                self.candidates.truncate(self.list_size);
            }

            let new_active = self.candidates.len();
            self.used.fill(false);
            self.deferred.clear();
            for i in 0..new_active {
                let (m, p, b) = self.candidates[i];
                if !self.used[p] {
                    self.used[p] = true;
                    self.metrics[p] = m;
                    self.sums[p * self.sum_stride + v] = b;
                    self.uleaf[p * self.sum_stride + v] = b;
                    if has_pc {
                        self.pc_reg[p][0] ^= b;
                    }
                    if taps != 0 {
                        let shifted = self.pac_state[p] & !1;
                        self.pac_state[p] = shifted | u64::from(b ^ parity(shifted & taps));
                    }
                } else {
                    self.deferred.push(i);
                }
            }

            let mut free_slot = 0;
            for di in 0..self.deferred.len() {
                let (m, p, b) = self.candidates[self.deferred[di]];
                while self.used[free_slot] {
                    free_slot += 1;
                }
                self.used[free_slot] = true;
                self.metrics[free_slot] = m;
                self.llrs.copy_within(
                    p * self.llr_stride..p * self.llr_stride + self.llr_stride,
                    free_slot * self.llr_stride,
                );
                if v > 0 {
                    self.sums.copy_within(
                        p * self.sum_stride..p * self.sum_stride + v,
                        free_slot * self.sum_stride,
                    );
                    self.uleaf.copy_within(
                        p * self.sum_stride..p * self.sum_stride + v,
                        free_slot * self.sum_stride,
                    );
                }
                self.sums[free_slot * self.sum_stride + v] = b;
                self.uleaf[free_slot * self.sum_stride + v] = b;
                if taps != 0 {
                    let shifted = self.pac_state[p] & !1;
                    self.pac_state[free_slot] = shifted | u64::from(b ^ parity(shifted & taps));
                }
                if has_pc {
                    self.pc_reg[free_slot] = self.pc_reg[p];
                    self.pc_reg[free_slot][0] ^= true;
                }
            }
            self.active = new_active;
            return;
        }

        let h = 1usize << (d - 1);
        let po = (1usize << d) - 1;
        let co = (1usize << (d - 1)) - 1;
        for p in 0..self.active {
            let b = p * self.llr_stride;
            for i in 0..h {
                let a = self.llrs[b + po + i];
                let c = self.llrs[b + po + h + i];
                self.llrs[b + co + i] = min_sum(a, c);
            }
        }
        self.step(d - 1, off);
        for p in 0..self.active {
            let b = p * self.llr_stride;
            let s = p * self.sum_stride + off;
            for i in 0..h {
                let a = self.llrs[b + po + i];
                let c = self.llrs[b + po + h + i];
                self.llrs[b + co + i] = if self.sums[s + i] { c - a } else { c + a };
            }
        }
        self.step(d - 1, off | h);
        for p in 0..self.active {
            let s = p * self.sum_stride + off;
            let (l, u) = self.sums[s..s + 2 * h].split_at_mut(h);
            for (a, c) in l.iter_mut().zip(u) {
                *a ^= *c;
            }
        }
    }

    fn run(&mut self, llr: &[f64]) {
        let n = self.code.n;
        self.active = 1;
        self.metrics[0] = 0.0;
        self.pac_state[0] = 0;
        self.pruned_min = f64::INFINITY;
        if self.code.has_pc {
            self.pc_reg[0] = [false; 5];
        }
        let ro = n - 1;
        self.llrs[ro..ro + n].copy_from_slice(llr);
        self.step(self.dims, 0);
    }

    /// SCL decoding (no CRC) that also reports whether the answer is provably maximum
    /// likelihood: by the Taipale–Pursley distance test or by the pruning bound.
    /// Returns `(payload, certified)`.
    fn decode_checked(&mut self, llr: &[f64], min_distance: usize) -> (Vec<bool>, bool) {
        let n = self.code.n;
        for i in 0..n {
            self.tmp[i] = llr[i] < 0.0;
        }
        let mut t = std::mem::take(&mut self.tmp);
        butterfly(&mut t);
        let fast = self.code.extract_checked(&t);
        self.tmp = t;
        if let Some(m) = fast {
            return (m, true);
        }
        self.run(llr);
        let best = (0..self.active)
            .min_by(|&a, &b| self.metrics[a].total_cmp(&self.metrics[b]))
            .unwrap();
        let word = &self.sums[best * n..(best + 1) * n];
        let cost = disagreement_cost(word, llr);
        let certified = cost <= self.pruned_min || is_ml_certified(word, llr, min_distance);
        let msg = if self.code.pac_poly != 0 {
            self.code.pac_data(&self.uleaf[best * n..(best + 1) * n], false).unwrap()
        } else {
            let mut u = word.to_vec();
            butterfly(&mut u);
            self.code.info_positions.iter().take(self.code.payload_len).map(|&i| u[i]).collect()
        };
        (msg, certified)
    }

    /// Returns `(decoded_payload, crc_passed)`.
    fn decode(&mut self, llr: &[f64]) -> (Vec<bool>, bool) {
        let n = self.code.n;
        for i in 0..n {
            self.tmp[i] = llr[i] < 0.0;
        }
        let mut t = std::mem::take(&mut self.tmp);
        butterfly(&mut t);
        let fast = self.code.extract_checked(&t);
        self.tmp = t;
        if let Some(m) = fast {
            return (m, true);
        }

        self.run(llr);

        self.order.clear();
        self.order.extend(0..self.active);
        let met = &self.metrics;
        self.order.sort_by(|&a, &b| met[a].total_cmp(&met[b]));

        let mut first = None;
        for &p in &self.order {
            let mut u = self.sums[p * n..(p + 1) * n].to_vec();
            butterfly(&mut u);
            if first.is_none() {
                first = Some(u.clone());
            }
            if let Some(m) = self.code.extract_checked(&u) {
                return (m, true);
            }
        }
        let u = first.unwrap();
        let m: Vec<bool> = self
            .code
            .info_positions
            .iter()
            .take(self.code.payload_len)
            .map(|&i| u[i])
            .collect();
        (m, false)
    }
}

/// Polar LLR convention: `llr < 0` favors bit 1.
fn disagreement_cost(word: &[bool], llr: &[f64]) -> f64 {
    word.iter().zip(llr).filter(|&(&c, &l)| c != (l < 0.0)).map(|(_, &l)| l.abs()).sum()
}

/// Taipale–Pursley (1991): no codeword is closer if the cheapest `d - |D|` agreeing
/// positions already cost more than the word's own disagreements `D`.
fn is_ml_certified(word: &[bool], llr: &[f64], min_distance: usize) -> bool {
    let mut cost = 0.0;
    let mut agreeing = Vec::with_capacity(llr.len());
    let mut disagreements = 0;
    for (&c, &l) in word.iter().zip(llr) {
        if c != (l < 0.0) {
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

/// Sanity checks for the benchmark-only RM + CRC and PAC constructions.
fn self_check() {
    // Same CRC rows as the former library `ReedMuller::with_crc(3, 7, 8)`.
    let c = PolarCode5G::rm_crc(7, 3, 8, 0x07);
    let mut shell: Vec<usize> =
        c.info_positions.iter().copied().filter(|&i| i.count_ones() == 3).collect();
    shell.sort_by(|&a, &b| beta_weight(b as u32).total_cmp(&beta_weight(a as u32)).then(b.cmp(&a)));
    assert_eq!(shell, [112, 104, 100, 88, 98, 84, 97, 56]);

    // PAC(2, 5): every non-zero codeword keeps RM(2, 5)'s minimum weight 8, noiseless words
    // decode, and every certified decision equals brute-force maximum likelihood.
    let pac = PolarCode5G::rm_pac(5, 2, DEFAULT_PAC_POLY);
    let k = pac.payload_len;
    let codewords: Vec<(Vec<bool>, Vec<bool>)> = (0..1usize << k)
        .map(|i| {
            let msg: Vec<bool> = (0..k).map(|b| i >> b & 1 == 1).collect();
            (pac.encode(&msg), msg)
        })
        .collect();
    assert!(codewords[1..].iter().all(|(cw, _)| cw.iter().filter(|&&b| b).count() >= 8));
    let mut dec = PolarSclDecoder::new(pac.clone(), 4);
    let mut rng = Rng(0x7AC);
    let sigma = (1.0 / (2.0 * (k as f64 / 32.0) * 10f64.powf(0.2))).sqrt();
    let mut certified_count = 0;
    for _ in 0..300 {
        let (sent, msg) = &codewords[(rng.next() % codewords.len() as u64) as usize];
        let clean: Vec<f64> = sent.iter().map(|&b| if b { -1.0 } else { 1.0 }).collect();
        assert_eq!(&dec.decode_checked(&clean, 8).0, msg);
        let llr: Vec<f64> =
            sent.iter().map(|&b| (if b { -1.0 } else { 1.0 }) + sigma * rng.gaussian()).collect();
        let (_, ml_msg) = codewords
            .iter()
            .min_by(|a, b| disagreement_cost(&a.0, &llr).total_cmp(&disagreement_cost(&b.0, &llr)))
            .unwrap();
        let (got, certified) = dec.decode_checked(&llr, 8);
        assert!(!certified || &got == ml_msg, "PAC(2,5) certified a non-ML word");
        certified_count += usize::from(certified);
    }
    assert!(certified_count > 0);
}

fn mix_seed(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    (z ^ (z >> 31)) | 1
}

fn main() {
    let threads: u64 = std::thread::available_parallelism()
        .map(|n| n.get().min(8) as u64)
        .unwrap_or(1);
    let secs_per_row: f64 = std::env::var("SECS_PER_ROW")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.20);
    let bench_mode = std::env::var("BENCH_MODE")
        .unwrap_or_else(|_| "all".to_string())
        .to_lowercase();

    self_check();
    println!(
        "threads {threads} (secs_per_row = {secs_per_row:.2}s, mode = {bench_mode}; set BENCH_MODE=all|5g|nocrc|samecrc|ae|cert|rmcrc|pac SECS_PER_ROW=6.0)"
    );

    if bench_mode == "pac" {
        println!("\n=== Mode PAC: PolarCode5G::rm_pac (RM rate profile + 0o133 precoder) with ML-certified adaptive SCL ===");
        println!("code dB scheme words errs wer avg_us uncertified cert_errs");
        let configs: &[(u32, u32, &[f64])] = &[
            (3, 7, &[2.75, 3.0, 3.25, 3.5]),
            (3, 8, &[2.5, 3.0]),
        ];
        // Schemes:
        // 0 = Pure RM L=32
        // 1 = PAC L=8
        // 2 = PAC L=32
        // 3 = PAC L=128
        // 4 = PAC adaptive 8->32
        // 5 = PAC adaptive 8->32->128
        // 6 = 5G-CRC11 L=32
        let schemes: &[(&str, usize)] = &[
            ("RM-L32", 0),
            ("PAC-L8", 1),
            ("PAC-L32", 2),
            ("PAC-L128", 3),
            ("PAC-8->32", 4),
            ("PAC-8->32->128", 5),
            ("5G-CRC11-L32", 6),
        ];
        for &(r, m, dbs) in configs {
            for &db in dbs {
                for &(label, kind) in schemes {
                    let handles: Vec<_> = (0..threads)
                        .map(|t| {
                            std::thread::spawn(move || {
                                let rm = ReedMuller::new(r, m).unwrap();
                                let (n, k) = (rm.block_len(), rm.message_len());
                                let min_distance = 1usize << (m - r);
                                let polar = PolarCode5G::new(n, k);
                                let mut polar_dec = PolarSclDecoder::new(polar.clone(), 32);
                                let pac = PolarCode5G::rm_pac(m, r, DEFAULT_PAC_POLY);
                                let mut pac_decs: Vec<PolarSclDecoder> = [8, 32, 128]
                                    .iter()
                                    .map(|&l| PolarSclDecoder::new(pac.clone(), l))
                                    .collect();
                                let sigma = (1.0
                                    / (2.0 * (k as f64 / n as f64) * 10f64.powf(db / 10.0)))
                                .sqrt();
                                let scale = 2.0 / (sigma * sigma);
                                let mut rng = Rng(mix_seed(t * 1000 + (db * 100.0) as u64 * 7));
                                let (mut words, mut errs, mut decode_ns) = (0u64, 0u64, 0u128);
                                let (mut uncert, mut cert_errs) = (0u64, 0u64);
                                let start = Instant::now();
                                while start.elapsed().as_secs_f64() < secs_per_row {
                                    let msg: Vec<bool> =
                                        (0..k).map(|_| rng.next() & 1 == 1).collect();
                                    if kind == 6 {
                                        let y: Vec<f64> = polar
                                            .encode(&msg)
                                            .iter()
                                            .map(|&b| {
                                                scale
                                                    * ((if b { -1.0 } else { 1.0 })
                                                        + sigma * rng.gaussian())
                                            })
                                            .collect();
                                        let t0 = Instant::now();
                                        let (dec, crc_ok) = polar_dec.decode(&y);
                                        decode_ns += t0.elapsed().as_nanos();
                                        errs += u64::from(dec != msg || !crc_ok);
                                    } else if kind == 0 {
                                        let y: Vec<f64> = rm
                                            .encode(&msg)
                                            .unwrap()
                                            .iter()
                                            .map(|&b| {
                                                scale
                                                    * ((if b { 1.0 } else { -1.0 })
                                                        + sigma * rng.gaussian())
                                            })
                                            .collect();
                                        let t0 = Instant::now();
                                        let d = rm.decode_soft_checked(&y, 32).unwrap();
                                        decode_ns += t0.elapsed().as_nanos();
                                        errs += u64::from(d.message != msg);
                                        uncert += u64::from(!d.certified);
                                        cert_errs += u64::from(d.certified && d.message != msg);
                                    } else {
                                        let y: Vec<f64> = pac
                                            .encode(&msg)
                                            .iter()
                                            .map(|&b| {
                                                scale
                                                    * ((if b { -1.0 } else { 1.0 })
                                                        + sigma * rng.gaussian())
                                            })
                                            .collect();
                                        // Ladder of list sizes (indices into 8, 32, 128); adaptive
                                        // schemes retry only while the answer is unproven.
                                        let ladder: &[usize] = match kind {
                                            1 => &[0],
                                            2 => &[1],
                                            3 => &[2],
                                            4 => &[0, 1],
                                            _ => &[0, 1, 2],
                                        };
                                        let t0 = Instant::now();
                                        let mut d = (Vec::new(), false);
                                        for &i in ladder {
                                            d = pac_decs[i].decode_checked(&y, min_distance);
                                            if d.1 {
                                                break;
                                            }
                                        }
                                        decode_ns += t0.elapsed().as_nanos();
                                        let (dec, certified) = d;
                                        errs += u64::from(dec != msg);
                                        uncert += u64::from(!certified);
                                        cert_errs += u64::from(certified && dec != msg);
                                    }
                                    words += 1;
                                }
                                (words, errs, decode_ns, n, k, uncert, cert_errs)
                            })
                        })
                        .collect();
                    let v: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
                    let w: u64 = v.iter().map(|x| x.0).sum();
                    let e: u64 = v.iter().map(|x| x.1).sum();
                    let ns: u128 = v.iter().map(|x| x.2).sum();
                    let (n, k) = (v[0].3, v[0].4);
                    let uncert: u64 = v.iter().map(|x| x.5).sum();
                    let cert_errs: u64 = v.iter().map(|x| x.6).sum();
                    println!(
                        "({n},{k}) {db} {label} {w} {e} {:.2e} {:.2} {:.2e} {cert_errs}",
                        e as f64 / w as f64,
                        ns as f64 / 1000.0 / w as f64,
                        uncert as f64 / w as f64
                    );
                }
            }
        }
    }

    if bench_mode == "all" || bench_mode == "ae" || bench_mode == "ae512" {
        println!("\n=== Mode 0: RM automorphism-ensemble SCL (M copies x L paths) vs. plain SCL ===");
        println!("code dB M L words errs wer avg_us uncertified cert_errs");
        let configs: &[(u32, u32, &[f64])] = if bench_mode == "ae512" {
            &[
                (3, 9, &[2.0, 2.5]),
                (4, 9, &[2.5, 3.0]),
                (4, 10, &[2.5, 3.0]),
                (5, 10, &[3.0, 3.5]),
            ]
        } else {
            &[(3, 7, &[3.0, 3.5]), (3, 8, &[2.5, 3.0])]
        };
        let schemes: &[(usize, usize)] =
            &[(1, 8), (1, 32), (8, 1), (32, 1), (8, 4), (4, 8), (8, 8), (32, 8)];
        for &(r, m, dbs) in configs {
            for &db in dbs {
                for &(copies, l) in schemes {
                    let handles: Vec<_> = (0..threads)
                        .map(|t| {
                            std::thread::spawn(move || {
                                let rm = ReedMuller::new(r, m).unwrap();
                                let (n, k) = (rm.block_len(), rm.message_len());
                                let sigma = (1.0
                                    / (2.0 * (k as f64 / n as f64) * 10f64.powf(db / 10.0)))
                                .sqrt();
                                let scale = 2.0 / (sigma * sigma);
                                let mut rng = Rng(mix_seed(t * 1000 + (db * 10.0) as u64 * 7));
                                let (mut words, mut errs, mut decode_ns) = (0u64, 0u64, 0u128);
                                let (mut uncert, mut cert_errs) = (0u64, 0u64);
                                let start = Instant::now();
                                while start.elapsed().as_secs_f64() < secs_per_row {
                                    let msg: Vec<bool> =
                                        (0..k).map(|_| rng.next() & 1 == 1).collect();
                                    let y: Vec<f64> = rm
                                        .encode(&msg)
                                        .unwrap()
                                        .iter()
                                        .map(|&b| {
                                            scale
                                                * ((if b { 1.0 } else { -1.0 })
                                                    + sigma * rng.gaussian())
                                        })
                                        .collect();
                                    let t0 = Instant::now();
                                    let d = rm.decode_soft_ensemble_checked(&y, l, copies).unwrap();
                                    decode_ns += t0.elapsed().as_nanos();
                                    errs += u64::from(d.message != msg);
                                    uncert += u64::from(!d.certified);
                                    cert_errs += u64::from(d.certified && d.message != msg);
                                    words += 1;
                                }
                                (words, errs, decode_ns, n, k, uncert, cert_errs)
                            })
                        })
                        .collect();
                    let v: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
                    let w: u64 = v.iter().map(|x| x.0).sum();
                    let e: u64 = v.iter().map(|x| x.1).sum();
                    let ns: u128 = v.iter().map(|x| x.2).sum();
                    let (n, k) = (v[0].3, v[0].4);
                    let uncert: u64 = v.iter().map(|x| x.5).sum();
                    let cert_errs: u64 = v.iter().map(|x| x.6).sum();
                    println!(
                        "({n},{k}) {db} {copies} {l} {w} {e} {:.2e} {:.2} {:.2e} {cert_errs}",
                        e as f64 / w as f64,
                        ns as f64 / 1000.0 / w as f64,
                        uncert as f64 / w as f64
                    );
                }
                for &l in &[8usize, 32] {
                    for &(label, with_crc) in &[("5G-noCRC", false), ("5G-CRC11", true)] {
                        let handles: Vec<_> = (0..threads)
                            .map(|t| {
                                std::thread::spawn(move || {
                                    let rm = ReedMuller::new(r, m).unwrap();
                                    let (n, k) = (rm.block_len(), rm.message_len());
                                    let polar = if with_crc {
                                        PolarCode5G::new(n, k)
                                    } else {
                                        PolarCode5G::new_subcode(n, k, k, 0, 0, None)
                                    };
                                    let mut polar_dec = PolarSclDecoder::new(polar.clone(), l);
                                    let sigma = (1.0
                                        / (2.0 * (k as f64 / n as f64) * 10f64.powf(db / 10.0)))
                                    .sqrt();
                                    let scale = 2.0 / (sigma * sigma);
                                    let mut rng = Rng(mix_seed(t * 1000 + (db * 10.0) as u64 * 7));
                                    let (mut words, mut errs, mut decode_ns) = (0u64, 0u64, 0u128);
                                    let start = Instant::now();
                                    while start.elapsed().as_secs_f64() < secs_per_row {
                                        let msg: Vec<bool> =
                                            (0..k).map(|_| rng.next() & 1 == 1).collect();
                                        let y: Vec<f64> = polar
                                            .encode(&msg)
                                            .iter()
                                            .map(|&b| {
                                                scale
                                                    * ((if b { -1.0 } else { 1.0 })
                                                        + sigma * rng.gaussian())
                                            })
                                            .collect();
                                        let t0 = Instant::now();
                                        let (dec, crc_ok) = polar_dec.decode(&y);
                                        decode_ns += t0.elapsed().as_nanos();
                                        errs += u64::from(dec != msg || !crc_ok);
                                        words += 1;
                                    }
                                    (words, errs, decode_ns, n, k)
                                })
                            })
                            .collect();
                        let v: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
                        let w: u64 = v.iter().map(|x| x.0).sum();
                        let e: u64 = v.iter().map(|x| x.1).sum();
                        let ns: u128 = v.iter().map(|x| x.2).sum();
                        let (n, k) = (v[0].3, v[0].4);
                        println!(
                            "({n},{k}) {db} {label} {l} {w} {e} {:.2e} {:.2}",
                            e as f64 / w as f64,
                            ns as f64 / 1000.0 / w as f64
                        );
                    }
                }
            }
        }
    }

    if bench_mode == "all" || bench_mode == "cert" {
        println!("\n=== Mode 0: CRC-free certificates on plain RM ===");
        println!("adaptive = L=8, retried at L=32 only when the L=8 answer is not certified ML.");
        println!("Then flag uncertified words, plus the smallest runner-up margins up to `flag`; ud = errors not flagged.");
        println!("code dB | wer L8 L32 adaptive | retry us_L8 us_L32 us_adaptive | uncert ml_errs | ud@uncert ud@1e-3 ud@1e-2");
        let configs: &[(u32, u32, &[f64])] = &[
            (2, 5, &[4.0]),
            (3, 6, &[4.0]),
            (3, 7, &[3.0, 3.5, 4.0]),
            (3, 8, &[2.5, 3.0]),
        ];
        for &(r, m, dbs) in configs {
            for &db in dbs {
                let handles: Vec<_> = (0..threads)
                    .map(|t| {
                        std::thread::spawn(move || {
                            let rm = ReedMuller::new(r, m).unwrap();
                            let (n, k) = (rm.block_len(), rm.message_len());
                            let sigma = (1.0
                                / (2.0 * (k as f64 / n as f64) * 10f64.powf(db / 10.0)))
                            .sqrt();
                            let scale = 2.0 / (sigma * sigma);
                            let mut rng = Rng(mix_seed(t * 1000 + (db * 10.0) as u64 * 7));
                            // per word: (margin, adaptive error, certified, L8 error, L32 error,
                            // retried, ns L8, ns L32)
                            let mut rows = Vec::new();
                            let start = Instant::now();
                            while start.elapsed().as_secs_f64() < secs_per_row {
                                let msg: Vec<bool> = (0..k).map(|_| rng.next() & 1 == 1).collect();
                                let y: Vec<f64> = rm
                                    .encode(&msg)
                                    .unwrap()
                                    .iter()
                                    .map(|&b| {
                                        scale * ((if b { 1.0 } else { -1.0 }) + sigma * rng.gaussian())
                                    })
                                    .collect();
                                let t0 = Instant::now();
                                let d8 = rm.decode_soft_checked(&y, 8).unwrap();
                                let ns8 = t0.elapsed().as_nanos();
                                let t1 = Instant::now();
                                let d32 = rm.decode_soft_checked(&y, 32).unwrap();
                                let ns32 = t1.elapsed().as_nanos();
                                let retried = !d8.certified;
                                let d = if retried { &d32 } else { &d8 };
                                rows.push((
                                    d.runner_up_cost - d.cost,
                                    d.message != msg,
                                    d.certified,
                                    d8.message != msg,
                                    d32.message != msg,
                                    retried,
                                    ns8,
                                    ns32,
                                ));
                            }
                            (rows, n, k)
                        })
                    })
                    .collect();
                let v: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
                let (n, k) = (v[0].1, v[0].2);
                let rows: Vec<_> = v.into_iter().flat_map(|x| x.0).collect();
                let w = rows.len() as f64;
                let count = |f: &dyn Fn(&(f64, bool, bool, bool, bool, bool, u128, u128)) -> bool| {
                    rows.iter().filter(|x| f(x)).count() as f64
                };
                let us8 = rows.iter().map(|x| x.6).sum::<u128>() as f64 / 1000.0 / w;
                let us32 = rows.iter().map(|x| x.7).sum::<u128>() as f64 / 1000.0 / w;
                let us_adaptive =
                    rows.iter().map(|x| x.6 + if x.5 { x.7 } else { 0 }).sum::<u128>() as f64 / 1000.0 / w;
                let uncert = count(&|x| !x.2);
                // Certified errors are maximum-likelihood errors: no decoder could fix them.
                let ml_errs = count(&|x| x.1 && x.2);
                let mut certified: Vec<(f64, bool)> =
                    rows.iter().filter(|x| x.2).map(|x| (x.0, x.1)).collect();
                certified.sort_by(|a, b| a.0.total_cmp(&b.0));
                let ud_at = |flag: f64| -> String {
                    let extra = ((flag * w) as usize).saturating_sub(uncert as usize).min(certified.len());
                    let missed = certified[extra..].iter().filter(|x| x.1).count();
                    format!("{:.2e}", missed as f64 / w)
                };
                println!(
                    "({n},{k}) {db} | {:.2e} {:.2e} {:.2e} | {:.4} {us8:.1} {us32:.1} {us_adaptive:.1} | {:.2e} {:.2e} | {} {} {}",
                    count(&|x| x.3) / w,
                    count(&|x| x.4) / w,
                    count(&|x| x.1) / w,
                    count(&|x| x.5) / w,
                    uncert / w,
                    ml_errs / w,
                    ud_at(0.0),
                    ud_at(1e-3),
                    ud_at(1e-2)
                );
            }
        }
    }

    if bench_mode == "all" || bench_mode == "5g" {
        println!("\n=== Mode 1: Pure RM (no CRC) vs. 5G NR Uplink Polar (with CRC/PC) at equal payload ===");
        println!("code scheme dB L words errs wer undetected_wer avg_us perseed_min perseed_max");

        let configs: &[(u32, u32, &[f64])] = &[
            (3, 7, &[2.5, 3.0, 3.5]),
            (2, 5, &[3.0, 4.0]),
            (3, 6, &[3.0, 4.0]),
            (3, 8, &[2.5, 3.0]),
            (2, 7, &[3.0]),
            (4, 7, &[4.0]),
        ];

        for &(r, m, dbs) in configs {
            for &db in dbs {
                for &l in &[8usize, 32] {
                    for rmmode in [true, false] {
                        let handles: Vec<_> = (0..threads)
                            .map(|t| {
                                std::thread::spawn(move || {
                                    let rm = ReedMuller::new(r, m).unwrap();
                                    let (n, k) = (rm.block_len(), rm.message_len());
                                    let polar = PolarCode5G::new(n, k);
                                    let mut polar_dec = PolarSclDecoder::new(polar.clone(), l);

                                    let sigma = (1.0
                                        / (2.0 * (k as f64 / n as f64) * 10f64.powf(db / 10.0)))
                                    .sqrt();
                                    let scale = 2.0 / (sigma * sigma);

                                    let mut rng = Rng(mix_seed(
                                        t * 1000
                                            + (db * 10.0) as u64 * 7
                                            + l as u64 * 131
                                            + ((r * 100 + m) as u64) * 100_003,
                                    ));
                                    for _ in 0..16 {
                                        rng.next();
                                    }

                                    let (mut words, mut errs, mut undet) = (0u64, 0u64, 0u64);
                                    let mut decode_ns = 0u128;
                                    let start = Instant::now();
                                    while start.elapsed().as_secs_f64() < secs_per_row {
                                        let msg: Vec<bool> =
                                            (0..k).map(|_| rng.next() & 1 == 1).collect();
                                        if rmmode {
                                            let x = rm.encode(&msg).unwrap();
                                            let y: Vec<f64> = x
                                                .iter()
                                                .map(|&b| {
                                                    scale
                                                        * ((if b { 1.0 } else { -1.0 })
                                                            + sigma * rng.gaussian())
                                                })
                                                .collect();
                                            let t0 = Instant::now();
                                            let dec = rm.decode_soft_list(&y, l).unwrap();
                                            decode_ns += t0.elapsed().as_nanos();
                                            if dec != msg {
                                                errs += 1;
                                                undet += 1;
                                            }
                                        } else {
                                            let x = polar.encode(&msg);
                                            let y: Vec<f64> = x
                                                .iter()
                                                .map(|&b| {
                                                    scale
                                                        * ((if b { -1.0 } else { 1.0 })
                                                            + sigma * rng.gaussian())
                                                })
                                                .collect();
                                            let t0 = Instant::now();
                                            let (dec, crc_ok) = polar_dec.decode(&y);
                                            decode_ns += t0.elapsed().as_nanos();
                                            if dec != msg || !crc_ok {
                                                errs += 1;
                                                if crc_ok {
                                                    undet += 1;
                                                }
                                            }
                                        }
                                        words += 1;
                                    }
                                    (words, errs, undet, decode_ns, n, k)
                                })
                            })
                            .collect();

                        let results: Vec<_> =
                            handles.into_iter().map(|h| h.join().unwrap()).collect();
                        let words: u64 = results.iter().map(|x| x.0).sum();
                        let errs: u64 = results.iter().map(|x| x.1).sum();
                        let undet: u64 = results.iter().map(|x| x.2).sum();
                        let total_ns: u128 = results.iter().map(|x| x.3).sum();
                        let avg_us = (total_ns as f64 / 1000.0) / words as f64;
                        let min_wer = results
                            .iter()
                            .map(|x| x.1 as f64 / x.0 as f64)
                            .fold(1.0, f64::min);
                        let max_wer = results
                            .iter()
                            .map(|x| x.1 as f64 / x.0 as f64)
                            .fold(0.0, f64::max);
                        println!(
                            "({},{}) {} {db} {l} {words} {errs} {:.2e} {:.2e} {avg_us:.2} {:.2e} {:.2e}",
                            results[0].4,
                            results[0].5,
                            if rmmode { "RM" } else { "NR" },
                            errs as f64 / words as f64,
                            undet as f64 / words as f64,
                            min_wer,
                            max_wer
                        );
                    }
                }
            }
        }
    }

    if bench_mode == "all" || bench_mode == "nocrc" || bench_mode == "samecrc" {
        println!("\n=== Mode 2 & 3: Like-for-like No-CRC and Same-CRC (RM vs. Polar, unified SCL) ===");
        println!("code mode scheme dB L words errs wer undetected_wer");

        let subcode_configs: &[(u32, u32, &[f64])] = &[
            (2, 5, &[3.0, 4.0]),
            (3, 6, &[3.0, 4.0]),
            (3, 7, &[3.0, 3.5]),
            (2, 7, &[3.0]),
            (4, 7, &[4.0]),
            (3, 8, &[2.5, 3.0]),
        ];

        for &(r, m, dbs) in subcode_configs {
            let rm0 = ReedMuller::new(r, m).unwrap();
            let (n, k) = (rm0.block_len(), rm0.message_len());
            let (crc_w, crc_p) = if n == 32 {
                (6u32, 0b100001u32)
            } else {
                (11u32, 0b11000100001u32)
            };

            for (mode_name, cwm, polym) in [("noCRC", 0u32, 0u32), ("sameCRC", crc_w, crc_p)] {
                if bench_mode == "nocrc" && mode_name != "noCRC" {
                    continue;
                }
                if bench_mode == "samecrc" && mode_name != "sameCRC" {
                    continue;
                }
                let a = k - cwm as usize;
                for &db in dbs {
                    for &l in &[8usize, 32] {
                        for is_rm in [true, false] {
                            let handles: Vec<_> = (0..threads)
                                .map(|t| {
                                    std::thread::spawn(move || {
                                        let code = PolarCode5G::new_subcode(
                                            n,
                                            k,
                                            a,
                                            cwm,
                                            polym,
                                            if is_rm { Some(m - r) } else { None },
                                        );
                                        let mut dec = PolarSclDecoder::new(code.clone(), l);
                                        let sigma = (1.0
                                            / (2.0 * (a as f64 / n as f64) * 10f64.powf(db / 10.0)))
                                        .sqrt();
                                        let scale = 2.0 / (sigma * sigma);
                                        let mut rng = Rng(mix_seed(
                                            t * 1000
                                                + (db * 10.0) as u64 * 7
                                                + l as u64 * 131
                                                + ((r * 100 + m) as u64) * 100_003
                                                + cwm as u64 * 977,
                                        ));
                                        for _ in 0..16 {
                                            rng.next();
                                        }
                                        let (mut w, mut e, mut ud) = (0u64, 0u64, 0u64);
                                        let start = Instant::now();
                                        while start.elapsed().as_secs_f64() < secs_per_row {
                                            let msg: Vec<bool> =
                                                (0..a).map(|_| rng.next() & 1 == 1).collect();
                                            let x = code.encode(&msg);
                                            let y: Vec<f64> = x
                                                .iter()
                                                .map(|&b| {
                                                    scale
                                                        * ((if b { -1.0 } else { 1.0 })
                                                            + sigma * rng.gaussian())
                                                })
                                                .collect();
                                            let (d, passed) = dec.decode(&y);
                                            if d != msg || !passed {
                                                e += 1;
                                                if passed {
                                                    ud += 1;
                                                }
                                            }
                                            w += 1;
                                        }
                                        (w, e, ud)
                                    })
                                })
                                .collect();
                            let v: Vec<_> =
                                handles.into_iter().map(|h| h.join().unwrap()).collect();
                            let w: u64 = v.iter().map(|x| x.0).sum();
                            let e: u64 = v.iter().map(|x| x.1).sum();
                            let ud: u64 = v.iter().map(|x| x.2).sum();
                            println!(
                                "({n},{a}+{cwm}) {mode_name} {} {db} {l} {w} {e} {:.2e} {:.2e}",
                                if is_rm { "RM" } else { "Polar" },
                                e as f64 / w as f64,
                                ud as f64 / w as f64
                            );
                        }
                    }
                }
            }
        }
    }

    if bench_mode == "all" || bench_mode == "rmcrc" {
        println!("\n=== Mode 4: RM + CRC on the parity shell (PolarCode5G::rm_crc) vs. Polar + CRC across CRC widths (0, 4, 6, 8, 11) ===");
        println!("code scheme dB L words errs det ud wer undetected_wer avg_us");

        let crc_polys: &[(u32, u32)] = &[
            (0, 0x0),
            (4, 0x3),
            (6, 0x21),
            (8, 0x07),
            (11, 0x621),
        ];
        let rmcrc_configs: &[(u32, u32, f64, &[u32])] = &[
            (2, 5, 3.0, &[0, 4, 6]),
            (3, 6, 4.0, &[0, 4, 6, 8, 11]),
            (3, 7, 3.5, &[0, 4, 6, 8, 11]),
        ];

        for &(r, m, db, crc_widths) in rmcrc_configs {
            let rm0 = ReedMuller::new(r, m).unwrap();
            let n = rm0.block_len();
            for &crc_w in crc_widths {
                let poly = crc_polys.iter().find(|&&(w, _)| w == crc_w).unwrap().1;
                for &l in &[8usize, 32] {
                    for is_rm in [true, false] {
                        let handles: Vec<_> = (0..threads)
                            .map(|t| {
                                std::thread::spawn(move || {
                                    let rm = PolarCode5G::rm_crc(m, r, crc_w, poly);
                                    let mut rm_dec = PolarSclDecoder::new(rm.clone(), l);
                                    let a = rm.payload_len;
                                    let polar = PolarCode5G::new_subcode(
                                        n,
                                        a + crc_w as usize,
                                        a,
                                        crc_w,
                                        poly,
                                        None,
                                    );
                                    let mut polar_dec = PolarSclDecoder::new(polar.clone(), l);

                                    let eff_rate = a as f64 / n as f64;
                                    let sigma =
                                        (1.0 / (2.0 * eff_rate * 10f64.powf(db / 10.0))).sqrt();
                                    let scale = 2.0 / (sigma * sigma);
                                    let mut rng = Rng(mix_seed(
                                        t * 1000
                                            + (db * 10.0) as u64 * 7
                                            + l as u64 * 131
                                            + ((r * 100 + m) as u64) * 100_003
                                            + crc_w as u64 * 977,
                                    ));
                                    for _ in 0..16 {
                                        rng.next();
                                    }
                                    let (mut w, mut e, mut det, mut ud) = (0u64, 0u64, 0u64, 0u64);
                                    let mut decode_ns = 0u128;
                                    let start = Instant::now();
                                    while start.elapsed().as_secs_f64() < secs_per_row {
                                        let msg: Vec<bool> =
                                            (0..a).map(|_| rng.next() & 1 == 1).collect();
                                        let (code, dec) = if is_rm {
                                            (&rm, &mut rm_dec)
                                        } else {
                                            (&polar, &mut polar_dec)
                                        };
                                        {
                                            let x = code.encode(&msg);
                                            let y: Vec<f64> = x
                                                .iter()
                                                .map(|&b| {
                                                    scale
                                                        * ((if b { -1.0 } else { 1.0 })
                                                            + sigma * rng.gaussian())
                                                })
                                                .collect();
                                            let t0 = Instant::now();
                                            let (got, passed) = dec.decode(&y);
                                            decode_ns += t0.elapsed().as_nanos();
                                            if got != msg || !passed {
                                                e += 1;
                                                if passed {
                                                    ud += 1;
                                                } else {
                                                    det += 1;
                                                }
                                            }
                                        }
                                        w += 1;
                                    }
                                    (w, e, det, ud, decode_ns, a)
                                })
                            })
                            .collect();
                        let v: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
                        let w: u64 = v.iter().map(|x| x.0).sum();
                        let e: u64 = v.iter().map(|x| x.1).sum();
                        let det: u64 = v.iter().map(|x| x.2).sum();
                        let ud: u64 = v.iter().map(|x| x.3).sum();
                        let total_ns: u128 = v.iter().map(|x| x.4).sum();
                        let avg_us = (total_ns as f64 / 1000.0) / w as f64;
                        let a = v[0].5;
                        println!(
                            "({n},{a}+{crc_w}) {} {db} {l} {w} {e} {det} {ud} {:.2e} {:.2e} {avg_us:.2}",
                            if is_rm { "RM_CRC" } else { "Polar_CRC" },
                            e as f64 / w as f64,
                            ud as f64 / w as f64
                        );
                    }
                }
            }
        }
    }
}
