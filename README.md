# ECC-HyperCube

Reed–Muller error-correcting codes decoded on the Boolean hypercube, in dependency-free Rust:
from a 2013 majority-vote decoder to soft decoding that keeps pace with the 5G NR polar codes.

## Why it matters

Most open-source error-correction libraries sit at two extremes: small crates for classical
hard-decision codes (Reed–Solomon, Hamming, CRC, basic Viterbi), or large C++/Python research
frameworks (AFF3CT, Sionna) with heavy dependencies. `ecc-hypercube` sits in between: a
dependency-free Rust crate for **short-block soft-decision decoding**, built entirely on
long-published Reed–Muller techniques.

- **On par with 5G at short blocks.** In the two codes measured so far: at (128,64) the
  ensemble decoder matches the word error rate of 5G NR polar codes with CRC-aided list decoding
  (L = 32) in a fifth of the time (18 µs vs. 90 µs). At (256,93) it makes about half as many
  word errors (about 0.15 dB) in about the same time, though with no CRC it cannot flag its own
  errors.
- **It can prove its answer is the best possible.** A maximum-likelihood certificate says when no
  codeword is closer to what was received. At (128,64) it proves about 97% of words at 3.5 dB, so
  the decoder stops early, and it shows when a retry with more effort (or a retransmission
  request) could help.
- **Geometry instead of heavy algebra.** Textbooks teach these codes through finite fields,
  Boolean polynomials and Kronecker matrices. Here every idea is a picture on the hypercube:
  parity checks are subcubes, the 2013 voter polls parallel subcubes, the ensemble relabels the
  cube's corners (including tilts, not just rotations), and the minimum distance behind the ML
  proof is the smallest subcube. The [blog series](#history) draws them step by step (Parts 1–2
  published, Parts 3–5 coming), and the 2013 decoder is kept bit-for-bit so you can trace the
  path from a hand-drawn cube to a modern soft decoder.
- **One butterfly, ~1,500 lines.** A 12-line `superset_xor` sweep does encoding, the hard
  voter's cancellation and the clean-word check; list and ensemble decoding run the same halving
  pattern on soft values. All in readable, dependency-free Rust.
