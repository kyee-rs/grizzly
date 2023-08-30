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
use std::fs;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use anyhow::{Error, Result};

use crate::progress::Progress;

pub(crate) struct Platform {
    url: String,
    binary_name: String,
    binary_size: Option<u64>,
}

impl Platform {
    // Load the platform information based on the name
    pub(crate) fn from(name: String) -> Result<Platform> {
        let prefix = "https://github.com/12subnet/zippo/releases/latest/download";

        // Use URL based on the platform name.
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
            let binary_name = url.split('/').last().unwrap_or("zippo").to_string();
            Ok(Platform {
                url,
                binary_name,
                binary_size: None,
            })
        }
    }

    // Cache Zippo Executable if not found in the cache directory.
    pub(crate) async fn cache_if_needed(&mut self) -> Result<Vec<u8>> {
        let path = format!(
            "{}/.grizzly/cache/{}",
            home::home_dir().unwrap().display(),
            &self.binary_name
        );

        if !Path::new(path.as_str()).exists() {
            fs::create_dir_all(format!(
                "{}/.grizzly/cache/",
                home::home_dir().unwrap().display()
            ))?;

            let body = reqwest::get(&self.url).await?.bytes().await?; // Cache Zippo unpacker if needed

            fs::write(path, &body)?;
        }

        let binary = fs::read(format!(
            "{}/.grizzly/cache/{}",
            home::home_dir().unwrap().display(),
            &self.binary_name
        ))?;

        self.binary_size = Some(binary.len() as u64);

        Ok(binary)
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
) -> Result<(u64, u64)> {
    let mut filename;
    let mut binary = platform.cache_if_needed().await?;

    Progress::insert_pg();

    binary.append(&mut zip_buffer);

    match name.is_empty() {
        true => filename = cool_id_generator::get_id(cool_id_generator::Size::Medium),
        false => filename = name,
    }

    if platform.binary_name.ends_with(".exe") {
        filename.push_str(".exe");
    }

    Progress::zippo_pg();
    let mut file = File::create(filename.clone())?;
    file.write_all(&binary)?;

    Ok((
        file.seek(SeekFrom::End(0))?,
        platform.binary_size.unwrap_or(1u64),
    ))
}
