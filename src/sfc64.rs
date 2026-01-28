// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An implementation of the sfc64 random number generator.

use rand_core::{RngCore, SeedableRng, impls, le};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A sfc64 random number generator.
///
/// The sfc64 algorithm is not suitable for cryptographic uses but is very fast.
/// This algorithm has a 256-bit state and outputs 64-bit random numbers. The
/// average period of this algorithm is approximately 2<sup>255</sup>, and the
/// minimum period is greater than or equal to 2<sup>64</sup>.
///
/// The algorithm used here is translated from the reference implementation
/// provided by [PractRand] version pre0.95, which is licensed under the [public
/// domain].
///
/// # Examples
///
/// ```
/// # use sfc_prng::{
/// #     Sfc64,
/// #     rand_core::{RngCore, SeedableRng},
/// # };
/// #
/// let mut rng = Sfc64::from_seed([0; 24]);
/// assert_eq!(rng.next_u64(), 0xDB90_9C81_8901_599D);
/// ```
///
/// [PractRand]: https://pracrand.sourceforge.net/
/// [public domain]: https://pracrand.sourceforge.net/license.txt
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Sfc64 {
    a: u64,
    b: u64,
    c: u64,
    counter: u64,
}

impl Sfc64 {
    /// Creates a new `Sfc64` using the given seeds.
    ///
    /// If `rounds` is [`None`], the state is mixed up 18 rounds during
    /// initialization.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sfc_prng::{Sfc64, rand_core::RngCore};
    /// #
    /// let mut rng = Sfc64::new(0, 0, 0, None);
    /// assert_eq!(rng.next_u64(), 0xDB90_9C81_8901_599D);
    /// ```
    #[must_use]
    pub fn new(a: u64, b: u64, c: u64, rounds: Option<u64>) -> Self {
        let mut state = Self {
            a,
            b,
            c,
            counter: 1,
        };
        let rounds = rounds.unwrap_or(18);
        for _ in 0..rounds {
            state.next_u64();
        }
        state
    }

    /// Creates a new `Sfc64` using a [`u64`] seed.
    ///
    /// If `rounds` is [`None`], the state is mixed up 12 rounds during
    /// initialization.
    ///
    /// <div class="warning">
    ///
    /// Note that the result of this method is different from the result of
    /// [`Sfc64::seed_from_u64`].
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// # use sfc_prng::{Sfc64, rand_core::RngCore};
    /// #
    /// let mut rng = Sfc64::new_u64(0, None);
    /// assert_eq!(rng.next_u64(), 0x3ACF_A029_E3CC_6041);
    /// ```
    #[must_use]
    pub fn new_u64(seed: u64, rounds: Option<u64>) -> Self {
        let (a, b, c) = (seed, seed, seed);
        let rounds = rounds.or(Some(12));
        Self::new(a, b, c, rounds)
    }
}

impl RngCore for Sfc64 {
    #[allow(clippy::cast_possible_truncation)]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        const ROTATION: u32 = 24;
        const RIGHT_SHIFT: u32 = 11;
        const LEFT_SHIFT: u32 = 3;

        let tmp = self.a.wrapping_add(self.b).wrapping_add(self.counter);
        self.counter = self.counter.wrapping_add(1);
        self.a = self.b ^ (self.b >> RIGHT_SHIFT);
        self.b = self.c.wrapping_add(self.c << LEFT_SHIFT);
        self.c = self.c.rotate_left(ROTATION).wrapping_add(tmp);
        tmp
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        impls::fill_bytes_via_next(self, dst);
    }
}

impl SeedableRng for Sfc64 {
    type Seed = [u8; 24];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut s = [u64::default(); 3];
        le::read_u64_into(&seed, &mut s);
        Self::new(s[0], s[1], s[2], None)
    }
}

#[cfg(test)]
mod tests {
    use core::{any, mem};

    use super::*;

