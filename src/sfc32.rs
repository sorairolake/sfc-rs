// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An implementation of the sfc32 random number generator.

use rand_core::{RngCore, SeedableRng, impls, le};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A sfc32 random number generator.
///
/// The sfc32 algorithm is not suitable for cryptographic uses but is very fast.
/// This algorithm has a 128-bit state and outputs 32-bit random numbers. The
/// average period of this algorithm is approximately 2<sup>127</sup>, and the
/// minimum period is greater than or equal to 2<sup>32</sup>.
///
/// The algorithm used here is translated from the reference implementation
/// provided by [PractRand] version pre0.95, which is licensed under the [public
/// domain].
///
/// # Examples
///
/// ```
/// # use sfc_prng::{
/// #     Sfc32,
/// #     rand_core::{RngCore, SeedableRng},
/// # };
/// #
/// let mut rng = Sfc32::from_seed([0; 12]);
/// assert_eq!(rng.next_u32(), 0xfb52_c520);
/// ```
///
/// [PractRand]: https://pracrand.sourceforge.net/
/// [public domain]: https://pracrand.sourceforge.net/license.txt
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Sfc32 {
    a: u32,
    b: u32,
    c: u32,
    counter: u32,
}

impl Sfc32 {
    /// Creates a new `Sfc32` using the given seeds.
    ///
    /// If `rounds` is [`None`], the state is mixed up 15 rounds during
    /// initialization.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sfc_prng::{Sfc32, rand_core::RngCore};
    /// #
    /// let mut rng = Sfc32::new(0, 0, 0, None);
    /// assert_eq!(rng.next_u32(), 0xfb52_c520);
    /// ```
    #[must_use]
    #[inline]
    pub fn new(a: u32, b: u32, c: u32, rounds: Option<u32>) -> Self {
        let mut state = Self {
            a,
            b,
            c,
            counter: 1,
        };
        let rounds = rounds.unwrap_or(15);
        for _ in 0..rounds {
            state.next_u32();
        }
        state
    }

    #[allow(clippy::cast_possible_truncation)]
    /// Creates a new `Sfc32` using a [`u64`] seed.
    ///
    /// If `rounds` is [`None`], the state is mixed up 12 rounds during
    /// initialization.
    ///
    /// <div class="warning">
    ///
    /// Note that the result of this method is different from the result of
    /// [`Sfc32::seed_from_u64`].
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// # use sfc_prng::{Sfc32, rand_core::RngCore};
    /// #
    /// let mut rng = Sfc32::new_u64(0, None);
    /// assert_eq!(rng.next_u32(), 0x5146_76c3);
    /// ```
    #[must_use]
    #[inline]
    pub fn new_u64(seed: u64, rounds: Option<u32>) -> Self {
        let (a, b, c) = (0, seed as u32, (seed >> u32::BITS) as u32);
        let rounds = rounds.or(Some(12));
        Self::new(a, b, c, rounds)
    }
}

impl RngCore for Sfc32 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        const ROTATION: u32 = 21;
        const RIGHT_SHIFT: u32 = 9;
        const LEFT_SHIFT: u32 = 3;

        let tmp = self.a.wrapping_add(self.b).wrapping_add(self.counter);
        self.a = self.b ^ (self.b >> RIGHT_SHIFT);
        self.b = self.c.wrapping_add(self.c << LEFT_SHIFT);
        self.c = self.c.rotate_left(ROTATION).wrapping_add(tmp);
        self.counter = self.counter.wrapping_add(1);
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
        Self::new(s[0], s[1], s[2], None)
    }
}

#[cfg(test)]
mod tests {
    use core::{any, mem};

    use super::*;

