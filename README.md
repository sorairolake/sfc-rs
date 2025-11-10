<!--
SPDX-FileCopyrightText: 2025 Shun Sakai

SPDX-License-Identifier: CC-BY-4.0
-->

# sfc-rs

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
![MSRV][msrv-badge]
[![Docs][docs-badge]][docs-url]
![License][license-badge]

**sfc-rs** ([`sfc-prng`][version-url]) is an implementation of
[Chris Doty-Humphrey's Small Fast Counting PRNGs] written in pure [Rust].

This crate provides:

- [ ] sfc16
- [x] sfc32
- [x] sfc64

This crate supports version 4 of the SFC algorithms.

> [!IMPORTANT]
> This crate is not part of the [Rand project].

## Usage

Run the following command in your project directory:

```sh
cargo add sfc-prng
```

### Crate features

#### `serde`

Enables the [`serde`] crate.

### `no_std` support

This crate supports `no_std` mode.

### Documentation

See the [documentation][docs-url] for more details.

## Minimum supported Rust version

The minimum supported Rust version (MSRV) of this library is v1.85.0.

## Source code

The upstream repository is available at
<https://github.com/sorairolake/sfc-rs.git>.

## Changelog

Please see [CHANGELOG.adoc].

## Contributing

Please see [CONTRIBUTING.adoc].

## Acknowledgment

This crate depends on the [`rand_core`] crate created by the [Rand project].

The implementation of this crate is based on the reference implementation
provided by [PractRand] version pre0.95, which is licensed under the
[public domain].

## License

Copyright (C) 2025 Shun Sakai (see [AUTHORS.adoc])

This library is distributed under the terms of either the _Apache License 2.0_
or the _MIT License_.

This project is compliant with version 3.3 of the [_REUSE Specification_]. See
copyright notices of individual files for more details on copyright and
licensing information.

[ci-badge]: https://img.shields.io/github/actions/workflow/status/sorairolake/sfc-rs/CI.yaml?branch=develop&style=for-the-badge&logo=github&label=CI
[ci-url]: https://github.com/sorairolake/sfc-rs/actions?query=branch%3Adevelop+workflow%3ACI++
[version-badge]: https://img.shields.io/crates/v/sfc-prng?style=for-the-badge&logo=rust
[version-url]: https://crates.io/crates/sfc-prng
[msrv-badge]: https://img.shields.io/crates/msrv/sfc-prng?style=for-the-badge&logo=rust
[docs-badge]: https://img.shields.io/docsrs/sfc-prng?style=for-the-badge&logo=docsdotrs&label=Docs.rs
[docs-url]: https://docs.rs/sfc-prng
[license-badge]: https://img.shields.io/crates/l/sfc-prng?style=for-the-badge
[Chris Doty-Humphrey's Small Fast Counting PRNGs]: https://pracrand.sourceforge.net/RNG_engines.txt
[Rust]: https://www.rust-lang.org/
[Rand project]: https://github.com/rust-random/rand
[`serde`]: https://serde.rs/
[CHANGELOG.adoc]: CHANGELOG.adoc
[CONTRIBUTING.adoc]: CONTRIBUTING.adoc
[`rand_core`]: https://crates.io/crates/rand_core
[PractRand]: https://pracrand.sourceforge.net/
[public domain]: https://pracrand.sourceforge.net/license.txt
[AUTHORS.adoc]: AUTHORS.adoc
[_REUSE Specification_]: https://reuse.software/spec-3.3/
