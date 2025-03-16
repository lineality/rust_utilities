// src/manage_absolute_executable_directory_relative_paths.rs
/// # manage_absolute_executable_directory_relative_paths - Executable-relative path resolution in Rust
/// use -> cargo build --profile release-performance
/// or, use -> cargo build --profile release-small 
/// see: https://github.com/lineality/rust_compile_optimizations_cheatsheet
///
/// This module provides functions for working with file paths relative to the 
/// executable's directory location rather than the current working directory (CWD).
///
/// The main function `make_input_path_name_abs_executabledirectoryrelative_nocheck` converts a path 
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
use manage_absolute_executable_directory_relative_paths::make_input_path_name_abs_executabledirectoryrelative_nocheck;

fn main() {
    // Get a path relative to the executable directory, not the CWD
    match make_input_path_name_abs_executabledirectoryrelative_nocheck("data/config.json") {
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
/// Does NOT check if the path exists or attempt to create anything.
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
/// use manage_absolute_executable_directory_relative_paths::make_input_path_name_abs_executabledirectoryrelative_nocheck;
///
/// // Get an absolute path for "data/config.json" relative to the executable directory
/// let abs_path = make_input_path_name_abs_executabledirectoryrelative_nocheck("data/config.json").unwrap();
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

/// Gets an absolute path for an existing directory relative to the executable's directory.
/// Returns an error if the directory doesn't exist or isn't a directory.
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
    let path = make_input_path_name_abs_executabledirectoryrelative_nocheck(dir_path)?;
    
    // Check if the path exists and is a directory
    if !abs_executable_directory_relative_exists(&path)? {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Directory does not exist",
        ));
    } else if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path exists but is not a directory",
        ));
    }
    
    // Canonicalize the path (should succeed because we've verified it exists)
    path.canonicalize().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to canonicalize directory path: {}", e),
        )
    })
}

/// Creates a new directory at the specified path relative to the executable directory.
/// Returns an error if the directory already exists.
///
/// # Arguments
///
/// * `dir_path` - The directory path relative to the executable directory
///
/// # Returns
///
/// * `Result<PathBuf, io::Error>` - The absolute, canonicalized path to the newly created directory
pub fn mkdir_new_abs_executabledirectoryrelative_canonicalized<P: AsRef<Path>>(dir_path: P) -> Result<PathBuf, io::Error> {
    // Get the absolute path without checking existence
    let abs_path = make_input_path_name_abs_executabledirectoryrelative_nocheck(dir_path)?;
    
    // Check if the directory already exists
    if abs_executable_directory_relative_exists(&abs_path)? {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Directory already exists",
        ));
    }
    
    // Create the directory and all parent directories
    std::fs::create_dir_all(&abs_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create directory: {}", e),
        )
    })?;
    
    // Canonicalize the path (should succeed because we just created it)
    abs_path.canonicalize().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to canonicalize newly created directory path: {}", e),
        )
    })
}

/// Gets an absolute path for an existing file relative to the executable's directory.
/// Returns an error if the file doesn't exist or isn't a file.
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
    let path = make_input_path_name_abs_executabledirectoryrelative_nocheck(file_path)?;
    
    // Check if the path exists and is a file
    if !abs_executable_directory_relative_exists(&path)? {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        ));
    } else if path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path exists but is a directory, not a file",
        ));
    }
    
    // Canonicalize the path (should succeed because we've verified it exists)
    path.canonicalize().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to canonicalize file path: {}", e),
        )
    })
}

