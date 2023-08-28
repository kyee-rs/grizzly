use std::path::Path;
use std::{env, fs};

use anyhow::{Error, Result};

use crate::progress::Progress;

pub(crate) struct Platform {
    url: String,
    name: String,
}

impl Platform {
    pub(crate) fn from(name: String) -> Result<Platform> {
        let prefix = "https://github.com/12subnet/zippo/releases/download/v0.1.0";
        let url = match name.as_str() {
            "windows/x86_64" => format!("{}/zippo-windows-x86_64.exe", prefix),
            "windows/x86" => format!("{}/zippo-windows-i686.exe", prefix),
            "windows/aarch64" => format!("{}/zippo-windows-aarch64.exe", prefix),
            "linux/x86_64" => format!("{}/zippo-linux-x86_64-musl", prefix),
            "linux/x86" => format!("{}/zippo-linux-i686-musl", prefix),
            "linux/arm" => format!("{}/zippo-linux-arm-musl", prefix),
            "macos/x86_64" => format!("{}/zippo-darwin-x86_64", prefix),
            "macos/aarch64" => format!("{}/zippo-darwin-aarch64", prefix),
            _ => String::new(),
        };
        if url.is_empty() {
            Err(Error::msg(format!(
                "Platform you chose ({}) isn't currently supported. Please refer to grizzly --help",
                name
            )))
        } else {
            Ok(Platform { url, name })
        }
    }
}

/// Clone a precompiled Rust Unpacker Binary (Codename: Zippo) and insert ZIP in it.
/// Zippo source code: https://github.com/12subnet/zippo
/// Zippo precompiled binaries: https://github.com/12subnet/zippo/releases/latest
/// ## Panics
/// - Failed to execute
/// - IO Errors (failed to create/read/write a file)
pub(crate) async fn generate_executable(
    mut zip_buffer: Vec<u8>,
    name: String,
    mut platform: Platform,
) -> Result<()> {
    let url = platform.url;
    let bin_name = url.split('/').last().unwrap();

    if !Path::exists(
        format!(
            "{}/.grizzly/zippo/{}",
            home::home_dir().unwrap().display(),
            platform.name
        )
        .as_ref(),
    ) {
        fs::create_dir_all(format!(
            "{}/.grizzly/zippo/",
            home::home_dir().unwrap().display()
        ))?;
        let body = reqwest::get(url.clone()).await?.bytes().await?; // Cache Zippo unpacker if needed
        fs::write(
            format!(
                "{}/.grizzly/zippo/{}",
                home::home_dir().unwrap().display(),
                bin_name
            ),
            body,
        )?;
    }

    let mut filename;
    let mut binary = fs::read(format!(
        "{}/.grizzly/zippo/{}",
        home::home_dir().unwrap().display(),
        bin_name
    ))?;

    Progress::insert_pg();

    binary.append(&mut zip_buffer);

    match name.is_empty() {
        true => filename = cool_id_generator::get_id(cool_id_generator::Size::Medium),
        false => filename = name,
    }

    if cfg!(windows) {
        filename.push_str(".exe");
    }

    Progress::zippo_pg();
    fs::write(filename, binary)?;

    Ok(())
}
