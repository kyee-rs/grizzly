use std::fs::File;
use std::io;
use std::io::Cursor;
use std::path::PathBuf;

use anyhow::Result;
use zip::write::FileOptions;
use zip::ZipWriter;

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
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        for file in files {
            if let Some(filename) = file.file_name() {
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
