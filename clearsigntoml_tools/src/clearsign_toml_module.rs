// clearsign_toml_module.rs
//! This is a module. You can import functions
//! from this module as the examples below.
//!
//! this is vanilla rust code for managing clearsign-toml files, 
//! .toml files that are stay clearsigned as they are used
//! including gpg decrytpion of newly recieved files
//! 
//! GPG handling code for clearsigning and encrypting files.
//! This module provides functionality to clearsign files with your private key
//! and encrypt them with a recipient's public key file.
/*

mod clearsign_toml_module;  // This declares the module and tells Rust to look for clearsign_toml_module.rs
use crate::clearsign_toml_module::{
    manual_q_and_a_new_encrypted_clearsigntoml_verification,
}; 

fn main() -> Result<(), String> {
    println!("=== GPG Clearsigned TOML File Processor ===");
    println!("This tool helps process encrypted clearsigned TOML files");
    println!("Make sure GPG is properly installed on your system");
    println!("-----------------------------------------------");
    
    match manual_q_and_a_new_encrypted_clearsigntoml_verification() {
        Ok(()) => {
            println!("Operation completed successfully!");
            Ok(())
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(format!("Failed to process encrypted clearsigned TOML file: {}", e))
        }
    }
}
*/

/*
```markdown
## GPG Code Overview
This module provides secure GPG file processing capabilities, specifically handling:
1. Clearsigning files with a private key
2. Encrypting clearsigned files for specific recipients
3. Decrypting and validating received files

## Prerequisites
- GPG (GnuPG) must be installed and configured on the system
- Users must have appropriate GPG keys generated/imported:
  - Sender needs their private key for signing
  - Sender needs recipient's public key for encryption
  - Recipient needs their private key for decryption
  - Recipient needs sender's public key for validation

## Directory Structure
```
project_root/
├── invites_updates/
│   ├── incoming/     # For received public keys
│   └── outgoing/     # For encrypted output files
└── src/
    └── handle_gpg.rs # This module
```

## Key Functions

### For Senders
```rust
pub fn clearsign_and_encrypt_file_for_recipient(
    input_file_path: &Path,
    your_signing_key_id: &str,
    recipient_public_key_path: &Path,
) -> Result<(), GpgError>
```
- Takes a file, clearsigns it, and encrypts it for a recipient
- Output: `invites_updates/outgoing/<original_filename>.gpg`

### For Recipients
```rust
pub fn decrypt_and_validate_file(
    encrypted_file_path: &Path,
    validator_key_id: &str,
    output_path: &Path,
) -> Result<(), GpgError>
```
- Decrypts received file and validates the clearsign signature

## Error Handling
All operations return `Result<T, GpgError>` where `GpgError` includes:
- FileSystemError
- GpgOperationError
- TempFileError
- PathError
- ValidationError
- DecryptionError

## Security Features
1. No unsafe code
2. Temporary files automatically cleaned up
3. No unwrap() calls - all errors properly handled
4. GPG trust model set to "always" for encryption operations
5. Signature validation enforced
6. All operations use separate temporary files

## Common GPG Commands for Users
```bash
# List secret keys (for signing)
gpg --list-secret-keys --keyid-format=long

# List public keys (for validation)
gpg --list-keys --keyid-format=long

# Export public key
gpg --armor --export KEYID > public_key.asc

# Import public key
gpg --import public_key.asc
```

## Usage Examples

### Sending a File
```rust
let input_file = Path::new("config.toml");
let signing_key = "3AA5C34371567BD2";
let recipient_key = Path::new("invites_updates/incoming/recipient_key.asc");

clearsign_and_encrypt_file_for_recipient(
    input_file,
    signing_key,
    recipient_key
)?;
```

### Receiving a File
```rust
let encrypted_file = Path::new("invites_updates/outgoing/config.toml.gpg");
let validator_key = "1234567890ABCDEF";
let output_file = Path::new("decrypted_config.toml");

decrypt_and_validate_file(
    encrypted_file,
    validator_key,
    output_file
)?;
```

## Process Flow

### Sending
1. Validate signing key exists
2. Create temporary file paths
3. Clearsign original file
4. Encrypt clearsigned file
5. Clean up temporary files
6. Output to `invites_updates/outgoing/`

### Receiving
1. Decrypt received file
2. Verify clearsign signature
3. Extract verified content
4. Clean up temporary files
5. Output decrypted and verified file

## Maintenance Notes
- No third-party dependencies
- All file operations use temporary files
- Extensive error handling throughout
- Clear, descriptive variable names
- Full documentation coverage

## Testing
Recommended test scenarios:
1. Valid signing key, valid recipient key
2. Invalid signing key
3. Invalid recipient key
4. File permission issues
5. Missing directories
6. Large files
7. Invalid file paths
8. Malformed GPG keys

## Future Improvements
Consider adding:
1. Async support
2. Multiple recipient support
3. Key validation caching
4. Configurable output directories
5. Logging integration
6. Stream processing for large files

## Support
For questions or issues:
1. Check GPG key validity
2. Verify file permissions
3. Ensure GPG is properly installed
4. Check error messages in GpgError enum
5. Verify directory structure exists

## Security Notes
1. Never share private keys
2. Regularly backup GPG keys
3. Use strong passphrases
4. Keep GPG updated
5. Monitor file permissions
6. Verify key fingerprints
```

# Module Use Example:

```rust
use std::path::Path;
use std::io::{self, Write};

mod clearsign_toml_module;  // This declares the module and tells Rust to look for handle_gpg.rs
use crate::clearsign_toml_module::{
    GpgError, 
    clearsign_and_encrypt_file_for_recipient, 
    decrypt_and_validate_file,
    rust_gpg_tools_interface,
};

// call module
pub fn main() -> Result<(), GpgError> {
    rust_gpg_tools_interface();
}
```
*/

