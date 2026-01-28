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
/// assert_eq!(rng.next_u32(), 0xFB52_C520);
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
    /// assert_eq!(rng.next_u32(), 0xFB52_C520);
    /// ```
    #[must_use]
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
    /// assert_eq!(rng.next_u32(), 0x5146_76C3);
    /// ```
    #[must_use]
    pub fn new_u64(seed: u64, rounds: Option<u32>) -> Self {
        let (a, b, c) = (0, seed as u32, (seed >> u32::BITS) as u32);
        let rounds = rounds.or(Some(12));
        Self::new(a, b, c, rounds)
    }
}

impl RngCore for Sfc32 {
    fn next_u32(&mut self) -> u32 {
        const ROTATION: u32 = 21;
        const RIGHT_SHIFT: u32 = 9;
        const LEFT_SHIFT: u32 = 3;

        let tmp = self.a.wrapping_add(self.b).wrapping_add(self.counter);
        self.counter = self.counter.wrapping_add(1);
        self.a = self.b ^ (self.b >> RIGHT_SHIFT);
        self.b = self.c.wrapping_add(self.c << LEFT_SHIFT);
        self.c = self.c.rotate_left(ROTATION).wrapping_add(tmp);
        tmp
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        impls::fill_bytes_via_next(self, dst);
    }
}

impl SeedableRng for Sfc32 {
    type Seed = [u8; 12];

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
        0xFB52_C520,
        0x3880_2BE1,
        0x9482_79E6,
        0xEC4B_F1D9,
        0x7CB0_A909,
        0xFAD8_B4A8,
        0x3CA4_B808,
        0x3821_B4C5,
        0x5E70_23CA,
        0x50F2_6BF7,
        0xF1E1_B0A2,
        0x6163_032F,
        0x3BF3_C9A4,
        0x6DB6_C5E0,
        0x5733_1C8C,
        0x2AAF_9993,
    ];
    static EXPECTED_BYTES_1: [u8; 64] = [
        0x20, 0xC5, 0x52, 0xFB, 0xE1, 0x2B, 0x80, 0x38, 0xE6, 0x79, 0x82, 0x94, 0xD9, 0xF1, 0x4B,
        0xEC, 0x09, 0xA9, 0xB0, 0x7C, 0xA8, 0xB4, 0xD8, 0xFA, 0x08, 0xB8, 0xA4, 0x3C, 0xC5, 0xB4,
        0x21, 0x38, 0xCA, 0x23, 0x70, 0x5E, 0xF7, 0x6B, 0xF2, 0x50, 0xA2, 0xB0, 0xE1, 0xF1, 0x2F,
        0x03, 0x63, 0x61, 0xA4, 0xC9, 0xF3, 0x3B, 0xE0, 0xC5, 0xB6, 0x6D, 0x8C, 0x1C, 0x33, 0x57,
        0x93, 0x99, 0xAF, 0x2A,
    ];

    const SEED_2: [u8; 12] = [
        0x00, 0x00, 0x00, 0x00, 0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01,
    ];
    static EXPECTED_2: [u32; 16] = [
        0x35E0_5B54,
        0x4C62_7CA1,
        0x33A0_43E2,
        0xB611_3C67,
        0x7CAB_9699,
        0x4A52_EFEB,
        0x5936_797F,
        0x139E_2B9F,
        0xC7DF_3DB1,
        0x61CE_1717,
        0x5581_E344,
        0xBC30_16EA,
        0xA6BF_D381,
        0xCFED_5524,
        0x8A34_536C,
        0x3E3F_A43B,
    ];
    static EXPECTED_BYTES_2: [u8; 64] = [
        0x54, 0x5B, 0xE0, 0x35, 0xA1, 0x7C, 0x62, 0x4C, 0xE2, 0x43, 0xA0, 0x33, 0x67, 0x3C, 0x11,
        0xB6, 0x99, 0x96, 0xAB, 0x7C, 0xEB, 0xEF, 0x52, 0x4A, 0x7F, 0x79, 0x36, 0x59, 0x9F, 0x2B,
        0x9E, 0x13, 0xB1, 0x3D, 0xDF, 0xC7, 0x17, 0x17, 0xCE, 0x61, 0x44, 0xE3, 0x81, 0x55, 0xEA,
        0x16, 0x30, 0xBC, 0x81, 0xD3, 0xBF, 0xA6, 0x24, 0x55, 0xED, 0xCF, 0x6C, 0x53, 0x34, 0x8A,
        0x3B, 0xA4, 0x3F, 0x3E,
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
            let mut rng = Sfc32::new(u32::default(), 0x89AB_CDEF, 0x0123_4567, None);
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
                0x5146_76C3,
                0x08A8_09DF,
                0x3034_9D2B,
                0xFB52_C520,
                0x3880_2BE1,
                0x9482_79E6,
                0xEC4B_F1D9,
                0x7CB0_A909,
                0xFAD8_B4A8,
                0x3CA4_B808,
                0x3821_B4C5,
                0x5E70_23CA,
                0x50F2_6BF7,
                0xF1E1_B0A2,
                0x6163_032F,
                0x3BF3_C9A4,
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
            // ./RNG_output sfc32 64 0x123456789ABCDEF | xxd -i
            // ```
            let expected = [
                0x8471_2D97,
                0xF5A3_D9C8,
                0x5CD0_A295,
                0x35E0_5B54,
                0x4C62_7CA1,
                0x33A0_43E2,
                0xB611_3C67,
                0x7CAB_9699,
                0x4A52_EFEB,
                0x5936_797F,
                0x139E_2B9F,
                0xC7DF_3DB1,
                0x61CE_1717,
                0x5581_E344,
                0xBC30_16EA,
                0xA6BF_D381,
            ];

            let mut rng = Sfc32::new_u64(0x0123_4567_89AB_CDEF, None);
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
                0x03B8_0BB8,
                0xA87D_BC7E,
                0x1787_178C,
                0x4C7B_7234,
                0xC65D_ADE2,
                0x2C69_2349,
                0xF52C_2153,
                0xDF09_8072,
                0x9D49_B03C,
                0x9562_381A,
                0xC9B4_1738,
                0x64B7_5E54,
                0x36CE_9B32,
                0xF106_947E,
                0x0AFC_726B,
                0x549B_BC87,
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
