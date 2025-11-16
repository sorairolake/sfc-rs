// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

use sfc_prng::{
    Sfc32,
    rand_core::{RngCore, SeedableRng},
};
use test::Bencher;

#[bench]
fn equality(b: &mut Bencher) {
    b.iter(|| Sfc32::from_seed(Default::default()) == Sfc32::from_seed(Default::default()));
}

#[bench]
fn new(b: &mut Bencher) {
    b.iter(|| Sfc32::new(u32::default(), u32::default(), u32::default(), None));
}

#[bench]
fn new_u64(b: &mut Bencher) {
    b.iter(|| Sfc32::new_u64(u64::default(), None));
}

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
    let mut dst = [u8::default(); 4];
    b.iter(|| rng.fill_bytes(&mut dst));
}

#[bench]
fn from_seed(b: &mut Bencher) {
    b.iter(|| Sfc32::from_seed(Default::default()));
}
