// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of generating a random number.

use clap::{Parser, ValueEnum};
use rand_sfc::{
    rand_core::{RngCore, SeedableRng},
    Sfc32, Sfc64,
};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// Random number generator to use.
    #[arg(
        short,
        long,
        value_enum,
        default_value_t,
        value_name("RNG"),
        ignore_case(true)
    )]
    rng: Rng,

    /// Random seed to use.
    ///
    /// If [SEED] is not specified, the RNG seeded via random data from system
    /// sources.
    seed: Option<u64>,
}

#[derive(Clone, Debug, Default, ValueEnum)]
enum Rng {
    /// sfc64.
    #[default]
    Sfc64,

    /// sfc32.
    Sfc32,
}

fn main() {
    let opt = Opt::parse();

    match opt.rng {
        Rng::Sfc64 => {
            let mut rng = opt
                .seed
                .map_or_else(Sfc64::from_os_rng, Sfc64::seed_from_u64);
            println!("{:#018x?}", rng.next_u64());
        }
        Rng::Sfc32 => {
            let mut rng = opt
                .seed
                .map_or_else(Sfc32::from_os_rng, Sfc32::seed_from_u64);
            println!("{:#010x?}", rng.next_u32());
        }
    }
}
