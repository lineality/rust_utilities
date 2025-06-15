//! Input Validation System
//! 
//! This module provides a comprehensive input validation system that can validate
//! integers within specified ranges and integer-string pairs with length constraints.
//! The system processes multiple inputs and returns structured validation results.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::num::ParseIntError;

/// Custom error type for validation operations
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Error occurred while parsing integer input
    ParseError(String),
    /// Error occurred during I/O operations
    IoError(String),
    /// Error occurred due to invalid configuration
    ConfigurationError(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::ParseError(message) => write!(formatter, "Parse error: {}", message),
            ValidationError::IoError(message) => write!(formatter, "I/O error: {}", message),
            ValidationError::ConfigurationError(message) => write!(formatter, "Configuration error: {}", message),
        }
    }
}

impl Error for ValidationError {}

impl From<ParseIntError> for ValidationError {
    fn from(error: ParseIntError) -> Self {
        ValidationError::ParseError(error.to_string())
    }
}

impl From<io::Error> for ValidationError {
    fn from(error: io::Error) -> Self {
        ValidationError::IoError(error.to_string())
    }
}

/// Represents the validation status of an input
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationStatus {
    /// Input is valid according to the specified rules
    Valid,
    /// Input is invalid according to the specified rules
    Invalid,
}

impl fmt::Display for ValidationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationStatus::Valid => write!(formatter, "valid"),
            ValidationStatus::Invalid => write!(formatter, "invalid"),
        }
    }
}

/// Represents a range of valid integers with inclusive bounds
/// 
/// # Examples
/// ```
/// let range = IntegerValidationRange::new(1, 10);
/// assert!(range.contains_value(5));
/// assert!(!range.contains_value(15));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerValidationRange {
    /// The minimum value (inclusive) of the range
    minimum_value: i32,
    /// The maximum value (inclusive) of the range
    maximum_value: i32,
}

impl IntegerValidationRange {
    /// Creates a new integer validation range
    /// 
    /// # Arguments
    /// * `minimum_value` - The minimum value (inclusive) of the range
    /// * `maximum_value` - The maximum value (inclusive) of the range
    /// 
    /// # Returns
    /// A new `IntegerValidationRange` instance
    /// 
    /// # Panics
    /// This function will panic if `minimum_value` is greater than `maximum_value`
    pub fn new(minimum_value: i32, maximum_value: i32) -> Self {
        if minimum_value > maximum_value {
            panic!("Minimum value cannot be greater than maximum value");
        }
        
        Self {
            minimum_value,
            maximum_value,
        }
    }

    /// Checks if a given value falls within this range (inclusive)
    /// 
    /// # Arguments
    /// * `value` - The value to check
    /// 
    /// # Returns
    /// `true` if the value is within the range, `false` otherwise
    pub fn contains_value(&self, value: i32) -> bool {
        value >= self.minimum_value && value <= self.maximum_value
    }
}

/// Represents a validation rule for integer-string pairs
/// 
/// This struct defines a validation rule where the integer part must fall within
/// a specified range and the string part must not exceed a maximum length.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerStringValidationRule {
    /// The valid range for the integer part
    integer_range: IntegerValidationRange,
    /// The maximum allowed length for the string part
    maximum_string_length: usize,
}

impl IntegerStringValidationRule {
    /// Creates a new integer-string validation rule
    /// 
    /// # Arguments
    /// * `integer_range` - The valid range for the integer part
    /// * `maximum_string_length` - The maximum allowed length for the string part
    /// 
    /// # Returns
    /// A new `IntegerStringValidationRule` instance
    pub fn new(integer_range: IntegerValidationRange, maximum_string_length: usize) -> Self {
        Self {
            integer_range,
            maximum_string_length,
        }
    }

    /// Validates an integer-string pair against this rule
    /// 
    /// # Arguments
    /// * `integer_value` - The integer part to validate
    /// * `string_value` - The string part to validate
    /// 
    /// # Returns
    /// `true` if both parts are valid according to this rule, `false` otherwise
    pub fn validate_pair(&self, integer_value: i32, string_value: &str) -> bool {
        self.integer_range.contains_value(integer_value) && 
        string_value.len() <= self.maximum_string_length
    }
}

