use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error, Result};

use crate::progress::Progress;
use crate::TEMPLATE;

/// `create_go_file(temp_dir, tarball_path, environment)` - Generate a Go file, embed the tarball there and compile.
/// ## Panics
/// - Go binary wasn't found
/// - Failed to execute
/// - IO Errors (failed to create/read/write a file)
pub async fn create_go_file(
    temp_dir: tempfile::TempDir,
    tarball_path: PathBuf,
    environment: HashMap<&str, &str>,
    name: &String,
) -> Result<()> {
    // Set an ID for a file. Will be used as a binary name after compilation
    let id = {
        if name.is_empty() {
            let id = minimal_id::Generator::new_id().to_string();
            let mut i: Vec<&str> = id.split("").collect();
            if i[0] == "_" {
                i[0] = "-"
            }
            i.join("")
        } else {
            name.to_string()
        }
    };

    // Detect the compiler path (`go` or `~/.grizzly/go/bin/go`)
    let compiler_path = {
        if Path::exists(
            format!("{}/.grizzly/go/bin/go", home::home_dir().unwrap().display()).as_ref(),
        ) {
            format!("{}/.grizzly/go/bin/go", home::home_dir().unwrap().display())
        } else {
            String::from("go")
        }
    };

    let file_name = temp_dir.path().join(id + ".go"); // Set a Filename (generated_id + .go)
    let mut f = File::create(file_name.clone())?; // Create a file in the filesystem with specified name

    Progress::generating_code_pg(); // Notify the user about the current stage.

    // Replace `// embed_replace` with `//go:embed [PATH]` to embed a tarball inside
    let content = String::from_utf8_lossy(TEMPLATE).to_string().replace(
        "// embed_replace",
        &format!(
            "//go:embed {}",
            tarball_path.file_name().unwrap().to_str().unwrap()
        ),
    );

    f.write_all(content.as_bytes())
        .with_context(|| "failed to write the temporary .go file.")?; // Write the file with contents

    Progress::compile_pg(); // Notify the user about the current stage.

    let cmd = std::process::Command::new(compiler_path) // Try to execute `go build` with environment set.
        .args(["build", file_name.to_str().unwrap()])
        .envs(environment)
        .output();

    // Handle errors if command execution fails
    match cmd {
        Ok(output) => match output.stderr.is_empty() {
            true => Ok(()),
            false => Err(
                Error::msg(String::from_utf8_lossy(&output.stderr).to_string())
                    .context("Failed to execute the compilation command."),
            ),
        },
        Err(_) => Err(Error::msg(
            "Compiler wasn't found. Please, download and install Go first, or use `grizzly prepare`.",
        )),
    }
}
