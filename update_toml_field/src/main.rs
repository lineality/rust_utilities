use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
/*
This implementation includes several safety features:

Uses a temporary file for atomic updates
Proper error handling throughout
File existence checking
Maintains the original file structure
Includes comprehensive documentation
Includes unit tests

Key features:

The function creates a new line with proper TOML formatting
Processes the file line by line to maintain the original structure
Uses a temporary file to ensure atomic updates
Includes proper error handling and file safety checks
Provides both a direct implementation and a safe wrapper function
Includes documentation and tests
*/


/// Updates a specified field in a TOML file with a new value.
/// 
/// # Arguments
/// 
/// * `path` - A string slice containing the path to the TOML file
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
/// # fs::write("example.toml", "field = \"old_value\"").unwrap();
/// let result = update_toml_field("example.toml", "new_value", "field");
/// # fs::remove_file("example.toml").unwrap();
/// ```
pub fn update_toml_field(path: &str, new_string: &str, field: &str) -> io::Result<()> {
    // Read the entire file content
    let content = fs::read_to_string(path)?;
    
    // Create a temporary file
    let temp_path = format!("{}.tmp", path);
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
/// * `path` - A string slice containing the path to the TOML file
/// * `new_string` - A string slice containing the new value to be set
/// * `field` - A string slice containing the name of the field to update
/// 
/// # Returns
/// 
/// * `Result<(), String>` - Ok(()) on success, or an error message if the operation fails
pub fn safe_update_toml_field(path: &str, new_string: &str, field: &str) -> Result<(), String> {
    // Validate inputs
    if field.is_empty() {
        return Err("Field name cannot be empty".to_string());
    }
    
    if !Path::new(path).exists() {
        return Err(format!("File not found: {}", path));
    }
    
    update_toml_field(path, new_string, field)
        .map_err(|e| format!("Failed to update TOML file: {}", e))
}

fn main() {
    // Create a sample TOML file if it doesn't exist
    if !Path::new("config.toml").exists() {
        fs::write("config.toml", "# Sample TOML file\n").expect("Failed to create config file");
    }

    match safe_update_toml_field("config.toml", "alice", "user_name") {
        Ok(_) => println!("Successfully updated TOML file"),
        Err(e) => eprintln!("Error: {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_update_field() {
        // Create a test file
        let test_content = "directory_path = \"old/path\"\nupdated_at_timestamp = 1735690073";
        let test_file = "test_config.toml";
        fs::write(test_file, test_content).expect("Failed to create test file");
        
        // Update the field
        let result = update_toml_field(test_file, "new/path", "directory_path");
        assert!(result.is_ok());
        
        // Verify the update
        let updated_content = fs::read_to_string(test_file).expect("Failed to read test file");
        assert!(updated_content.contains("directory_path = \"new/path\""));
        
        // Cleanup
        fs::remove_file(test_file).expect("Failed to remove test file");
    }
}

// fn main() {
//     match safe_update_toml_field("config.toml", "user_name", "alice") {
//         Ok(_) => println!("Successfully updated TOML file"),
//         Err(e) => eprintln!("Error: {}", e)
//     }
// }