- **Long-published techniques, zero dependencies.** Everything in the crate is published
  Reed–Muller and list-decoding theory (see [Prior art](#prior-art)); patented constructions such
  as PAC and CRC-aided polar variants are kept out. Pure Rust with no external crates, licensed
  MIT OR Apache-2.0.

## Results vs. 5G NR polar codes

`benches/polar_5g.rs` compares the RM decoders with the 5G NR polar code (3GPP TS 38.212,
CRC-11, CRC-aided SCL with L = 32) at the same block length and payload, on AWGN with BPSK.
Word error rate (WER) and average time per word:

| Code | Decoder | WER | µs |
| --- | --- | --- | --- |
| (128,64) at 3.5 dB | 2013 hard decoder (`decode`) | 0.56 | |
| | SCL, L = 8 (`decode_soft_list`) | 8.1e-4 | 29 |
| | SCL, L = 32 | 1.2e-4 | 84 |
| | ensemble, 8 copies × L = 4 (`decode_soft_ensemble`) | **1.0e-4** | **18** |
| | 5G NR + CRC-11, L = 32 | 1.5e-4 | 90 |
| (256,93) at 2.5 dB | ensemble, 32 copies × L = 8 | **1.1e-4** (15 errors) | 168 |
| | 5G NR + CRC-11, L = 32 | 2.2e-4 (30 errors) | 171 |

Eb/N0 needed for WER 1e-4, against the normal-approximation limit for each code:

| Code | Limit | RM ensemble | 5G NR + CRC-11, L = 32 |
| --- | --- | --- | --- |
| (128,64) | 2.92 dB | 3.53 dB (8 × 4) | ~3.58 dB |
| (256,93) | 2.04 dB | ~2.50 dB (32 × 8) | ~2.65 dB |

Soft decoding gains roughly 3 dB over the 2013 hard decoder. At (256,93) the ensemble is what
closes the gap to 5G: plain SCL with L = 32 loses by about 100× (2.0e-3 vs. 2.2e-5 at 3 dB).

Caveats:

- Measured on 2026-11-14/15 with 8 decodes running in parallel, so the times compare with each
  other, not with single-core latency.
- The 5G code is this repository's own implementation. It has not yet been cross-checked against
  a reference library such as AFF3CT or Sionna.
- At (256,93) the ensemble searches up to 256 paths against 5G's 32. It also has no CRC, so
  unlike 5G it cannot flag its own errors. The ML certificate says "no codeword is closer", not
  "correct".
- Only these two codes were measured. RM sizes are fixed (n = 2^m, k a sum of binomials), with
  no puncturing or shortening.

Reproduce the RM rows with `BENCH_MODE=ae cargo bench --bench polar_5g` and the 5G rows with
`BENCH_MODE=5g`. Add `SECS_PER_ROW=60` or more to collect enough errors at WER 1e-4.

## Prior art

This library started as a Rust rewrite of my [2013 C++ implementation](#history) of
hypercube-based majority-logic decoding, and grew along with the blog series
[*Hamming codes on the hypercube*](#history). The codes and algorithms implemented here are
classical:

- **The code and hypercube geometry:** Reed–Muller codes (Muller, 1954; Reed, 1954), where codewords are evaluations of Boolean polynomials on the $m$-dimensional hypercube and "unit cubes" are monomial supports. `CubeCode(m, m - r - 1)` is the punctured length-$(2^m - 1)$ code (leaving vertex `0` empty, like the 7-bit Hamming `(7, 4)` 3-cube in the blog); `ReedMuller::new(r, m)` wraps `CubeCode` and appends an overall parity bit at vertex `0` for the full length-$2^m$ code.
- **Hard decoder (`decode`):** Irving Reed's 1954 majority-logic decoder over parallel flats (independently rediscovered in the 2013 C++ program via the hypercube picture), accelerated here with a butterfly sweep (`superset_xor`, the standard Boolean/Plotkin transform $F^{\otimes m}$) and 64-bit SWAR popcount voting. It produces bit-for-bit identical results to the 2013 C++ program across 1,134 randomized cross-check jobs ($m = 2\text{--}15$).
- **Soft list decoder (`decode_soft`, `decode_soft_list`):** Dumer–Shabunov recursive list decoding of Reed–Muller codes (2006) and Tal–Vardy Successive Cancellation List (SCL) decoding (2015), using Balatsoukas-Stimming et al.'s hardware-friendly min-sum LLR path metrics (2015), with the same butterfly pattern as `superset_xor` run on soft values.
- **Automorphism ensemble (`decode_soft_ensemble`):** Geiselhart et al.'s automorphism-ensemble decoding of Reed–Muller codes (2021). RM codes are unchanged by affine relabellings of the cube, $x \mapsto Ax \oplus b$, so the decoder runs on several relabelled copies and keeps the closest codeword. The first copy is the identity, so it is never worse than plain SCL.
- **Maximum-likelihood certificate (`decode_soft_checked`, `decode_soft_ensemble_checked`):** the Taipale–Pursley sufficient condition for maximum likelihood (1991), which uses the minimum distance $2^{m-r}$, plus the standard branch-and-bound argument that no pruned list path can beat a codeword costing no more than the cheapest pruned metric. The ensemble stops at the first copy whose answer is certified.

## How it works

Every non-zero vertex `c` of the `m`-dimensional boolean hypercube owns one codeword bit;
an extra overall parity bit stands in for vertex 0, giving a block of `2^m` bits.

- **Unit cube** of `c`: all vertices that are supersets of `c`.
- **Data bits** are the vertices with at least `m - r` ones; the rest are **parity bits**,
  each chosen so the XOR over its unit cube is zero.
- **Decoding** goes level by level, from the data vertices with the fewest ones (`m - r`) up to
  the all-ones vertex. Each data bit is
  decided by a majority vote over `2^(m-r) - 1` cubes parallel to its unit cube, and
  the decided value is cancelled out of the received word before the next level.

RM(r, m) has message length `k = C(m,0) + … + C(m,r)`, block length `n = 2^m` and corrects
up to `2^(m-r-1) - 1` bit errors.

## Build & run

```sh
cargo build --release
./target/release/cubecode < tests/fixtures/input
```

Input is a job count followed by one job per line:

```
r m activity cache data
```

- `r`, `m`: order and log-length of the code, `0 <= r < m <= 24`
- `activity`: `e` to encode, `d` to decode
- `cache`: `n`, `p` or `f`, accepted for compatibility with the C++ tool and ignored
  (cubes are enumerated on the fly, so nothing needs caching)
- `data`: a binary string, `k` bits when encoding, `2^m` bits when decoding

Example:

```sh
$ printf '2\n1 4 e n 10110\n1 4 d n 1011100010000101\n' | ./target/release/cubecode
Initialization in:2.79e-06 sec.
Message length(k):5
Block length(n):16
Rate(k/n):0.3125
Bit error tolerance:3 bit(s)
message:10110
encoded:1011000110001101
encoded in:5.42e-07 sec.
--------------------------------
...
received_code:1011100010000101
______decoded:10110
```

## Library

```rust
use ecc_hypercube::{format_bits, parse_bits, ReedMuller};

// Hard-decision RM(1, 4) — 16-bit tesseract code
let rm = ReedMuller::new(1, 4)?;
let encoded = rm.encode(&parse_bits("10110")?)?;
let decoded = rm.decode(&encoded)?;

// Soft-decision SCL decoding (positive LLR favors '1', negative favors '0')
let llrs: Vec<f64> = encoded.iter().map(|&b| if b { 1.0 } else { -1.0 }).collect();
let soft_decoded = rm.decode_soft(&llrs)?; // default L = 8 (or decode_soft_list(&llrs, 32))

// Automorphism ensemble: L = 8 on up to 4 relabelled copies of the cube
let ae_decoded = rm.decode_soft_ensemble(&llrs, 8, 4)?;

// Same decoding, plus evidence: cost, runner-up cost, and an ML certificate
let decision = rm.decode_soft_ensemble_checked(&llrs, 8, 4)?;
if decision.certified {
    // decision.message is provably the maximum-likelihood message
}
```

`SoftDecision` reports `message`, `cost` (the sum of `|llr|` where the codeword disagrees
with the received signs), `runner_up_cost`, `pruned_bound`, `distance_certified` (proved by
the minimum-distance test alone) and `certified` (proved by either test).

## Tests & Benchmarks

```sh
cargo test --release
```

Covers the original C++ unit tests, random round trips with up to the guaranteed
number of bit errors, soft SCL round trips, checks that the automorphism ensemble is never
worse than plain SCL and that every certified decision is truly maximum likelihood, and a
bit-exact comparison against the C++ tool's recorded output (including received words beyond
the correction bound).

Run the dependency-free benchmarks with:

- `cargo bench --bench codec`: hard-decision latency across code sizes, then hard, SC, SCL-8
  and SCL-32 latency per code.
- `cargo bench --bench polar_5g`: AWGN block-error-rate comparison of Reed–Muller and 5G NR
  Polar codes (3GPP TS 38.212) on the same SCL decoder. Its modes are `5g`, `nocrc`, `samecrc`,
  `ae` (automorphism ensemble), `cert` (ML certificate), `rmcrc` and `pac`; pick one with
  `BENCH_MODE=<mode>` (default: all).

The `rmcrc` (RM with a CRC on the parity shell) and `pac` (PAC with the RM rate profile)
constructions exist only inside `benches/polar_5g.rs`, as comparison baselines. They rely on
techniques covered by active patents, so they are deliberately kept out of the library. That
benchmark is not included in the published crate.

## History

In 2013, while writing a blog post visualizing Hamming `(7, 4)` on a 3D cube, I generalized the picture to higher-dimensional hypercubes and wrote a C++ decoder that voted over parallel subcubes level by level ([github.com/bomot113/ECC-HyperCube](https://github.com/bomot113/ECC-HyperCube), August 29 – October 1, 2013)—unaware at the time that I had independently found my way back to Irving Reed's 1954 majority-logic decoding of Reed–Muller codes.

That 2013 repository is kept exactly as it was, as the dated record of the original work. It is no
longer maintained (its author lost access to that account), it is licensed GPL-2.0-or-later, and
its code is the unpolished 2013 version. **Use this repository instead:** it is tested bit-for-bit against the 2013 program's output, adds fast SWAR voting, soft list decoding, automorphism-ensemble decoding and
maximum-likelihood certificates, and is licensed MIT OR Apache-2.0.

The visual path from the 3-cube to Reed–Muller majority voting, the fast butterfly sweep, soft SCL decoding, and 5G polar codes is told in the 5-part blog series *Hamming codes on the hypercube*:
- [Part 1: Catching a cheater with a cube (Hamming codes)](https://tuebui.com/intuitive-information/hamming-hypercube/)
- [Part 2: Catching three cheaters with a tesseract (Reed–Muller codes)](https://tuebui.com/intuitive-information/hamming-hypercube-part-2/)
- Part 3: Catching cheaters faster: three shortcuts on the hypercube (coming soon)
- Part 4: When cheaters whisper: soft list decoding on the hypercube (coming soon)
- Part 5: Seeing the cube from every angle: ensemble decoding on the hypercube (coming soon)

## Questions

Ask in [Discussions → Q&A](https://github.com/btue113/ECC-HyperCube/discussions/categories/q-a),
report bugs in [Issues](https://github.com/btue113/ECC-HyperCube/issues), or email bomot113@gmail.com.

## License

Copyright (c) 2013-2026 Tue Bui.

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

The original 2013 C++ implementation at `github.com/bomot113/ECC-HyperCube` was released under
GPL-2.0-or-later, and that code remains under those terms. This Rust rewrite is a new
implementation by the same author and is licensed only as above; nothing in this repository is
GPL-licensed.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
