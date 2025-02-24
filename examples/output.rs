// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of generating random bytes. The result will be output to standard
//! output.

use std::io::{self, Write};

use anyhow::Context;
use clap::{Parser, ValueEnum};
use rand_sfc::{Sfc32, Sfc64, rand_core::RngCore};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// Random number generator to use.
    #[arg(value_enum, ignore_case(true))]
    rng: Rng,

    /// Number of bytes to output.
    bytes: usize,

    /// Random seed to use.
    #[arg(default_value_t)]
    seed: u64,
}

#[derive(Clone, Debug, ValueEnum)]
enum Rng {
    /// sfc32.
    Sfc32,

    /// sfc64.
    Sfc64,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    let mut buf = vec![u8::default(); opt.bytes];
    match opt.rng {
        Rng::Sfc32 => {
            let mut rng = Sfc32::new_u64(opt.seed);
            rng.fill_bytes(&mut buf);
        }
        Rng::Sfc64 => {
            let mut rng = Sfc64::new_u64(opt.seed);
            rng.fill_bytes(&mut buf);
        }
    }

    io::stdout()
        .write_all(&buf)
        .context("could not write random bytes to standard output")
}
