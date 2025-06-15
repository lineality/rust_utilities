use std::num::ParseIntError;

/// Struct to hold a range of valid integers.
#[derive(Clone)]
struct IntegerRange {
    min: i32,
    max: i32,
}

/// Struct to hold a range of valid integers and an associated string.
#[derive(Clone)]
struct IntegerRangeWithString {
    range: IntegerRange,
    max_string_length: usize,
}

/// Function to check if an input string is a valid integer within the given ranges.
///
/// # Arguments
///
/// * `input` - A string slice that holds the user input.
/// * `ranges` - A slice of `IntegerRange` structs that define the valid ranges.
///
/// # Returns
///
/// * `Ok(true)` if the input is a valid integer within the given ranges.
/// * `Ok(false)` if the input is not a valid integer within the given ranges.
/// * `Err(ParseIntError)` if the input cannot be parsed as an integer.
fn is_valid_integer(input: &str, ranges: &[IntegerRange]) -> Result<bool, ParseIntError> {
    // Parse the input string to an integer.
    let parsed_input: i32 = input.parse()?;

    // Check if the parsed integer is within any of the given ranges.
    for range in ranges {
        if parsed_input >= range.min && parsed_input <= range.max {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Function to check if an input string is a valid string with a maximum length.
///
/// # Arguments
///
/// * `input` - A string slice that holds the user input.
/// * `max_length` - The maximum allowed length for the input string.
///
/// # Returns
///
/// * `true` if the input string is within the maximum length.
/// * `false` if the input string exceeds the maximum length.
fn is_valid_string(input: &str, max_length: usize) -> bool {
    input.len() <= max_length
}

/// Function to parse and validate the input format.
///
/// # Arguments
///
/// * `input` - A string slice that holds the user input.
/// * `ranges` - A slice of `IntegerRange` structs that define the valid ranges for integers.
/// * `range_with_string` - A slice of `IntegerRangeWithString` structs that define the valid ranges for integers with associated strings.
///
/// # Returns
///
/// * `true` if the input is valid according to the specified rules.
/// * `false` otherwise.
fn validate_input(
    input: &str,
    ranges: &[IntegerRange],
    range_with_string: &[IntegerRangeWithString],
) -> bool {
    // Check if the input is an integer.
    if let Ok(true) = is_valid_integer(input, ranges) {
        return true;
    }

    // Check if the input is in the format `3:cats`, `3:'cats'`, `3:"cats"`, `{3:'cats'}`, `{3:"cats"}`, or `{3:cats}`.
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() == 2 {
        let key = parts[0].trim();
        let value = parts[1].trim();

        // Remove curly braces if present.
        let key = key.trim_matches(|c| c == '{' || c == '}');
        let value = value.trim_matches(|c| c == '\'' || c == '"');

        // Check if the key is a valid integer within the given ranges with associated strings.
        for range_string in range_with_string {
            if let Ok(true) = is_valid_integer(key, &[range_string.range.clone()]) {
                return is_valid_string(value, range_string.max_string_length);
            }
        }
    }

    false
}

/// Example usage of the `validate_input` function.
fn main() {
    // Define the ranges for valid integers.
    let ranges = vec![
        IntegerRange { min: 1, max: 3 },
        IntegerRange { min: 5, max: 8 },
    ];

    // Define the ranges for valid integers with associated strings.
    let range_with_string = vec![
        IntegerRangeWithString {
            range: IntegerRange { min: 3, max: 5 },
            max_string_length: 20,
        },
    ];

    // Example inputs to test.
    let inputs = vec![
        "1", "4", "3", "hello", "world!", "3:cats", "3:'cats'", "3:\"cats\"", "{3:'cats'}", "{3:\"cats\"}", "{3:cats}", "5:world"
    ];

    for input in inputs {
        if validate_input(input, &ranges, &range_with_string) {
            println!("The input '{}' is valid.", input);
        } else {
            println!("The input '{}' is invalid.", input);
        }
    }
}

