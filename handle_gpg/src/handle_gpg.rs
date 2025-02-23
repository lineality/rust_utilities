//! GPG handling module for clearsigning and encrypting files.
//! This module provides functionality to clearsign files with your private key
//! and encrypt them with a recipient's public key file.

// use std::fs;
// use std::path::{Path, PathBuf};
// use std::process::Command;
// use std::time::{SystemTime, UNIX_EPOCH};

// /// Custom error type for GPG operations
// #[derive(Debug)]
// pub enum GpgError {
//     /// Errors related to file system operations
//     FileSystemError(std::io::Error),
//     /// Errors related to GPG operations
//     GpgOperationError(String),
//     /// Errors related to temporary file management
//     TempFileError(String),
//     /// Errors related to path manipulation
//     PathError(String),
// }

// /// Generate a current Unix timestamp for unique file naming
// fn generate_timestamp() -> u64 {
//     SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap_or_default()
//         .as_secs()
// }

// /// Creates a temporary file path with a unique name
// fn create_temp_file_path(original_filename: &str) -> Result<PathBuf, GpgError> {
//     let mut temp_dir = std::env::temp_dir();
//     let timestamp = generate_timestamp();
//     let temp_filename = format!("gpg_temp_{}_{}", timestamp, original_filename);
//     temp_dir.push(temp_filename);
//     Ok(temp_dir)
// }

// /// Clearsigns a file using GPG with the specified private key
// fn clearsign_file(
//     input_file_path: &Path,
//     temp_file_path: &Path,
//     signing_key_id: &str,
// ) -> Result<(), GpgError> {
//     let clearsign_output = Command::new("gpg")
//         .arg("--clearsign")
//         .arg("--default-key")
//         .arg(signing_key_id)
//         .arg("--output")
//         .arg(temp_file_path)
//         .arg(input_file_path)
//         .output()
//         .map_err(|e| GpgError::GpgOperationError(e.to_string()))?;

//     if !clearsign_output.status.success() {
//         let error_message = String::from_utf8_lossy(&clearsign_output.stderr);
//         return Err(GpgError::GpgOperationError(error_message.to_string()));
//     }

//     Ok(())
// }

// /// Encrypts a file using GPG with the specified recipient's public key
// fn encrypt_file(
//     input_file_path: &Path,
//     output_file_path: &Path,
//     recipient_key_id: &str,
// ) -> Result<(), GpgError> {
//     let encrypt_output = Command::new("gpg")
//         .arg("--encrypt")
//         .arg("--recipient")
//         .arg(recipient_key_id)
//         .arg("--output")
//         .arg(output_file_path)
//         .arg(input_file_path)
//         .output()
//         .map_err(|e| GpgError::GpgOperationError(e.to_string()))?;

//     if !encrypt_output.status.success() {
//         let error_message = String::from_utf8_lossy(&encrypt_output.stderr);
//         return Err(GpgError::GpgOperationError(error_message.to_string()));
//     }

//     Ok(())
// }

// /// Main function to process a file: clearsign and then encrypt it
// pub fn process_gpg_file(
//     input_file_path: &Path,
//     signing_key_id: &str,
//     recipient_key_id: &str,
// ) -> Result<(), GpgError> {
//     // Create paths for temporary and final files
//     let original_filename = input_file_path
//         .file_name()
//         .and_then(|n| n.to_str())
//         .ok_or_else(|| GpgError::PathError("Invalid input file name".to_string()))?;

//     let clearsigned_temp_path = create_temp_file_path(&format!("clearsigned_{}", original_filename))?;
    
//     let mut final_output_path = PathBuf::from("invites_updates/outgoing");
//     fs::create_dir_all(&final_output_path)
//         .map_err(|e| GpgError::FileSystemError(e))?;
//     final_output_path.push(format!("{}.gpg", original_filename));

//     // Perform clearsigning
//     clearsign_file(input_file_path, &clearsigned_temp_path, signing_key_id)?;

//     // Encrypt the clearsigned file
//     encrypt_file(&clearsigned_temp_path, &final_output_path, recipient_key_id)?;

//     // Cleanup temporary file
//     if clearsigned_temp_path.exists() {
//         fs::remove_file(&clearsigned_temp_path)
//             .map_err(|e| GpgError::TempFileError(e.to_string()))?;
//     }

//     Ok(())
// }

/// Validates that a GPG key ID exists in the keyring
pub fn validate_gpg_key(key_id: &str) -> Result<bool, GpgError> {
    let validation_output = Command::new("gpg")
        .arg("--list-keys")
        .arg(key_id)
        .output()
        .map_err(|e| GpgError::GpgOperationError(e.to_string()))?;

    Ok(validation_output.status.success())
}




use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

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
pub fn process_gpg_file(
    input_file_path: &Path,
    your_signing_key_id: &str,
    recipient_public_key_path: &Path,
) -> Result<(), GpgError> {
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