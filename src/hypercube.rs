//! Sub-cubes of the `dims`-dimensional boolean hypercube.
//!
//! A vertex of the hypercube is a `dims`-bit integer. A sub-cube is described
//! by a set of *fixed* bit positions (`fixed_mask`) and the values those bits
//! must take (`fixed_value`); every other bit is free. A sub-cube with `k`
//! fixed bits therefore has `2^(dims - k)` vertices.
//!
//! The *unit cube* of vertex `c` fixes every bit that is set in `c` to 1, so
//! its vertices are exactly the supersets of `c`. Its *parallel cubes* fix
//! the same positions to other values (the all-zero assignment is excluded,
//! since it would contain vertex 0, which the punctured code does not use).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HyperCube {
    dims: u32,
    fixed_mask: u32,
    fixed_value: u32,
}

impl HyperCube {
    pub fn new(dims: u32, fixed_mask: u32, fixed_value: u32) -> Self {
        assert!(dims < 32, "dims must be < 32");
        assert_eq!(fixed_mask >> dims, 0, "fixed bits out of range");
        assert_eq!(fixed_value & !fixed_mask, 0, "value sets a non-fixed bit");
        Self { dims, fixed_mask, fixed_value }
    }

    /// The unit cube of vertex `index`: all supersets of `index`.
    pub fn unit(dims: u32, index: u32) -> Self {
        Self::new(dims, index, index)
    }

    pub fn fixed_mask(&self) -> u32 {
        self.fixed_mask
    }

    pub fn fixed_value(&self) -> u32 {
        self.fixed_value
    }

    pub fn size(&self) -> usize {
        1 << (self.dims - self.fixed_mask.count_ones())
    }

    /// Vertices in ascending order (free bits count up from zero).
    pub fn elements(&self) -> Elements {
        let free = ((1u32 << self.dims) - 1) & !self.fixed_mask;
        Elements { base: self.fixed_value, free, next: Some(0) }
    }

    /// Up to `limit` cubes parallel to this one: same fixed positions, every
    /// non-zero assignment of their values. Assignments are ordered by number
    /// of ones (descending), then lexicographically descending with the lowest
    /// fixed position as the most significant digit. The first cube is
    /// therefore the all-ones assignment, i.e. the unit cube itself.
    pub fn parallel_cubes(&self, limit: usize) -> Vec<HyperCube> {
        let positions: Vec<u32> = (0..self.dims)
            .filter(|&p| self.fixed_mask >> p & 1 == 1)
            .collect();
        let k = positions.len() as u32;
        let all = (1u64 << k) - 1;
        // `w` holds an assignment with positions[i] at digit k-1-i.
        let to_value = |w: u64| {
            positions
                .iter()
                .enumerate()
                .filter(|&(i, _)| w >> (k - 1 - i as u32) & 1 == 1)
                .fold(0u32, |acc, (_, &p)| acc | 1 << p)
        };

        let mut cubes = Vec::with_capacity(limit.min(all as usize));
        for ones in (1..=k).rev() {
            // Descending assignments with `ones` ones are ascending
            // complements with `k - ones` ones, which Gosper's hack enumerates.
            let mut complement = (1u64 << (k - ones)) - 1;
            while complement <= all {
                if cubes.len() == limit {
                    return cubes;
                }
                cubes.push(HyperCube::new(self.dims, self.fixed_mask, to_value(all ^ complement)));
                if complement == 0 {
                    break;
                }
                let t = complement | (complement - 1);
                complement = (t + 1) | (((!t & (t + 1)) - 1) >> (complement.trailing_zeros() + 1));
            }
        }
        cubes
    }
}

/// Iterator over a cube's vertices, via ascending sub-mask enumeration.
pub struct Elements {
    base: u32,
    free: u32,
    next: Option<u32>,
}

impl Iterator for Elements {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let s = self.next?;
        let following = s.wrapping_sub(self.free) & self.free;
        self.next = (following != 0).then_some(following);
        Some(self.base | s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elements_with_fixed_bits() {
        // bit 0 fixed to 0, bit 3 fixed to 1
        let cube = HyperCube::new(4, 0b1001, 0b1000);
        assert_eq!(cube.size(), 4);
        assert_eq!(cube.elements().collect::<Vec<_>>(), vec![8, 10, 12, 14]);
    }

    #[test]
    fn unit_cube_is_supersets() {
        let cube = HyperCube::unit(4, 0b0101);
        assert_eq!(cube.elements().collect::<Vec<_>>(), vec![5, 7, 13, 15]);
        let full = HyperCube::unit(4, 0b1111);
        assert_eq!(full.elements().collect::<Vec<_>>(), vec![15]);
    }

    #[test]
    fn parallel_cubes_order() {
        // Fixed positions 0, 1, 2 => digits (p0 p1 p2):
        // 111, then 110 101 011, then 100 010 001.
        let values: Vec<u32> = HyperCube::unit(4, 0b0111)
            .parallel_cubes(usize::MAX)
            .iter()
            .map(HyperCube::fixed_value)
            .collect();
        assert_eq!(values, vec![0b111, 0b011, 0b101, 0b110, 0b001, 0b010, 0b100]);
    }

    #[test]
    fn parallel_cubes_limit() {
        let cubes = HyperCube::unit(4, 0b0111).parallel_cubes(3);
        assert_eq!(cubes.len(), 3);
        assert_eq!(cubes[0], HyperCube::unit(4, 0b0111));
    }
}
