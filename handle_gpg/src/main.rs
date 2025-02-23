mod handle_gpg;  // This declares the module and tells Rust to look for handle_gpg.rs

use std::path::Path;
use crate::handle_gpg::{
    GpgError, 
    process_gpg_file, 
    // validate_gpg_key
};  // Import the specific items we need

// fn main() -> Result<(), GpgError> {
//     let input_file = Path::new("test.toml");
//     let signing_key_id = "7673C969D81E94C63D641CF84ED13C31924928A5";
//     let recipient_key_id = "7673C969D81E94C63D641CF84ED13C31924928A5";

//     // Validate keys before processing
//     if !validate_gpg_key(signing_key_id)? {
//         return Err(GpgError::GpgOperationError("Invalid signing key".to_string()));
//     }
//     if !validate_gpg_key(recipient_key_id)? {
//         return Err(GpgError::GpgOperationError("Invalid recipient key".to_string()));
//     }

//     process_gpg_file(input_file, signing_key_id, recipient_key_id)?;
//     Ok(())
// }


fn main() -> Result<(), GpgError> {
    let input_file = Path::new("test.toml");
    let your_signing_key_id = "YOUR KEY";  // Your key ID for signing
    
    println!("To get key: $ gpg --list-secret-keys --keyid-format=long");
    println!("$ gpg --armor --export KEYID > public_key.asc");
    
    /*
    
    $ gpg --list-secret-keys --keyid-format=long
    $ gpg --armor --export KEYID > public_key.asc
    
    */
    
    // path/to/recipients/public_key.gpg
    let recipient_public_key = Path::new(
        "invites_updates/incoming/public_key.asc"
    );  // path/to recipient's public key file

    process_gpg_file(input_file, your_signing_key_id, recipient_public_key)?;
    Ok(())
}