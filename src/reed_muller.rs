//! RM(r, m): the punctured cube code extended with an overall parity bit.

use crate::cube_code::CubeCode;
use crate::{Bits, Error, RmScratch, SoftDecision};

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

    pub fn decode_soft(&self, received: &[f64]) -> Result<Bits, Error> {
        self.decode_soft_list(received, 8)
    }

    pub fn decode_soft_list(&self, received: &[f64], list_size: usize) -> Result<Bits, Error> {
        if received.len() != self.block_len() {
            return Err(Error::Length { expected: self.block_len(), got: received.len() });
        }
        let (&zero_llr, rest) = received.split_last().unwrap();
        self.code.decode_soft_list_with_zero(rest, Some(zero_llr), list_size)
    }

    /// Automorphism-ensemble decoding: run SCL with `list_size` paths on `ensemble_size`
    /// relabelled copies of the cube and keep the closest codeword. `ensemble_size = 1`
    /// is plain `decode_soft_list`.
    pub fn decode_soft_ensemble(
        &self,
        received: &[f64],
        list_size: usize,
        ensemble_size: usize,
    ) -> Result<Bits, Error> {
        Ok(self.decode_soft_ensemble_checked(received, list_size, ensemble_size)?.message)
    }

    /// Like `decode_soft_ensemble`, with the cost, runner-up and certificate of
    /// `decode_soft_checked`. Stops early at the first copy whose answer is proven best.
    pub fn decode_soft_ensemble_checked(
        &self,
        received: &[f64],
        list_size: usize,
        ensemble_size: usize,
    ) -> Result<SoftDecision, Error> {
        if received.len() != self.block_len() {
            return Err(Error::Length { expected: self.block_len(), got: received.len() });
        }
        let (&zero_llr, rest) = received.split_last().unwrap();
        self.code.decode_soft_ensemble_with_zero(rest, zero_llr, list_size, ensemble_size)
    }

    /// Like `decode_soft_list`, but also reports the decoded codeword's cost, the
    /// runner-up's cost, and whether the answer is provably maximum likelihood.
    pub fn decode_soft_checked(&self, received: &[f64], list_size: usize) -> Result<SoftDecision, Error> {
        if received.len() != self.block_len() {
            return Err(Error::Length { expected: self.block_len(), got: received.len() });
        }
        let (&zero_llr, rest) = received.split_last().unwrap();
        self.code.decode_soft_checked_with_zero(rest, Some(zero_llr), list_size)
    }

    /// Lock-free `decode_soft_list` using a caller-owned [`RmScratch`].
    pub fn decode_soft_list_with_scratch(
        &self,
        received: &[f64],
        list_size: usize,
        scratch: &mut RmScratch,
    ) -> Result<Bits, Error> {
        if received.len() != self.block_len() {
            return Err(Error::Length { expected: self.block_len(), got: received.len() });
        }
        let (&zero_llr, rest) = received.split_last().unwrap();
        self.code.decode_soft_list_with_zero_scratch(rest, Some(zero_llr), list_size, scratch)
    }

    /// Lock-free `decode_soft_checked` using a caller-owned [`RmScratch`].
    pub fn decode_soft_checked_with_scratch(
        &self,
        received: &[f64],
        list_size: usize,
        scratch: &mut RmScratch,
    ) -> Result<SoftDecision, Error> {
        if received.len() != self.block_len() {
            return Err(Error::Length { expected: self.block_len(), got: received.len() });
        }
        let (&zero_llr, rest) = received.split_last().unwrap();
        self.code.decode_soft_checked_with_zero_scratch(rest, Some(zero_llr), list_size, scratch)
    }

    /// Lock-free `decode_soft_ensemble` using a caller-owned [`RmScratch`].
    pub fn decode_soft_ensemble_with_scratch(
        &self,
        received: &[f64],
        list_size: usize,
        ensemble_size: usize,
        scratch: &mut RmScratch,
    ) -> Result<Bits, Error> {
        Ok(self
            .decode_soft_ensemble_checked_with_scratch(received, list_size, ensemble_size, scratch)?
            .message)
    }

    /// Lock-free `decode_soft_ensemble_checked` using a caller-owned [`RmScratch`].
    pub fn decode_soft_ensemble_checked_with_scratch(
        &self,
        received: &[f64],
        list_size: usize,
        ensemble_size: usize,
        scratch: &mut RmScratch,
    ) -> Result<SoftDecision, Error> {
        if received.len() != self.block_len() {
            return Err(Error::Length { expected: self.block_len(), got: received.len() });
        }
        let (&zero_llr, rest) = received.split_last().unwrap();
        self.code.decode_soft_ensemble_with_zero_scratch(
            rest,
            zero_llr,
            list_size,
            ensemble_size,
            scratch,
        )
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

    #[test]
    fn scratch_matches_and_shares_across_threads() {
        use std::sync::Arc;
        let rm = Arc::new(ReedMuller::new(3, 7).unwrap());
        let msg: Vec<bool> = (0..rm.message_len()).map(|i| (i * 7 + 3) % 2 == 0).collect();
        let cw = rm.encode(&msg).unwrap();
        let mut llrs: Vec<f64> = cw.iter().map(|&b| if b { 2.0 } else { -2.0 }).collect();
        for &pos in &[3usize, 17, 42, 88, 105] {
            llrs[pos] = -llrs[pos];
        }
        let expected = rm.decode_soft_ensemble_checked(&llrs, 16, 4).unwrap();
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let rm = Arc::clone(&rm);
                let llrs = llrs.clone();
                let expected = expected.clone();
                std::thread::spawn(move || {
                    let mut scratch = RmScratch::new();
                    let got = rm.decode_soft_ensemble_checked_with_scratch(&llrs, 16, 4, &mut scratch).unwrap();
                    assert_eq!(got, expected);
                    let got_list = rm.decode_soft_list_with_scratch(&llrs, 8, &mut scratch).unwrap();
                    assert_eq!(got_list, rm.decode_soft_list(&llrs, 8).unwrap());
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
    }
}
