
use std::path::Path;
use std::io::{self, Write};

mod handle_gpg;  // This declares the module and tells Rust to look for handle_gpg.rs
use crate::handle_gpg::{
    GpgError, 
    clearsign_and_encrypt_file_for_recipient, 
    // validate_gpg_key
};  // Import the specific items we need


fn main() -> Result<(), GpgError> {
    let input_file = Path::new("test.toml");

    // First prompt - for signing key
    println!("\nTo get your signing key ID, run: $ gpg --list-secret-keys --keyid-format=long");
    print!("Enter your GPG signing key ID: ");
    io::stdout().flush().expect("Failed to flush stdout");
    
    let mut your_signing_key_id = String::new();
    io::stdin()
        .read_line(&mut your_signing_key_id)
        .expect("Failed to read key ID");
    let your_signing_key_id = your_signing_key_id.trim();

    if your_signing_key_id.is_empty() {
        return Err(GpgError::GpgOperationError("No signing key ID provided".to_string()));
    }

    // Second prompt - for recipient's public key path
    println!("\nTo export recipient's public key: $ gpg --armor --export KEYID > public_key.asc");
    print!("Enter path to recipient's public key file (e.g., invites_updates/incoming/public_key.asc): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut recipient_key_path = String::new();
    io::stdin()
        .read_line(&mut recipient_key_path)
        .expect("Failed to read path");
    let recipient_key_path = recipient_key_path.trim();

    if recipient_key_path.is_empty() {
        return Err(GpgError::GpgOperationError("No public key path provided".to_string()));
    }

    let recipient_public_key = Path::new(recipient_key_path);

    // Verify the public key file exists 
    if !recipient_public_key.exists() {
        return Err(GpgError::PathError(format!(
            "Public key file not found at: {}", 
            recipient_key_path
        )));
    }

    println!("\nProcessing with:");
    println!("Signing Key ID: {}", your_signing_key_id);
    println!("Recipient's public key: {}", recipient_key_path);
    println!("Input file: {}", input_file.display());

    clearsign_and_encrypt_file_for_recipient(input_file, your_signing_key_id, recipient_public_key)?;
    println!("\nOperation completed successfully!");
    Ok(())
}