/// The main validation engine that processes inputs against defined rules
/// 
/// This struct contains all the validation rules and provides methods to
/// validate individual inputs and batches of inputs.
#[derive(Debug)]
pub struct InputValidationEngine {
    /// List of valid integer ranges for standalone integer validation
    integer_validation_ranges: Vec<IntegerValidationRange>,
    /// List of validation rules for integer-string pairs
    integer_string_validation_rules: Vec<IntegerStringValidationRule>,
}

impl InputValidationEngine {
    /// Creates a new validation engine with the specified rules
    /// 
    /// # Arguments
    /// * `integer_validation_ranges` - Vector of valid integer ranges
    /// * `integer_string_validation_rules` - Vector of integer-string validation rules
    /// 
    /// # Returns
    /// A new `InputValidationEngine` instance
    pub fn new(
        integer_validation_ranges: Vec<IntegerValidationRange>,
        integer_string_validation_rules: Vec<IntegerStringValidationRule>,
    ) -> Self {
        Self {
            integer_validation_ranges,
            integer_string_validation_rules,
        }
    }

    /// Validates a standalone integer input against all integer ranges
    /// 
    /// # Arguments
    /// * `input_string` - The string representation of the integer to validate
    /// 
    /// # Returns
    /// `Ok(true)` if the integer is valid, `Ok(false)` if invalid, or an error
    fn validate_standalone_integer(&self, input_string: &str) -> Result<bool, ValidationError> {
        let parsed_integer: i32 = input_string.parse()?;
        
        // Check if the integer falls within any of the valid ranges
        for validation_range in &self.integer_validation_ranges {
            if validation_range.contains_value(parsed_integer) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Validates an integer-string pair input against all integer-string rules
    /// 
    /// # Arguments
    /// * `integer_part` - The integer part of the input
    /// * `string_part` - The string part of the input
    /// 
    /// # Returns
    /// `Ok(true)` if the pair is valid, `Ok(false)` if invalid, or an error
    fn validate_integer_string_pair(&self, integer_part: &str, string_part: &str) -> Result<bool, ValidationError> {
        // Clean the integer part of any surrounding braces
        let cleaned_integer_part = integer_part.trim_matches(|character: char| character == '{' || character == '}');
        
        // Clean the string part of any surrounding quotes
        let cleaned_string_part = string_part.trim_matches(|character: char| character == '\'' || character == '"');

        // Try to parse the integer part
        let parsed_integer: i32 = cleaned_integer_part.parse()?;

        // Check against all integer-string validation rules
        for validation_rule in &self.integer_string_validation_rules {
            if validation_rule.validate_pair(parsed_integer, cleaned_string_part) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Validates a single input string against all validation rules
    /// 
    /// # Arguments
    /// * `input_string` - The input string to validate
    /// 
    /// # Returns
    /// The validation status of the input
    pub fn validate_single_input(&self, input_string: &str) -> ValidationStatus {
        // First, try to validate as a standalone integer
        if let Ok(true) = self.validate_standalone_integer(input_string) {
            return ValidationStatus::Valid;
        }

        // Then, try to validate as an integer-string pair
        let input_parts: Vec<&str> = input_string.split(':').collect();
        if input_parts.len() == 2 {
            let integer_part = input_parts[0].trim();
            let string_part = input_parts[1].trim();

            if let Ok(true) = self.validate_integer_string_pair(integer_part, string_part) {
                return ValidationStatus::Valid;
            }
        }

        ValidationStatus::Invalid
    }

    /// Validates multiple inputs and returns a structured result
    /// 
    /// # Arguments
    /// * `input_strings` - Vector of input strings to validate
    /// 
    /// # Returns
    /// A HashMap mapping each input string to its validation status
    pub fn validate_multiple_inputs(&self, input_strings: &[String]) -> HashMap<String, ValidationStatus> {
        let mut validation_results = HashMap::new();

        for input_string in input_strings {
            let trimmed_input = input_string.trim();
            let validation_status = self.validate_single_input(trimmed_input);
            validation_results.insert(trimmed_input.to_string(), validation_status);
        }

        validation_results
    }
}

/// Collects integer validation ranges from user input
/// 
/// # Returns
/// A vector of `IntegerValidationRange` instances or an error
fn collect_integer_validation_ranges_from_user() -> Result<Vec<IntegerValidationRange>, ValidationError> {
    let mut validation_ranges = Vec::new();
    
    println!("Enter the number of integer ranges you want to add:");
    io::stdout().flush()?;
    
    let mut number_of_ranges_input = String::new();
    io::stdin().read_line(&mut number_of_ranges_input)?;
    
    let number_of_ranges: usize = number_of_ranges_input.trim().parse()
        .map_err(|_| ValidationError::ParseError("Please enter a valid number".to_string()))?;

    for range_index in 0..number_of_ranges {
        println!("Enter the minimum value for range {}:", range_index + 1);
        io::stdout().flush()?;
        
        let mut minimum_value_input = String::new();
        io::stdin().read_line(&mut minimum_value_input)?;
        
        let minimum_value: i32 = minimum_value_input.trim().parse()
            .map_err(|_| ValidationError::ParseError("Please enter a valid integer".to_string()))?;

        println!("Enter the maximum value for range {}:", range_index + 1);
        io::stdout().flush()?;
        
        let mut maximum_value_input = String::new();
        io::stdin().read_line(&mut maximum_value_input)?;
        
        let maximum_value: i32 = maximum_value_input.trim().parse()
            .map_err(|_| ValidationError::ParseError("Please enter a valid integer".to_string()))?;

        if minimum_value > maximum_value {
            return Err(ValidationError::ConfigurationError(
                "Minimum value cannot be greater than maximum value".to_string()
            ));
        }

        validation_ranges.push(IntegerValidationRange::new(minimum_value, maximum_value));
    }

    Ok(validation_ranges)
}

/// Collects integer-string validation rules from user input
/// 
/// # Returns
/// A vector of `IntegerStringValidationRule` instances or an error
fn collect_integer_string_validation_rules_from_user() -> Result<Vec<IntegerStringValidationRule>, ValidationError> {
    let mut validation_rules = Vec::new();
    
    println!("Enter the number of integer ranges with string constraints you want to add:");
    io::stdout().flush()?;
    
    let mut number_of_rules_input = String::new();
    io::stdin().read_line(&mut number_of_rules_input)?;
    
    let number_of_rules: usize = number_of_rules_input.trim().parse()
        .map_err(|_| ValidationError::ParseError("Please enter a valid number".to_string()))?;

    for rule_index in 0..number_of_rules {
        println!("Enter the minimum value for range {}:", rule_index + 1);
        io::stdout().flush()?;
        
        let mut minimum_value_input = String::new();
        io::stdin().read_line(&mut minimum_value_input)?;
        
        let minimum_value: i32 = minimum_value_input.trim().parse()
            .map_err(|_| ValidationError::ParseError("Please enter a valid integer".to_string()))?;

        println!("Enter the maximum value for range {}:", rule_index + 1);
        io::stdout().flush()?;
        
        let mut maximum_value_input = String::new();
        io::stdin().read_line(&mut maximum_value_input)?;
        
        let maximum_value: i32 = maximum_value_input.trim().parse()
            .map_err(|_| ValidationError::ParseError("Please enter a valid integer".to_string()))?;

        if minimum_value > maximum_value {
            return Err(ValidationError::ConfigurationError(
                "Minimum value cannot be greater than maximum value".to_string()
            ));
        }

        println!("Enter the maximum string length for range {}:", rule_index + 1);
        io::stdout().flush()?;
        
        let mut maximum_string_length_input = String::new();
        io::stdin().read_line(&mut maximum_string_length_input)?;
        
        let maximum_string_length: usize = maximum_string_length_input.trim().parse()
            .map_err(|_| ValidationError::ParseError("Please enter a valid number".to_string()))?;

        let integer_range = IntegerValidationRange::new(minimum_value, maximum_value);
        let validation_rule = IntegerStringValidationRule::new(integer_range, maximum_string_length);
        validation_rules.push(validation_rule);
    }

    Ok(validation_rules)
}

/// Parses a comma-separated input string into individual input strings
/// 
/// # Arguments
/// * `input_line` - The comma-separated input string
/// 
/// # Returns
/// A vector of trimmed individual input strings
fn parse_comma_separated_inputs(input_line: &str) -> Vec<String> {
    input_line
        .split(',')
        .map(|input_part| input_part.trim().to_string())
        .filter(|input_part| !input_part.is_empty())
        .collect()
}

/// Displays the validation results in a formatted manner
/// 
/// # Arguments
/// * `validation_results` - HashMap containing validation results to display
fn display_validation_results(validation_results: &HashMap<String, ValidationStatus>) {
    println!("\nValidation Results:");
    println!("{{");
    
    for (input_string, validation_status) in validation_results {
        println!("  \"{}\": {},", input_string, validation_status);
    }
    
    println!("}}");
}

/// Main function that orchestrates the input validation system
/// 
/// This function:
/// 1. Collects validation rules from the user
/// 2. Creates a validation engine with those rules
/// 3. Continuously accepts input and validates it
/// 4. Displays structured validation results
fn main() -> Result<(), ValidationError> {
    println!("=== Input Validation System ===\n");

    // Collect integer validation ranges from user
    let integer_validation_ranges = collect_integer_validation_ranges_from_user()?;

    // Collect integer-string validation rules from user
    let integer_string_validation_rules = collect_integer_string_validation_rules_from_user()?;

    // Create the validation engine with the collected rules
    let validation_engine = InputValidationEngine::new(
        integer_validation_ranges,
        integer_string_validation_rules,
    );

    println!("\n=== Validation Engine Ready ===");
    println!("Enter inputs to validate (separated by commas), or Ctrl+C to exit:");

    // Main validation loop
    loop {
        print!("\nInput: ");
        io::stdout().flush()?;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line)?;

        // Parse the comma-separated inputs
        let individual_inputs = parse_comma_separated_inputs(&input_line);

        if individual_inputs.is_empty() {
            println!("No inputs provided. Please enter at least one input.");
            continue;
        }

        // Validate all inputs and get structured results
        let validation_results = validation_engine.validate_multiple_inputs(&individual_inputs);

        // Display the results in the requested format
        display_validation_results(&validation_results);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_validation_range_creation() {
        let range = IntegerValidationRange::new(1, 10);
        assert_eq!(range.minimum_value, 1);
        assert_eq!(range.maximum_value, 10);
    }

    #[test]
    fn test_integer_validation_range_contains_value() {
        let range = IntegerValidationRange::new(1, 10);
        assert!(range.contains_value(5));
        assert!(range.contains_value(1));
        assert!(range.contains_value(10));
        assert!(!range.contains_value(0));
        assert!(!range.contains_value(11));
    }

    #[test]
    fn test_integer_string_validation_rule() {
        let range = IntegerValidationRange::new(1, 10);
        let rule = IntegerStringValidationRule::new(range, 5);
        
        assert!(rule.validate_pair(5, "test"));
        assert!(rule.validate_pair(1, ""));
        assert!(!rule.validate_pair(0, "test"));
        assert!(!rule.validate_pair(5, "toolong"));
    }

    #[test]
    fn test_validation_engine_single_input() {
        let integer_ranges = vec![IntegerValidationRange::new(1, 3)];
        let string_rules = vec![IntegerStringValidationRule::new(
            IntegerValidationRange::new(10, 20),
            10,
        )];
        
        let engine = InputValidationEngine::new(integer_ranges, string_rules);
        
        assert_eq!(engine.validate_single_input("2"), ValidationStatus::Valid);
        assert_eq!(engine.validate_single_input("5"), ValidationStatus::Invalid);
        assert_eq!(engine.validate_single_input("15:test"), ValidationStatus::Valid);
        assert_eq!(engine.validate_single_input("25:test"), ValidationStatus::Invalid);
    }

    #[test]
    fn test_parse_comma_separated_inputs() {
        let inputs = parse_comma_separated_inputs("1,2,3,10:frogs");
        assert_eq!(inputs, vec!["1", "2", "3", "10:frogs"]);
        
        let inputs_with_spaces = parse_comma_separated_inputs("1 , 2 , 3 , 10:frogs ");
        assert_eq!(inputs_with_spaces, vec!["1", "2", "3", "10:frogs"]);
    }
}
