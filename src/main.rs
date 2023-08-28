use std::env;
use std::path::PathBuf;

use anyhow::{Error, Result};
use clap::{arg, ArgAction, Command};

use crate::binary_cloner::{generate_executable, Platform};
use crate::compressor::compress;
use crate::progress::Progress;

mod binary_cloner;
mod compressor;
mod progress;

/// Create the CLI Command with specified subcommands and flags.
fn cli() -> Command {
    Command::new("grizzly")
        .name("grizzly")
        .version("v0.0.1-rc4")
        .bin_name("grizzly")
        .author("morph-ua / 12subnet (github.com/12subnet)")
        .about("SFX (Self-extractable) archives creator.")
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
                  arg!(-n --name <name> "Set the name for binary. [Default: Random ID]")
                      .num_args(1)
                      .action(ArgAction::Set),
              ],
        )
        .arg_required_else_help(true)
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches(); // Parse the CLI command that user have requested.
    let binding = String::new();
    let name = matches
        .get_one::<String>("name")
        .unwrap_or(&binding)
        .to_string();
    let platform = matches
        .get_one::<String>("platform")
        .unwrap_or(&format!("{}/{}", env::consts::OS, env::consts::ARCH))
        .to_string();

    if matches.contains_id("file") {
        let paths = matches
            .get_many::<PathBuf>("file")
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(); // Collect files from the request

        let zip_buffer = compress(paths).await?; // Call compressor and get back zip buffer
        generate_executable(zip_buffer, name, Platform::from(platform)?).await?; // Generate a Go unpacker file

        Progress::done_pg(); // Print the last stage (Done!)
        Ok(())
    } else {
        Err(Error::msg("Not found.")
            .context("Grizzly doesn't currently support the function you have requested."))
        // Throw an error if user requested invalid subcommand.
    }
}