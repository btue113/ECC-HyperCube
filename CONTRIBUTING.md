# Contributing to ECC-HyperCube

Thanks for your interest in contributing!

## Scope and design principles

This crate is intentionally small, portable, and self-contained:

- **Zero external runtime dependencies** and **no `unsafe` code**.
- **Published hypercube techniques only** in the library (`src/`): Reed–Muller codes and majority-logic decoding (1954), the Green fast Hadamard transform (1966), Dumer's recursive and list decoding (1999–2006), automorphism-ensemble decoding, and Tanner/GLDPC codes. Patent-encumbered constructions (PAC, the 5G NR polar construction, and RM with a CRC on the parity shell) appear only as comparison baselines in `benches/polar_5g.rs`, never in `src/`; see the README's Prior art section.
- **Bit-exact compatibility** for the hard-decision majority-vote decoder (`CubeCode::decode`), verified against the 2013 C++ reference in `tests/reference.rs` and `tests/cli.rs`.

## Development workflow

```sh
# Run the full test suite (release mode is much faster for reference & roundtrip tests)
cargo test --release

# Build examples and benchmarks
cargo build --release --examples --benches

# Optional: build for the host CPU (gains vary by CPU; measure before relying on it)
RUSTFLAGS="-C target-cpu=native" cargo test --release

# Check lints before opening a PR
cargo clippy --lib --bins --tests
```

## Good places to contribute

- **Performance:** `Subtree::Spc` (Wagner 1954 single-parity-check) fast-node.
- **Features:** `no_std` + `alloc` support; puncturing/shortening helpers; fading-channel evaluation in `examples/wer.rs`; Python bindings (`PyO3` / `maturin`); WebAssembly interactive demo.
- **Reproducibility & docs:** Scripts to regenerate all README tables into CSV; WER plots for the README.

## License

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this repository shall be dual-licensed under the MIT and Apache-2.0 licenses, as stated in `README.md`, without any additional terms or conditions.
