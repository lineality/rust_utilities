// src/manage_absolute_executable_directory_relative_paths.rs
/// # manage_absolute_executable_directory_relative_paths - Executable-relative path resolution in Rust
/// use -> cargo build --profile release-performance
/// or, use -> cargo build --profile release-small 
/// see: https://github.com/lineality/rust_compile_optimizations_cheatsheet
///
/// This module provides functions for working with file paths relative to the 
/// executable's directory location rather than the current working directory (CWD).
///
/// The main function `make_input_path_absolute_executable_directory_relative` converts a path 
/// to an absolute path that's resolved relative to the executable's location.

/* Docs:
# Executable-Directory-Relative Path Resolution

This module solves the common issue where paths are resolved relative to the current
working directory, which can lead to problems when your executable is run from different
locations. Instead, it ensures paths are resolved relative to where your executable is located.

### Sample main file to use this module
```rust
// src/main.rs

// import manage_absolute_executable_directory_relative_paths module w/ these 2 lines
mod manage_absolute_executable_directory_relative_paths;
use manage_absolute_executable_directory_relative_paths::make_input_path_absolute_executable_directory_relative;

fn main() {
    // Get a path relative to the executable directory, not the CWD
    match make_input_path_absolute_executable_directory_relative("data/config.json") {
        Ok(absolute_path) => println!("Absolute path: {}", absolute_path.display()),
        Err(e) => {
            eprintln!("Error resolving path: {}", e);
            std::process::exit(1);
        }
    }
}
```

## Always
```
Always best practice.
Always extensive doc strings.
Always comments.
Always clear, meaningful, unique names.
Always absolute file paths.
Always error handling.
Never unsafe code.
Never use unwrap.
```
*/

use std::path::{Path, PathBuf};
use std::io;

/// Gets the directory where the current executable is located.
///
/// # Returns
///
/// * `Result<PathBuf, io::Error>` - The absolute directory path containing the executable or an error
///   if it cannot be determined.
pub fn get_absolute_path_to_executable_parentdirectory() -> Result<PathBuf, io::Error> {
    // Get the path to the current executable
    let executable_path = std::env::current_exe().map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to determine current executable path: {}", e),
        )
    })?;
    
    // Get the directory containing the executable
    let executable_directory = executable_path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to determine parent directory of executable",
        )
    })?;
    
    Ok(executable_directory.to_path_buf())
}

/// Converts a path to an absolute path based on the executable's directory location.
///
/// # Arguments
///
/// * `path_to_make_absolute` - A path to convert to an absolute path relative to 
///   the executable's directory location.
///
/// # Returns
///
/// * `Result<PathBuf, io::Error>` - The absolute path based on the executable's directory or an error
///   if the executable's path cannot be determined or if the path cannot be resolved.
///
/// # Examples
///
/// ```
/// use manage_absolute_executable_directory_relative_paths::make_input_path_absolute_executable_directory_relative;
///
/// // Get an absolute path for "data/config.json" relative to the executable directory
/// let abs_path = make_input_path_absolute_executable_directory_relative("data/config.json").unwrap();
/// println!("Absolute path: {}", abs_path.display());
/// ```
pub fn make_input_path_name_abs_executabledirectoryrelative_nocheck<P: AsRef<Path>>(path_to_make_absolute: P) -> Result<PathBuf, io::Error> {
    // Get the directory where the executable is located
    let executable_directory = get_absolute_path_to_executable_parentdirectory()?;
    
    // Create a path by joining the executable directory with the provided path
    let target_path = executable_directory.join(path_to_make_absolute);
    
    // If the path doesn't exist, we still return the absolute path without trying to canonicalize
    if !abs_executable_directory_relative_exists(&target_path)? {
        // Ensure the path is absolute (it should be since we joined with executable_directory)
        if target_path.is_absolute() {
            return Ok(target_path);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Failed to create absolute path",
            ));
        }
    }
    
    // Path exists, so we can canonicalize it to resolve any ".." or "." segments
    target_path.canonicalize().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to canonicalize path: {}", e),
        )
    })
}

/// Checks if a path exists (either as a file or directory).
///
/// # Arguments
///
/// * `path_to_check` - The path to check for existence
///
/// # Returns
///
/// * `Result<bool, io::Error>` - Whether the path exists or an error
pub fn abs_executable_directory_relative_exists<P: AsRef<Path>>(path_to_check: P) -> Result<bool, io::Error> {
    let path = path_to_check.as_ref();
    Ok(path.exists())
}

/// Gets an absolute path for a directory relative to the executable's directory.
///
/// # Arguments
///
/// * `dir_path` - A directory path to convert to an absolute path relative to 
///   the executable's directory location.
///
/// # Returns
///
/// * `Result<PathBuf, io::Error>` - The absolute directory path or an error
pub fn make_dir_path_abs_executabledirectoryrelative_canonicalized_or_error<P: AsRef<Path>>(dir_path: P) -> Result<PathBuf, io::Error> {
    let path = make_input_path_absolute_executable_directory_relative(dir_path)?;
    
    // Create the directory if it doesn't exist
    if !abs_executable_directory_relative_exists(&path)? {
        std::fs::create_dir_all(&path).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to create directory: {}", e),
            )
        })?;
    } else if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Path exists but is not a directory",
        ));
    }
    
    Ok(path)
}

TODO NEW
pub fn mkdir_new_abs_executabledirectoryrelative_canonicalized() {
    
}

/// Gets an absolute path for a file relative to the executable's directory.
///
/// # Arguments
///
/// * `file_path` - A file path to convert to an absolute path relative to 
///   the executable's directory location.
///
/// # Returns
///
/// * `Result<PathBuf, io::Error>` - The absolute file path or an error
pub fn make_file_path_abs_executabledirectoryrelative_canonicalized_or_error<P: AsRef<Path>>(file_path: P) -> Result<PathBuf, io::Error> {
    let path = make_input_path_absolute_executable_directory_relative(file_path)?;
    
    // If the path exists but is a directory, that's an error
    if abs_executable_directory_relative_exists(&path)? && path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Path exists but is a directory, not a file",
        ));
    }
    
    // Ensure the parent directory exists
    if let Some(parent) = path.parent() {
        if !abs_executable_directory_relative_exists(parent)? {
            std::fs::create_dir_all(parent).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to create parent directory: {}", e),
                )
            })?;
        }
    }
    
    Ok(path)
}
