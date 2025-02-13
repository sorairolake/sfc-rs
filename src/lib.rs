// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `rand_sfc` crate is an implementation of [Chris Doty-Humphrey's Small
//! Fast Chaotic PRNGs].
//!
//! [Chris Doty-Humphrey's Small Fast Chaotic PRNGs]: https://pracrand.sourceforge.net/RNG_engines.txt

#![doc(html_root_url = "https://docs.rs/rand_sfc/0.1.0/")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
// Lint levels of rustc.
#![deny(missing_docs)]

/// Computes `left + right`.
#[must_use]
pub const fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
