//! RM(r, m): the punctured cube code extended with an overall parity bit.

use crate::cube_code::CubeCode;
use crate::{Bits, Error};

#[derive(Debug)]
pub struct ReedMuller {
    code: CubeCode,
    r: u32,
    m: u32,
}

impl ReedMuller {
    pub fn new(r: u32, m: u32) -> Result<Self, Error> {
        if r >= m {
            return Err(Error::InvalidOrder { r, m });
        }
        Ok(Self { code: CubeCode::new(m, m - r - 1)?, r, m })
    }

    /// k = C(m,0) + C(m,1) + ... + C(m,r)
    pub fn message_len(&self) -> usize {
        self.code.data_len()
    }

    /// n = 2^m
    pub fn block_len(&self) -> usize {
        self.code.codeword_len() + 1
    }

    pub fn rate(&self) -> f64 {
        self.message_len() as f64 / self.block_len() as f64
    }

    /// Errors guaranteed correctable: half the minimum distance 2^(m-r), less one.
    pub fn error_tolerance(&self) -> usize {
        (1 << (self.m - self.r - 1)) - 1
    }

    pub fn encode(&self, message: &[bool]) -> Result<Bits, Error> {
        let mut encoded = self.code.encode(message)?;
        let parity = encoded.iter().filter(|&&b| b).count() % 2 == 1;
        encoded.push(parity);
        Ok(encoded)
    }

    pub fn decode(&self, received: &[bool]) -> Result<Bits, Error> {
        if received.len() != self.block_len() {
            return Err(Error::Length { expected: self.block_len(), got: received.len() });
        }
        self.code.decode(&received[..received.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{format_bits, parse_bits};

    #[test]
    fn parameters() {
        let rm = ReedMuller::new(1, 4).unwrap();
        assert_eq!(rm.message_len(), 5);
        assert_eq!(rm.block_len(), 16);
        assert_eq!(rm.rate(), 0.3125);
        assert_eq!(rm.error_tolerance(), 3);
        assert!(ReedMuller::new(4, 4).is_err());
    }

    #[test]
    fn encode_decode() {
        let rm = ReedMuller::new(1, 4).unwrap();
        let encoded = rm.encode(&parse_bits("10110").unwrap()).unwrap();
        assert_eq!(format_bits(&encoded), "1011000110001101");
        let decoded = rm.decode(&parse_bits("1011100010000101").unwrap()).unwrap();
        assert_eq!(format_bits(&decoded), "10110");
    }
}
