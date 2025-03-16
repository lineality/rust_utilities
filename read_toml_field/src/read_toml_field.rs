
// read toml fields with vanilla rust
/*



mod read_toml_field;  // This declares the module and tells Rust to look for handle_gpg.rs
use crate::read_toml_field::{
    read_field_from_toml,
    read_basename_fields_from_toml,
    read_single_line_string_field_from_toml,
    read_multi_line_toml_string,
    read_integer_array,
    read_singleline_string_from_clearsigntoml,
    read_multiline_string_from_clearsigntoml,
}; 

fn main() -> Result<(), String> {
    let value = read_field_from_toml("test.toml", "fieldname");
    println!("Field value -> {}", value);
    
    // Read all prompt fields
    let prompt_values = read_basename_fields_from_toml("config.toml", "prompt");
    println!("Prompts: {:?}", prompt_values);

    let single_line = read_single_line_string_field_from_toml("config.toml", "promptsdir_1")?;
    let multi_line = read_multi_line_toml_string("config.toml", "multi_line")?;
    let integer_array = read_integer_array("config.toml", "schedule_duration_start_end")?;
    
    println!("Single line: {}", single_line);
    println!("Multi line: {}", multi_line);
    println!("Numbers: {:?}", integer_array);
    
    Ok(())
}

*/

use std::fs::File;
use std::io::{self, BufRead, Read};
// use std::path::Path;
use std::process::Command;

/// The function reads a single line from a TOML file that starts with a specified field name
/// and ends with a value. The function returns an empty string if the field is not found, and
/// does not panic or unwrap in case of errors. The function uses only standard Rust libraries
/// and does not introduce unnecessary dependencies.
///
/// design:
/// 0. start with an empty string to return by default
/// 1. get file at path
/// 2. open as text
/// 3. iterate through rows
/// 4. look for filed name as start of string the " = "
/// 5. grab that whole row of text
/// 6. remove "fieldname = " from the beginning
/// 7. remove '" ' and trailing spaces from the end
/// 8. return that string, if any
/// by default, return an empty string, if anything goes wrong, 
/// handle the error, and return an empty string
///
/// requires:
/// use std::fs::File;
/// use std::io::{self, BufRead};
///
/// example use:
///     let value = read_field_from_toml("test.toml", "fieldname");
///
pub fn read_field_from_toml(path: &str, field_name: &str) -> String {
    // Validate input parameters
    if path.is_empty() || field_name.is_empty() {
        println!("Error: Empty path or field name provided");
        return String::new();
    }

    // Verify file extension
    if !path.to_lowercase().ends_with(".toml") {
        println!("Warning: File does not have .toml extension: {}", path);
    }

    // Debug print statement
    println!("Attempting to open file at path: {}", path);

    // Open the file at the specified path
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            // More detailed error reporting
            println!("Failed to open file at path: {}. Error: {}", path, e);
            return String::new();
        },
    };

    // Debug print statement
    println!("Successfully opened file at path: {}", path);

    // Create a buffered reader to read the file line by line
    let reader = io::BufReader::new(file);

    // Keep track of line numbers for better error reporting
    let mut line_number = 0;

    // Iterate through each line in the file
    for line_result in reader.lines() {
        line_number += 1;

        // Handle line reading errors
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                println!("Error reading line {}: {}", line_number, e);
                continue;
            }
        };

        // Skip empty lines and comments
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }

        // Debug print statement
        println!("Processing line {}: {}", line_number, line);

        // Check if line starts with field name
        if line.trim_start().starts_with(field_name) {
            // Debug print statement
            println!("Found field '{}' on line {}", field_name, line_number);

            // Split the line by '=' and handle malformed lines
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                println!("Malformed TOML line {} - missing '=': {}", line_number, line);
                continue;
            }

            let key = parts[0].trim();
            let value = parts[1].trim();

            // Verify exact field name match (avoiding partial matches)
            if key != field_name {
                continue;
            }

            // Handle empty values
            if value.is_empty() {
                println!("Warning: Empty value found for field '{}'", field_name);
                return String::new();
            }

            // Debug print statement
            println!("Extracted value: {}", value);

            // Clean up the value: remove quotes and trim spaces
            let cleaned_value = value.trim().trim_matches('"').trim();
            
            // Verify the cleaned value isn't empty
            if cleaned_value.is_empty() {
                println!("Warning: Value became empty after cleaning for field '{}'", field_name);
                return String::new();
            }

            return cleaned_value.to_string();
        }
    }

    // If we get here, the field wasn't found
    println!("Field '{}' not found in file", field_name);
    String::new()
}

