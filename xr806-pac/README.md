# `xr806-pac`

[![crates.io](https://img.shields.io/crates/v/xr806-pac.svg)](https://crates.io/crates/xr806-pac)

> Peripheral access API for Allwinner XR806 SoC generated from unofficial SVD file

This project is currently developed and maintained by [duskmoon (Campbell He)](https://github.com/duskmoon314).

> **NOTICE:** UNTESTED YET

## Introduction

XR806 is an SoC developed and sold by Allwinner.

This crate provides an unofficial version CMSIS-SVD file of XR806 SoC and a Rust crate generated via `svd2rust`.

Most peripherals only provide the address of registers, a few peripherals add the contents of each field of registers. For more details, please refer to the official user manual and datasheet provided by Allwinner.

I have now added the descriptions of most of the peripherals to the SVD file. If you find the descriptions are wrong or poorly named in use, **please feel free to submit an Issue or Pull Request to improve this crate**.

## [Documentation](https://docs.rs/crate/xr806-pac)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
