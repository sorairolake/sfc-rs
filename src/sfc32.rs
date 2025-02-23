// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An implementation of the sfc32 random number generator.

use rand_core::{RngCore, SeedableRng, impls, le};

/// A sfc32 random number generator.
///
/// The sfc32 algorithm is not suitable for cryptographic uses but is very fast.
/// This algorithm has a 128-bit state and outputs 32-bit random numbers. The
/// average period of this algorithm is approximately 2<sup>127</sup>, and the
/// minimum period is greater than or equal to 2<sup>32</sup>.
///
/// # Examples
///
/// ```
/// # use rand_sfc::{
/// #     Sfc32,
/// #     rand_core::{RngCore, SeedableRng},
/// # };
/// #
/// let mut rng = Sfc32::from_seed([0; 12]);
/// assert_eq!(rng.next_u32(), 0x5146_76c3);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Sfc32 {
    a: u32,
    b: u32,
    c: u32,
    counter: u32,
}

impl Sfc32 {
    /// Creates a new `Sfc32` using the given seeds `a`, `b`, and `c`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rand_sfc::{Sfc32, rand_core::RngCore};
    /// #
    /// let mut rng = Sfc32::new(0, 0, 0);
    /// assert_eq!(rng.next_u32(), 0x5146_76c3);
    /// ```
    #[must_use]
    #[inline]
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        let mut state = Self {
            a,
            b,
            c,
            counter: 1,
        };
        for _ in 0..12 {
            state.next_u32();
        }
        state
    }
}

impl RngCore for Sfc32 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        const ROTATION: u32 = 21;
        const RIGHT_SHIFT: u32 = 9;
        const LEFT_SHIFT: u32 = 3;

        let tmp = self.a.wrapping_add(self.b).wrapping_add(self.counter);
        self.counter += 1;
        self.a = self.b ^ (self.b >> RIGHT_SHIFT);
        self.b = self.c.wrapping_add(self.c << LEFT_SHIFT);
        self.c = self.c.rotate_left(ROTATION).wrapping_add(tmp);
        tmp
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    #[inline]
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        impls::fill_bytes_via_next(self, dst);
    }
}

impl SeedableRng for Sfc32 {
    type Seed = [u8; 12];

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        let mut s = [u32::default(); 3];
        le::read_u32_into(&seed, &mut s);
        Self::new(s[0], s[1], s[2])
    }
}

#[cfg(test)]
mod tests {
    use core::{any, mem};

    use super::*;

    // This test vector was generated by the `RNG_output` command of PractRand
    // version pre0.95.
    //
    // To generate a hex dump:
    //
    // ```sh
    // ./RNG_output sfc32 64 0 | xxd -i
    // ```
    const EXPECTED: [u32; 16] = [
        0x5146_76c3,
        0x08a8_09df,
        0x3034_9d2b,
        0xfb52_c520,
        0x3880_2be1,
        0x9482_79e6,
        0xec4b_f1d9,
        0x7cb0_a909,
        0xfad8_b4a8,
        0x3ca4_b808,
        0x3821_b4c5,
        0x5e70_23ca,
        0x50f2_6bf7,
        0xf1e1_b0a2,
        0x6163_032f,
        0x3bf3_c9a4,
    ];
    const EXPECTED_BYTES: [u8; 64] = [
        0xc3, 0x76, 0x46, 0x51, 0xdf, 0x09, 0xa8, 0x08, 0x2b, 0x9d, 0x34, 0x30, 0x20, 0xc5, 0x52,
        0xfb, 0xe1, 0x2b, 0x80, 0x38, 0xe6, 0x79, 0x82, 0x94, 0xd9, 0xf1, 0x4b, 0xec, 0x09, 0xa9,
        0xb0, 0x7c, 0xa8, 0xb4, 0xd8, 0xfa, 0x08, 0xb8, 0xa4, 0x3c, 0xc5, 0xb4, 0x21, 0x38, 0xca,
        0x23, 0x70, 0x5e, 0xf7, 0x6b, 0xf2, 0x50, 0xa2, 0xb0, 0xe1, 0xf1, 0x2f, 0x03, 0x63, 0x61,
        0xa4, 0xc9, 0xf3, 0x3b,
    ];

    #[test]
    fn clone() {
        let rng = Sfc32::from_seed(Default::default());
        assert_eq!(rng.clone(), rng);
    }

    #[test]
    fn debug() {
        let rng = Sfc32::from_seed(Default::default());
        assert_eq!(
            format!("{rng:?}"),
            "Sfc32 { a: 3287285385, b: 2371254317, c: 4048138432, counter: 13 }"
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            Sfc32::from_seed(Default::default()),
            Sfc32::from_seed(Default::default())
        );
        assert_ne!(
            Sfc32::from_seed(Default::default()),
            Sfc32::from_seed([u8::MAX; 12])
        );
    }

    #[test]
    fn new() {
        let mut rng = Sfc32::new(u32::default(), u32::default(), u32::default());
        for e in EXPECTED {
            assert_eq!(rng.next_u32(), e);
        }
    }

    #[test]
    fn next_u32() {
        let mut rng = Sfc32::from_seed(Default::default());
        for e in EXPECTED {
            assert_eq!(rng.next_u32(), e);
        }
    }

    #[test]
    fn next_u64() {
        let mut rng = Sfc32::from_seed(Default::default());
        for e in EXPECTED.map(u64::from).chunks_exact(2) {
            assert_eq!(rng.next_u64(), (e[1] << u32::BITS) | e[0]);
        }
    }

    #[test]
    fn fill_bytes() {
        let mut rng = Sfc32::from_seed(Default::default());
        let mut dst = [u8::default(); 64];
        rng.fill_bytes(&mut dst);
        assert_eq!(dst, EXPECTED_BYTES);
    }

    #[test]
    fn seed_type() {
        assert_eq!(
            any::type_name::<<Sfc32 as SeedableRng>::Seed>(),
            any::type_name::<[u8; 12]>()
        );
        assert_eq!(
            mem::size_of::<<Sfc32 as SeedableRng>::Seed>(),
            mem::size_of::<[u8; 12]>()
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde() {
        let mut rng = Sfc32::from_seed(Default::default());

        let json = serde_json::to_string(&rng).unwrap();
        assert_eq!(
            json,
            r#"{"a":3287285385,"b":2371254317,"c":4048138432,"counter":13}"#
        );

        let mut deserialized_rng = serde_json::from_str::<Sfc32>(&json).unwrap();
        assert_eq!(deserialized_rng, rng);
        assert_eq!(deserialized_rng.next_u32(), rng.next_u32());
    }
}