    static EXPECTED_1: [u64; 16] = [
        0xDB90_9C81_8901_599D,
        0x8FFD_1953_6521_6F57,
        0xE8C4_AD5E_258A_C04A,
        0x8F8E_F2C8_9FDB_63CA,
        0xF986_5B01_D98D_8E2F,
        0x4655_5871_A65D_08BA,
        0x6686_8677_C629_8FCD,
        0x2CE1_5A7E_6329_F57D,
        0x0B2F_1833_CA91_CA79,
        0x4B08_90AC_9BF4_53CA,
        0xD128_9E0D_ECD4_F85C,
        0x39AD_57D6_E346_B912,
        0x98D1_7CC0_0F53_0BDA,
        0x13AC_08E9_8D77_D759,
        0xCB06_088A_9B16_64A3,
        0x6A00_DF8E_97A8_3FA5,
    ];
    static EXPECTED_BYTES_1: [u8; 128] = [
        0x9D, 0x59, 0x01, 0x89, 0x81, 0x9C, 0x90, 0xDB, 0x57, 0x6F, 0x21, 0x65, 0x53, 0x19, 0xFD,
        0x8F, 0x4A, 0xC0, 0x8A, 0x25, 0x5E, 0xAD, 0xC4, 0xE8, 0xCA, 0x63, 0xDB, 0x9F, 0xC8, 0xF2,
        0x8E, 0x8F, 0x2F, 0x8E, 0x8D, 0xD9, 0x01, 0x5B, 0x86, 0xF9, 0xBA, 0x08, 0x5D, 0xA6, 0x71,
        0x58, 0x55, 0x46, 0xCD, 0x8F, 0x29, 0xC6, 0x77, 0x86, 0x86, 0x66, 0x7D, 0xF5, 0x29, 0x63,
        0x7E, 0x5A, 0xE1, 0x2C, 0x79, 0xCA, 0x91, 0xCA, 0x33, 0x18, 0x2F, 0x0B, 0xCA, 0x53, 0xF4,
        0x9B, 0xAC, 0x90, 0x08, 0x4B, 0x5C, 0xF8, 0xD4, 0xEC, 0x0D, 0x9E, 0x28, 0xD1, 0x12, 0xB9,
        0x46, 0xE3, 0xD6, 0x57, 0xAD, 0x39, 0xDA, 0x0B, 0x53, 0x0F, 0xC0, 0x7C, 0xD1, 0x98, 0x59,
        0xD7, 0x77, 0x8D, 0xE9, 0x08, 0xAC, 0x13, 0xA3, 0x64, 0x16, 0x9B, 0x8A, 0x08, 0x06, 0xCB,
        0xA5, 0x3F, 0xA8, 0x97, 0x8E, 0xDF, 0x00, 0x6A,
    ];

    const SEED_2: [u8; 24] = [
        0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01, 0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23,
        0x01, 0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01,
    ];
    static EXPECTED_2: [u64; 16] = [
        0x6DE8_9713_E3E2_D060,
        0x7AFC_0F39_1D6A_354D,
        0x7827_56A8_00E1_098F,
        0xB91A_7D8C_96EA_B14D,
        0x6129_E834_D58D_BB5A,
        0xF7EE_D803_09CA_4E5C,
        0x1B7C_74C2_B415_D0BD,
        0xBA25_E41A_6615_5825,
        0x5249_B3B7_0925_F9F3,
        0x4CD8_CC40_A71D_92B5,
        0x73B9_0774_F4ED_D216,
        0xB8E8_17F6_3727_81FE,
        0x470D_F4FC_F363_F4B2,
        0x9537_2CB0_387B_C9C5,
        0x4F3F_01B5_41B1_4300,
        0x2EDF_770D_F7DD_D1C5,
    ];
    static EXPECTED_BYTES_2: [u8; 128] = [
        0x60, 0xD0, 0xE2, 0xE3, 0x13, 0x97, 0xE8, 0x6D, 0x4D, 0x35, 0x6A, 0x1D, 0x39, 0x0F, 0xFC,
        0x7A, 0x8F, 0x09, 0xE1, 0x00, 0xA8, 0x56, 0x27, 0x78, 0x4D, 0xB1, 0xEA, 0x96, 0x8C, 0x7D,
        0x1A, 0xB9, 0x5A, 0xBB, 0x8D, 0xD5, 0x34, 0xE8, 0x29, 0x61, 0x5C, 0x4E, 0xCA, 0x09, 0x03,
        0xD8, 0xEE, 0xF7, 0xBD, 0xD0, 0x15, 0xB4, 0xC2, 0x74, 0x7C, 0x1B, 0x25, 0x58, 0x15, 0x66,
        0x1A, 0xE4, 0x25, 0xBA, 0xF3, 0xF9, 0x25, 0x09, 0xB7, 0xB3, 0x49, 0x52, 0xB5, 0x92, 0x1D,
        0xA7, 0x40, 0xCC, 0xD8, 0x4C, 0x16, 0xD2, 0xED, 0xF4, 0x74, 0x07, 0xB9, 0x73, 0xFE, 0x81,
        0x27, 0x37, 0xF6, 0x17, 0xE8, 0xB8, 0xB2, 0xF4, 0x63, 0xF3, 0xFC, 0xF4, 0x0D, 0x47, 0xC5,
        0xC9, 0x7B, 0x38, 0xB0, 0x2C, 0x37, 0x95, 0x00, 0x43, 0xB1, 0x41, 0xB5, 0x01, 0x3F, 0x4F,
        0xC5, 0xD1, 0xDD, 0xF7, 0x0D, 0x77, 0xDF, 0x2E,
    ];

