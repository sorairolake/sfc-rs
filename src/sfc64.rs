// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An implementation of the sfc64 random number generator.

use rand_core::{RngCore, SeedableRng, impls, le};

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
/// # use rand_sfc::{
/// #     Sfc64,
/// #     rand_core::{RngCore, SeedableRng},
/// # };
/// #
/// let mut rng = Sfc64::from_seed([0; 24]);
/// assert_eq!(rng.next_u64(), 0x3acf_a029_e3cc_6041);
/// ```
///
/// [PractRand]: https://pracrand.sourceforge.net/
/// [public domain]: https://pracrand.sourceforge.net/license.txt
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Sfc64 {
    a: u64,
    b: u64,
    c: u64,
    counter: u64,
}

impl Sfc64 {
    /// Creates a new `Sfc64` using the given seeds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rand_sfc::{Sfc64, rand_core::RngCore};
    /// #
    /// let mut rng = Sfc64::new(0, 0, 0);
    /// assert_eq!(rng.next_u64(), 0x3acf_a029_e3cc_6041);
    /// ```
    #[must_use]
    #[inline]
    pub fn new(a: u64, b: u64, c: u64) -> Self {
        let mut state = Self {
            a,
            b,
            c,
            counter: 1,
        };
        for _ in 0..12 {
            state.next_u64();
        }
        state
    }

