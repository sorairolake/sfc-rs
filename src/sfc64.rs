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
/// assert_eq!(rng.next_u64(), 0xdb90_9c81_8901_599d);
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
    /// assert_eq!(rng.next_u64(), 0xdb90_9c81_8901_599d);
    /// ```
    #[must_use]
    #[inline]
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
    /// assert_eq!(rng.next_u64(), 0x3acf_a029_e3cc_6041);
    /// ```
    #[must_use]
    #[inline]
    pub fn new_u64(seed: u64, rounds: Option<u64>) -> Self {
        let (a, b, c) = (seed, seed, seed);
        let rounds = rounds.or(Some(12));
        Self::new(a, b, c, rounds)
    }
}

impl RngCore for Sfc64 {
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        const ROTATION: u32 = 24;
        const RIGHT_SHIFT: u32 = 11;
        const LEFT_SHIFT: u32 = 3;

        let tmp = self.a.wrapping_add(self.b).wrapping_add(self.counter);
        self.a = self.b ^ (self.b >> RIGHT_SHIFT);
        self.b = self.c.wrapping_add(self.c << LEFT_SHIFT);
        self.c = self.c.rotate_left(ROTATION).wrapping_add(tmp);
        self.counter = self.counter.wrapping_add(1);
        tmp
    }

    #[inline]
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        impls::fill_bytes_via_next(self, dst);
    }
}

impl SeedableRng for Sfc64 {
    type Seed = [u8; 24];

