# `d1-pac`

[![crates.io](https://img.shields.io/crates/v/d1-pac.svg)](https://crates.io/crates/d1-pac)
[![Continuous integration](https://github.com/duskmoon314/d1-pac/actions/workflows/ci.yaml/badge.svg)](https://github.com/duskmoon314/d1-pac/actions/workflows/ci.yaml)

> Peripheral access API for Allwinner D1 SoC generated from unofficial SVD file

This project is currently developed and maintained by [duskmoon (Campbell He)](https://github.com/duskmoon314).

## Introduction

D1 is an SoC developed and sold by Allwinner. There is another SoC named D1S (also called F133) which is a "D1" with a lack of some features.

This crate provides an unofficial version CMSIS-SVD file of D1 SoC and a Rust crate generated via `svd2rust`. Most features should be available on D1S.

> Actually only tested on D1S so far

Most peripherals only provide the address of registers, a few peripherals add the contents of each field of registers. For more details, please refer to the official user manual and datasheet provided by Allwinner.

> It might be hard to find the user manual.
>
> [D1 User Manual v0.1 provided by RVBoards](https://www.rvboards.org/forum/cn/assets/uploads/files/1620442756342-d1_user_manual_v0.1-draft-version.pdf) and [F133 User Manual v1.0 provided by Mangopi](https://mangopi.org.cn/_media/f133_user_manual_v1.0.pdf) can be used as reference
>
> `d1-pac` is now developed according to `D1 User Manual v0.1`

I have now added the descriptions of most of the peripherals to the SVD file. If you find the descriptions are wrong or poorly named in use, **please feel free to submit an Issue or Pull Request to improve this crate**.

## [Documentation](https://docs.rs/crate/d1-pac)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
