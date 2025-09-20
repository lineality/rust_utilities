use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::env;

/// Reads the package version from a Cargo.toml file.
///
/// This function specifically looks for the `version` field within the `[package]`
/// section of a Cargo.toml file. It reads the file line by line to avoid loading
/// the entire file into memory, making it efficient even for large files.
///
/// The function correctly handles:
/// - Comments (lines starting with #)
/// - [package] section appearing anywhere in the file
/// - Version fields in other sections (which are ignored)
/// - Both single and double quotes around version values
/// - Inline comments after the version value
/// - Whitespace variations around the = sign
///
/// # Arguments
///
/// * `cargo_toml_path` - Path to the Cargo.toml file to read
///
/// # Returns
///
/// * `Ok(String)` - The version string if found in the [package] section
/// * `Err(io::Error)` - If the file cannot be read or version is not found
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
///
/// let version = get_package_version(Path::new("Cargo.toml"))
///     .expect("Failed to read version");
/// println!("Package version: {}", version);
/// ```
pub fn get_package_version(cargo_toml_path: &Path) -> io::Result<String> {
    // Open the Cargo.toml file
    let file = File::open(cargo_toml_path)?;
    let reader = BufReader::new(file);

    // State tracking: are we currently inside the [package] section?
    let mut in_package_section = false;

    // Process the file line by line
    for line_result in reader.lines() {
        // Handle potential IO errors when reading each line
        let line = line_result?;

        // Remove leading/trailing whitespace for analysis
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Skip comment lines (TOML comments start with #)
        if trimmed.starts_with('#') {
            continue;
        }

        // Check if we're entering a new section
        // TOML sections are denoted by [section_name]
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            // Update our state: are we in the [package] section?
            in_package_section = trimmed == "[package]";
            continue;
        }

        // Only process lines when we're in the [package] section
        if !in_package_section {
            continue;
        }

        // Now we're in [package], look for the version field
        // Check if this line contains a version assignment
        if let Some(equals_pos) = trimmed.find('=') {
            // Split into key and value parts
            let key_part = trimmed[..equals_pos].trim();
            let value_part = trimmed[equals_pos + 1..].trim();

            // Check if the key is exactly "version"
            if key_part == "version" {
                // Extract the version value, removing quotes
                // TOML strings can use single or double quotes

                // Handle potential inline comments (e.g., version = "1.0" # comment)
                let value_without_comment = if let Some(comment_pos) = value_part.find('#') {
                    value_part[..comment_pos].trim()
                } else {
                    value_part
                };

                // Remove quotes (both single and double)
                let version = value_without_comment
                    .trim_start_matches('"')
                    .trim_end_matches('"')
                    .trim_start_matches('\'')
                    .trim_end_matches('\'');

                return Ok(version.to_string());
            }
        }
    }

    // If we get here, we didn't find a version in the [package] section
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No version field found in [package] section"
    ))
}

/// Gets the path to the current crate's Cargo.toml file.
///
/// This function determines the location of Cargo.toml for the current crate.
/// When running via `cargo run` or `cargo test`, this will find the correct
/// Cargo.toml in the crate root.
///
/// # Returns
///
/// * `Ok(PathBuf)` - Path to the Cargo.toml file
/// * `Err(io::Error)` - If the current directory cannot be determined
///
/// # Note
///
/// This uses the current directory, which should be the crate root when
/// running cargo commands. For more complex scenarios (like workspaces),
/// additional logic might be needed.
pub fn get_current_crate_cargo_toml() -> io::Result<PathBuf> {
    // Get the current directory (should be crate root when running via cargo)
    let mut path = env::current_dir()?;

    // Append Cargo.toml to the path
    path.push("Cargo.toml");

    // Verify the file exists
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Cargo.toml not found at: {}", path.display())
        ));
    }

    Ok(path)
}

fn main() {
    // Get the path to this crate's Cargo.toml
    let cargo_toml_path = match get_current_crate_cargo_toml() {
        Ok(path) => {
            println!("Found Cargo.toml at: {}", path.display());
            path
        },
        Err(e) => {
            eprintln!("Failed to locate Cargo.toml: {}", e);
            return;
        }
    };

    // Read and display the version
    match get_package_version(&cargo_toml_path) {
        Ok(version) => {
            println!("This crate's version: {}", version);
        },
        Err(e) => {
            eprintln!("Failed to read version from Cargo.toml: {}", e);
        }
    }
}

#[cfg(test)]
mod get_crate_version_tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    /// Creates a temporary test file with the given content
    fn create_test_file(filename: &str, content: &str) -> io::Result<PathBuf> {
        let temp_dir = env::temp_dir();
        let file_path = temp_dir.join(filename);
        let mut file = File::create(&file_path)?;
        write!(file, "{}", content)?;
        Ok(file_path)
    }

    /// Test with [package] section appearing after other sections
    #[test]
    fn test_package_section_not_first() -> io::Result<()> {
        let content = r#"[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35" }

[dev-dependencies]
version-compare = "0.1"
criterion = { version = "0.5" }

[package]
name = "get_crate_version"
version = "9.2.3"
edition = "2024"
authors = ["Test Author"]
description = "Test crate for version parsing"

[build-dependencies]
version_check = "0.9""#;

        let path = create_test_file("test_package_later.toml", content)?;
        let version = get_package_version(&path)?;

        assert_eq!(version, "9.2.3", "Should find version even when [package] is not first");

        fs::remove_file(path)?;
        Ok(())
    }

    /// Test reading a standard Cargo.toml with version in [package]
    #[test]
    fn test_standard_cargo_toml() -> io::Result<()> {
        let content = r#"[package]
name = "my-crate"
version = "1.2.3"
authors = ["Someone"]

[dependencies]
serde = { version = "1.0" }"#;

        let path = create_test_file("test_standard.toml", content)?;
        let version = get_package_version(&path)?;
        assert_eq!(version, "1.2.3");

        fs::remove_file(path)?;
        Ok(())
    }

    /// Test that version in [dependencies] is not returned
    #[test]
    fn test_ignores_dependency_version() -> io::Result<()> {
        let content = r#"[dependencies]
version = "999.999.999"

[package]
name = "my-crate"
version = "1.2.3""#;

        let path = create_test_file("test_deps.toml", content)?;
        let version = get_package_version(&path)?;
        assert_eq!(version, "1.2.3", "Should find version in [package], not [dependencies]");

        fs::remove_file(path)?;
        Ok(())
    }

    /// Test with inline comments
    #[test]
    fn test_inline_comments() -> io::Result<()> {
        let content = r#"[package]
name = "my-crate"
version = "7.8.9"  # This is the version
authors = ["Someone"]"#;

        let path = create_test_file("test_comments.toml", content)?;
        let version = get_package_version(&path)?;
        assert_eq!(version, "7.8.9");

        fs::remove_file(path)?;
        Ok(())
    }

    /// Test when version is missing from [package]
    #[test]
    fn test_missing_version() {
        let content = r#"[package]
name = "my-crate"
authors = ["Someone"]

[dependencies]"#;

        let path = create_test_file("test_missing.toml", content)
            .expect("Failed to create test file");
        let result = get_package_version(&path);

        assert!(result.is_err(), "Should error when version is missing");

        let _ = fs::remove_file(path);
    }
}