/*
Sample use 
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

use std::io::{
    self, 
    Write, 
    BufRead, 
    Read,
};
use std::fs::{
    self,
    File,
};
use std::path::{
    Path, 
    PathBuf,
};
use std::process::Command;
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

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

////////////
// gpg code
////////////

// Add these to the existing GpgError enum:
impl GpgError {
    /// Returns a user-friendly error message
    pub fn to_string(&self) -> String {
        match self {
            GpgError::FileSystemError(e) => format!("File system error: {}", e),
            GpgError::GpgOperationError(s) => format!("GPG operation error: {}", s),
            GpgError::TempFileError(s) => format!("Temporary file error: {}", s),
            GpgError::PathError(s) => format!("Path error: {}", s),
            GpgError::ValidationError(s) => format!("Validation error: {}", s),
            GpgError::DecryptionError(s) => format!("Decryption error: {}", s),
        }
    }
}

/// Clearsigns a file with the user's private key and saves the output to a specified location.
///
/// # Arguments
/// * `input_file_path` - Path to the file that needs to be clearsigned
/// * `output_file_path` - Path where the clearsigned file will be saved
/// * `signing_key_id` - GPG key ID to use for signing
///
/// # Returns
/// * `Ok(())` if clearsigning succeeds
/// * `Err(GpgError)` if any operation fails
///
/// # Example
/// ```no_run
/// let input = Path::new("document.txt");
/// let output = Path::new("document.txt.asc");
/// let key_id = "3AA5C34371567BD2";
/// clearsign_file(input, output, key_id)?;
/// ```
pub fn clearsign_file(
    input_file_path: &Path,
    output_file_path: &Path,
    signing_key_id: &str,
) -> Result<(), GpgError> {
    // Validate that the signing key exists and is available
    if !validate_gpg_key(signing_key_id)? {
        return Err(GpgError::GpgOperationError(
            format!("Signing key '{}' not found in keyring", signing_key_id)
        ));
    }
    
    // Ensure the output directory exists
    if let Some(parent) = output_file_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| GpgError::FileSystemError(e))?;
    }
    
    // Directly clearsign the file to the specified output path
    let clearsign_output = Command::new("gpg")
        .arg("--clearsign")
        .arg("--default-key")
        .arg(signing_key_id)
        .arg("--output")
        .arg(output_file_path)
        .arg(input_file_path)
        .output()
        .map_err(|e| GpgError::GpgOperationError(e.to_string()))?;

    if !clearsign_output.status.success() {
        let error_message = String::from_utf8_lossy(&clearsign_output.stderr);
        return Err(GpgError::GpgOperationError(error_message.to_string()));
    }

    Ok(())
}

/// Decrypts and validates a clearsigned, encrypted file
/// 
/// # Arguments
/// * `encrypted_file_path` - Path to the encrypted .gpg file
/// * `validator_key_id` - GPG key ID to validate the clearsign signature
/// * `output_path` - Where to save the decrypted and verified file
/// 
/// # Returns
/// * `Ok(())` if decryption and validation succeed
/// * `Err(GpgError)` if any operation fails
pub fn decrypt_and_validate_file(
    encrypted_file_path: &Path,
    validator_key_id: &str,
    output_path: &Path,
) -> Result<(), GpgError> {
    // Create temporary paths for intermediate files
    let decrypted_temp_path = create_temp_file_path("decrypted_temp")?;
    
    // First decrypt the file
    decrypt_gpg_file(encrypted_file_path, &decrypted_temp_path)?;
    
    // Then verify the clearsign signature
    verify_clearsign_signature(&decrypted_temp_path, validator_key_id)?;
    
    // If verification succeeded, extract the original content
    extract_verified_content(&decrypted_temp_path, output_path)?;
    
    // Cleanup
    if decrypted_temp_path.exists() {
        fs::remove_file(&decrypted_temp_path)
            .map_err(|e| GpgError::TempFileError(e.to_string()))?;
    }
    
    Ok(())
}

/// Decrypts a GPG encrypted file.
///
/// # Arguments
/// * `encrypted_file_path` - Path to the encrypted GPG file
/// * `output_path` - Path where the decrypted output will be saved
///
/// # Returns
/// * `Ok(())` - If decryption succeeds
/// * `Err(GpgError)` - If any operation fails
///
/// # Notes
/// This function requires that the user has the appropriate private key
/// in their GPG keyring to decrypt the file.
pub fn decrypt_gpg_file(
    encrypted_file_path: &Path,
    output_path: &Path,
) -> Result<(), GpgError> {
    let decrypt_output = Command::new("gpg")
        .arg("--decrypt")
        .arg("--output")
        .arg(output_path)
        .arg(encrypted_file_path)
        .output()
        .map_err(|e| GpgError::DecryptionError(e.to_string()))?;

    if !decrypt_output.status.success() {
        let error_message = String::from_utf8_lossy(&decrypt_output.stderr);
        return Err(GpgError::DecryptionError(error_message.to_string()));
    }

    Ok(())
}

/// Verifies a clearsigned file's signature.
///
/// # Arguments
/// * `clearsigned_file_path` - Path to the clearsigned file
/// * `validator_key_id` - GPG key ID to use for validation
///
/// # Returns
/// * `Ok(())` - If signature validation succeeds
/// * `Err(GpgError)` - If validation fails or any other operation fails
///
/// # Notes
/// This function first checks if the validator key exists in the keyring
/// before attempting to verify the signature.
fn verify_clearsign_signature(
    clearsigned_file_path: &Path,
    validator_key_id: &str,
) -> Result<(), GpgError> {
    // First check if the validator key exists
    if !validate_gpg_key(validator_key_id)? {
        return Err(GpgError::ValidationError(
            format!("Validator key '{}' not found in keyring", validator_key_id)
        ));
    }

    let verify_output = Command::new("gpg")
        .arg("--verify")
        .arg(clearsigned_file_path)
        .output()
        .map_err(|e| GpgError::ValidationError(e.to_string()))?;

    if !verify_output.status.success() {
        let error_message = String::from_utf8_lossy(&verify_output.stderr);
        return Err(GpgError::ValidationError(error_message.to_string()));
    }

    Ok(())
}

/// Extracts the original content from a verified clearsigned file.
///
/// # Arguments
/// * `clearsigned_file_path` - Path to the verified clearsigned file
/// * `output_path` - Path where the extracted content will be saved
///
/// # Returns
/// * `Ok(())` - If content extraction succeeds
/// * `Err(GpgError)` - If any operation fails
///
/// # Notes
/// This function parses the clearsigned file to extract only the content
/// between the PGP header and signature sections
fn extract_verified_content(
    clearsigned_file_path: &Path,
    output_path: &Path,
) -> Result<(), GpgError> {
    // Read the clearsigned file
    let content = fs::read_to_string(clearsigned_file_path)
        .map_err(|e| GpgError::FileSystemError(e))?;
    
    // Extract the content between the clearsign markers
    let content_lines: Vec<&str> = content.lines().collect();
    let mut extracted_content = Vec::new();
    let mut in_content = false;

    for line in content_lines {
        if line.starts_with("-----BEGIN PGP SIGNED MESSAGE-----") {
            in_content = true;
            continue;
        } else if line.starts_with("-----BEGIN PGP SIGNATURE-----") {
            break;
        } else if in_content && !line.starts_with("Hash: ") {
            extracted_content.push(line);
        }
    }

    // Write the extracted content to the output file
    fs::write(output_path, extracted_content.join("\n"))
        .map_err(|e| GpgError::FileSystemError(e))?;

    Ok(())
}

/// Validates that a GPG key ID exists in the keyring.
///
/// # Arguments
/// * `key_id` - The GPG key ID to check for existence
///
/// # Returns
/// * `Ok(bool)` - True if the key exists, false otherwise
/// * `Err(GpgError)` - If there was an error executing the GPG command
///
/// # Example
/// ```no_run
/// let key_exists = validate_gpg_key("3AA5C34371567BD2")?;
/// if key_exists {
///     println!("Key found in keyring");
/// } else {
///     println!("Key not found in keyring");
/// }
/// ```
pub fn validate_gpg_key(key_id: &str) -> Result<bool, GpgError> {
    let validation_output = Command::new("gpg")
        .arg("--list-keys")
        .arg(key_id)
        .output()
        .map_err(|e| GpgError::GpgOperationError(e.to_string()))?;

    Ok(validation_output.status.success())
}

/// Custom error type for GPG operations
#[derive(Debug)]
pub enum GpgError {
    /// Errors related to file system operations
    FileSystemError(std::io::Error),
    /// Errors related to GPG operations
    GpgOperationError(String),
    /// Errors related to temporary file management
    TempFileError(String),
    /// Errors related to path manipulation
    PathError(String),
    /// Errors related to signature validation
    ValidationError(String),
    /// Errors related to decryption
    DecryptionError(String),
}

/// Generates a current Unix timestamp for unique file naming.
///
/// # Returns
/// * `u64` - Current Unix timestamp in seconds
///
/// # Notes
/// This function is used internally to create unique filenames
/// for temporary files used during GPG operations.
fn generate_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Creates a temporary file path with a unique name.
///
/// # Arguments
/// * `prefix` - Prefix to use for the temporary file name
///
/// # Returns
/// * `Ok(PathBuf)` - Path to the new temporary file
/// * `Err(GpgError)` - If there was an error creating the path
///
/// # Notes
/// This function creates the parent directory if it doesn't exist
/// and returns an absolute path to the temporary file.
fn create_temp_file_path(prefix: &str) -> Result<PathBuf, GpgError> {
    let mut temp_dir = std::env::temp_dir();
    let timestamp = generate_timestamp();
    let temp_filename = format!("gpg_temp_{}_{}", timestamp, prefix);
    temp_dir.push(temp_filename);
    
    // Ensure the parent directory exists
    if let Some(parent) = temp_dir.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| GpgError::TempFileError(format!("Failed to create temp directory: {}", e)))?;
    }
    
    Ok(temp_dir)
}

/// Clearsigns a file using your GPG private key.
///
/// # Arguments
/// * `input_file_path` - Path to the file to be clearsigned
/// * `temp_file_path` - Path where the clearsigned output will be saved
/// * `your_key_id` - Your private key ID for signing
///
/// # Returns
/// * `Ok(())` - If clearsigning succeeds
/// * `Err(GpgError)` - If any operation fails
///
/// # Notes
/// This is an internal function used by higher-level functions
/// like `clearsign_and_encrypt_file_for_recipient`.
fn clearsign_file_with_private_key(
    input_file_path: &Path,
    temp_file_path: &Path,
    your_key_id: &str,  // Your private key ID for signing
) -> Result<(), GpgError> {
    let clearsign_output = Command::new("gpg")
        .arg("--clearsign")
        .arg("--default-key")
        .arg(your_key_id)
        .arg("--output")
        .arg(temp_file_path)
        .arg(input_file_path)
        .output()
        .map_err(|e| GpgError::GpgOperationError(e.to_string()))?;

    if !clearsign_output.status.success() {
        let error_message = String::from_utf8_lossy(&clearsign_output.stderr);
        return Err(GpgError::GpgOperationError(error_message.to_string()));
    }

    Ok(())
}

/// Encrypts a file using a recipient's public key file.
///
/// # Arguments
/// * `input_file_path` - Path to the file to be encrypted
/// * `output_file_path` - Path where the encrypted output will be saved
/// * `recipient_public_key_path` - Path to the recipient's public key file
///
/// # Returns
/// * `Ok(())` - If encryption succeeds
/// * `Err(GpgError)` - If any operation fails
///
/// # Notes
/// This function uses GnuPG's "always" trust model to allow
/// encrypting for recipients whose keys might not be fully trusted
/// in the local GPG keyring.
fn encrypt_file_with_public_key(
    input_file_path: &Path,
    output_file_path: &Path,
    recipient_public_key_path: &Path,
) -> Result<(), GpgError> {
    // First, import the recipient's public key for this operation
    let encrypt_output = Command::new("gpg")
        .arg("--encrypt")
        .arg("--trust-model")
        .arg("always")  // Trust the key for this operation
        .arg("--recipient-file")
        .arg(recipient_public_key_path)
        .arg("--output")
        .arg(output_file_path)
        .arg(input_file_path)
        .output()
        .map_err(|e| GpgError::GpgOperationError(e.to_string()))?;

    if !encrypt_output.status.success() {
        let error_message = String::from_utf8_lossy(&encrypt_output.stderr);
        return Err(GpgError::GpgOperationError(error_message.to_string()));
    }

    Ok(())
}

/// Main function to process a file: clearsign with your key and encrypt with recipient's public key
/// Clearsigns a file with your GPG key and encrypts it with a recipient's public key.
///
/// # Overview
/// This function provides a secure way to share files with specific recipients by:
/// 1. Clearsigning the file with your private GPG key (provides authentication)
/// 2. Encrypting the clearsigned file with the recipient's public key (provides confidentiality)
///
/// # Path Handling
/// IMPORTANT: All file paths are processed as follows:
/// - Input file path: Used as provided (should be an absolute path if possible)
/// - Recipient public key path: Used as provided (should be an absolute path if possible)
/// - Output file path: Automatically generated in `{EXECUTABLE_DIR}/invites_updates/outgoing/{original_filename}.gpg`
/// - Temporary files: Created in the system's temporary directory with unique names
///
/// # Process Flow
/// 1. Validates that your signing key exists in the GPG keyring
/// 2. Creates necessary temporary and output directories
/// 3. Clearsigns the input file with your private key
/// 4. Encrypts the clearsigned file with the recipient's public key
/// 5. Cleans up temporary files
/// 6. Saves the final encrypted file to the output location
///
/// # Security Considerations
/// - Uses GPG's "always" trust model for encryption (recipient key doesn't need to be fully trusted)
/// - Creates and cleans up temporary files securely
/// - Does not permanently import recipient keys to your keyring
/// - Verifies key availability before beginning operations
///
/// # Arguments
/// * `input_file_path` - Path to the file to be clearsigned and encrypted
/// * `your_signing_key_id` - Your GPG key ID used for clearsigning (e.g., "7673C969D81E94C63D641CF84ED13C31924928A5")
/// * `recipient_public_key_path` - Path to the recipient's public key file (ASCII-armored format)
///
/// # Returns
/// * `Ok(())` - If the operation completes successfully
/// * `Err(GpgError)` - If any step fails, with detailed error information
///
/// # Errors
/// May return various `GpgError` types:
/// * `GpgError::GpgOperationError` - If GPG operations fail (missing keys, invalid keys, etc.)
/// * `GpgError::FileSystemError` - If file operations fail (permission issues, disk full, etc.)
/// * `GpgError::PathError` - If path operations fail (invalid paths, missing directories, etc.)
/// * `GpgError::TempFileError` - If temporary file operations fail
///
/// # Example
/// ```
/// use std::path::Path;
/// 
/// // Clearsign and encrypt a configuration file for a collaborator
/// let result = clearsign_and_encrypt_file_for_recipient(
///     &Path::new("/path/to/config.toml"),
///     "7673C969D81E94C63D641CF84ED13C31924928A5",  // Your key ID
///     &Path::new("/path/to/recipient_key.asc")
/// );
///
/// match result {
///     Ok(()) => println!("File successfully clearsigned and encrypted"),
///     Err(e) => eprintln!("Error: {}", e.to_string()),
/// }
/// ```
///
/// # Related Functions
/// * `clearsign_file_with_private_key()` - Lower-level function to just clearsign a file
/// * `encrypt_file_with_public_key()` - Lower-level function to just encrypt a file
/// * `validate_gpg_key()` - Used to check if a GPG key exists in the keyring
///
/// # GPG Requirements
/// - GPG must be installed and available in the system PATH
/// - Your private key must be available in your GPG keyring
/// - Your private key should not have a passphrase, or GPG must be configured for non-interactive use
///
pub fn clearsign_and_encrypt_file_for_recipient(
    input_file_path: &Path,
    your_signing_key_id: &str,
    recipient_public_key_path: &Path,
) -> Result<(), GpgError> {
    // First validate that your signing key exists and is available
    if !validate_gpg_key(your_signing_key_id)? {
        return Err(GpgError::GpgOperationError(
            format!("Signing key '{}' not found in keyring", your_signing_key_id)
        ));
    }

    // Create paths for temporary and final files
    let original_filename = input_file_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| GpgError::PathError("Invalid input file name".to_string()))?;
    
    // Create a simple temp file name without directory paths embedded in it
    let clearsigned_temp_path = create_temp_file_path(&format!("clearsigned_{}", original_filename))?;
    
    // Create absolute path for the output directory relative to executable
    let relative_output_dir = "invites_updates/outgoing";
    let absolute_output_dir = gpg_make_input_path_name_abs_executabledirectoryrelative_nocheck(relative_output_dir)
        .map_err(|e| GpgError::PathError(format!("Failed to resolve output directory path: {}", e)))?;
    
    // Ensure the output directory exists
    fs::create_dir_all(&absolute_output_dir)
        .map_err(|e| GpgError::FileSystemError(e))?;
    
    // Create the final output path
    let final_output_path = absolute_output_dir.join(format!("{}.gpg", original_filename));
    
    // Log the paths being used
    println!("Input file: {}", input_file_path.display());
    println!("Temporary clearsigned file: {}", clearsigned_temp_path.display());
    println!("Final output path: {}", final_output_path.display());
    
    // Clearsign with your private key
    clearsign_file_with_private_key(input_file_path, &clearsigned_temp_path, your_signing_key_id)?;
    
    // Encrypt with recipient's public key
    encrypt_file_with_public_key(&clearsigned_temp_path, &final_output_path, recipient_public_key_path)?;

    // Cleanup temporary file
    if clearsigned_temp_path.exists() {
        fs::remove_file(&clearsigned_temp_path)
            .map_err(|e| GpgError::TempFileError(e.to_string()))?;
    }

    // Log completion
    println!("\nSuccessfully completed clearsigning and encryption");
    println!("Output file: {}", final_output_path.display());
    
    Ok(())
}

/// Interactive workflow for decrypting and validating files.
///
/// # Purpose
/// Guides the user through providing necessary information to decrypt
/// and validate a clearsigned, encrypted GPG file.
///
/// # Process
/// 1. Prompts for validator's GPG key ID
/// 2. Validates input parameters
/// 3. Decrypts and validates the file
/// 4. Reports results to the user
///
/// # Returns
/// * `Ok(())` - If the workflow completes successfully
/// * `Err(GpgError)` - If any step fails
///
/// # Notes
/// Uses default file paths for input and output if not specified.
fn decrypt_and_validate_workflow() -> Result<(), GpgError> {
    // Specify the default encrypted file path
    let encrypted_file = Path::new("invites_updates/outgoing/test.toml.gpg");
    
    // Specify where the decrypted and verified file will be saved
    let output_file = Path::new("invites_updates/decrypted_and_verified.toml");

    // Display helpful information about finding GPG key IDs
    println!("\nTo get the validator's key ID, run: $ gpg --list-keys --keyid-format=long");
    print!("Enter validator's GPG key ID: ");
    io::stdout().flush()
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to flush stdout: {}", e)))?;
    
    // Get the validator's key ID from user input
    let mut validator_key_id = String::new();
    io::stdin()
        .read_line(&mut validator_key_id)
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to read input: {}", e)))?;
    let validator_key_id = validator_key_id.trim();

    // Validate that a key ID was provided
    if validator_key_id.is_empty() {
        return Err(GpgError::ValidationError(
            "No validator key ID provided".to_string()
        ));
    }

    // Display the parameters that will be used
    println!("\nProcessing with the following parameters:");
    println!("Encrypted file path: {}", encrypted_file.display());
    println!("Validator key ID: {}", validator_key_id);
    println!("Output file path: {}", output_file.display());

    // Perform the decryption and validation
    decrypt_and_validate_file(encrypted_file, &validator_key_id, output_file)?;
    
    // Confirm successful completion
    println!("\nSuccess: File has been decrypted and signature verified!");
    println!("Decrypted file location: {}", output_file.display());
    
    Ok(())
}

/// Interactive workflow for clearsigning files.
///
/// # Purpose
/// Guides the user through providing necessary information to clearsign
/// a file with their GPG private key.
///
/// # Process
/// 1. Prompts for file path to clearsign
/// 2. Prompts for output file path (with default option)
/// 3. Prompts for signing key ID
/// 4. Validates all inputs
/// 5. Performs the clearsigning operation
/// 6. Reports results to the user
///
/// # Returns
/// * `Ok(())` - If the workflow completes successfully
/// * `Err(GpgError)` - If any step fails
fn clearsign_workflow() -> Result<(), GpgError> {
    // Get input file path from user
    print!("Enter the path to the file you want to clearsign: ");
    io::stdout().flush()
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to flush stdout: {}", e)))?;
    
    let mut input_file_path_str = String::new();
    io::stdin()
        .read_line(&mut input_file_path_str)
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to read input: {}", e)))?;
    let input_file_path = Path::new(input_file_path_str.trim());
    
    // Validate input file exists
    if !input_file_path.exists() {
        return Err(GpgError::FileSystemError(
            std::io::Error::new(std::io::ErrorKind::NotFound, "Input file not found")
        ));
    }
    
    // Get output file path (use default if empty)
    print!("Enter the output file path (or press Enter for default): ");
    io::stdout().flush()
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to flush stdout: {}", e)))?;
    
    let mut output_file_path_str = String::new();
    io::stdin()
        .read_line(&mut output_file_path_str)
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to read input: {}", e)))?;
    
    let output_file_path = if output_file_path_str.trim().is_empty() {
        // Create default output file path with .asc extension
        let input_filename = input_file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| GpgError::PathError("Invalid input file name".to_string()))?;
            
        let mut output_path = PathBuf::from("clearsigned");
        fs::create_dir_all(&output_path)
            .map_err(|e| GpgError::FileSystemError(e))?;
        output_path.push(format!("{}.asc", input_filename));
        output_path
    } else {
        PathBuf::from(output_file_path_str.trim())
    };
    
    // Get signing key ID
    println!("\nTo get your signing key ID, run: $ gpg --list-secret-keys --keyid-format=long");
    print!("Enter your GPG signing key ID: ");
    io::stdout().flush()
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to flush stdout: {}", e)))?;
    
    let mut signing_key_id = String::new();
    io::stdin()
        .read_line(&mut signing_key_id)
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to read input: {}", e)))?;
    let signing_key_id = signing_key_id.trim();
    
    // Validate that a key ID was provided
    if signing_key_id.is_empty() {
        return Err(GpgError::ValidationError(
            "No signing key ID provided".to_string()
        ));
    }
    
    // Display the parameters that will be used
    println!("\nProcessing with the following parameters:");
    println!("Input file path: {}", input_file_path.display());
    println!("Output file path: {}", output_file_path.display());
    println!("Signing key ID: {}", signing_key_id);
    
    // Perform the clearsigning
    clearsign_file(input_file_path, &output_file_path, &signing_key_id)?;
    
    // Confirm successful completion
    println!("\nSuccess: File has been clearsigned!");
    println!("Clearsigned file location: {}", output_file_path.display());
    
    Ok(())
}

/// Interactive workflow for clearsigning and encrypting files.
///
/// # Purpose
/// Guides the user through providing necessary information to clearsign
/// a file with their GPG private key and encrypt it for a recipient.
///
/// # Process
/// 1. Prompts for file path to process
/// 2. Prompts for signing key ID
/// 3. Prompts for recipient's public key file path
/// 4. Validates all inputs
/// 5. Performs clearsigning and encryption
/// 6. Reports results to the user
///
/// # Returns
/// * `Ok(())` - If the workflow completes successfully
/// * `Err(GpgError)` - If any step fails
///
/// # Notes
/// Output is saved to invites_updates/outgoing/ directory.
fn clearsign_and_encrypt_workflow() -> Result<(), GpgError> {
    // Get input file path from user
    print!("Enter the path to the file you want to clearsign and encrypt: ");
    io::stdout().flush()
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to flush stdout: {}", e)))?;
    
    let mut input_file_path_str = String::new();
    io::stdin()
        .read_line(&mut input_file_path_str)
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to read input: {}", e)))?;
    let input_file_path = Path::new(input_file_path_str.trim());
    
    // Validate input file exists
    if !input_file_path.exists() {
        return Err(GpgError::FileSystemError(
            std::io::Error::new(std::io::ErrorKind::NotFound, "Input file not found")
        ));
    }
    
    // Get signing key ID
    println!("\nTo get your signing key ID, run: $ gpg --list-secret-keys --keyid-format=long");
    print!("Enter your GPG signing key ID: ");
    io::stdout().flush()
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to flush stdout: {}", e)))?;
    
    let mut signing_key_id = String::new();
    io::stdin()
        .read_line(&mut signing_key_id)
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to read input: {}", e)))?;
    let signing_key_id = signing_key_id.trim();
    
    // Validate that a key ID was provided
    if signing_key_id.is_empty() {
        return Err(GpgError::ValidationError(
            "No signing key ID provided".to_string()
        ));
    }
    
    // Get recipient's public key path
    print!("Enter path to recipient's public key file: ");
    io::stdout().flush()
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to flush stdout: {}", e)))?;
    
    let mut recipient_key_path_str = String::new();
    io::stdin()
        .read_line(&mut recipient_key_path_str)
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to read input: {}", e)))?;
    let recipient_key_path = Path::new(recipient_key_path_str.trim());
    
    // Validate recipient key exists
    if !recipient_key_path.exists() {
        return Err(GpgError::FileSystemError(
            std::io::Error::new(std::io::ErrorKind::NotFound, "Recipient key file not found")
        ));
    }
    
    // Display the parameters that will be used
    println!("\nProcessing with the following parameters:");
    println!("Input file path: {}", input_file_path.display());
    println!("Signing key ID: {}", signing_key_id);
    println!("Recipient public key path: {}", recipient_key_path.display());
    println!("Output will be saved to: invites_updates/outgoing/");
    
    // Perform the clearsigning and encryption
    clearsign_and_encrypt_file_for_recipient(
        input_file_path,
        &signing_key_id,
        recipient_key_path
    )?;
    
    // Calculate the output path for display
    let original_filename = input_file_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| GpgError::PathError("Invalid input file name".to_string()))?;
    
    let output_path = PathBuf::from(format!("invites_updates/outgoing/{}.gpg", original_filename));
    
    // Confirm successful completion
    println!("\nSuccess: File has been clearsigned and encrypted!");
    println!("Encrypted file location: {}", output_path.display());
    
    Ok(())
}

/// Main entry point for GPG file decryption and validation.
/// 
/// # Purpose
/// Provides an interactive command-line interface for decrypting and validating
/// GPG encrypted files that have been clearsigned.
/// 
/// # Process
/// 1. Prompts for necessary GPG key information
/// 2. Validates input parameters
/// 3. Decrypts the specified encrypted file
/// 4. Verifies the clearsign signature
/// 5. Outputs the decrypted and verified file
/// 
/// # Arguments
/// None - Interactive prompts gather needed information
/// 
/// # Returns
/// * `Ok(())` - Operation completed successfully
/// * `Err(GpgError)` - Operation failed with specific error details
/// 
/// # Example Usage
/// ```no_run
/// fn main() -> Result<(), GpgError> {
///     // ... function contents ...
/// }
/// ```
/// 
/// # Notes
/// - Requires GPG to be installed and configured
/// - Requires appropriate private keys to be available in the GPG keyring
/// - Default input file location: invites_updates/outgoing/*.gpg
pub fn rust_gpg_tools_interface() -> Result<(), GpgError> {
    // Ask user which operation they want to perform
    println!("GPG File Processing Utility");
    println!("---------------------------");
    println!("1. Decrypt and validate an encrypted file");
    println!("2. Clearsign a file");
    println!("3. Clearsign and encrypt a file for a recipient");
    
    print!("\nSelect an operation (1-3): ");
    io::stdout().flush()
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to flush stdout: {}", e)))?;
    
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .map_err(|e| GpgError::GpgOperationError(format!("Failed to read input: {}", e)))?;
    
    match operation.trim() {
        "1" => decrypt_and_validate_workflow()?,
        "2" => clearsign_workflow()?,
        "3" => clearsign_and_encrypt_workflow()?,
        _ => return Err(GpgError::ValidationError("Invalid selection".to_string())),
    }
    
    Ok(())
}

// helpers


/// Gets the directory where the current executable is located.
///
/// # Returns
///
/// * `Result<PathBuf, io::Error>` - The absolute directory path containing the executable or an error
///   if it cannot be determined.
pub fn gpg_get_absolute_path_to_executable_parentdirectory() -> Result<PathBuf, io::Error> {
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
pub fn gpg_make_input_path_name_abs_executabledirectoryrelative_nocheck<P: AsRef<Path>>(path_to_make_absolute: P) -> Result<PathBuf, io::Error> {
    // Get the directory where the executable is located
    let executable_directory = gpg_get_absolute_path_to_executable_parentdirectory()?;
    
    // Create a path by joining the executable directory with the provided path
    let target_path = executable_directory.join(path_to_make_absolute);
    
    // If the path doesn't exist, we still return the absolute path without trying to canonicalize
    if !gpg_abs_executable_directory_relative_exists(&target_path)? {
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
pub fn gpg_abs_executable_directory_relative_exists<P: AsRef<Path>>(path_to_check: P) -> Result<bool, io::Error> {
    let path = path_to_check.as_ref();
    Ok(path.exists())
}

/// Store the entire clearsigned file (still in clearsigned format with all signatures intact) to the output path
/// Decrypts a GPG-encrypted file containing a clearsigned document, verifies the signature,
/// and stores the entire clearsigned file (still clearsigned) to the output path.
///
/// # Arguments
/// * `incoming_gpg_encrypted_path` - Path to the GPG-encrypted file
/// * `gpg_key_id` - GPG key ID to verify the clearsign signature
/// * `output_verified_clearsign_path` - Path where to store the verified clearsigned file
///
/// # Returns
/// * `Result<(), GpgError>` - Success or failure
///
/// # Description
/// This function:
/// 1. Decrypts the GPG-encrypted file
/// 2. Verifies the clearsign signature within the decrypted content
/// 3. Stores the entire verified clearsigned document (with signatures intact)
///    to the specified output path
///
/// Unlike other functions that extract content from clearsigned files,
/// this function preserves the entire clearsigned structure including
/// signature blocks.
pub fn extract_verify_store_gpg_encrypted_clearsign_toml(
    incoming_gpg_encrypted_path: &Path,
    gpg_key_id: &str,
    output_verified_clearsign_path: &Path,
) -> Result<(), GpgError> {
    // Step 1: Create a temporary path for the decrypted file
    let decrypted_temp_path = create_temp_file_path("decrypted_clearsign")?;
    
    // Step 2: Decrypt the GPG file
    decrypt_gpg_file(incoming_gpg_encrypted_path, &decrypted_temp_path)?;
    
    // Step 3: Verify the clearsign signature
    verify_clearsign_signature(&decrypted_temp_path, gpg_key_id)?;
    
    // Step 4: If verification succeeded, ensure the output directory exists
    if let Some(parent) = output_verified_clearsign_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| GpgError::FileSystemError(e))?;
    }
    
    // Step 5: Copy the entire clearsigned file to the output location
    fs::copy(&decrypted_temp_path, output_verified_clearsign_path)
        .map_err(|e| GpgError::FileSystemError(e))?;
    
    // Step 6: Clean up the temporary file
    if decrypted_temp_path.exists() {
        fs::remove_file(&decrypted_temp_path)
            .map_err(|e| GpgError::TempFileError(e.to_string()))?;
    }
    
    // Success - the verified clearsigned file has been stored to the output path
    println!("Successfully verified and stored clearsigned file to: {}", 
             output_verified_clearsign_path.display());
    
    Ok(())
}

pub fn manual_q_and_a_new_encrypted_clearsigntoml_verification() -> Result<(), String> {
    println!("GPG Encrypted Clearsigned TOML File Processor");
    println!("---------------------------------------------");
    
    // Get input file path from user
    print!("Enter path to the GPG-encrypted file: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).map_err(|e| e.to_string())?;
    let input_path = Path::new(input_path.trim());
    
    // Verify the file exists
    if !input_path.exists() {
        return Err(format!("Error: File not found at {}", input_path.display()));
    }
    
    // Get GPG key ID for verification
    println!("\nTo find your GPG key ID, run: gpg --list-keys --keyid-format=long");
    print!("Enter the GPG key ID to verify the signature: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut key_id = String::new();
    io::stdin().read_line(&mut key_id).map_err(|e| e.to_string())?;
    let key_id = key_id.trim();
    
    // Get output file path
    print!("Enter path where to save the verified clearsigned file: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut output_path = String::new();
    io::stdin().read_line(&mut output_path).map_err(|e| e.to_string())?;
    let output_path = Path::new(output_path.trim());
    
    // Display the parameters
    println!("\nProcessing with the following parameters:");
    println!("Input encrypted file: {}", input_path.display());
    println!("GPG key ID for verification: {}", key_id);
    println!("Output path for verified clearsigned file: {}", output_path.display());
    println!("\nProcessing...");
    
    // Call the function from the module
    extract_verify_store_gpg_encrypted_clearsign_toml(
        input_path,
        key_id,
        output_path
    ).map_err(|e| e.to_string())?;
    
    println!("Done! The verified clearsigned file has been saved.");
    Ok(())
}