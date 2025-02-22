// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

use rand_sfc::{
    Sfc32,
    rand_core::{RngCore, SeedableRng},
};
use test::Bencher;

#[bench]
fn next_u32(b: &mut Bencher) {
    let mut rng = Sfc32::from_os_rng();
    b.iter(|| rng.next_u32());
}

#[bench]
fn next_u64(b: &mut Bencher) {
    let mut rng = Sfc32::from_os_rng();
    b.iter(|| rng.next_u64());
}

#[bench]
fn fill_bytes(b: &mut Bencher) {
    let mut rng = Sfc32::from_os_rng();
    let mut dst = [u8::default(); 64];
    b.iter(|| rng.fill_bytes(&mut dst));
}
