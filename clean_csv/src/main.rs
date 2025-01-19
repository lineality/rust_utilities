//! # CSV Cleaner for Multi-line Records
//! This module provides functionality to clean CSV files where records span multiple lines,
//! converting them into a standard single-line-per-record format.
//! 
//! ## Use Case
//! Particularly useful for academic paper datasets where abstracts or other text fields
//! may span multiple lines, making standard CSV processing difficult.
//! 
//! ## Process
//! 1. Reads original CSV with multi-line records
//! 2. Identifies record boundaries using ID field
//! 3. Combines multi-line records into single lines
//! 4. Writes cleaned CSV with one record per line

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

/// Cleans a CSV file by combining multi-line records into single lines
/// 
/// # Arguments
/// * `input_path` - Path to the input CSV file with multi-line records
/// * `output_path` - Path where the cleaned CSV will be written
/// 
/// # Returns
/// * `io::Result<()>` - Success or error status of the cleaning operation
/// 
/// # Process
/// 1. Preserves the header row unchanged
/// 2. Identifies new records by checking for numeric ID at start
/// 3. Combines continuation lines with their parent record
/// 4. Writes each complete record as a single line
/// 
/// # Example
/// ```
/// clean_csv_file("input.csv", "cleaned_output.csv")?;
///
/// # Error Handling
/// The code uses Rust's Result type for error handling:
/// - File operations (open, create) may fail
/// - Reading lines may fail
/// - Writing output may fail
/// 
/// # Performance Considerations
/// - Uses buffered reading for efficient file I/O
/// - Processes one line at a time to manage memory
/// - Joins record lines only when ready to write
/// 
/// # Limitations
/// - Assumes first field is numeric ID
/// - Assumes header row exists
/// - Joins lines with spaces (may need adjustment for specific formats)
fn clean_csv_file(input_path: &str, output_path: &str) -> io::Result<()> {
    // Open input file with buffered reading for efficiency
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    
    // Create output file
    let mut writer = File::create(output_path)?;
    
    // Create iterator over lines
    let mut lines = reader.lines();
    
    // Handle header separately to preserve CSV structure
    if let Some(Ok(header)) = lines.next() {
        writeln!(writer, "{}", header)?;
    }

    // Buffer for building complete records
    let mut current_record = Vec::new();
    // Flag to handle first record specially
    let mut is_first = true;

    // Process all remaining lines
    for line in lines {
        let line = line?;
        
        // Check if line starts a new record by looking for numeric ID
        let is_new_record = line
            .split(',')
            .next()  // Get first field
            .and_then(|s| s.parse::<u32>().ok())  // Try to parse as number
            .is_some();  // Check if successful
        
        if is_new_record {
            // Write previous record if it exists
            if !current_record.is_empty() {
                // Combine all lines of the record with spaces
                let combined = current_record.join(" ");
                // Don't write first empty record
                if !is_first {
                    writeln!(writer, "{}", combined)?;
                }
                current_record.clear();
                is_first = false;
            }
            // Start new record
            current_record.push(line);
        } else {
            // Add continuation line to current record
            if !current_record.is_empty() {
                current_record.push(line);
            }
        }
    }

    // Handle final record
    if !current_record.is_empty() {
        let combined = current_record.join(" ");
        writeln!(writer, "{}", combined)?;
    }

    Ok(())
}

/// Inspects a CSV file by printing its first few records
/// 
/// # Arguments
/// * `path` - Path to the CSV file to inspect
/// 
/// # Returns
/// * `io::Result<()>` - Success or error status of the inspection
/// 
/// # Output
/// Prints first 5 records of the CSV file for inspection
fn inspect_csv(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    println!("Inspecting cleaned CSV:");
    // Print first 5 lines for inspection
    for (i, line) in reader.lines().take(5).enumerate() {
        println!("Record {}: {:?}", i, line?);
    }
    Ok(())
}


/// Inspect Old, Run, Inspect New
fn main() -> io::Result<()> {
    // Inspect original CSV structure
    println!("Inspecting train.csv structure:");
    inspect_csv("data_files/train.csv")?;
        
    // Clean the CSV
    println!("Cleaning CSV file...");
    clean_csv_file(
        "data_files/train.csv",
        "data_files/train_cleaned.csv"
    )?;
    println!("CSV cleaning complete. Output saved to train_cleaned.csv");

    // Inspect cleaned CSV to verify results
    println!("Inspecting cleaned CSV structure:");
    inspect_csv("data_files/train_cleaned.csv")?;
    
    Ok(())
}

