/* Copyright (C) 2023 Saputskyi Petro - All Rights Reserved
 * You may use, distribute and modify this code under the
 * terms of the CC-BY-SA-4.0 license.
 *
 * ----------------------------------------------------------------------------------------------------
 * Commercial use - YES
 * Distribution - YES
 * Modification - YES
 * Private use - YES
 * ----------------------------------------------------------------------------------------------------
 * Liability - NO
 * Patent use - NO
 * Trademark use - NO
 * Warranty - NO
 * ----------------------------------------------------------------------------------------------------
 * A copy of the license and copyright notice must be included with the licensed material.
 * Modifications must be released under the same license when distributing the licensed material.
 * In some cases a similar or related license may be used.
 * Changes made to the licensed material must be documented.
 * ----------------------------------------------------------------------------------------------------
 *
 * You should have received a copy of the CC-BY-SA-4.0 license with
 * this file. If not, please write to: hello@lowt.live, or visit: https://github.com/12subnet
 */
use std::env;
use std::path::PathBuf;

use crate::binary_operations::Binary;
use anyhow::Result;
use clap::{arg, ArgAction, Command};
use log::LevelFilter;
use tokio::runtime::Runtime;

use crate::compression_engine::compress;

mod binary_operations;
mod compression_engine;

fn cli() -> Command {
    Command::new("grizzly")
        .name("grizzly")
        .version("v0.1.2-rc-2")
        .bin_name("grizzly")
        .author("morph-ua / 12subnet (github.com/12subnet)")
        .about("A powerful CLI tool for creating self-extractable (SFX) archives.")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .args([
                  arg!(-f --file <file> "File to compress (use multiple -f flags to compress multiple files).")
                      .required(true)
                      .num_args(1)
                      .value_parser(clap::value_parser!(PathBuf))
                      .action(ArgAction::Append),
                  arg!(-p --platform <platform> "Choose the platform for your binary.")
                      .num_args(1)
                      .action(ArgAction::Set)
                      .value_parser(["windows/x86_64", "windows/x86", "windows/aarch64", "linux/x86_64", "linux/x86", "linux/arm", "macos/x86_64", "macos/aarch64"]),
                  arg!(-n --name <name> "Set the name for binary. [default: Random ID]")
                      .num_args(1)
                      .action(ArgAction::Set),
              ],
        )
        .arg_required_else_help(true)
}

fn main() -> Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    let matches = cli().get_matches();

    let (name, platform, paths) = (
        matches
            .get_one::<String>("name")
            .unwrap_or(&String::new())
            .to_string(),
        matches
            .get_one::<String>("platform")
            .unwrap_or(&format!("{}/{}", env::consts::OS, env::consts::ARCH))
            .to_string(),
        matches
            .get_many::<PathBuf>("file")
            .into_iter()
            .flatten()
            .cloned()
            .collect(),
    );

    let rt = Runtime::new()?;

    let (binary, zip) = rt.block_on(async {
        let binary = tokio::spawn(Binary::cache(platform));
        let zip = tokio::spawn(compress(paths));

        tokio::try_join!(binary, zip)
    })?;

    let (file_size, platform_size) = binary?.generate_executable(zip?, name)?;

    log::info!(
        "Bundled. Final size: {} ({} binary overhead).",
        human_bytes::human_bytes(file_size as f64),
        human_bytes::human_bytes(platform_size as f64)
    );

    Ok(())
}
