use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;
use tempfile::TempDir;

use crate::progress::Progress;

/// Compress the `files: Vec<&PathBuf>` into a temporary tarball (*.tar.gz).
/// - Returns: `anyhow::Result<(TempDir, PathBuf)>`.
///     - `TempDir` is a directory, where file is stored
///     - `PathBuf` is a file name in directory from `TempDir`
pub async fn compress(files: Vec<&PathBuf>) -> Result<(TempDir, PathBuf)> {
    Progress::temp_storage_pg(); // Print the first stage (Creating temporary storage)

    // Create a tarball
    let temp_dir = tempfile::Builder::new().prefix("grizzly").tempdir()?;
    let file_name = temp_dir.path().join("grizzly_tarball.tar.gz");
    let f = File::create(file_name.clone())?;
    let enc = GzEncoder::new(f, Compression::fast());
    let mut tar = tar::Builder::new(enc);
    //////

    Progress::creating_tarball(); // Print the second stage (Creating a tarball)

    // Append files to a created tarball.
    for file in files {
        if file.to_str() == Some(".") {
            tar.append_dir_all("", ".")?;
        } else if file.is_dir() {
            let file_name = file.file_name().expect("Failed to extract the file name.");
            tar.append_dir_all(file_name, file)?;
        } else {
            let file_name = file.file_name().expect("Failed to extract the file name.");
            tar.append_file(file_name, &mut File::open(file).unwrap())?;
        }
    }
    //////

    drop(tar); // Release the tarball

    Ok((temp_dir, file_name))
}
