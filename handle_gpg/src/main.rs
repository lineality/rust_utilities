
use std::path::Path;
use std::io::{self, Write};

mod handle_gpg;  // This declares the module and tells Rust to look for handle_gpg.rs
use crate::handle_gpg::{
    GpgError, 
    clearsign_and_encrypt_file_for_recipient, 
    decrypt_and_validate_file,
};  // Import the specific items we need


// fn main() -> Result<(), GpgError> {
//     let input_file = Path::new("test.toml");

//     // First prompt - for signing key
//     println!("\nTo get your signing key ID, run: $ gpg --list-secret-keys --keyid-format=long");
//     print!("Enter your GPG signing key ID: ");
//     io::stdout().flush().expect("Failed to flush stdout");
    
//     let mut your_signing_key_id = String::new();
//     io::stdin()
//         .read_line(&mut your_signing_key_id)
//         .expect("Failed to read key ID");
//     let your_signing_key_id = your_signing_key_id.trim();

//     if your_signing_key_id.is_empty() {
//         return Err(GpgError::GpgOperationError("No signing key ID provided".to_string()));
//     }

//     // Second prompt - for recipient's public key path
//     println!("\nTo export recipient's public key: $ gpg --armor --export KEYID > public_key.asc");
//     print!("Enter path to recipient's public key file (e.g., invites_updates/incoming/public_key.asc): ");
//     io::stdout().flush().expect("Failed to flush stdout");

//     let mut recipient_key_path = String::new();
//     io::stdin()
//         .read_line(&mut recipient_key_path)
//         .expect("Failed to read path");
//     let recipient_key_path = recipient_key_path.trim();

//     if recipient_key_path.is_empty() {
//         return Err(GpgError::GpgOperationError("No public key path provided".to_string()));
//     }

//     let recipient_public_key = Path::new(recipient_key_path);

//     // Verify the public key file exists 
//     if !recipient_public_key.exists() {
//         return Err(GpgError::PathError(format!(
//             "Public key file not found at: {}", 
//             recipient_key_path
//         )));
//     }

//     println!("\nProcessing with:");
//     println!("Signing Key ID: {}", your_signing_key_id);
//     println!("Recipient's public key: {}", recipient_key_path);
//     println!("Input file: {}", input_file.display());

//     clearsign_and_encrypt_file_for_recipient(input_file, your_signing_key_id, recipient_public_key)?;
//     println!("\nOperation completed successfully!");
//     Ok(())
// }


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