    /// Creates a new `Sfc64` using a [`u64`] seed.
    ///
    /// This method is equivalent to providing `seed` for all parameters of
    /// [`Sfc64::new`].
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
    /// # use rand_sfc::{Sfc64, rand_core::RngCore};
    /// #
    /// let mut rng = Sfc64::new_u64(0);
    /// assert_eq!(rng.next_u64(), 0x3acf_a029_e3cc_6041);
    /// ```
    #[must_use]
    #[inline]
    pub fn new_u64(seed: u64) -> Self {
        Self::new(seed, seed, seed)
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
        self.counter += 1;
        self.a = self.b ^ (self.b >> RIGHT_SHIFT);
        self.b = self.c.wrapping_add(self.c << LEFT_SHIFT);
        self.c = self.c.rotate_left(ROTATION).wrapping_add(tmp);
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
    // ./RNG_output sfc64 128 0x0 | xxd -i
    // ```
    const EXPECTED_1: [u64; 16] = [
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
    const EXPECTED_BYTES_1: [u8; 128] = [
        0x41, 0x60, 0xcc, 0xe3, 0x29, 0xa0, 0xcf, 0x3a, 0x9c, 0x41, 0xee, 0xf2, 0x5b, 0x51, 0xb6,
        0xf5, 0x61, 0x9b, 0xa2, 0x94, 0x58, 0x63, 0x59, 0x12, 0xd6, 0xeb, 0xf8, 0x95, 0x53, 0xe7,
        0x6a, 0x0b, 0xe2, 0x02, 0xe3, 0x5c, 0x28, 0x22, 0x56, 0x22, 0x21, 0xcb, 0x95, 0x13, 0x61,
        0x28, 0x0d, 0x52, 0x9d, 0x59, 0x01, 0x89, 0x81, 0x9c, 0x90, 0xdb, 0x57, 0x6f, 0x21, 0x65,
        0x53, 0x19, 0xfd, 0x8f, 0x4a, 0xc0, 0x8a, 0x25, 0x5e, 0xad, 0xc4, 0xe8, 0xca, 0x63, 0xdb,
        0x9f, 0xc8, 0xf2, 0x8e, 0x8f, 0x2f, 0x8e, 0x8d, 0xd9, 0x01, 0x5b, 0x86, 0xf9, 0xba, 0x08,
        0x5d, 0xa6, 0x71, 0x58, 0x55, 0x46, 0xcd, 0x8f, 0x29, 0xc6, 0x77, 0x86, 0x86, 0x66, 0x7d,
        0xf5, 0x29, 0x63, 0x7e, 0x5a, 0xe1, 0x2c, 0x79, 0xca, 0x91, 0xca, 0x33, 0x18, 0x2f, 0x0b,
        0xca, 0x53, 0xf4, 0x9b, 0xac, 0x90, 0x08, 0x4b,
    ];

    // This test vector was generated by the `RNG_output` command of PractRand
    // version pre0.95.
    //
    // To generate a hex dump:
    //
    // ```sh
    // ./RNG_output sfc64 128 0x123456789abcdef | xxd -i
    // ```
    const SEED_2: [u8; 24] = [
        0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23,
        0x01, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01,
    ];
    const EXPECTED_2: [u64; 16] = [
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
    const EXPECTED_BYTES_2: [u8; 128] = [
        0x43, 0x8f, 0x43, 0xe0, 0xfb, 0x8a, 0xd7, 0x79, 0x0e, 0x83, 0x6e, 0x3e, 0xcd, 0x06, 0x33,
        0x96, 0x1b, 0xef, 0x26, 0xd1, 0x24, 0x2a, 0x3b, 0x98, 0x58, 0x8c, 0xdf, 0x05, 0x05, 0x32,
        0x89, 0x7d, 0x09, 0xd2, 0x8e, 0xfe, 0x18, 0xa7, 0x42, 0x55, 0x89, 0xc1, 0xa2, 0x86, 0xbf,
        0x8a, 0xc3, 0x17, 0x60, 0xd0, 0xe2, 0xe3, 0x13, 0x97, 0xe8, 0x6d, 0x4d, 0x35, 0x6a, 0x1d,
        0x39, 0x0f, 0xfc, 0x7a, 0x8f, 0x09, 0xe1, 0x00, 0xa8, 0x56, 0x27, 0x78, 0x4d, 0xb1, 0xea,
        0x96, 0x8c, 0x7d, 0x1a, 0xb9, 0x5a, 0xbb, 0x8d, 0xd5, 0x34, 0xe8, 0x29, 0x61, 0x5c, 0x4e,
        0xca, 0x09, 0x03, 0xd8, 0xee, 0xf7, 0xbd, 0xd0, 0x15, 0xb4, 0xc2, 0x74, 0x7c, 0x1b, 0x25,
        0x58, 0x15, 0x66, 0x1a, 0xe4, 0x25, 0xba, 0xf3, 0xf9, 0x25, 0x09, 0xb7, 0xb3, 0x49, 0x52,
        0xb5, 0x92, 0x1d, 0xa7, 0x40, 0xcc, 0xd8, 0x4c,
    ];

    #[test]
    fn clone() {
        let rng = Sfc64::from_seed(Default::default());
        assert_eq!(rng.clone(), rng);
    }

    #[test]
    fn debug() {
        let rng = Sfc64::from_seed(Default::default());
        assert_eq!(
            format!("{rng:?}"),
            "Sfc64 { a: 3105171942637071872, b: 1132609933517779508, c: 3891116077132813732, counter: 13 }"
        );
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
            let mut rng = Sfc64::new(u64::default(), u64::default(), u64::default());
            for e in EXPECTED_1 {
                assert_eq!(rng.next_u64(), e);
            }
        }
        {
            let mut rng = Sfc64::new(
                0x0123_4567_89ab_cdef,
                0x0123_4567_89ab_cdef,
                0x0123_4567_89ab_cdef,
            );
            for e in EXPECTED_2 {
                assert_eq!(rng.next_u64(), e);
            }
        }
    }

    #[test]
    fn new_u64() {
        {
            let mut rng = Sfc64::new_u64(u64::default());
            for e in EXPECTED_1 {
                assert_eq!(rng.next_u64(), e);
            }
        }
        {
            let mut rng = Sfc64::new_u64(0x0123_4567_89ab_cdef);
            for e in EXPECTED_2 {
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
            r#"{"a":3105171942637071872,"b":1132609933517779508,"c":3891116077132813732,"counter":13}"#
        );

        let mut deserialized_rng = serde_json::from_str::<Sfc64>(&json).unwrap();
        assert_eq!(deserialized_rng, rng);
        assert_eq!(deserialized_rng.next_u64(), rng.next_u64());
    }
}
