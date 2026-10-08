# ECC-HyperCube

Fast encoding and decoding of Reed–Muller error correcting codes based on hypercubes.

A Rust rewrite of the original 2013 C++ implementation (`github.com/bomot113/ECC-HyperCube`).
It produces bit-for-bit identical results to the original on its sample input and on a
randomized cross-check of 1,134 encode/decode jobs (RM codes with m = 2–15), and runs
about 7× faster on that set. It has no dependencies.

## How it works

Every non-zero vertex `c` of the `m`-dimensional boolean hypercube owns one codeword bit;
an extra overall parity bit stands in for vertex 0, giving a block of `2^m` bits.

- **Unit cube** of `c`: all vertices that are supersets of `c`.
- **Data bits** are the vertices with at least `m - r` ones; the rest are **parity bits**,
  each chosen so the XOR over its unit cube is zero.
- **Decoding** goes level by level from the heaviest vertices down. Each data bit is
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

let rm = ReedMuller::new(1, 4)?;
let encoded = rm.encode(&parse_bits("10110")?)?;
let decoded = rm.decode(&encoded)?;
```

## Tests

```sh
cargo test --release
```

Covers the original C++ unit tests, random round trips with up to the guaranteed
number of bit errors, and a comparison against the C++ tool's recorded output.

## License

Copyright (c) 2013-2026 Tue Bui.

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

The original 2013 C++ implementation at `github.com/bomot113/ECC-HyperCube` was released under
GPL-2.0-or-later and remains available under those terms. This Rust rewrite is relicensed by its
author.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
