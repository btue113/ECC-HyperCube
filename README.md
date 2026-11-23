# ECC-HyperCube

Reed–Muller and Hypercube GLDPC error-correcting codes on the Boolean hypercube, in dependency-free Rust:
from a 2013 majority-vote decoder to soft decoders that keep pace with 5G-construction polar codes.

## Why it matters

Most open-source ECC libraries sit at two extremes: small hard-decision crates (Reed–Solomon, Hamming, CRC) or heavy C++/Python research frameworks (AFF3CT, Sionna). `ecc-hypercube` sits in between — a **zero-dependency, `#![forbid(unsafe_code)]` Rust crate** for short- and medium-block soft-decision decoding, built only on long-published hypercube techniques (a patent screen found no blocking claims; see [Prior art](#prior-art)).

- **Two complementary hypercube decoders (`n = 128..2048`):** RM automorphism ensemble (`ReedMuller`) for best WER at `n ≤ 512`, and Hypercube **GLDPC** (*Generalized Low-Density Parity-Check*, replacing single parity checks with extended-Hamming hypercube checks; `HypercubeGldpc`) for fast decoding + built-in error detection at `n ≥ 512`.
- **Provable ML certificates (RM):** `decode_soft_ensemble_checked` proves when no codeword is closer to the received word, letting the ensemble stop early.
- **Built-in error detection (GLDPC):** 0 undetected errors across ~30M decoded words and 0 false alarms in 4M noise-only trials (`< 1.5e-6` measured; `10⁸`-trial blind-decoding test pending).
- **Geometry over heavy algebra:** parity checks are subcubes, the 2013 voter polls parallel subcubes, and the ensemble relabels the cube's corners (see the [blog series](#history)).

### Learn coding theory on one shape

This project started in 2013 when visualizing Hamming `(7, 4)` on a 3D cube and generalizing it to higher dimensions independently led back to Reed–Muller codes (1954). Starting from the **hypercube** instead of finite fields or generator matrices is what made every later step click: halving a cube *is* the Plotkin/Polar butterfly, tilting a cube *is* an affine automorphism (`GA(m, 2)`), the smallest subcube *is* the minimum distance behind the ML proof, and slicing `n` vertices into small 4- and 5-cubes two different ways *is* Hypercube GLDPC.

Every stage stays in the crate (including the bit-for-bit 2013 decoder) so you can trace 70 years of coding theory on **one geometric object**:

| Stage | Idea on the hypercube | Where to read it |
| :--- | :--- | :--- |
| **1. Encoding & parity (1954)** | Bits sit on cube corners; every parity check is a subcube whose corners XOR to 0 (one 12-line `superset_xor` butterfly) | [`src/hypercube.rs`](src/hypercube.rs), [`src/cube_code.rs`](src/cube_code.rs) |
| **2. Hard voting (1954 / 2013)** | Majority vote across parallel subcubes, kept bit-for-bit identical to the original 2013 C++ decoder | `CubeCode::decode` |
| **3. Soft list decoding (2006)** | Run the same cube-halving butterfly on soft confidences (LLRs), with an `RM(1, d)` FWHT base case | [`src/list_decoder.rs`](src/list_decoder.rs) |
| **4. Ensemble + ML proof (1991 / 2021)** | Tilt and relabel the cube's corners (`GA(m, 2)`) to decode from multiple angles; the smallest subcube (`d_min`) certifies ML | `CubeCode::decode_soft_ensemble_checked` |
| **5. Hypercube GLDPC (1976 / 1981)** | Scale to `n = 512–2048` by slicing the vertices into small 4- and 5-cubes two different ways and passing FWHT APPs between cuts | [`src/gldpc.rs`](src/gldpc.rs) |

The visual walkthroughs live in the [5-part blog series](#history) (Parts 1–2 published, Parts 3–5 coming).

## Results vs. 5G-construction polar codes in AFF3CT

### Quick guide: which decoder to pick

| Block length | Use | WER vs. `SCL8` | Speed vs. software `SCL8` | Error detection |
| :--- | :--- | :--- | :--- | :--- |
| **`n ≤ 256`** | **RM Ensemble** (`8×4` / `32×8`) | **−0.16 to −0.59 dB** (`8×4`) / **−0.49 to −0.61 dB** (`32×8`) vs. `CRC-11` | `1–1.4×` (`8×4`) / `2.5–3.5×` slower (`32×8`) | None |
| **`n = 512`** | **RM Ensemble** *(best WER)* | **−0.31 to −0.32 dB** vs. `CRC-11` (`−0.14` to `−0.15` vs. `ASCL128`) | `7–25×` slower | None |
| **`n = 512`** | **Hypercube GLDPC** *(speed/detection)* | **−0.03 to −0.13 dB** vs. `CRC-24`; `+0.15` to `+0.27 dB` vs. `CRC-11` | **4–5× faster** | Built-in |
| **`n ≥ 1024`** | **Hypercube GLDPC** | `+0.03` to `+0.12 dB` vs. `CRC-11` | **2.6–3.4× faster** (`10–15×` vs. `SCL32`) | Built-in |

### Head-to-head at WER = 10⁻⁴

*BPSK/AWGN, random codewords, `Eb/N0` on payload rate `k/n`, 8 threads on an Apple M3 Pro (mixed P/E cores). Polar baselines: [AFF3CT](https://aff3ct.github.io/) v3.0.2 (`--enc-fb-gen-method 5G`, `--sim-stats` `decode_siho` pure decode only; CRC-24 appended, not distributed). Negative dB = `ecc-hypercube` better. Each point has about 100 word errors, so dB values are good to about ±0.05. `†` = extrapolated `0.2–0.3 dB`.*

#### RM Automorphism Ensemble

| Code | Setting | `Eb/N0` | vs. `SCL8` (`CRC-11`) | Pure decode | vs. software `SCL8` |
| :--- | :---: | :---: | :---: | :---: | :---: |
| `(128, 64)` | `8×4` | **3.50 dB** | **−0.59 dB** | 11 µs | `1.0×` (`11.5 µs`) |
| `(256, 93)` | `32×8` | **2.49 dB** | **−0.61 dB** | 68 µs | `3.5×` slower (`19.4 µs`) |
| `(256, 163)` | `32×8` | **3.32 dB** | **−0.49 dB** | 55 µs | `2.7×` slower (`20.7 µs`) |
| `(512, 256)` | `32×8` | **2.48 dB** | **−0.31 dB** | 930 µs | `25×` slower (`37.2 µs`) |
| `(1024, 386)` | `32×8` | 2.71 dB | `+0.59 dB` | ≈2,500 µs | `40×` slower (`63 µs`) |

#### Hypercube GLDPC (`RM(2, 4)` / `RM(3, 5)`, exact component APP via Hartmann–Rudolph FWHT, `f32`, 50 iters)

| Code | `Eb/N0` | vs. `SCL8` (`CRC-24`) | vs. `SCL8` (`CRC-11`) | Pure decode | vs. software `SCL8` |
| :--- | :---: | :---: | :---: | :---: | :---: |
| `(256, 97)` | **3.49 dB** | **−0.23 dB†** | — | **2.8 µs** | **7.3× faster** (`20.4 µs`) |
| `(512, 193)` | **2.79 dB** | **−0.03 dB** | `+0.27 dB` | **8.9 µs** | **4.2× faster** (`37 µs`) |
| `(512, 257)` | **2.95 dB** | **−0.13 dB** | `+0.15 dB` | **7.1 µs** | **5.2× faster** (`37 µs`) |
| `(1024, 385)` | **2.28 dB** | — | `+0.12 dB` | **24 µs** | **2.6× faster** (`62 µs`) |
| `(1024, 513)` | **2.46 dB** | — | `+0.03 dB` | **20 µs** | **3.4× faster** (`68 µs`) |

- **`n = 2048` (outside 5G tables; AFF3CT uses per-SNR Gaussian approximation):** GLDPC `(2048, 1025)` hits WER `10⁻⁴` at **2.09 dB** (`52 µs`) — within **0.05 dB** of `ASCL128` (`2.04 dB`, `9.6 µs`, `5.4×` faster) and **11× faster** than `SCL32` (`1.99 dB`, `597 µs`).
- **Worst-case latency (GLDPC):** all 50 iterations average **0.33 ms** at `n = 1024` (~3–4× below `ASCL128`'s full `L = 128` fallback, estimated by scaling `SCL32` to `1.1–1.2 ms`).
- **Where AFF3CT still wins:** `ASCL128` finishes most frames in a cheap first pass, making its *average* pure decode **3–7× faster** than GLDPC (`1.0–4.1 µs` at `n ≤ 1024`) and **0.05–0.60 dB** better on WER (`10⁻⁴`). AFF3CT's software `SCL8` is not its fastest path and hardware `SCL8` is far faster. The RM ensemble has no built-in error detection and a long worst-case tail when uncertified.

### Reproduce

`examples/wer.rs` is a short, std-only Monte-Carlo loop over the public API: random message,
`encode`, BPSK/AWGN, decode, compare. It prints WER and the decode call's time per word per thread.
Each table value is the Eb/N0 where log WER crosses `1e-4`, interpolated between bracketing points.

**1. ecc-hypercube**

```sh
git clone https://github.com/btue113/ECC-HyperCube && cd ECC-HyperCube
cargo build --release --example wer
W=target/release/examples/wer

# RM ensemble: rm <r> <m> <copies> <L> <Eb/N0 list>
$W rm 3 7  8 4 3.0,3.5,4.0          # (128,64)
$W rm 3 8 32 8 2.0,2.5,3.0          # (256,93)
$W rm 4 8 32 8 3.0,3.5              # (256,163)
$W rm 4 9 32 8 2.25,2.5,2.75        # (512,256)
$W rm 4 10 32 8 2.5,2.75            # (1024,386), slow

# GLDPC: gldpc <n> <3/8|1/2> <Eb/N0 list>
$W gldpc 256 3/8 3.25,3.5,3.75      # (256,97)
$W gldpc 512 3/8 2.5,2.75,3.0       # (512,193)
$W gldpc 512 1/2 2.75,3.0           # (512,257)
$W gldpc 1024 3/8 2.125,2.25,2.375  # (1024,385)
$W gldpc 1024 1/2 2.25,2.375,2.5    # (1024,513)
$W gldpc 2048 1/2 2.0,2.125         # (2048,1025)

# GLDPC false alarms: decode pure noise and count claimed codewords
$W gldpc-noise 512 3/8 1000000
```

**2. AFF3CT v3.0.2**

```sh
git clone --recursive --branch v3.0.2 https://github.com/aff3ct/aff3ct && cd aff3ct
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_FLAGS="-funroll-loops -mcpu=native"  # x86: -march=native
make -j
```

Run each polar code with the same payload `k` as the row it compares against, plus the CRC
(`-K` = `k + 11` for `11-5G`, `k + 24` for `24-5GC`). For example, against GLDPC `(1024, 513)`:

```sh
./bin/aff3ct -C POLAR -N 1024 -K 524 --enc-fb-gen-method 5G --crc-type 11-5G \
  -D SCL -L 8 -m 2.25 -M 2.5 -s 0.125 -e 100 -t 8 --sim-stats --ter-freq 0
```

- Other decoders: `-D SCL -L 32` or `-D ASCL -L 128`.
- Downlink CRC: `--crc-type 24-5GC` with `-K` = `k + 24`.
- `n = 2048`: `--enc-fb-gen-method GA` (the 5G tables stop at 1024).
- WER is the `FER` column. Pure decode time is the average `decode_siho` latency in the `--sim-stats` table.

## Prior art

This library started as a Rust rewrite of my [2013 C++ implementation](#history) of
hypercube-based majority-logic decoding, and grew along with the blog series
[*Hamming codes on the hypercube*](#history). The codes and algorithms implemented here are
classical:

- **The code and hypercube geometry:** Reed–Muller codes (Muller, 1954; Reed, 1954), where codewords are evaluations of Boolean polynomials on the $m$-dimensional hypercube and "unit cubes" are monomial supports. `CubeCode(m, m - r - 1)` is the punctured length-$(2^m - 1)$ code (leaving vertex `0` empty, like the 7-bit Hamming `(7, 4)` 3-cube in the blog); `ReedMuller::new(r, m)` wraps `CubeCode` and appends an overall parity bit at vertex `0` for the full length-$2^m$ code.
- **Hard decoder (`decode`):** Irving Reed's 1954 majority-logic decoder over parallel flats (independently rediscovered in the 2013 C++ program via the hypercube picture), accelerated here with a butterfly sweep (`superset_xor`, the standard Boolean/Plotkin transform $F^{\otimes m}$) and 64-bit SWAR popcount voting. It produces bit-for-bit identical results to the 2013 C++ program across 1,134 randomized cross-check jobs ($m = 2\text{--}15$).
- **Soft list decoder (`decode_soft`, `decode_soft_list`):** Dumer–Shabunov recursive list decoding of Reed–Muller codes (2006) and Tal–Vardy Successive Cancellation List (SCL) decoding (2015), using Balatsoukas-Stimming et al.'s hardware-friendly min-sum LLR path metrics (2015), with the same butterfly pattern as `superset_xor` run on soft values.
- **Automorphism ensemble (`decode_soft_ensemble`):** Geiselhart et al.'s automorphism-ensemble decoding of Reed–Muller codes (2021). RM codes are unchanged by affine relabellings of the cube, $x \mapsto Ax \oplus b$, so the decoder runs on several relabelled copies and keeps the closest codeword. The first copy is the identity, so it is never worse than plain SCL.
- **Hypercube GLDPC (`HypercubeGldpc`):** generalized LDPC codes, which replace single parity checks with short component codes on a Tanner graph (Tanner, 1981). Each bit sits in two extended-Hamming components, i.e. small RM(m − 2, m) cubes, as in Lentmaier and Zigangirov's Hamming-component GLDPC codes (1999). Each component is decoded with the exact symbol-by-symbol APP of Hartmann and Rudolph (1976), computed over the dual code RM(1, m) with a fast Walsh–Hadamard transform.
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

```rust
use ecc_hypercube::{GldpcRate, HypercubeGldpc};

// Hypercube GLDPC (1024, 513): two layers of extended-Hamming cube checks
let code = HypercubeGldpc::new(1024, GldpcRate::Rate1_2)?;
let msg = vec![true; code.message_len()];
let codeword = code.encode(&msg)?;

// f32 LLRs, positive favors '1'; decode with up to 50 iterations
let llrs: Vec<f32> = codeword.iter().map(|&b| if b { 2.0 } else { -2.0 }).collect();
let decision = code.decode_soft(&llrs, 50)?;
if decision.converged {
    // every parity check holds, so decision.message is a valid codeword's message;
    // when it is false, the decoder has detected its own failure
}
```

A `ReedMuller` locks its list decoder while decoding, so give each thread its own instance.

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

The results tables come from `examples/wer.rs` and AFF3CT (see [Reproduce](#reproduce)).
The dependency-free benchmarks are timing micro-benchmarks and research baselines:

- `cargo bench --bench codec`: hard-decision latency across code sizes, then hard, SC, SCL-8
  and SCL-32 latency per code.
- `cargo bench --bench simd_bench`: single-thread SCL and ensemble decoding time per word.
- `cargo bench --bench polar_5g`: Reed–Muller against an in-repo 5G NR polar construction
  (3GPP TS 38.212) on this crate's own SCL decoder. Its modes are `5g`, `nocrc`, `samecrc`,
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
its code is the unpolished 2013 version. **Use this repository instead:** it is tested bit-for-bit against the 2013 program's output, adds fast SWAR voting, soft list decoding, automorphism-ensemble decoding, maximum-likelihood certificates, and Hypercube GLDPC decoding, and is licensed MIT OR Apache-2.0.

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
