
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
/// Main entry point for the GPG tools application.
///
/// # Purpose
/// Initializes and runs the interactive GPG tools interface that allows
/// users to perform various GPG operations including clearsigning,
/// encrypting, and decrypting files.
///
/// # Process Flow
/// 1. Calls the GPG tools interface function
/// 2. Handles any errors that occur during execution
/// 3. Exits with appropriate status code
///
/// # Returns
/// * `Result<(), GpgError>` - Either successful completion or an error
///
/// # Error Handling
/// Any errors from the GPG operations are propagated up and returned
/// from this function, which the Rust runtime will print to stderr.
pub fn main() -> Result<(), GpgError> {
    // Call the GPG tools interface and return its result
    rust_gpg_tools_interface()
}
