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