    static EXPECTED_1: [u32; 16] = [
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
        0x6db6_c5e0,
        0x5733_1c8c,
        0x2aaf_9993,
    ];
    static EXPECTED_BYTES_1: [u8; 64] = [
        0x20, 0xc5, 0x52, 0xfb, 0xe1, 0x2b, 0x80, 0x38, 0xe6, 0x79, 0x82, 0x94, 0xd9, 0xf1, 0x4b,
        0xec, 0x09, 0xa9, 0xb0, 0x7c, 0xa8, 0xb4, 0xd8, 0xfa, 0x08, 0xb8, 0xa4, 0x3c, 0xc5, 0xb4,
        0x21, 0x38, 0xca, 0x23, 0x70, 0x5e, 0xf7, 0x6b, 0xf2, 0x50, 0xa2, 0xb0, 0xe1, 0xf1, 0x2f,
        0x03, 0x63, 0x61, 0xa4, 0xc9, 0xf3, 0x3b, 0xe0, 0xc5, 0xb6, 0x6d, 0x8c, 0x1c, 0x33, 0x57,
        0x93, 0x99, 0xaf, 0x2a,
    ];

    static SEED_2: [u8; 12] = [
        0x00, 0x00, 0x00, 0x00, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01,
    ];
    static EXPECTED_2: [u32; 16] = [
        0x35e0_5b54,
        0x4c62_7ca1,
        0x33a0_43e2,
        0xb611_3c67,
        0x7cab_9699,
        0x4a52_efeb,
        0x5936_797f,
        0x139e_2b9f,
        0xc7df_3db1,
        0x61ce_1717,
        0x5581_e344,
        0xbc30_16ea,
        0xa6bf_d381,
        0xcfed_5524,
        0x8a34_536c,
        0x3e3f_a43b,
    ];
    static EXPECTED_BYTES_2: [u8; 64] = [
        0x54, 0x5b, 0xe0, 0x35, 0xa1, 0x7c, 0x62, 0x4c, 0xe2, 0x43, 0xa0, 0x33, 0x67, 0x3c, 0x11,
        0xb6, 0x99, 0x96, 0xab, 0x7c, 0xeb, 0xef, 0x52, 0x4a, 0x7f, 0x79, 0x36, 0x59, 0x9f, 0x2b,
        0x9e, 0x13, 0xb1, 0x3d, 0xdf, 0xc7, 0x17, 0x17, 0xce, 0x61, 0x44, 0xe3, 0x81, 0x55, 0xea,
        0x16, 0x30, 0xbc, 0x81, 0xd3, 0xbf, 0xa6, 0x24, 0x55, 0xed, 0xcf, 0x6c, 0x53, 0x34, 0x8a,
        0x3b, 0xa4, 0x3f, 0x3e,
    ];

    #[test]
    fn clone() {
        let rng = Sfc32::from_seed(Default::default());
        assert_eq!(rng.clone(), rng);
    }

    #[test]
    fn debug() {
        {
            let rng = Sfc32::from_seed(Default::default());
            assert_eq!(
                format!("{rng:?}"),
                "Sfc32 { a: 3033783054, b: 1182722562, c: 4269119441, counter: 16 }"
            );
        }
        {
            let rng = Sfc32::seed_from_u64(1);
            assert_eq!(
                format!("{rng:?}"),
                "Sfc32 { a: 163349985, b: 1519831815, c: 3040613532, counter: 16 }"
            );
        }
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
        {
            let mut rng = Sfc32::new(u32::default(), u32::default(), u32::default(), None);
            for e in EXPECTED_1 {
                assert_eq!(rng.next_u32(), e);
            }
        }
        {
            let mut rng = Sfc32::new(u32::default(), 0x89ab_cdef, 0x0123_4567, None);
            for e in EXPECTED_2 {
                assert_eq!(rng.next_u32(), e);
            }
        }
    }

