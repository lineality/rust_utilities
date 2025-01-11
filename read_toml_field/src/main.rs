use std::fs::File;
use std::io::{self, BufRead};

// The function reads a single line from a TOML file that starts with a specified field name
// and ends with a value. The function returns an empty string if the field is not found, and
// does not panic or unwrap in case of errors. The function uses only standard Rust libraries
// and does not introduce unnecessary dependencies.
fn read_field_from_toml(path: &str, field_name: &str) -> String {
    // Debug print statement
    println!("Attempting to open file at path: {}", path);

    // Open the file at the specified path
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            // Debug print statement
            println!("Failed to open file at path: {}", path);
            return String::new();
        },
    };

    // Debug print statement
    println!("Successfully opened file at path: {}", path);

    // Create a buffered reader to read the file line by line
    let reader = io::BufReader::new(file);

    // Iterate through each line in the file
    for line in reader.lines() {
        // If the line starts with the specified field name and ends with a value,
        // extract the value and return it
        if let Ok(line) = line {
            // Debug print statement
            println!("Read line: {}", line);

            if line.starts_with(field_name) {
                // Debug print statement
                println!("Found line starting with field name: {}", field_name);

                // Extract the value part of the line
                if let Some(value) = line.split('=').nth(1) {
                    // Debug print statement
                    println!("Extracted value: {}", value);

                    return value.trim().trim_matches('"').to_string();
                }
            }
        }
    }

    // If the field is not found, return an empty string
    String::new()
}

fn main() {
    let value = read_field_from_toml("test.toml", "fieldname");
    println!("Field value -> {}", value);
}

