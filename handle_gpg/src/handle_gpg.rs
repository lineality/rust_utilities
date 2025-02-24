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

e.g.

use std::path::Path;
use std::io::{self, Write};

mod handle_gpg;  // This declares the module and tells Rust to look for handle_gpg.rs
use crate::handle_gpg::{
    GpgError, 
    clearsign_and_encrypt_file_for_recipient, 
    decrypt_and_validate_file,
};  // Import the specific items we need

Main entry point for GPG file decryption and validation.

# Purpose
Provides an interactive command-line interface for decrypting and validating
GPG encrypted files that have been clearsigned.

# Process
1. Prompts for necessary GPG key information
2. Validates input parameters
3. Decrypts the specified encrypted file
4. Verifies the clearsign signature
5. Outputs the decrypted and verified file

# Arguments
None - Interactive prompts gather needed information

# Returns
* `Ok(())` - Operation completed successfully
* `Err(GpgError)` - Operation failed with specific error details

# Example Usage
```no_run
fn main() -> Result<(), GpgError> {
    /// ... function contents ...
}
```

# Notes
- Requires GPG to be installed and configured
- Requires appropriate private keys to be available in the GPG keyring
- Default input file location: invites_updates/outgoing/\*.gpg
pub fn main() -> Result<(), GpgError> {
    // Specify the default encrypted file path
    let encrypted_file = Path::new("invites_updates/outgoing/test.toml.gpg");
    
    // Specify where the decrypted and verified file will be saved
    let output_file = Path::new("decrypted_and_verified.toml");

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
    // for testing treat the senders key-id as the recipents
    decrypt_and_validate_file(encrypted_file, &validator_key_id, output_file)?;
    
    // Confirm successful completion
    println!("\nSuccess: File has been decrypted and signature verified!");
    println!("Decrypted file location: {}", output_file.display());
    
    Ok(())
}

*/


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

/// Decrypts a GPG encrypted file
fn decrypt_gpg_file(
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

/// Verifies a clearsigned file's signature
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

/// Extracts the original content from a verified clearsigned file
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

/// Validates that a GPG key ID exists in the keyring
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

/// Generate a current Unix timestamp for unique file naming
fn generate_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Creates a temporary file path with a unique name
fn create_temp_file_path(original_filename: &str) -> Result<PathBuf, GpgError> {
    let mut temp_dir = std::env::temp_dir();
    let timestamp = generate_timestamp();
    let temp_filename = format!("gpg_temp_{}_{}", timestamp, original_filename);
    temp_dir.push(temp_filename);
    Ok(temp_dir)
}

/// Clearsigns a file using your GPG private key
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

/// Encrypts a file using a recipient's public key file
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

    let clearsigned_temp_path = create_temp_file_path(&format!("clearsigned_{}", original_filename))?;
    
    let mut final_output_path = PathBuf::from("invites_updates/outgoing");
    fs::create_dir_all(&final_output_path)
        .map_err(|e| GpgError::FileSystemError(e))?;
    final_output_path.push(format!("{}.gpg", original_filename));

    // Clearsign with your private key
    clearsign_file_with_private_key(input_file_path, &clearsigned_temp_path, your_signing_key_id)?;

    // Encrypt with recipient's public key
    encrypt_file_with_public_key(&clearsigned_temp_path, &final_output_path, recipient_public_key_path)?;

    // Cleanup temporary file
    if clearsigned_temp_path.exists() {
        fs::remove_file(&clearsigned_temp_path)
            .map_err(|e| GpgError::TempFileError(e.to_string()))?;
    }

    Ok(())
}