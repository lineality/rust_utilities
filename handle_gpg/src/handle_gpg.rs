//! handle_gpp.rs
//! GPG handling module for clearsigning and encrypting files.
//! This module provides functionality to clearsign files with your private key
//! and encrypt them with a recipient's public key file.
/*
```markdown
# GPG File Processing Module Documentation
`handle_gpg.rs` - Version 1.0

## Overview
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

mod handle_gpg;  // This declares the module and tells Rust to look for handle_gpg.rs
use crate::handle_gpg::{
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

use std::io::{self, Write};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

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

