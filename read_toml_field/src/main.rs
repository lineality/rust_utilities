use std::fs::File;
use std::io::{self, BufRead};

/// The function reads a single line from a TOML file that starts with a specified field name
/// and ends with a value. The function returns an empty string if the field is not found, and
/// does not panic or unwrap in case of errors. The function uses only standard Rust libraries
/// and does not introduce unnecessary dependencies.
///
/// design:
/// 0. start with an empty string to return by default
/// 1. get file at path
/// 2. open as text
/// 3. iterate through rows
/// 4. look for filed name as start of string the " = "
/// 5. grab that whole row of text
/// 6. remove "fieldname = " from the beginning
/// 7. remove '" ' and trailing spaces from the end
/// 8. return that string, if any
/// by default, return an empty string, if anything goes wrong, 
/// handle the error, and return an empty string
///
/// requires:
/// use std::fs::File;
/// use std::io::{self, BufRead};
///
/// example use:
///     let value = read_field_from_toml("test.toml", "fieldname");
///
fn read_field_from_toml(path: &str, field_name: &str) -> String {
    // Validate input parameters
    if path.is_empty() || field_name.is_empty() {
        println!("Error: Empty path or field name provided");
        return String::new();
    }

    // Verify file extension
    if !path.to_lowercase().ends_with(".toml") {
        println!("Warning: File does not have .toml extension: {}", path);
    }

    // Debug print statement
    println!("Attempting to open file at path: {}", path);

    // Open the file at the specified path
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            // More detailed error reporting
            println!("Failed to open file at path: {}. Error: {}", path, e);
            return String::new();
        },
    };

    // Debug print statement
    println!("Successfully opened file at path: {}", path);

    // Create a buffered reader to read the file line by line
    let reader = io::BufReader::new(file);

    // Keep track of line numbers for better error reporting
    let mut line_number = 0;

    // Iterate through each line in the file
    for line_result in reader.lines() {
        line_number += 1;

        // Handle line reading errors
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                println!("Error reading line {}: {}", line_number, e);
                continue;
            }
        };

        // Skip empty lines and comments
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }

        // Debug print statement
        println!("Processing line {}: {}", line_number, line);

        // Check if line starts with field name
        if line.trim_start().starts_with(field_name) {
            // Debug print statement
            println!("Found field '{}' on line {}", field_name, line_number);

            // Split the line by '=' and handle malformed lines
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                println!("Malformed TOML line {} - missing '=': {}", line_number, line);
                continue;
            }

            let key = parts[0].trim();
            let value = parts[1].trim();

            // Verify exact field name match (avoiding partial matches)
            if key != field_name {
                continue;
            }

            // Handle empty values
            if value.is_empty() {
                println!("Warning: Empty value found for field '{}'", field_name);
                return String::new();
            }

            // Debug print statement
            println!("Extracted value: {}", value);

            // Clean up the value: remove quotes and trim spaces
            let cleaned_value = value.trim().trim_matches('"').trim();
            
            // Verify the cleaned value isn't empty
            if cleaned_value.is_empty() {
                println!("Warning: Value became empty after cleaning for field '{}'", field_name);
                return String::new();
            }

            return cleaned_value.to_string();
        }
    }

    // If we get here, the field wasn't found
    println!("Field '{}' not found in file", field_name);
    String::new()
}

fn main() {
    let value = read_field_from_toml("test.toml", "fieldname");
    println!("Field value -> {}", value);
}
