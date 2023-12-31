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
use std::fs::File;
use std::io;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

pub(crate) async fn compress(files: Vec<PathBuf>) -> Result<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());

    {
        log::info!(target: "grizzly::async", "Compressing the files...");

        let mut zip = ZipWriter::new(&mut buffer);
        let options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o755);

        for file in &files {
            if file.is_dir() {
                let dir = WalkDir::new(file.to_string_lossy().to_string());
                let it = dir.into_iter();
                dir_compress(&mut it.filter_map(|e| e.ok()), &mut zip, options)?;
            } else if let Some(filename) = file.file_name() {
                if let Ok(mut f) = File::open(file) {
                    zip.start_file(filename.to_string_lossy(), options)?;
                    io::copy(&mut f, &mut zip)?;
                }
            }
        }

        zip.finish()?;
    }

    Ok(buffer.into_inner())
}

fn dir_compress(
    it: &mut dyn Iterator<Item = DirEntry>,
    zip: &mut ZipWriter<&mut Cursor<Vec<u8>>>,
    options: FileOptions,
) -> Result<()> {
    let mut buffer = Vec::new();

    for entry in it {
        let name = entry.path();

        if name.is_file() {
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = File::open(name)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }

    Ok(())
}
