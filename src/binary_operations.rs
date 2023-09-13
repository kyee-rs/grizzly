use std::fs;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use anyhow::{bail, Result};

pub(crate) struct Binary {
    pub(crate) name: String,
    pub(crate) size: u64,
    pub(crate) buffer: Vec<u8>,
}

impl Binary {
    pub(crate) async fn cache(platform_name: String) -> Result<Binary> {
        let prefix = "https://github.com/12subnet/zippo/releases/latest/download";

        let url = match platform_name.as_str() {
            "windows/x86_64" => format!("{}/zippo-windows-x86_64.exe", prefix),
            "windows/x86" => format!("{}/zippo-windows-i686.exe", prefix),
            "windows/aarch64" => format!("{}/zippo-windows-aarch64.exe", prefix),
            "linux/x86_64" => format!("{}/zippo-linux-x86_64-musl", prefix),
            "linux/x86" => format!("{}/zippo-linux-i686-musl", prefix),
            "linux/arm" => format!("{}/zippo-linux-arm-musl", prefix),
            "macos/x86_64" => format!("{}/zippo-darwin-x86_64", prefix),
            "macos/aarch64" => format!("{}/zippo-darwin-aarch64", prefix),
            _ => bail!(
                "Platform you chose ({}) isn't currently supported. Please refer to grizzly --help",
                platform_name
            ),
        };

        let name = url.split('/').last().unwrap_or("zippo").to_string();

        Self::inner_cache(name, url).await
    }

    async fn inner_cache(name: String, url: String) -> Result<Binary> {
        let path = format!(
            "{}/.grizzly/cache/{}",
            home::home_dir().unwrap().display(),
            name
        )
            .replace('\\', "/");

        if !Path::new(path.as_str()).exists() {
            fs::create_dir_all(format!(
                "{}/.grizzly/cache/",
                home::home_dir().unwrap().display()
            ))?;

            log::info!(
                target: "grizzly::async",
                "Downloading the pre-compiled unpacker binary into {}.",
                path.as_str()
            );
            let body = reqwest::get(url).await?.bytes().await?;

            fs::write(path, &body)?;
        }

        let binary = fs::read(format!(
            "{}/.grizzly/cache/{}",
            home::home_dir().unwrap().display(),
            name
        ))?;

        Ok(Self {
            name,
            size: binary.len() as u64,
            buffer: binary,
        })
    }

    pub(crate) fn append_bytes(&mut self, mut bytes: Vec<u8>) {
        log::info!("Appending the bytes on top of a binary...");

        self.buffer.append(&mut bytes)
    }

    pub(crate) fn generate_executable(
        &mut self,
        zip_buffer: Vec<u8>,
        name: String,
    ) -> Result<(u64, u64)> {
        let mut filename;

        self.append_bytes(zip_buffer);

        match name.is_empty() {
            true => filename = nanoid::nanoid!(10, &nanoid::alphabet::SAFE),
            false => filename = name,
        }

        if self.name.ends_with(".exe") {
            filename.push_str(".exe");
        }

        log::info!("Writing a file \"{}\"...", filename);
        let mut file = File::create(filename.clone())?;
        file.write_all(&self.buffer)?;

        Ok((file.seek(SeekFrom::End(0))?, self.size))
    }
}