    #[test]
    fn new_u64() {
        {
            // This test vector was generated by the `RNG_output` command of PractRand
            // version pre0.95.
            //
            // To generate a hex dump:
            //
            // ```sh
            // ./RNG_output sfc32 64 0x0 | xxd -i
            // ```
            let expected = [
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

            let mut rng = Sfc32::new_u64(u64::default(), None);
            for e in expected {
                assert_eq!(rng.next_u32(), e);
            }
        }
        {
            // This test vector was generated by the `RNG_output` command of PractRand
            // version pre0.95.
            //
            // To generate a hex dump:
            //
            // ```sh
            // ./RNG_output sfc32 64 0x123456789abcdef | xxd -i
            // ```
            let expected = [
                0x8471_2d97,
                0xf5a3_d9c8,
                0x5cd0_a295,
                0x35e0_5b54,
                0x4c62_7ca1,
                0x33a0_43e2,
                0xb611_3c67,
                0x7cab_9699,
                0x4a52_efeb,
                0x5936_797f,
                0x139e_2b9f,
                0xc7df_3db1,
                0x61ce_1717,
                0x5581_e344,
                0xbc30_16ea,
                0xa6bf_d381,
            ];

            let mut rng = Sfc32::new_u64(0x0123_4567_89ab_cdef, None);
            for e in expected {
                assert_eq!(rng.next_u32(), e);
            }
        }
    }

    #[test]
    fn next_u32() {
        {
            let mut rng = Sfc32::from_seed(Default::default());
            for e in EXPECTED_1 {
                assert_eq!(rng.next_u32(), e);
            }
        }
        {
            let mut rng = Sfc32::from_seed(SEED_2);
            for e in EXPECTED_2 {
                assert_eq!(rng.next_u32(), e);
            }
        }
        {
            let seed = [
                0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            ];
            let expected = [
                0x03b8_0bb8,
                0xa87d_bc7e,
                0x1787_178c,
                0x4c7b_7234,
                0xc65d_ade2,
                0x2c69_2349,
                0xf52c_2153,
                0xdf09_8072,
                0x9d49_b03c,
                0x9562_381a,
                0xc9b4_1738,
                0x64b7_5e54,
                0x36ce_9b32,
                0xf106_947e,
                0x0afc_726b,
                0x549b_bc87,
            ];

            let mut rng = Sfc32::from_seed(seed);
            for e in expected {
                assert_eq!(rng.next_u32(), e);
            }
        }
    }

    #[test]
    fn next_u64() {
        {
            let mut rng = Sfc32::from_seed(Default::default());
            for e in EXPECTED_1.map(u64::from).chunks_exact(2) {
                assert_eq!(rng.next_u64(), (e[1] << u32::BITS) | e[0]);
            }
        }
        {
            let mut rng = Sfc32::from_seed(SEED_2);
            for e in EXPECTED_2.map(u64::from).chunks_exact(2) {
                assert_eq!(rng.next_u64(), (e[1] << u32::BITS) | e[0]);
            }
        }
    }

    #[test]
    fn fill_bytes() {
        {
            let mut rng = Sfc32::from_seed(Default::default());
            let mut dst = [u8::default(); 64];
            rng.fill_bytes(&mut dst);
            assert_eq!(dst, EXPECTED_BYTES_1);
        }
        {
            let mut rng = Sfc32::from_seed(SEED_2);
            let mut dst = [u8::default(); 64];
            rng.fill_bytes(&mut dst);
            assert_eq!(dst, EXPECTED_BYTES_2);
        }
    }

    #[test]
    fn fill_bytes_per_chunk() {
        {
            let mut rng = Sfc32::from_seed(Default::default());
            let mut dst = [u8::default(); 4];
            for e in EXPECTED_BYTES_1.chunks_exact(dst.len()) {
                rng.fill_bytes(&mut dst);
                assert_eq!(dst, e);
            }
        }
        {
            let mut rng = Sfc32::from_seed(SEED_2);
            let mut dst = [u8::default(); 4];
            for e in EXPECTED_BYTES_2.chunks_exact(dst.len()) {
                rng.fill_bytes(&mut dst);
                assert_eq!(dst, e);
            }
        }
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
            r#"{"a":3033783054,"b":1182722562,"c":4269119441,"counter":16}"#
        );

        let mut deserialized_rng = serde_json::from_str::<Sfc32>(&json).unwrap();
        assert_eq!(deserialized_rng, rng);
        assert_eq!(deserialized_rng.next_u32(), rng.next_u32());
    }
}