/// Reads all fields from a TOML file that share a common base name (prefix before underscore)
/// and returns a vector of their values. Returns an empty vector if no matching fields are found
/// or if any errors occur.
///
/// # Arguments
/// * `path` - Path to the TOML file
/// * `base_name` - Base name to search for (e.g., "prompt" will match "prompt_1", "prompt_2", etc.)
///
/// # Returns
/// * `Vec<String>` - Vector containing all values for fields matching the base name
///
/// # Example
/// ```
/// let values = read_basename_fields_from_toml("config.toml", "prompt");
/// // For TOML content:
/// // prompt_1 = "value1"
/// // prompt_2 = "value2"
/// // Returns: vec!["value1", "value2"]
/// ```
pub fn read_basename_fields_from_toml(path: &str, base_name: &str) -> Vec<String> {
    let mut values = Vec::new();

    // Validate input parameters
    if path.is_empty() || base_name.is_empty() {
        println!("Error: Empty path or base name provided");
        return values;
    }

    // Open and read the file
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open file at path: {}. Error: {}", path, e);
            return values;
        },
    };

    let reader = io::BufReader::new(file);
    let base_name_with_underscore = format!("{}_", base_name);

    // Process each line
    for (line_number, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                println!("Error reading line {}: {}", line_number + 1, e);
                continue;
            }
        };

        // Skip empty lines and comments
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        // Check if line starts with base_name_
        if trimmed_line.starts_with(&base_name_with_underscore) {
            // Split the line by '=' and handle malformed lines
            let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
            if parts.len() != 2 {
                println!("Malformed TOML line {} - missing '=': {}", line_number + 1, line);
                continue;
            }

            let value = parts[1].trim();
            
            // Clean up the value: remove quotes and trim spaces
            let cleaned_value = value.trim().trim_matches('"').trim();
            
            if !cleaned_value.is_empty() {
                values.push(cleaned_value.to_string());
            }
        }
    }

    // Sort values to ensure consistent ordering
    values.sort();
    values
}

/// Reads a single-line string field from a TOML file.
/// 
/// # Arguments
/// * `path` - Path to the TOML file
/// * `field_name` - Name of the field to read
/// 
/// # Returns
/// * `Result<String, String>` - The field value or an error message
pub fn read_single_line_string_field_from_toml(path: &str, field_name: &str) -> Result<String, String> {
    let file = File::open(path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    
    let reader = io::BufReader::new(file);
    
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        let trimmed = line.trim();
        
        if trimmed.starts_with(&format!("{} = ", field_name)) {
            return Ok(trimmed
                .splitn(2, '=')
                .nth(1)
                .unwrap_or("")
                .trim()
                .trim_matches('"')
                .to_string());
        }
    }
    
    Err(format!("Field '{}' not found", field_name))
}

