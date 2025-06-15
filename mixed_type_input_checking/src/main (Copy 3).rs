use std::num::ParseIntError;
use std::io::{self, Write};

// Struct to hold a range of valid integers.
#[derive(Clone, Debug)]
struct IntegerRange {
    min: i32,
    max: i32,
}

// Struct to hold a range of valid integers and an associated string.
#[derive(Clone, Debug)]
struct IntegerRangeWithString {
    range: IntegerRange,
    max_string_length: usize,
}

// Function to check if an input string is a valid integer within the given ranges.
fn is_valid_integer(input: &str, ranges: &[IntegerRange]) -> Result<bool, ParseIntError> {
    let parsed_input: i32 = input.parse()?;
    for range in ranges {
        if parsed_input >= range.min && parsed_input <= range.max {
            return Ok(true);
        }
    }
    Ok(false)
}

// Function to check if an input string is a valid string with a maximum length.
fn is_valid_string(input: &str, max_length: usize) -> bool {
    input.len() <= max_length
}

// Function to parse and validate the input format.
fn validate_input(
    input: &str,
    ranges: &[IntegerRange],
    range_with_string: &[IntegerRangeWithString],
) -> bool {
    if let Ok(true) = is_valid_integer(input, ranges) {
        return true;
    }

    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() == 2 {
        let key = parts[0].trim();
        let value = parts[1].trim();

        let key = key.trim_matches(|c: char| c == '{' || c == '}');
        let value = value.trim_matches(|c: char| c == '\'' || c == '"');

        for range_string in range_with_string {
            if let Ok(true) = is_valid_integer(key, &[range_string.range.clone()]) {
                return is_valid_string(value, range_string.max_string_length);
            }
        }
    }

    false
}

// Function to get custom input ranges from the user.
fn get_custom_ranges() -> Vec<IntegerRange> {
    let mut ranges = Vec::new();
    println!("Enter the number of integer ranges you want to add:");
    let mut num_ranges = String::new();
    io::stdin().read_line(&mut num_ranges).expect("Failed to read line");
    let num_ranges: usize = num_ranges.trim().parse().expect("Please enter a valid number");

    for i in 0..num_ranges {
        println!("Enter the minimum value for range {}:", i + 1);
        let mut min = String::new();
        io::stdin().read_line(&mut min).expect("Failed to read line");
        let min: i32 = min.trim().parse().expect("Please enter a valid integer");

        println!("Enter the maximum value (inclusive, up to and including this value) for range {}:", i + 1);
        let mut max = String::new();
        io::stdin().read_line(&mut max).expect("Failed to read line");
        let max: i32 = max.trim().parse().expect("Please enter a valid integer");

        ranges.push(IntegerRange { min, max });
    }

    ranges
}

// Function to get custom integer ranges with string constraints from the user.
fn get_custom_range_with_string() -> Vec<IntegerRangeWithString> {
    let mut range_with_string = Vec::new();
    println!("Enter the number of integer ranges with string constraints you want to add:");
    let mut num_ranges = String::new();
    io::stdin().read_line(&mut num_ranges).expect("Failed to read line");
    let num_ranges: usize = num_ranges.trim().parse().expect("Please enter a valid number");

    for i in 0..num_ranges {
        println!("Enter the minimum value for range {}:", i + 1);
        let mut min = String::new();
        io::stdin().read_line(&mut min).expect("Failed to read line");
        let min: i32 = min.trim().parse().expect("Please enter a valid integer");

        println!("Enter the maximum value (inclusive, up to and including this value) for range {}:", i + 1);
        let mut max = String::new();
        io::stdin().read_line(&mut max).expect("Failed to read line");
        let max: i32 = max.trim().parse().expect("Please enter a valid integer");

        println!("Enter the maximum string length for range {}:", i + 1);
        let mut max_string_length = String::new();
        io::stdin().read_line(&mut max_string_length).expect("Failed to read line");
        let max_string_length: usize = max_string_length.trim().parse().expect("Please enter a valid number");

        range_with_string.push(IntegerRangeWithString {
            range: IntegerRange { min, max },
            max_string_length,
        });
    }

    range_with_string
}

fn main() {
    let ranges = get_custom_ranges();
    let range_with_string = get_custom_range_with_string();

    loop {
        println!("Enter the inputs to validate (separated by commas):");
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).expect("Failed to read line");
        let inputs: Vec<&str> = inputs.trim().split(',').map(|s| s.trim()).collect();

        for input in inputs {
            if validate_input(input, &ranges, &range_with_string) {
                println!("The input '{}' is valid.", input);
            } else {
                println!("The input '{}' is invalid.", input);
            }
        }
    }
}
