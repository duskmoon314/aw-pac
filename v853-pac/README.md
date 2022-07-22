# `v853-pac`

[![crates.io](https://img.shields.io/crates/v/v853-pac.svg)](https://crates.io/crates/v853-pac)

> Peripheral access API for Allwinner V853 SoC generated from unofficial SVD file

This project is currently developed and maintained by [duskmoon (Campbell He)](https://github.com/duskmoon314).

## Introduction

V853 is an SoC developed and sold by Allwinner. There is an ARM Cortex-A7 core, a RISC-V E907 core and an NPU.

This crate provides an unofficial version CMSIS-SVD file of V853 SoC and a Rust crate generated via `svd2rust`. This crate now only concerns Cortex-A7 target due to the limitation of `svd2rust`.

Most peripherals only provide the address of registers, a few peripherals add the contents of each field of registers. For more details, please refer to the official user manual and datasheet provided by Allwinner.

## [Documentation](https://docs.rs/crate/v853-pac)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
