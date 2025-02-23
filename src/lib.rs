// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `rand_sfc` crate is an implementation of [Chris Doty-Humphrey's Small
//! Fast Counting PRNGs].
//!
//! The SFC algorithms are not suitable for cryptographic uses but are very
//! fast.
//!
//! This crate provides:
//!
//! - [ ] sfc16
//! - [x] sfc32
//! - [x] sfc64
//!
//! The sfc32 algorithm is implemented as [`Sfc32`], and the sfc64 algorithm is
//! implemented as [`Sfc64`].
//!
//! This crate supports version 4 of the SFC algorithms.
//!
//! # Examples
//!
//! ```
//! use rand_sfc::{
//!     Sfc64,
//!     rand_core::{RngCore, SeedableRng},
//! };
//!
//! let mut rng = Sfc64::seed_from_u64(0);
//! let x = rng.next_u64();
//! assert_eq!(x, 0x68ee_fb29_e2a4_f4ab);
//! ```
//!
//! [Chris Doty-Humphrey's Small Fast Counting PRNGs]: https://pracrand.sourceforge.net/RNG_engines.txt

#![doc(html_root_url = "https://docs.rs/rand_sfc/0.1.1/")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
// Lint levels of rustc.
#![deny(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate alloc;

mod sfc32;
mod sfc64;

pub use rand_core;

pub use crate::{sfc32::Sfc32, sfc64::Sfc64};