    #[test]
    fn clone() {
        let rng = Sfc64::from_seed(Default::default());
        assert_eq!(rng.clone(), rng);
    }

    #[test]
    fn debug() {
        {
            let rng = Sfc64::from_seed(Default::default());
            assert_eq!(
                format!("{rng:?}"),
                "Sfc64 { a: 1074220252016367073, b: 14747097319099466665, c: 17960713684764683274, counter: 19 }"
            );
        }
        {
            let rng = Sfc64::seed_from_u64(1);
            assert_eq!(
                format!("{rng:?}"),
                "Sfc64 { a: 18086042397347456770, b: 1455245525186175675, c: 11873715530299442944, counter: 19 }"
            );
        }
    }

    #[test]
    fn equality() {
        assert_eq!(
            Sfc64::from_seed(Default::default()),
            Sfc64::from_seed(Default::default())
        );
        assert_ne!(
            Sfc64::from_seed(Default::default()),
            Sfc64::from_seed([u8::MAX; 24])
        );
    }

    #[test]
    fn new() {
        {
            let mut rng = Sfc64::new(u64::default(), u64::default(), u64::default(), None);
            for e in EXPECTED_1 {
                assert_eq!(rng.next_u64(), e);
            }
        }
        {
            let mut rng = Sfc64::new(
                0x0123_4567_89AB_CDEF,
                0x0123_4567_89AB_CDEF,
                0x0123_4567_89AB_CDEF,
                None,
            );
            for e in EXPECTED_2 {
                assert_eq!(rng.next_u64(), e);
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
            // ./RNG_output sfc64 128 0x0 | xxd -i
            // ```
            let expected = [
                0x3ACF_A029_E3CC_6041,
                0xF5B6_515B_F2EE_419C,
                0x1259_6358_94A2_9B61,
                0x0B6A_E753_95F8_EBD6,
                0x2256_2228_5CE3_02E2,
                0x520D_2861_1395_CB21,
                0xDB90_9C81_8901_599D,
                0x8FFD_1953_6521_6F57,
                0xE8C4_AD5E_258A_C04A,
                0x8F8E_F2C8_9FDB_63CA,
                0xF986_5B01_D98D_8E2F,
                0x4655_5871_A65D_08BA,
                0x6686_8677_C629_8FCD,
                0x2CE1_5A7E_6329_F57D,
                0x0B2F_1833_CA91_CA79,
                0x4B08_90AC_9BF4_53CA,
            ];

            let mut rng = Sfc64::new_u64(u64::default(), None);
            for e in expected {
                assert_eq!(rng.next_u64(), e);
            }
        }
        {
            // This test vector was generated by the `RNG_output` command of PractRand
            // version pre0.95.
            //
            // To generate a hex dump:
            //
            // ```sh
            // ./RNG_output sfc64 128 0x123456789ABCDEF | xxd -i
            // ```
            let expected = [
                0x79D7_8AFB_E043_8F43,
                0x9633_06CD_3E6E_830E,
                0x983B_2A24_D126_EF1B,
                0x7D89_3205_05DF_8C58,
                0x5542_A718_FE8E_D209,
                0x17C3_8ABF_86A2_C189,
                0x6DE8_9713_E3E2_D060,
                0x7AFC_0F39_1D6A_354D,
                0x7827_56A8_00E1_098F,
                0xB91A_7D8C_96EA_B14D,
                0x6129_E834_D58D_BB5A,
                0xF7EE_D803_09CA_4E5C,
                0x1B7C_74C2_B415_D0BD,
                0xBA25_E41A_6615_5825,
                0x5249_B3B7_0925_F9F3,
                0x4CD8_CC40_A71D_92B5,
            ];

            let mut rng = Sfc64::new_u64(0x0123_4567_89AB_CDEF, None);
            for e in expected {
                assert_eq!(rng.next_u64(), e);
            }
        }
    }

    #[test]
    fn next_u32() {
        {
            let mut rng = Sfc64::from_seed(Default::default());
            for e in EXPECTED_1 {
                assert_eq!(rng.next_u32(), e as u32);
            }
        }
        {
            let mut rng = Sfc64::from_seed(SEED_2);
            for e in EXPECTED_2 {
                assert_eq!(rng.next_u32(), e as u32);
            }
        }
    }

    #[test]
    fn next_u64() {
        {
            let mut rng = Sfc64::from_seed(Default::default());
            for e in EXPECTED_1 {
                assert_eq!(rng.next_u64(), e);
            }
        }
        {
            let mut rng = Sfc64::from_seed(SEED_2);
            for e in EXPECTED_2 {
                assert_eq!(rng.next_u64(), e);
            }
        }
        {
            let seed = [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ];
            let expected = [
                0xAD6F_DC72_9FEE_F3C1,
                0x2A20_433D_733F_77D5,
                0x0310_E213_6964_7420,
                0x331A_176B_C71D_CABC,
                0x5311_8F35_C249_4D94,
                0xA3A9_9DE7_E77E_16BF,
                0xA7B1_B70A_3E59_A1FF,
                0x8E11_27B2_8667_EB3C,
                0x3FC5_89DC_124C_F6E8,
                0x81E0_EAAA_CEB8_1D81,
                0x79F5_3465_2D26_2DF6,
                0x87F7_0C82_14E1_86C5,
                0x67AF_9C00_7B82_5917,
                0x5134_AEC9_998D_8629,
                0x205A_A249_9406_8634,
                0x1C76_2918_DBA3_E139,
            ];

            let mut rng = Sfc64::from_seed(seed);
            for e in expected {
                assert_eq!(rng.next_u64(), e);
            }
        }
    }

    #[test]
    fn fill_bytes() {
        {
            let mut rng = Sfc64::from_seed(Default::default());
            let mut dst = [u8::default(); 128];
            rng.fill_bytes(&mut dst);
            assert_eq!(dst, EXPECTED_BYTES_1);
        }
        {
            let mut rng = Sfc64::from_seed(SEED_2);
            let mut dst = [u8::default(); 128];
            rng.fill_bytes(&mut dst);
            assert_eq!(dst, EXPECTED_BYTES_2);
        }
    }

    #[test]
    fn fill_bytes_per_chunk() {
        {
            let mut rng = Sfc64::from_seed(Default::default());
            let mut dst = [u8::default(); 8];
            for e in EXPECTED_BYTES_1.chunks_exact(dst.len()) {
                rng.fill_bytes(&mut dst);
                assert_eq!(dst, e);
            }
        }
        {
            let mut rng = Sfc64::from_seed(SEED_2);
            let mut dst = [u8::default(); 8];
            for e in EXPECTED_BYTES_2.chunks_exact(dst.len()) {
                rng.fill_bytes(&mut dst);
                assert_eq!(dst, e);
            }
        }
    }

    #[test]
    fn seed_type() {
        assert_eq!(
            any::type_name::<<Sfc64 as SeedableRng>::Seed>(),
            any::type_name::<[u8; 24]>()
        );
        assert_eq!(
            mem::size_of::<<Sfc64 as SeedableRng>::Seed>(),
            mem::size_of::<[u8; 24]>()
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde() {
        let mut rng = Sfc64::from_seed(Default::default());

        let json = serde_json::to_string(&rng).unwrap();
        assert_eq!(
            json,
            r#"{"a":1074220252016367073,"b":14747097319099466665,"c":17960713684764683274,"counter":19}"#
        );

        let mut deserialized_rng = serde_json::from_str::<Sfc64>(&json).unwrap();
        assert_eq!(deserialized_rng, rng);
        assert_eq!(deserialized_rng.next_u64(), rng.next_u64());
    }
}
