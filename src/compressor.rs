use std::fs::File;
use std::io;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::progress::Progress;

/// Compress the `files: Vec<&PathBuf>` into a zip archive and return its buffer.
/// - Returns: `anyhow::Result<Vec<u8>>`.
///     - `Vec<u8>` is a buffer
pub(crate) async fn compress(files: Vec<&PathBuf>) -> Result<Vec<u8>> {
    Progress::allocating_space_pg();
    let mut buffer = Cursor::new(Vec::new()); // Allocate buffer for zip-compressed data

    Progress::creating_zip_pg();
    {
        let mut zip = ZipWriter::new(&mut buffer);
        let options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o755);

        for file in files {
            if file.is_dir() {
                let dir = WalkDir::new(file.to_string_lossy().to_string());
                let it = dir.into_iter();
                dir_compress(&mut it.filter_map(|e| e.ok()), &mut zip, options)?;
            } else if let Some(filename) = file.file_name() {
                if let Ok(mut f) = File::open(file) {
                    zip.start_file(filename.to_string_lossy(), options)?; // If file has a valid name, push it to zip
                    io::copy(&mut f, &mut zip)?;
                }
            }
        }

        zip.finish()?; // Close the zip data and save
    }

    Ok(buffer.into_inner())
}

/// **Description**: If user wants to compress a folder, compress its content recursively.
///
/// **Needs**:
///    - `dir`: `&PathBuf` - Directory to compress
///    - `zip`: `&mut ZipWriter<&mut Cursor<Vec<u8>>>` - ZIP Writer
///    - `options`: `FileOptions` - Options for the compression
///
/// **Returns**: `anyhow::Result<()>`
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
