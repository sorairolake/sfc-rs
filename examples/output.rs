// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of generating random bytes. The result will be output to standard
//! output.

use std::{
    fmt,
    io::{self, Write},
    num::ParseIntError,
    ops::Deref,
    str::FromStr,
};

use anyhow::Context;
use byte_unit::Byte;
use clap::{Parser, ValueEnum};
use rand_sfc::{Sfc32, Sfc64, rand_core::RngCore};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// Random number generator to use.
    #[arg(value_enum, ignore_case(true))]
    rng: Rng,

    /// Number of bytes to output.
    ///
    /// For the value which can be specified for <BYTES>, see <https://docs.rs/byte-unit>.
    bytes: Byte,

    /// Random seed to use.
    ///
    /// If [SEED] starts with "0x", it is considered to be hexadecimal,
    /// otherwise it is considered to be decimal.
    #[arg(default_value_t)]
    seed: Seed,
}

#[derive(Clone, Debug, ValueEnum)]
enum Rng {
    /// sfc32.
    Sfc32,

    /// sfc64.
    Sfc64,
}

#[derive(Clone, Debug, Default)]
struct Seed(u64);

impl Deref for Seed {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Seed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Seed {
    type Err = ParseIntError;

    fn from_str(seed: &str) -> Result<Self, ParseIntError> {
        if seed.starts_with("0x") {
            u64::from_str_radix(seed.trim_start_matches("0x"), 16)
        } else {
            seed.parse()
        }
        .map(Self)
    }
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    let bytes = opt.bytes.try_into()?;
    let mut buf = vec![u8::default(); bytes];
    match opt.rng {
        Rng::Sfc32 => {
            let mut rng = Sfc32::new_u64(*opt.seed);
            rng.fill_bytes(&mut buf);
        }
        Rng::Sfc64 => {
            let mut rng = Sfc64::new_u64(*opt.seed);
            rng.fill_bytes(&mut buf);
        }
    }

    io::stdout()
        .write_all(&buf)
        .context("could not write random bytes to standard output")
}
