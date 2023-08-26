use std::fs::File;
use std::io::ErrorKind::NotFound;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

use anyhow::{Error, Result};

/// `prepare()` - Download the Go compiler binary if user doesn't have it.
/// ## Panics
/// - If user has an unsupported OS.
/// - Fails to download the Go package.
/// - IO Errors (Failed to create/read/write a file).
/// - Go binary is already installed.
pub async fn prepare() -> Result<()> {
    // Check if Go binary already installed.
    match std::process::Command::new("go").spawn() {
        Ok(_) => Err(Error::msg("Go binary is already installed")),
        Err(e) => {
            if e.kind() != NotFound {
                return Err(Error::msg("Some strange error occurred"));
            }
            if Path::exists(format!("{}/.grizzly/go", home::home_dir().unwrap().display()).as_ref())
            {
                return Err(Error::msg("Go binary is already installed"));
            }
            Ok(())
        }
    }?;

    let os = env::consts::OS; // Detect OS

    // Set URL to download based on OS.
    let url = match os {
        "macos" => "https://golang.org/dl/go1.21.0.darwin-amd64.pkg",
        "windows" => "https://golang.org/dl/go1.21.0.windows-amd64.msi",
        "linux" => "https://golang.org/dl/go1.21.0.linux-amd64.tar.gz",
        _ => panic!("Unsupported OS"),
    };

    let body = reqwest::get(url).await?.bytes().await?;
    let filename = url.split('/').last().unwrap();
    let mut f = File::create(filename)?;

    f.write_all(&body)?; // Write a file with content from the downloaded package.

    match os {
        "macos" => {
            std::process::Command::new("open")
                .arg("go1.21.0.darwin-amd64.pkg")
                .spawn()
                .expect("Failed to install Golang");
        }
        "windows" => {
            std::process::Command::new("msiexec")
                .arg("/i")
                .arg("go1.21.0.windows-amd64.msi")
                .spawn()
                .expect("Failed to install Golang");
        }
        "linux" => {
            fs::create_dir_all(format!("{}/.grizzly/", home::home_dir().unwrap().display()))?; // To suppress SUDO warnings save file to ~/.grizzly instead of /usr/bin
            std::process::Command::new("tar")
                .arg("-C")
                .arg(format!("{}/.grizzly/", home::home_dir().unwrap().display()))
                .arg("-xzf")
                .arg("go1.21.0.linux-amd64.tar.gz")
                .spawn()
                .expect("Failed to install Golang");
        }
        _ => unreachable!(), // Unreachable, we have covered all available operating systems.
    }

    Ok(())
}