    #[inline]
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
        0xdb90_9c81_8901_599d,
        0x8ffd_1953_6521_6f57,
        0xe8c4_ad5e_258a_c04a,
        0x8f8e_f2c8_9fdb_63ca,
        0xf986_5b01_d98d_8e2f,
        0x4655_5871_a65d_08ba,
        0x6686_8677_c629_8fcd,
        0x2ce1_5a7e_6329_f57d,
        0x0b2f_1833_ca91_ca79,
        0x4b08_90ac_9bf4_53ca,
        0xd128_9e0d_ecd4_f85c,
        0x39ad_57d6_e346_b912,
        0x98d1_7cc0_0f53_0bda,
        0x13ac_08e9_8d77_d759,
        0xcb06_088a_9b16_64a3,
        0x6a00_df8e_97a8_3fa5,
    ];
    static EXPECTED_BYTES_1: [u8; 128] = [
        0x9d, 0x59, 0x01, 0x89, 0x81, 0x9c, 0x90, 0xdb, 0x57, 0x6f, 0x21, 0x65, 0x53, 0x19, 0xfd,
        0x8f, 0x4a, 0xc0, 0x8a, 0x25, 0x5e, 0xad, 0xc4, 0xe8, 0xca, 0x63, 0xdb, 0x9f, 0xc8, 0xf2,
        0x8e, 0x8f, 0x2f, 0x8e, 0x8d, 0xd9, 0x01, 0x5b, 0x86, 0xf9, 0xba, 0x08, 0x5d, 0xa6, 0x71,
        0x58, 0x55, 0x46, 0xcd, 0x8f, 0x29, 0xc6, 0x77, 0x86, 0x86, 0x66, 0x7d, 0xf5, 0x29, 0x63,
        0x7e, 0x5a, 0xe1, 0x2c, 0x79, 0xca, 0x91, 0xca, 0x33, 0x18, 0x2f, 0x0b, 0xca, 0x53, 0xf4,
        0x9b, 0xac, 0x90, 0x08, 0x4b, 0x5c, 0xf8, 0xd4, 0xec, 0x0d, 0x9e, 0x28, 0xd1, 0x12, 0xb9,
        0x46, 0xe3, 0xd6, 0x57, 0xad, 0x39, 0xda, 0x0b, 0x53, 0x0f, 0xc0, 0x7c, 0xd1, 0x98, 0x59,
        0xd7, 0x77, 0x8d, 0xe9, 0x08, 0xac, 0x13, 0xa3, 0x64, 0x16, 0x9b, 0x8a, 0x08, 0x06, 0xcb,
        0xa5, 0x3f, 0xa8, 0x97, 0x8e, 0xdf, 0x00, 0x6a,
    ];

    static SEED_2: [u8; 24] = [
        0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23,
        0x01, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01,
    ];
    static EXPECTED_2: [u64; 16] = [
        0x6de8_9713_e3e2_d060,
        0x7afc_0f39_1d6a_354d,
        0x7827_56a8_00e1_098f,
        0xb91a_7d8c_96ea_b14d,
        0x6129_e834_d58d_bb5a,
        0xf7ee_d803_09ca_4e5c,
        0x1b7c_74c2_b415_d0bd,
        0xba25_e41a_6615_5825,
        0x5249_b3b7_0925_f9f3,
        0x4cd8_cc40_a71d_92b5,
        0x73b9_0774_f4ed_d216,
        0xb8e8_17f6_3727_81fe,
        0x470d_f4fc_f363_f4b2,
        0x9537_2cb0_387b_c9c5,
        0x4f3f_01b5_41b1_4300,
        0x2edf_770d_f7dd_d1c5,
    ];
    static EXPECTED_BYTES_2: [u8; 128] = [
        0x60, 0xd0, 0xe2, 0xe3, 0x13, 0x97, 0xe8, 0x6d, 0x4d, 0x35, 0x6a, 0x1d, 0x39, 0x0f, 0xfc,
        0x7a, 0x8f, 0x09, 0xe1, 0x00, 0xa8, 0x56, 0x27, 0x78, 0x4d, 0xb1, 0xea, 0x96, 0x8c, 0x7d,
        0x1a, 0xb9, 0x5a, 0xbb, 0x8d, 0xd5, 0x34, 0xe8, 0x29, 0x61, 0x5c, 0x4e, 0xca, 0x09, 0x03,
        0xd8, 0xee, 0xf7, 0xbd, 0xd0, 0x15, 0xb4, 0xc2, 0x74, 0x7c, 0x1b, 0x25, 0x58, 0x15, 0x66,
        0x1a, 0xe4, 0x25, 0xba, 0xf3, 0xf9, 0x25, 0x09, 0xb7, 0xb3, 0x49, 0x52, 0xb5, 0x92, 0x1d,
        0xa7, 0x40, 0xcc, 0xd8, 0x4c, 0x16, 0xd2, 0xed, 0xf4, 0x74, 0x07, 0xb9, 0x73, 0xfe, 0x81,
        0x27, 0x37, 0xf6, 0x17, 0xe8, 0xb8, 0xb2, 0xf4, 0x63, 0xf3, 0xfc, 0xf4, 0x0d, 0x47, 0xc5,
        0xc9, 0x7b, 0x38, 0xb0, 0x2c, 0x37, 0x95, 0x00, 0x43, 0xb1, 0x41, 0xb5, 0x01, 0x3f, 0x4f,
        0xc5, 0xd1, 0xdd, 0xf7, 0x0d, 0x77, 0xdf, 0x2e,
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
                0x0123_4567_89ab_cdef,
                0x0123_4567_89ab_cdef,
                0x0123_4567_89ab_cdef,
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
                0x3acf_a029_e3cc_6041,
                0xf5b6_515b_f2ee_419c,
                0x1259_6358_94a2_9b61,
                0x0b6a_e753_95f8_ebd6,
                0x2256_2228_5ce3_02e2,
                0x520d_2861_1395_cb21,
                0xdb90_9c81_8901_599d,
                0x8ffd_1953_6521_6f57,
                0xe8c4_ad5e_258a_c04a,
                0x8f8e_f2c8_9fdb_63ca,
                0xf986_5b01_d98d_8e2f,
                0x4655_5871_a65d_08ba,
                0x6686_8677_c629_8fcd,
                0x2ce1_5a7e_6329_f57d,
                0x0b2f_1833_ca91_ca79,
                0x4b08_90ac_9bf4_53ca,
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
            // ./RNG_output sfc64 128 0x123456789abcdef | xxd -i
            // ```
            let expected = [
                0x79d7_8afb_e043_8f43,
                0x9633_06cd_3e6e_830e,
                0x983b_2a24_d126_ef1b,
                0x7d89_3205_05df_8c58,
                0x5542_a718_fe8e_d209,
                0x17c3_8abf_86a2_c189,
                0x6de8_9713_e3e2_d060,
                0x7afc_0f39_1d6a_354d,
                0x7827_56a8_00e1_098f,
                0xb91a_7d8c_96ea_b14d,
                0x6129_e834_d58d_bb5a,
                0xf7ee_d803_09ca_4e5c,
                0x1b7c_74c2_b415_d0bd,
                0xba25_e41a_6615_5825,
                0x5249_b3b7_0925_f9f3,
                0x4cd8_cc40_a71d_92b5,
            ];

            let mut rng = Sfc64::new_u64(0x0123_4567_89ab_cdef, None);
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
                0xad6f_dc72_9fee_f3c1,
                0x2a20_433d_733f_77d5,
                0x0310_e213_6964_7420,
                0x331a_176b_c71d_cabc,
                0x5311_8f35_c249_4d94,
                0xa3a9_9de7_e77e_16bf,
                0xa7b1_b70a_3e59_a1ff,
                0x8e11_27b2_8667_eb3c,
                0x3fc5_89dc_124c_f6e8,
                0x81e0_eaaa_ceb8_1d81,
                0x79f5_3465_2d26_2df6,
                0x87f7_0c82_14e1_86c5,
                0x67af_9c00_7b82_5917,
                0x5134_aec9_998d_8629,
                0x205a_a249_9406_8634,
                0x1c76_2918_dba3_e139,
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
