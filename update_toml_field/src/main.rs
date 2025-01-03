use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};  // Added PathBuf import

/// Updates a specified field in a TOML file with a new value.
/// 
/// # Arguments
/// 
/// * `path` - A PathBuf containing the path to the TOML file
/// * `new_string` - A string slice containing the new value to be set
/// * `field` - A string slice containing the name of the field to update
/// 
/// # Returns
/// 
/// * `io::Result<()>` - Ok(()) on success, or an error if the operation fails
/// 
/// # Example
/// 
/// ```
/// # use std::fs;
/// # use std::path::PathBuf;
/// # fs::write("example.toml", "field = \"old_value\"").unwrap();
/// let path = PathBuf::from("example.toml");
/// let result = update_toml_field(&path, "new_value", "field");
/// # fs::remove_file("example.toml").unwrap();
/// ```
pub fn update_toml_field(path: &PathBuf, new_string: &str, field: &str) -> io::Result<()> {
    // Read the entire file content using PathBuf's as_path() method
    let content = fs::read_to_string(path.as_path())?;
    
    // Create a temporary file with the same name plus .tmp
    let temp_path = path.with_extension("tmp");
    let mut temp_file = File::create(&temp_path)?;
    
    let mut field_found = false;
    
    // Process each line
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(field) && trimmed.contains('=') {
            // Write the new line for the matching field
            writeln!(temp_file, "{} = \"{}\"", field, new_string)?;
            field_found = true;
        } else {
            // Write the original line
            writeln!(temp_file, "{}", line)?;
        }
    }
    
    // If field wasn't found, append it
    if !field_found {
        writeln!(temp_file, "{} = \"{}\"", field, new_string)?;
    }
    
    // Ensure all data is written
    temp_file.flush()?;
    
    // Replace the original file with the temporary file
    fs::rename(temp_path, path)?;
    
    Ok(())
}

/// A safer wrapper function that includes additional error checking.
/// 
/// # Arguments
/// 
/// * `path` - A PathBuf containing the path to the TOML file
/// * `new_string` - A string slice containing the new value to be set
/// * `field` - A string slice containing the name of the field to update
/// 
/// # Returns
/// 
/// * `Result<(), String>` - Ok(()) on success, or an error message if the operation fails
///
/// Example Use:
/// ```
/// use std::path::PathBuf;
/// let config_path = PathBuf::from("config.toml");
/// match safe_update_toml_field(&config_path, "alice", "user_name") {
///     Ok(_) => println!("Successfully updated TOML file"),
///     Err(e) => eprintln!("Error: {}", e)
/// }
/// ```
pub fn safe_update_toml_field(path: &PathBuf, new_string: &str, field: &str) -> Result<(), String> {
    // Validate inputs
    if field.is_empty() {
        return Err("Field name cannot be empty".to_string());
    }
    
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }
    
    update_toml_field(path, new_string, field)
        .map_err(|e| format!("Failed to update TOML file: {}", e))
}

fn main() {
    // Create a PathBuf for the config file
    let config_path = PathBuf::from("config.toml");

    // Create a sample TOML file if it doesn't exist
    if !config_path.exists() {
        fs::write(&config_path, "# Sample TOML file\n")
            .expect("Failed to create config file");
    }

    match safe_update_toml_field(&config_path, "alice", "user_name") {
        Ok(_) => println!("Successfully updated TOML file"),
        Err(e) => eprintln!("Error: {}", e)
    }
}

/// run with: cargo test
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_update_field() {
        // Create a test file using PathBuf
        let test_content = "directory_path = \"old/path\"\nupdated_at_timestamp = 1735690073";
        let test_path = PathBuf::from("test_config.toml");
        fs::write(&test_path, test_content).expect("Failed to create test file");
        
        // Update the field
        let result = update_toml_field(&test_path, "new/path", "directory_path");
        assert!(result.is_ok());
        
        // Verify the update
        let updated_content = fs::read_to_string(&test_path).expect("Failed to read test file");
        assert!(updated_content.contains("directory_path = \"new/path\""));
        
        // Cleanup
        fs::remove_file(&test_path).expect("Failed to remove test file");
    }
}