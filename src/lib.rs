//! Reed-Muller error correcting codes built on hypercubes.
//!
//! ```
//! use ecc_hypercube::{format_bits, parse_bits, ReedMuller};
//!
//! let rm = ReedMuller::new(1, 4).unwrap();
//! let encoded = rm.encode(&parse_bits("10110").unwrap()).unwrap();
//! assert_eq!(format_bits(&encoded), "1011000110001101");
//!
//! let mut received = encoded.clone();
//! received[3] ^= true; // flip a bit in transit
//! assert_eq!(format_bits(&rm.decode(&received).unwrap()), "10110");
//! ```

pub mod cube_code;
pub mod hypercube;
pub mod reed_muller;

pub use cube_code::CubeCode;
pub use hypercube::HyperCube;
pub use reed_muller::ReedMuller;

use std::fmt;

/// A bit string; index 0 is the leftmost character of its text form.
pub type Bits = Vec<bool>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidParams { cw_dims: u32, p_dims: u32 },
    InvalidOrder { r: u32, m: u32 },
    Length { expected: usize, got: usize },
    NotBinary(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidParams { cw_dims, p_dims } => write!(
                f,
                "invalid cube code: need 0 < dims <= {} and parity dims < dims, got dims={cw_dims}, parity dims={p_dims}",
                cube_code::MAX_DIMS
            ),
            Error::InvalidOrder { r, m } => write!(f, "invalid Reed-Muller code RM({r},{m}): need r < m"),
            Error::Length { expected, got } => write!(f, "expected {expected} bits, got {got}"),
            Error::NotBinary(c) => write!(f, "not a binary digit: {c:?}"),
        }
    }
}

impl std::error::Error for Error {}

pub fn parse_bits(s: &str) -> Result<Bits, Error> {
    s.chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            other => Err(Error::NotBinary(other)),
        })
        .collect()
}

pub fn format_bits(bits: &[bool]) -> String {
    bits.iter().map(|&b| if b { '1' } else { '0' }).collect()
}