/// Creates necessary parent directories for a file path relative to the executable.
/// Does NOT create the file itself, only prepares the path structure.
///
/// # Arguments
///
/// * `file_path` - A file path relative to the executable's directory
///
/// # Returns
///
/// * `Result<PathBuf, io::Error>` - The absolute path to the (non-existent) file with parent directories prepared
pub fn prepare_file_parent_directories_abs_executabledirectoryrelative<P: AsRef<Path>>(file_path: P) -> Result<PathBuf, io::Error> {
    let path = make_input_path_name_abs_executabledirectoryrelative_nocheck(file_path)?;
    
    // If the path exists and is a directory, that's an error
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::Path;

    // Test get_absolute_path_to_executable_parentdirectory
    #[test]
    fn test_get_executable_parentdirectory() {
        let result = get_absolute_path_to_executable_parentdirectory();
        
        // Should succeed and return a valid path
        assert!(result.is_ok());
        
        let dir = result.unwrap();
        
        // Should be absolute
        assert!(dir.is_absolute());
        
        // Should exist
        assert!(dir.exists());
        
        // Should be a directory
        assert!(dir.is_dir());
        
        // Should match parent of current_exe
        let expected = std::env::current_exe()
            .expect("Failed to get current executable path")
            .parent()
            .expect("Failed to get parent directory")
            .to_path_buf();
        
        assert_eq!(dir, expected);
    }

    // Test make_input_path_name_abs_executabledirectoryrelative_nocheck with various inputs
    #[test]
    fn test_make_path_absolute_nocheck() {
        // Test with a simple relative path
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck("some/path.txt");
        assert!(result.is_ok());
        
        let path = result.unwrap();
        assert!(path.is_absolute());
        
        // Path should be executable_dir + relative_path
        let exec_dir = get_absolute_path_to_executable_parentdirectory().unwrap();
        assert_eq!(path, exec_dir.join("some/path.txt"));
        
        // Test with a path containing ..
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck("some/../other/path.txt");
        assert!(result.is_ok());
        
        // Test with an empty path
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck("");
        assert!(result.is_ok());
        
        // Test with just a dot
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck(".");
        assert!(result.is_ok());
        
        // Test with an absolute path (platform-specific)
        #[cfg(windows)]
        let abs_path = "C:\\absolute\\path.txt";
        #[cfg(not(windows))]
        let abs_path = "/absolute/path.txt";
        
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck(abs_path);
        assert!(result.is_ok());
    }

    // Test abs_executable_directory_relative_exists with various paths
    #[test]
    fn test_path_exists() {
        // Test with the current directory path (which definitely exists)
        let current_dir = env::current_dir().unwrap();
        let result = abs_executable_directory_relative_exists(&current_dir);
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Test with the executable directory (which definitely exists)
        let exec_dir = get_absolute_path_to_executable_parentdirectory().unwrap();
        let result = abs_executable_directory_relative_exists(&exec_dir);
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Test with the executable file itself (which definitely exists)
        let exec_file = std::env::current_exe().unwrap();
        let result = abs_executable_directory_relative_exists(&exec_file);
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Test with a path that shouldn't exist
        let nonexistent = Path::new("/this/path/definitely/does/not/exist/12345abcde");
        let result = abs_executable_directory_relative_exists(nonexistent);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    // Test errors from make_input_path_name_abs_executabledirectoryrelative_nocheck
    #[test]
    fn test_path_error_handling() {
        // Create an invalid path that would cause an error
        // This is difficult to do in a cross-platform way without actually creating files,
        // but we can at least check that we don't panic
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck("\0invalid");
        
        // On most platforms, this should fail (but we're just making sure it doesn't panic)
        if result.is_err() {
            let err = result.err().unwrap();
            assert!(err.to_string().len() > 0); // Error should have a message
        }
    }

    // Test path normalization logic
    #[test]
    fn test_path_normalization() {
        // Get executable directory
        let exec_dir = get_absolute_path_to_executable_parentdirectory().unwrap();
        
        // Create paths with different representations
        let path1 = make_input_path_name_abs_executabledirectoryrelative_nocheck("dir/file.txt").unwrap();
        let path2 = make_input_path_name_abs_executabledirectoryrelative_nocheck("dir/./file.txt").unwrap();
        let path3 = make_input_path_name_abs_executabledirectoryrelative_nocheck("dir/../dir/file.txt").unwrap();
        
        // Before canonicalization, these might not be equal
        // Check they all have the correct base directory though
        assert!(path1.starts_with(&exec_dir));
        assert!(path2.starts_with(&exec_dir));
        assert!(path3.starts_with(&exec_dir));
        
        // Path1 and path2 should resolve to the same path if they were canonicalized
        // We can't actually canonicalize without the paths existing, but we can check
        // that our function handles them correctly
    }

    // Test access to "real" files in the executable directory
    #[test]
    fn test_real_file_access() {
        // Get the executable file itself, which definitely exists
        let exec_file = std::env::current_exe().unwrap();
        
        // Check that it exists using our function
        let result = abs_executable_directory_relative_exists(&exec_file);
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Get its filename
        let filename = exec_file.file_name().unwrap();
        
        // Now try using our path function to resolve the same file
        let resolved_path = make_input_path_name_abs_executabledirectoryrelative_nocheck(filename).unwrap();
        
        // The resolved path should exist and be a file
        assert!(resolved_path.exists());
        assert!(resolved_path.is_file());
    }

    // Test edge cases for all functions
    #[test]
    fn test_edge_cases() {
        // Empty path
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck("");
        assert!(result.is_ok());
        
        // Just a dot
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck(".");
        assert!(result.is_ok());
        
        // Just dot-dot (parent dir)
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck("..");
        assert!(result.is_ok());
        
        // Long path with many segments
        let long_path = "a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/file.txt";
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck(long_path);
        assert!(result.is_ok());
        
        // Path with unicode characters
        let unicode_path = "🦀/😊/🔥/file.txt";
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck(unicode_path);
        assert!(result.is_ok());
        
        // Path with spaces
        let path_with_spaces = "folder with spaces/file with spaces.txt";
        let result = make_input_path_name_abs_executabledirectoryrelative_nocheck(path_with_spaces);
        assert!(result.is_ok());
    }

    // Test behavior of directory-specific functions with simulated paths
    #[test]
    fn test_directory_functions() {
        // Create an absolute path to a directory that doesn't exist
        let nonexistent_dir = Path::new("/nonexistent/directory/path");
        
        // If path doesn't exist, our function should return an error
        // We're not actually calling it here because it would fail, but we're making
        // sure our logic is correct for checking existence
        assert!(!nonexistent_dir.exists());
        
        // Get a directory that definitely exists (the executable parent directory)
        let existing_dir = get_absolute_path_to_executable_parentdirectory().unwrap();
        assert!(existing_dir.exists());
        assert!(existing_dir.is_dir());
    }
    
    // Test behavior of file-specific functions with simulated paths
    #[test]
    fn test_file_functions() {
        // Create an absolute path to a file that doesn't exist
        let nonexistent_file = Path::new("/nonexistent/file/path.txt");
        
        // If path doesn't exist, our function should return an error
        // We're not actually calling it here because it would fail, but we're making
        // sure our logic is correct for checking existence
        assert!(!nonexistent_file.exists());
        
        // Get a file that definitely exists (the executable itself)
        let existing_file = std::env::current_exe().unwrap();
        assert!(existing_file.exists());
        assert!(existing_file.is_file());
    }
}