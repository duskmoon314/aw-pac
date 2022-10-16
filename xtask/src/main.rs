use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Error, Result};
use clap::{Args, Parser, Subcommand};
use form::create_directory_structure;
use phf::phf_map;
use svd2rust::{generate::device::render, load_from, util::build_rs, Config, Target};
use svdtools::convert::convert_cli::{convert, ParserConfig};

#[macro_use]
extern crate log;

struct ChipMeta {
    svd_name: &'static str,
    crate_name: &'static str,
    target: Target,
}

static CHIPS: phf::Map<&'static str, ChipMeta> = phf_map! {
    "d1" => ChipMeta {
        svd_name: "d1_unofficial",
        crate_name: "d1-pac",
        target: Target::RISCV
    },
};

#[derive(Debug, Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate code from svd files
    Gen(GenArgs),

    /// Format svd files
    Fmt(FmtArgs),
}

#[derive(Debug, Args)]
struct GenArgs {
    /// Names of Chips
    chips: Vec<String>,
}

#[derive(Debug, Args)]
struct FmtArgs {
    /// Names of Chips
    chips: Vec<String>,
}

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_module("xtask", log::LevelFilter::Info)
        .init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen(args) => {
            for chip in args.chips {
                let meta = CHIPS.get(chip.as_str()).unwrap();
                if fs::remove_dir_all(&PathBuf::from(meta.crate_name).join("src")).is_err() {
                    warn!("unable to remove src directory");
                }
                generate_code(meta)?;
                format(meta)?;

                info!("Finished generating code for {}", meta.crate_name);
            }

            Ok(())
        }

        Commands::Fmt(args) => {
            for chip in args.chips {
                let meta = CHIPS.get(chip.as_str()).unwrap();
                format_svd(meta)?;
            }

            Ok(())
        }
    }
}

fn generate_code(meta: &ChipMeta) -> Result<()> {
    info!(
        "Generating code for {} from {}.svd",
        meta.crate_name, meta.svd_name
    );

    let svd_path = format!("svd/{}.svd", meta.svd_name);
    let out_path = PathBuf::from(meta.crate_name);

    let config = Config {
        target: meta.target,
        output_dir: out_path.clone(),
        const_generic: true,

        ..Default::default()
    };

    let input = fs::read_to_string(svd_path)?;
    let device = load_from(&input, &config)?;

    let mut device_x = String::new();
    let items = render(&device, &config, &mut device_x)?;
    let data = items.to_string();

    let mut file = File::create(out_path.join("lib.rs"))?;
    file.write_all(data.as_ref())?;

    writeln!(File::create(out_path.join("device.x"))?, "{}", device_x)?;
    writeln!(File::create(out_path.join("build.rs"))?, "{}", build_rs())?;

    Ok(())
}

fn format(meta: &ChipMeta) -> Result<()> {
    info!("running `form` and `rustfmt` on PAC");
    let path = Path::new(meta.crate_name);
    let lib_file = path.join("lib.rs");

    let base_dir = path.join("src");
    let string_contents = fs::read_to_string(&lib_file)?;
    create_directory_structure(base_dir, &string_contents).map_err(Error::msg)?;

    fs::remove_file(&lib_file)?;

    Command::new("cargo")
        .arg("fmt")
        .current_dir(path)
        .output()?;

    Ok(())
}

fn format_svd(meta: &ChipMeta) -> Result<()> {
    info!("formatting svd file: {}.svd", meta.svd_name);

    let svd_path = PathBuf::from(format!("svd/{}.svd", meta.svd_name));
    let svd_format_config = PathBuf::from("svd/.svdfmt.yaml");

    convert(
        &svd_path,
        &svd_path,
        None,
        None,
        ParserConfig {
            expand: false,
            expand_properties: false,
            ignore_enums: false,
        },
        Some(&svd_format_config),
    )?;

    Ok(())
}
