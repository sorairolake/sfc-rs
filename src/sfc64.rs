// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An implementation of the sfc64 random number generator.

use rand_core::{RngCore, SeedableRng, impls, le};

/// A sfc64 random number generator.
///
/// The sfc64 algorithm is not suitable for cryptographic uses but is very fast.
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
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Sfc64 {
    a: u64,
    b: u64,
    c: u64,
    counter: u64,
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
        let mut state = Self {
            a: s[0],
            b: s[1],
            c: s[2],
            counter: 1,
        };
        for _ in 0..12 {
            state.next_u64();
        }
        state
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
    // ./RNG_output sfc64 128 0 | xxd -i
    // ```
    const EXPECTED: [u64; 16] = [
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
    const EXPECTED_BYTES: [u8; 128] = [
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

    #[test]
    fn clone() {
        let rng = Sfc64::from_seed([u8::default(); 24]);
        assert_eq!(rng.clone(), rng);
    }

    #[test]
    fn debug() {
        let rng = Sfc64::from_seed([u8::default(); 24]);
        assert_eq!(
            format!("{rng:?}"),
            "Sfc64 { a: 3105171942637071872, b: 1132609933517779508, c: 3891116077132813732, counter: 13 }"
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            Sfc64::from_seed([u8::default(); 24]),
            Sfc64::from_seed([u8::default(); 24])
        );
        assert_ne!(
            Sfc64::from_seed([u8::default(); 24]),
            Sfc64::from_seed([u8::MAX; 24])
        );
    }

    #[test]
    fn next_u32() {
        let mut rng = Sfc64::from_seed([u8::default(); 24]);
        for e in EXPECTED {
            assert_eq!(rng.next_u32(), e as u32);
        }
    }

    #[test]
    fn next_u64() {
        let mut rng = Sfc64::from_seed([u8::default(); 24]);
        for e in EXPECTED {
            assert_eq!(rng.next_u64(), e);
        }
    }

    #[test]
    fn fill_bytes() {
        let mut rng = Sfc64::from_seed([u8::default(); 24]);
        let mut dst = [u8::default(); 128];
        rng.fill_bytes(&mut dst);
        assert_eq!(dst, EXPECTED_BYTES);
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
        let mut rng = Sfc64::from_seed([u8::default(); 24]);

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
