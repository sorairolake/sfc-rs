// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `rand_sfc` crate is an implementation of [Chris Doty-Humphrey's Small
//! Fast Chaotic PRNGs].
//!
//! The SFC algorithms are not suitable for cryptographic uses but are very
//! fast.
//!
//! This crate supports version 4 of the SFC algorithms.
//!
//! [Chris Doty-Humphrey's Small Fast Chaotic PRNGs]: https://pracrand.sourceforge.net/RNG_engines.txt

#![doc(html_root_url = "https://docs.rs/rand_sfc/0.1.0/")]
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
