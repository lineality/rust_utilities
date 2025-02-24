
use std::path::Path;
use std::io::{self, Write};

mod handle_gpg;  // This declares the module and tells Rust to look for handle_gpg.rs
use crate::handle_gpg::{
    GpgError, 
    clearsign_and_encrypt_file_for_recipient, 
    decrypt_and_validate_file,
};  // Import the specific items we need

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