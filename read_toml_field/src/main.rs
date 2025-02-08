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

/// Reads all fields from a TOML file that share a common base name (prefix before underscore)
/// and returns a vector of their values. Returns an empty vector if no matching fields are found
/// or if any errors occur.
///
/// # Arguments
/// * `path` - Path to the TOML file
/// * `base_name` - Base name to search for (e.g., "prompt" will match "prompt_1", "prompt_2", etc.)
///
/// # Returns
/// * `Vec<String>` - Vector containing all values for fields matching the base name
///
/// # Example
/// ```
/// let values = read_basename_fields_from_toml("config.toml", "prompt");
/// // For TOML content:
/// // prompt_1 = "value1"
/// // prompt_2 = "value2"
/// // Returns: vec!["value1", "value2"]
/// ```
fn read_basename_fields_from_toml(path: &str, base_name: &str) -> Vec<String> {
    let mut values = Vec::new();

    // Validate input parameters
    if path.is_empty() || base_name.is_empty() {
        println!("Error: Empty path or base name provided");
        return values;
    }

    // Open and read the file
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open file at path: {}. Error: {}", path, e);
            return values;
        },
    };

    let reader = io::BufReader::new(file);
    let base_name_with_underscore = format!("{}_", base_name);

    // Process each line
    for (line_number, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                println!("Error reading line {}: {}", line_number + 1, e);
                continue;
            }
        };

        // Skip empty lines and comments
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        // Check if line starts with base_name_
        if trimmed_line.starts_with(&base_name_with_underscore) {
            // Split the line by '=' and handle malformed lines
            let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
            if parts.len() != 2 {
                println!("Malformed TOML line {} - missing '=': {}", line_number + 1, line);
                continue;
            }

            let value = parts[1].trim();
            
            // Clean up the value: remove quotes and trim spaces
            let cleaned_value = value.trim().trim_matches('"').trim();
            
            if !cleaned_value.is_empty() {
                values.push(cleaned_value.to_string());
            }
        }
    }

    // Sort values to ensure consistent ordering
    values.sort();
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use std::fs::remove_file;

    #[test]
    fn test_read_basename_fields() {
        // Create a temporary test TOML file
        let test_content = r#"
            # Test TOML file
            prompt_1 = "value1"
            prompt_2 = "value2"
            other_field = "other"
            prompt_3 = "value3"
        "#;
        let test_file = "test_basename.toml";
        write(test_file, test_content).unwrap();

        // Test reading basename fields
        let values = read_basename_fields_from_toml(test_file, "prompt");
        
        // Clean up
        let _ = remove_file(test_file);

        assert_eq!(values, vec!["value1", "value2", "value3"]);
    }

    #[test]
    fn test_empty_input() {
        let values = read_basename_fields_from_toml("", "prompt");
        assert!(values.is_empty());
    }
}



fn main() {
    let value = read_field_from_toml("test.toml", "fieldname");
    println!("Field value -> {}", value);
    
    // Read all prompt fields
    let prompt_values = read_basename_fields_from_toml("config.toml", "prompt");
    println!("Prompts: {:?}", prompt_values);
}
