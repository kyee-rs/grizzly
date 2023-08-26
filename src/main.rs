use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Error, Result};
use clap::{arg, ArgAction, Command};

use crate::compressor::compress;
use crate::cross_compiler::create_go_file;
use crate::prepare::prepare;
use crate::progress::Progress;

mod compressor;
mod cross_compiler;
mod prepare;
mod progress;

const TEMPLATE: &[u8] = include_bytes!("../includes/unpacker.go"); // Include the `unpacker.go` template.

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
                      .value_parser(["linux/386", "linux/amd64", "linux/arm", "linux/arm64", "windows/386", "windows/amd64", "windows/arm", "windows/arm64", "darwin/386", "darwin/amd64", "darwin/arm", "darwin/arm64", ]),
                  arg!(-n --name <name> "Set the name for binary. [Default: Random ID]")
                      .num_args(1)
                      .action(ArgAction::Set),
              ],
        )
        .arg_required_else_help(true)
        .subcommand(
            Command::new("prepare")
                .name("prepare")
                .about("Download the Go compiler to create an archive.")
        )
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches(); // Parse the CLI command that user have requested.
    let binding = "".to_string();
    let name = matches.get_one::<String>("name").unwrap_or(&binding);

    if matches.contains_id("file") {
        let paths = matches
            .get_many::<PathBuf>("file")
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(); // Collect files from the request
        let mut environment = HashMap::new();

        // If user specified the platform, set it for Go compiler as the environment variables
        if matches.contains_id("platform") {
            let platform = matches.get_one::<String>("platform").unwrap();
            let split = platform.split('/').collect::<Vec<&str>>();

            environment.insert("GOOS", split[0]);
            environment.insert("GOARCH", split[1]);
        }

        let (temp_dir, tarball_path) = compress(paths).await?; // Call compressor and get back values
        create_go_file(temp_dir, tarball_path, environment, name).await?; // Generate a Go unpacker file

        Progress::done_pg(); // Print the last stage (Done!)
        Ok(())
    } else if let Some(("prepare", _)) = matches.subcommand() {
        prepare().await?;
        Ok(())
    } else {
        Err(Error::msg("Not found.")
            .context("Grizzly doesn't currently support the function you have requested."))
        // Throw an error if user requested invalid subcommand.
    }
}
