# pac-xtask

A very simple xtask crate for pac crates, with a few useful commands.

> This crate may not fit your needs, please modify it to your needs.

## Add to your pac crate

I suggest reading [`matklad/cargo-xtask`](https://github.com/matklad/cargo-xtask) first to understand what xtask is and how it works.

### Prerequisites

#### Make the xtask crate a workspace member

First, you need to make the xtask crate a workspace member.

See [below](#add-xtask) for adding the xtask crate to your workspace.

##### Single pac crate

If you only have a single pac crate, you can just add the xtask crate to the workspace members in the root `Cargo.toml`:

```toml
# Your pac's Cargo.toml

[package]
name = "<your soc>-pac"
...

[workspace]
members = ["xtask"]
```

##### Multiple pac crates

If you have multiple pac crates in one workspace already, you can simply add the xtask crate to the workspace members in the root `Cargo.toml`:

```toml
[workspace]
members = ["soc1-pac", "soc2-pac", "xtask"]
```

#### Add workspace configuration

You need to add the following configuration to the `.cargo/config.toml` file in the root of your workspace:

```toml
[alias]
xtask = "run --package xtask --release --"

[env]
CARGO_WORKSPACE_DIR = { value = "", relative = true }
```

The `alias` part does the trick of making `cargo xtask` work, and the `env` part is needed to pass the workspace directory to the `xtask` crate.

#### Add xtask configuration

`xtask` needs to get metadata of pac crates in the workspace, so you need to add the following configuration to the `.xtask.toml` in the root of your workspace:

```toml
[[pacs]]
chip_name = ["soc1", "SOC1"]    # A list of an soc's names
crate_name = "soc1-pac"         # The name (dir) of the pac crate
svd_name = "soc1"               # The name of the svd file
target = "riscv"                # The target of the pac crate
patch = false                   # Whether to patch the svd (Not impl yet)

[[pacs]]
...                             # other pac crates
```

> This configuration is currently designed for [`duskmoon314/aw-pac`](https://github.com/duskmoon314/aw-pac). Thus, the `crate_name` is actually the directory name of the pac crate and `xtask` assume that the svd file is at `svd/<svd_name>.svd`. If you have a different structure, you need to modify the `xtask` crate.
>
> Need to refactor this part.

### Add xtask

You can just download the source code, using `git subtree` or anything you like. I have tested the `git subtree` and it seems working well.

```shell
# First, you can add this repo as a remote
# HTTPS way
git remote add -f xtask https://github.com/duskmoon314/pac-xtask.git
# SSH way
git remote add -f xtask git@github.com:duskmoon314/pac-xtask.git

# Then, you can add the xtask crate to your workspace
# Strongly suggest to use the `--squash` option and modify the message
git subtree add --prefix xtask xtask main --squash -m "build: add pac-xtask as subtree"

# You can check the history after adding the xtask crate in the above way
git log --oneline --graph

*   659efd5 (HEAD -> main) build: add pac-xtask as subtree
|\
| * e324fb7 Squashed 'xtask/' content from commit af40bdd
#### Previous commits
```

## Usage

You can use `cargo xtask` to run the xtask crate. A Brief usage is shown below:

```shell
> cargo xtask --help
    Finished release [optimized] target(s) in 0.13s
     Running `target/release/xtask --help`
Usage: xtask [CHIPS]... <COMMAND>

Commands:
  codegen  Generate code from svd files
  svd-fmt  Format svd files
  help     Print this message or the help of the given subcommand(s)

Arguments:
  [CHIPS]...  Names of chips to do xtask on

Options:
  -h, --help  Print help information
```

### Codegen

This command will use `svd2rust` to generate code and `form` to format the code.

```shell
cargo xtask codegen <a name in soc's chip_name>

# Example
cargo xtask codegen soc1
```

### SVD format

This command will use `svdtools` to format the svd file.

```shell
cargo xtask svd-fmt <a name in soc's chip_name>

# Example
cargo xtask svd-fmt soc1
```

To use this command, currently you need to have an `.svdfmt.yaml` file at `<workspace root>/svd/.svdfmt.yaml`. You can modify the path in the `xtask` crate if you want.

The content of the `.svdfmt.yaml` file is like this:

```yaml
# An experimental config file for svdtools fmt feature.
peripheral_name: Constant
peripheral_base_address: UpperHex16
address_block_offset: UpperHex
address_block_size: UpperHex
interrupt_name: Constant
cluster_name: Snake
cluster_address_offset: UpperHex
register_name: Snake
register_address_offset: UpperHex
register_size: Dec
register_reset_value: UpperHex16
register_reset_mask: UpperHex16
field_name: Snake
field_bit_range: BitRange
enumerated_values_name: Pascal
enumerated_value_name: Constant
enumerated_value_value: UpperHex
dim_dim: Dec
dim_increment: UpperHex
```

To see all the options, you can view the [`svd-encoder`'s source code](https://github.com/rust-embedded/svd/blob/master/svd-encoder/src/config.rs)

## License

This project is licensed under

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