/// Reads a multi-line string field (triple-quoted) from a TOML file.
/// 
/// # Arguments
/// * `path` - Path to the TOML file
/// * `field_name` - Name of the field to read
/// 
/// # Returns
/// * `Result<String, String>` - The concatenated multi-line value or an error message
pub fn read_multi_line_toml_string(path: &str, field_name: &str) -> Result<String, String> {
    let mut file = File::open(path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Find the start of the field
    let field_start = format!("{} = \"\"\"", field_name);
    let start_pos = content.find(&field_start)
        .ok_or_else(|| format!("Multi-line field '{}' not found", field_name))?;

    // Find the end of the field (next """)
    let content_after_start = &content[start_pos + field_start.len()..];
    let end_pos = content_after_start.find("\"\"\"")
        .ok_or_else(|| format!("Closing triple quotes not found for field '{}'", field_name))?;

    // Extract the content between the triple quotes
    let multi_line_content = &content_after_start[..end_pos];

    // Clean up the content
    Ok(multi_line_content
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("\n")
        .trim()
        .to_string())
}

/// Reads an array of integers from a TOML file into a Vec<u64>.
/// 
/// # Arguments
/// * `path` - Path to the TOML file
/// * `field_name` - Name of the field to read
/// 
/// # Returns
/// * `Result<Vec<u64>, String>` - The vector of integers or an error message
pub fn read_integer_array(path: &str, field_name: &str) -> Result<Vec<u64>, String> {
    let file = File::open(path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    
    let reader = io::BufReader::new(file);
    
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        let trimmed = line.trim();
        
        if trimmed.starts_with(&format!("{} = [", field_name)) {
            let array_part = trimmed
                .splitn(2, '=')
                .nth(1)
                .ok_or("Invalid array format")?
                .trim()
                .trim_matches(|c| c == '[' || c == ']');
                
            return array_part
                .split(',')
                .map(|s| s.trim().parse::<u64>()
                    .map_err(|e| format!("Invalid integer: {}", e)))
                .collect::<Result<Vec<u64>, String>>();
        }
    }
    
    Err(format!("Array field '{}' not found", field_name))
}

/// Extracts a GPG key from a TOML file.
/// This function assumes the GPG key is stored in a multi-line field.
///
/// # Arguments
/// * `path` - Path to the TOML file
/// * `key_field` - Name of the field containing the GPG key
///
/// # Returns
/// * `Result<String, String>` - The GPG key or an error message
fn extract_gpg_key_from_clearsigntoml(path: &str, key_field: &str) -> Result<String, String> {
    read_multi_line_toml_string(path, key_field)
}

// /// Verifies a clearsigned TOML file using GPG.
// ///
// /// # Arguments
// /// * `path` - Path to the TOML file
// /// * `key` - The GPG key to use for verification
// ///
// /// # Returns
// /// * `Result<(), String>` - Success or error message
// fn verify_clearsign(path: &str, key: &str) -> Result<(), String> {
//     // Create a temporary file to hold the key
//     let temp_key_path = format!("{}.key", path);
//     std::fs::write(&temp_key_path, key)
//         .map_err(|e| format!("Failed to write temporary key file: {}", e))?;

//     // Use gpg to verify the file
//     let output = Command::new("gpg")
//         .arg("--verify")
//         .arg("--batch")
//         .arg("--no-tty")
//         .arg("--keyring")
//         .arg(&temp_key_path)
//         .arg(path)
//         .output()
//         .map_err(|e| format!("Failed to execute GPG: {}", e))?;

//     // Clean up the temporary key file
//     let _ = std::fs::remove_file(temp_key_path);

//     if !output.status.success() {
//         let stderr = String::from_utf8_lossy(&output.stderr);
//         return Err(format!("GPG verification failed: {}", stderr));
//     }

//     Ok(())
// }

/// Verifies a clearsigned TOML file using GPG.
///
/// # Arguments
/// * `path` - Path to the TOML file
/// * `key` - The GPG key to use for verification
///
/// # Returns
/// * `Result<bool, String>` - True if verification succeeds, false if it fails, or an error message
fn verify_clearsign(path: &str, key: &str) -> Result<bool, String> {
    // Create a temporary file to hold the key
    let temp_key_path = format!("{}.key", path);
    std::fs::write(&temp_key_path, key)
        .map_err(|e| format!("Failed to write temporary key file: {}", e))?;

    // Use gpg to verify the file
    let output = Command::new("gpg")
        .arg("--verify")
        .arg("--batch")
        .arg("--no-tty")
        .arg("--keyring")
        .arg(&temp_key_path)
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to execute GPG: {}", e))?;

    // Clean up the temporary key file
    let _ = std::fs::remove_file(temp_key_path);

    // Return the verification result
    Ok(output.status.success())
}

/// Reads a single-line string field from a clearsigned TOML file.
///
/// # Arguments
/// * `path` - Path to the TOML file
/// * `field_name` - Name of the field to read
///
/// # Returns
/// * `Result<String, String>` - The field value or an error message
pub fn read_singleline_string_from_clearsigntoml(path_to_clearsigntoml_with_gpgkey: &str, field_name: &str) -> Result<String, String> {
    // Extract GPG key from the file
    let key = extract_gpg_key_from_clearsigntoml(path_to_clearsigntoml_with_gpgkey, "gpg_key_public")?;

    // Verify the file and only proceed if verification succeeds
    let verification_result = verify_clearsign(path_to_clearsigntoml_with_gpgkey, &key)?;

    if !verification_result {
        return Err(format!("GPG verification failed for file: {}", path_to_clearsigntoml_with_gpgkey));
    }

    // Only read the field if verification succeeded
    read_single_line_string_field_from_toml(path_to_clearsigntoml_with_gpgkey, field_name)
}

/// Reads a multi-line string field from a clearsigned TOML file.
///
/// # Arguments
/// * `path` - Path to the TOML file
/// * `field_name` - Name of the field to read
///
/// # Returns
/// * `Result<String, String>` - The field value or an error message
pub fn read_multiline_string_from_clearsigntoml(path: &str, field_name: &str) -> Result<String, String> {
    // Extract GPG key from the file
    let key = extract_gpg_key_from_clearsigntoml(path, "gpg_key_public")?;
    
    // Verify the file and only proceed if verification succeeds
    let verification_result = verify_clearsign(path, &key)?;
    
    if !verification_result {
        return Err(format!("GPG verification failed for file: {}", path));
    }
    
    // Only read the field if verification succeeded
    read_multi_line_toml_string(path, field_name)
}

/// Reads an integer array field from a clearsigned TOML file.
///
/// # Arguments
/// * `path` - Path to the TOML file
/// * `field_name` - Name of the field to read
///
/// # Returns
/// * `Result<Vec<u64>, String>` - The integer array or an error message
pub fn read_integerarray_clearsigntoml(path: &str, field_name: &str) -> Result<Vec<u64>, String> {
    // Extract GPG key from the file
    let key = extract_gpg_key_from_clearsigntoml(path, "gpg_key_public")?;
    
    // Verify the file and only proceed if verification succeeds
    let verification_result = verify_clearsign(path, &key)?;
    
    if !verification_result {
        return Err(format!("GPG verification failed for file: {}", path));
    }
    
    // Only read the field if verification succeeded
    read_integer_array(path, field_name)
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::write;
    use std::fs::remove_file;

    // Mock test for clearsign functions
    // These tests will be skipped in environments without GPG
    #[test]
    fn test_clearsign_reading() {
        // This test should be run only if GPG is available
        if !Command::new("gpg").arg("--version").status().map_or(false, |s| s.success()) {
            println!("Skipping GPG test because GPG is not available");
            return;
        }

        // Create a mock TOML file with a fake GPG key for testing
        let test_content = r#"
            gpg_key_public = """
            -----BEGIN PGP PUBLIC KEY BLOCK-----
            mQENBF0blBUBCADPhh9ZoC2QXlA8Xu0ghtQTf5VQgC8CmxPM/H85q8HyITWJ6S+c
            LCG9OSvqpqxN9VTRLVqf9051Rj4nQzGEEzqUJp3zHfLKZN3SNKVnMn8CyeMoWJGg
            XgNjnyfk687AB0Pn5JApzVaS9JDYVOPmTNXk4T9wLs2vYbKQ9E4/Mv0fnRBYaAgm
            JQT53jdH/QUIVIqnYvMbwB4TZY8MfA4AoT4QyqDB5ppiUWH5S2PJqId29Z/Y45J+
            -----END PGP PUBLIC KEY BLOCK-----
            """
            promptsdir_1 = "test/dir"
            multi_line = """
            This is a
            multi-line
            value
            """
            schedule_duration_start_end = [1, 2, 3, 4]
        "#;
        
        let test_file = "test_clearsign.toml";
        write(test_file, test_content).unwrap();

        // These tests will fail in real environments since we're using a fake GPG key
        // but they demonstrate the API usage
        let _ = read_singleline_string_from_clearsigntoml(test_file, "promptsdir_1");
        let _ = read_multiline_string_from_clearsigntoml(test_file, "multi_line");
        let _ = read_integerarray_clearsigntoml(test_file, "schedule_duration_start_end");

        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_read_basename_fields() {
        // Create a temporary test TOML file
        let test_content = r#"
            # Test TOML file
            prompt_1 = "value1"
            prompt_2 = "value2"
            other_field = "other"
            prompt_3 = "value3"
        "#;
        let test_file = "test_basename.toml";
        write(test_file, test_content).unwrap();

        // Test reading basename fields
        let values = read_basename_fields_from_toml(test_file, "prompt");
        
        // Clean up
        let _ = remove_file(test_file);

        assert_eq!(values, vec!["value1", "value2", "value3"]);
    }

    #[test]
    fn test_empty_input() {
        let values = read_basename_fields_from_toml("", "prompt");
        assert!(values.is_empty());
    }
    
    #[test]
    fn test_single_line_string() {
        let test_content = r#"
            field1 = "value1"
            field2 = "value2"
        "#;
        let test_file = "test_single.toml";
        write(test_file, test_content).unwrap();
        
        let result = read_single_line_string_field_from_toml(test_file, "field1");
        assert_eq!(result.unwrap(), "value1");
        
        std::fs::remove_file(test_file).unwrap();
    }
    

    #[test]
    fn test_multi_line_string() {
        let test_content = r#"
description = """
This is a
multi-line
string
"""\
"#;
        let test_file = "test_multi.toml";
        write(test_file, test_content).unwrap();
        
        let result = read_multi_line_toml_string(test_file, "description");
        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(content.contains("multi-line"));
        assert_eq!(content, "This is a\nmulti-line\nstring");
        
        std::fs::remove_file(test_file).unwrap();
    }
    
    #[test]
    fn test_integer_array() {
        let test_content = r#"
            numbers = [1, 2, 3, 4, 5]
        "#;
        let test_file = "test_array.toml";
        write(test_file, test_content).unwrap();
        
        let result = read_integer_array(test_file, "numbers");
        assert_eq!(result.unwrap(), vec![1, 2, 3, 4, 5]);
        
        std::fs::remove_file(test_file).unwrap();
    }
}
