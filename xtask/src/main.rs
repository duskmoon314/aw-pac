use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
use form::create_directory_structure;
use serde::Deserialize;
use svd2rust::{generate::device::render, load_from, util::build_rs, Config, Target};
use svdtools::convert::convert_cli::{convert, ParserConfig};

#[macro_use]
extern crate log;

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct PacMeta {
    chip_name: Vec<String>,
    crate_name: String,
    svd_name: String,
    target: String,
    patch: bool,
}

#[derive(Deserialize)]
struct XtaskConfig {
    pacs: Vec<PacMeta>,
}

fn parse_config() -> Result<XtaskConfig> {
    let config =
        fs::read_to_string(PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join(".xtask.toml"))?;
    let config: XtaskConfig = toml::from_str(&config)?;
    Ok(config)
}

fn get_meta_from_config(config: &XtaskConfig, chip_name: &str) -> Result<PacMeta> {
    let meta = config
        .pacs
        .iter()
        .find(|meta| meta.chip_name.contains(&chip_name.to_string()))
        .ok_or_else(|| Error::msg(format!("Chip {} not found in config", chip_name)))?;
    Ok(meta.clone())
}

#[derive(Debug, Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Names of chips to do xtask on
    #[arg(global = true)]
    chips: Vec<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate code from svd files
    Codegen,

    /// Format svd files
    SvdFmt,
}

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_module("xtask", log::LevelFilter::Info)
        .init();
    let cli = Cli::parse();
    let cfg = parse_config()?;

    match cli.command {
        Commands::Codegen => {
            for chip in cli.chips {
                let meta = get_meta_from_config(&cfg, &chip)?;
                if fs::remove_dir_all(&PathBuf::from(&meta.crate_name).join("src")).is_err() {
                    warn!("unable to remove src directory");
                }
                generate_code(&meta)?;
                format_code(&meta)?;

                info!("Finished generating code for {}", &meta.crate_name);
            }

            Ok(())
        }

        Commands::SvdFmt => {
            for chip in cli.chips {
                let meta = get_meta_from_config(&cfg, &chip)?;
                format_svd(&meta)?;
            }

            Ok(())
        }
    }
}

fn generate_code(meta: &PacMeta) -> Result<()> {
    info!(
        "Generating code for {} from {}.svd",
        meta.crate_name, meta.svd_name
    );

    let svd_path =
        PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join(format!("svd/{}.svd", meta.svd_name));
    let out_path = PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join(&meta.crate_name);

    let config = Config {
        target: Target::parse(&meta.target)?,
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

fn format_code(meta: &PacMeta) -> Result<()> {
    info!("running `form` and `rustfmt` on PAC");
    let path = PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join(&meta.crate_name);
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

fn format_svd(meta: &PacMeta) -> Result<()> {
    info!("formatting svd file: {}.svd", meta.svd_name);

    let svd_path =
        PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join(format!("svd/{}.svd", meta.svd_name));
    let svd_format_config = PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join("svd/.svdfmt.yaml");

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
