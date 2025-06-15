//! Input Validation System with Configuration Import/Export
//! 
//! This module provides a comprehensive input validation system that can validate
//! integers within specified ranges and integer-string pairs with length constraints.
//! The system supports importing and exporting validation configurations to/from JSON files.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::num::ParseIntError;
use std::path::Path;

/// Custom error type for validation operations
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Error occurred while parsing integer input
    ParseError(String),
    /// Error occurred during I/O operations
    IoError(String),
    /// Error occurred due to invalid configuration
    ConfigurationError(String),
    /// Error occurred during file operations
    FileError(String),
    /// Error occurred during JSON parsing/serialization
    JsonError(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::ParseError(message) => write!(formatter, "Parse error: {}", message),
            ValidationError::IoError(message) => write!(formatter, "I/O error: {}", message),
            ValidationError::ConfigurationError(message) => write!(formatter, "Configuration error: {}", message),
            ValidationError::FileError(message) => write!(formatter, "File error: {}", message),
            ValidationError::JsonError(message) => write!(formatter, "JSON error: {}", message),
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

    /// Gets the minimum value of the range
    /// 
    /// # Returns
    /// The minimum value (inclusive) of the range
    pub fn get_minimum_value(&self) -> i32 {
        self.minimum_value
    }

    /// Gets the maximum value of the range
    /// 
    /// # Returns
    /// The maximum value (inclusive) of the range
    pub fn get_maximum_value(&self) -> i32 {
        self.maximum_value
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

    /// Converts the range to a JSON-like string representation
    /// 
    /// # Returns
    /// A string representation of the range in JSON format
    fn to_json_string(&self) -> String {
        format!(r#"{{"min": {}, "max": {}}}"#, self.minimum_value, self.maximum_value)
    }

    /// Creates an IntegerValidationRange from a JSON-like string
    /// 
    /// # Arguments
    /// * `json_string` - The JSON string representation of the range
    /// 
    /// # Returns
    /// Result containing the parsed range or an error
    fn from_json_string(json_string: &str) -> Result<Self, ValidationError> {
        // Simple JSON parsing without external dependencies
        let trimmed = json_string.trim().trim_start_matches('{').trim_end_matches('}');
        let mut min_value = None;
        let mut max_value = None;

        for part in trimmed.split(',') {
            let part = part.trim();
            if part.starts_with(r#""min""#) || part.starts_with("\"min\"") {
                let value_str = part.split(':').nth(1)
                    .ok_or_else(|| ValidationError::JsonError("Missing min value".to_string()))?
                    .trim();
                min_value = Some(value_str.parse()?);
            } else if part.starts_with(r#""max""#) || part.starts_with("\"max\"") {
                let value_str = part.split(':').nth(1)
                    .ok_or_else(|| ValidationError::JsonError("Missing max value".to_string()))?
                    .trim();
                max_value = Some(value_str.parse()?);
            }
        }

        match (min_value, max_value) {
            (Some(min), Some(max)) => Ok(Self::new(min, max)),
            _ => Err(ValidationError::JsonError("Missing min or max value".to_string())),
        }
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

    /// Gets the integer range for this rule
    /// 
    /// # Returns
    /// A reference to the integer validation range
    pub fn get_integer_range(&self) -> &IntegerValidationRange {
        &self.integer_range
    }

    /// Gets the maximum string length for this rule
    /// 
    /// # Returns
    /// The maximum allowed string length
    pub fn get_maximum_string_length(&self) -> usize {
        self.maximum_string_length
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

    /// Converts the rule to a JSON-like string representation
    /// 
    /// # Returns
    /// A string representation of the rule in JSON format
    fn to_json_string(&self) -> String {
        format!(
            r#"{{"range": {}, "max_string_length": {}}}"#,
            self.integer_range.to_json_string(),
            self.maximum_string_length
        )
    }

    /// Creates an IntegerStringValidationRule from a JSON-like string
    /// 
    /// # Arguments
    /// * `json_string` - The JSON string representation of the rule
    /// 
    /// # Returns
    /// Result containing the parsed rule or an error
    fn from_json_string(json_string: &str) -> Result<Self, ValidationError> {
        let trimmed = json_string.trim().trim_start_matches('{').trim_end_matches('}');
        let mut range_json = None;
        let mut max_length = None;

        // Find the range object and max_string_length
        let mut brace_count = 0;
        let mut current_part = String::new();
        let mut in_range = false;

        for ch in trimmed.chars() {
            match ch {
                '{' => {
                    brace_count += 1;
                    if brace_count == 1 && current_part.trim().ends_with("range\":") {
                        in_range = true;
                        current_part.push(ch);
                    } else {
                        current_part.push(ch);
                    }
                }
                '}' => {
                    brace_count -= 1;
                    current_part.push(ch);
                    if brace_count == 0 && in_range {
                        let range_start = current_part.rfind('{').unwrap();
                        range_json = Some(current_part[range_start..].to_string());
                        in_range = false;
                        current_part.clear();
                    }
                }
                ',' if brace_count == 0 => {
                    // Process the current part
                    let part = current_part.trim();
                    if part.starts_with(r#""max_string_length""#) {
                        let value_str = part.split(':').nth(1)
                            .ok_or_else(|| ValidationError::JsonError("Missing max_string_length value".to_string()))?
                            .trim();
                        max_length = Some(value_str.parse()
                            .map_err(|_| ValidationError::JsonError("Invalid max_string_length value".to_string()))?);
                    }
                    current_part.clear();
                }
                _ => current_part.push(ch),
            }
        }

        // Process the last part
        if !current_part.is_empty() {
            let part = current_part.trim();
            if part.starts_with(r#""max_string_length""#) {
                let value_str = part.split(':').nth(1)
                    .ok_or_else(|| ValidationError::JsonError("Missing max_string_length value".to_string()))?
                    .trim();
                max_length = Some(value_str.parse()
                    .map_err(|_| ValidationError::JsonError("Invalid max_string_length value".to_string()))?);
            }
        }

        match (range_json, max_length) {
            (Some(range_str), Some(length)) => {
                let range = IntegerValidationRange::from_json_string(&range_str)?;
                Ok(Self::new(range, length))
            }
            _ => Err(ValidationError::JsonError("Missing range or max_string_length".to_string())),
        }
    }
}

/// Configuration structure that holds all validation rules
/// 
/// This struct can be serialized to and deserialized from JSON format
/// for easy import/export of validation configurations.
#[derive(Debug, Clone)]
pub struct ValidationConfiguration {
    /// List of integer validation ranges
    integer_ranges: Vec<IntegerValidationRange>,
    /// List of integer-string validation rules
    integer_string_rules: Vec<IntegerStringValidationRule>,
    /// Optional name/description for this configuration
    configuration_name: Option<String>,
}

impl ValidationConfiguration {
    /// Creates a new validation configuration
    /// 
    /// # Arguments
    /// * `integer_ranges` - Vector of integer validation ranges
    /// * `integer_string_rules` - Vector of integer-string validation rules
    /// * `configuration_name` - Optional name for this configuration
    /// 
    /// # Returns
    /// A new `ValidationConfiguration` instance
    pub fn new(
        integer_ranges: Vec<IntegerValidationRange>,
        integer_string_rules: Vec<IntegerStringValidationRule>,
        configuration_name: Option<String>,
    ) -> Self {
        Self {
            integer_ranges,
            integer_string_rules,
            configuration_name,
        }
    }

    /// Gets the integer ranges from this configuration
    /// 
    /// # Returns
    /// A reference to the vector of integer validation ranges
    pub fn get_integer_ranges(&self) -> &Vec<IntegerValidationRange> {
        &self.integer_ranges
    }

    /// Gets the integer-string rules from this configuration
    /// 
    /// # Returns
    /// A reference to the vector of integer-string validation rules
    pub fn get_integer_string_rules(&self) -> &Vec<IntegerStringValidationRule> {
        &self.integer_string_rules
    }

    /// Gets the configuration name
    /// 
    /// # Returns
    /// An optional reference to the configuration name
    pub fn get_configuration_name(&self) -> Option<&String> {
        self.configuration_name.as_ref()
    }

    /// Exports the configuration to a JSON file
    /// 
    /// # Arguments
    /// * `file_path` - The absolute path where to save the configuration file
    /// 
    /// # Returns
    /// Result indicating success or failure
    pub fn export_to_file<P: AsRef<Path>>(&self, file_path: P) -> Result<(), ValidationError> {
        let json_content = self.to_json_string()?;
        
        fs::write(file_path, json_content)
            .map_err(|error| ValidationError::FileError(format!("Failed to write configuration file: {}", error)))?;
        
        Ok(())
    }

    /// Imports a configuration from a JSON file
    /// 
    /// # Arguments
    /// * `file_path` - The absolute path to the configuration file to load
    /// 
    /// # Returns
    /// Result containing the loaded configuration or an error
    pub fn import_from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, ValidationError> {
        let json_content = fs::read_to_string(file_path)
            .map_err(|error| ValidationError::FileError(format!("Failed to read configuration file: {}", error)))?;
        
        Self::from_json_string(&json_content)
    }

    /// Converts the configuration to a JSON string
    /// 
    /// # Returns
    /// Result containing the JSON string representation or an error
    fn to_json_string(&self) -> Result<String, ValidationError> {
        let mut json_parts = Vec::new();

        // Add configuration name if present
        if let Some(ref name) = self.configuration_name {
            json_parts.push(format!(r#"  "name": "{}""#, name));
        }

        // Add integer ranges
        if !self.integer_ranges.is_empty() {
            let ranges_json: Vec<String> = self.integer_ranges
                .iter()
                .map(|range| format!("    {}", range.to_json_string()))
                .collect();
            json_parts.push(format!(r#"  "integer_ranges": [
{}
  ]"#, ranges_json.join(",\n")));
        } else {
            json_parts.push(r#"  "integer_ranges": []"#.to_string());
        }

        // Add integer-string rules
        if !self.integer_string_rules.is_empty() {
            let rules_json: Vec<String> = self.integer_string_rules
                .iter()
                .map(|rule| format!("    {}", rule.to_json_string()))
                .collect();
            json_parts.push(format!(r#"  "integer_string_rules": [
{}
  ]"#, rules_json.join(",\n")));
        } else {
            json_parts.push(r#"  "integer_string_rules": []"#.to_string());
        }

        Ok(format!("{{\n{}\n}}", json_parts.join(",\n")))
    }

    /// Creates a ValidationConfiguration from a JSON string
    /// 
    /// # Arguments
    /// * `json_string` - The JSON string representation of the configuration
    /// 
    /// # Returns
    /// Result containing the parsed configuration or an error
    fn from_json_string(json_string: &str) -> Result<Self, ValidationError> {
        let trimmed = json_string.trim().trim_start_matches('{').trim_end_matches('}');
        
        let mut configuration_name = None;
        let mut integer_ranges = Vec::new();
        let mut integer_string_rules = Vec::new();

        // Simple JSON parsing - split by top-level commas, but respect nested structures
        let mut parts = Vec::new();
        let mut current_part = String::new();
        let mut brace_depth = 0;
        let mut bracket_depth = 0;
        let mut in_quotes = false;
        let mut escape_next = false;

        for ch in trimmed.chars() {
            if escape_next {
                current_part.push(ch);
                escape_next = false;
                continue;
            }

            match ch {
                '\\' => {
                    escape_next = true;
                    current_part.push(ch);
                }
                '"' => {
                    in_quotes = !in_quotes;
                    current_part.push(ch);
                }
                '{' if !in_quotes => {
                    brace_depth += 1;
                    current_part.push(ch);
                }
                '}' if !in_quotes => {
                    brace_depth -= 1;
                    current_part.push(ch);
                }
                '[' if !in_quotes => {
                    bracket_depth += 1;
                    current_part.push(ch);
                }
                ']' if !in_quotes => {
                    bracket_depth -= 1;
                    current_part.push(ch);
                }
                ',' if !in_quotes && brace_depth == 0 && bracket_depth == 0 => {
                    parts.push(current_part.trim().to_string());
                    current_part.clear();
                }
                _ => current_part.push(ch),
            }
        }

        if !current_part.is_empty() {
            parts.push(current_part.trim().to_string());
        }

        // Parse each part
        for part in parts {
            let part = part.trim();
            
            if part.starts_with(r#""name":"#) {
                let name_value = part.split(':').nth(1)
                    .ok_or_else(|| ValidationError::JsonError("Missing name value".to_string()))?
                    .trim()
                    .trim_matches('"');
                configuration_name = Some(name_value.to_string());
            } else if part.starts_with(r#""integer_ranges":"#) {
                let array_content = part.split(':').skip(1).collect::<Vec<_>>().join(":");
                let array_content = array_content.trim().trim_start_matches('[').trim_end_matches(']');
                
                if !array_content.trim().is_empty() {
                    integer_ranges = Self::parse_integer_ranges_array(array_content)?;
                }
            } else if part.starts_with(r#""integer_string_rules":"#) {
                let array_content = part.split(':').skip(1).collect::<Vec<_>>().join(":");
                let array_content = array_content.trim().trim_start_matches('[').trim_end_matches(']');
                
                if !array_content.trim().is_empty() {
                    integer_string_rules = Self::parse_integer_string_rules_array(array_content)?;
                }
            }
        }

        Ok(Self::new(integer_ranges, integer_string_rules, configuration_name))
    }

    /// Parses an array of integer ranges from JSON content
    /// 
    /// # Arguments
    /// * `array_content` - The content between the array brackets
    /// 
    /// # Returns
    /// Result containing the parsed ranges or an error
    fn parse_integer_ranges_array(array_content: &str) -> Result<Vec<IntegerValidationRange>, ValidationError> {
        let mut ranges = Vec::new();
        let mut current_object = String::new();
        let mut brace_depth = 0;
        let mut in_quotes = false;

        for ch in array_content.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current_object.push(ch);
                }
                '{' if !in_quotes => {
                    brace_depth += 1;
                    current_object.push(ch);
                }
                '}' if !in_quotes => {
                    brace_depth -= 1;
                    current_object.push(ch);
                    if brace_depth == 0 {
                        let range = IntegerValidationRange::from_json_string(current_object.trim())?;
                        ranges.push(range);
                        current_object.clear();
                    }
                }
                ',' if !in_quotes && brace_depth == 0 => {
                    // Skip comma between objects
                }
                _ => current_object.push(ch),
            }
        }

        Ok(ranges)
    }

    /// Parses an array of integer-string rules from JSON content
    /// 
    /// # Arguments
    /// * `array_content` - The content between the array brackets
    /// 
    /// # Returns
    /// Result containing the parsed rules or an error
    fn parse_integer_string_rules_array(array_content: &str) -> Result<Vec<IntegerStringValidationRule>, ValidationError> {
        let mut rules = Vec::new();
        let mut current_object = String::new();
        let mut brace_depth = 0;
        let mut in_quotes = false;

        for ch in array_content.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current_object.push(ch);
                }
                '{' if !in_quotes => {
                    brace_depth += 1;
                    current_object.push(ch);
                }
                '}' if !in_quotes => {
                    brace_depth -= 1;
                    current_object.push(ch);
                    if brace_depth == 0 {
                        let rule = IntegerStringValidationRule::from_json_string(current_object.trim())?;
                        rules.push(rule);
                        current_object.clear();
                    }
                }
                ',' if !in_quotes && brace_depth == 0 => {
                    // Skip comma between objects
                }
                _ => current_object.push(ch),
            }
        }

        Ok(rules)
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

    /// Creates a new validation engine from a configuration
    /// 
    /// # Arguments
    /// * `configuration` - The validation configuration to use
    /// 
    /// # Returns
    /// A new `InputValidationEngine` instance
    pub fn from_configuration(configuration: &ValidationConfiguration) -> Self {
        Self::new(
            configuration.integer_ranges.clone(),
            configuration.integer_string_rules.clone(),
        )
    }

    /// Gets the current configuration from this engine
    /// 
    /// # Arguments
    /// * `configuration_name` - Optional name for the configuration
    /// 
    /// # Returns
    /// A `ValidationConfiguration` representing the current engine state
    pub fn to_configuration(&self, configuration_name: Option<String>) -> ValidationConfiguration {
        ValidationConfiguration::new(
            self.integer_validation_ranges.clone(),
            self.integer_string_validation_rules.clone(),
            configuration_name,
        )
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

/// Prompts user to choose configuration source
/// 
/// # Returns
/// Result containing the user's choice or an error
fn prompt_for_configuration_source() -> Result<ConfigurationSource, ValidationError> {
    println!("Choose configuration source:");
    println!("1. Manual input");
    println!("2. Import from file");
    print!("Enter your choice (1 or 2): ");
    io::stdout().flush()?;

    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input)?;

    match choice_input.trim() {
        "1" => Ok(ConfigurationSource::Manual),
        "2" => Ok(ConfigurationSource::File),
        _ => Err(ValidationError::ParseError("Invalid choice. Please enter 1 or 2.".to_string())),
    }
}

/// Enum representing different configuration sources
#[derive(Debug, Clone, PartialEq, Eq)]
enum ConfigurationSource {
    /// Configuration will be entered manually
    Manual,
    /// Configuration will be imported from a file
    File,
}

/// Prompts user for a file path to import configuration
/// 
/// # Returns
/// Result containing the validation configuration or an error
fn import_configuration_from_file() -> Result<ValidationConfiguration, ValidationError> {
    println!("Enter the absolute path to the configuration file:");
    print!("File path: ");
    io::stdout().flush()?;

    let mut file_path_input = String::new();
    io::stdin().read_line(&mut file_path_input)?;
    let file_path = file_path_input.trim();

    if file_path.is_empty() {
        return Err(ValidationError::FileError("File path cannot be empty".to_string()));
    }

    ValidationConfiguration::import_from_file(file_path)
}

/// Prompts user to optionally export the current configuration
/// 
/// # Arguments
/// * `configuration` - The configuration to potentially export
/// 
/// # Returns
/// Result indicating success or failure
fn prompt_for_configuration_export(configuration: &ValidationConfiguration) -> Result<(), ValidationError> {
    println!("\nWould you like to export this configuration to a file? (y/n):");
    print!("Choice: ");
    io::stdout().flush()?;

    let mut export_choice = String::new();
    io::stdin().read_line(&mut export_choice)?;

    if export_choice.trim().to_lowercase() == "y" || export_choice.trim().to_lowercase() == "yes" {
        println!("Enter the absolute path where you want to save the configuration:");
        print!("File path: ");
        io::stdout().flush()?;

        let mut file_path_input = String::new();
        io::stdin().read_line(&mut file_path_input)?;
        let file_path = file_path_input.trim();

        if file_path.is_empty() {
            return Err(ValidationError::FileError("File path cannot be empty".to_string()));
        }

        configuration.export_to_file(file_path)?;
        println!("Configuration exported successfully to: {}", file_path);
    }

    Ok(())
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

/// Creates a validation configuration from user input or file import
/// 
/// # Returns
/// Result containing the validation configuration or an error
fn create_validation_configuration() -> Result<ValidationConfiguration, ValidationError> {
    let configuration_source = prompt_for_configuration_source()?;

    match configuration_source {
        ConfigurationSource::Manual => {
            println!("\n=== Manual Configuration Setup ===");
            
            // Collect integer validation ranges from user
            let integer_validation_ranges = collect_integer_validation_ranges_from_user()?;

            // Collect integer-string validation rules from user
            let integer_string_validation_rules = collect_integer_string_validation_rules_from_user()?;

            // Ask for optional configuration name
            println!("Enter an optional name for this configuration (or press Enter to skip):");
            print!("Configuration name: ");
            io::stdout().flush()?;
            
            let mut config_name_input = String::new();
            io::stdin().read_line(&mut config_name_input)?;
            let config_name = if config_name_input.trim().is_empty() {
                None
            } else {
                Some(config_name_input.trim().to_string())
            };

            Ok(ValidationConfiguration::new(
                integer_validation_ranges,
                integer_string_validation_rules,
                config_name,
            ))
        }
        ConfigurationSource::File => {
            println!("\n=== Import Configuration from File ===");
            import_configuration_from_file()
        }
    }
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
/// 1. Creates or imports a validation configuration
/// 2. Creates a validation engine with those rules
/// 3. Optionally exports the configuration
/// 4. Continuously accepts input and validates it
/// 5. Displays structured validation results
fn main() -> Result<(), ValidationError> {
    println!("=== Input Validation System with Configuration Import/Export ===\n");

    // Create or import validation configuration
    let validation_configuration = create_validation_configuration()?;

    // Display configuration info
    if let Some(name) = validation_configuration.get_configuration_name() {
        println!("\nLoaded configuration: '{}'", name);
    }
    println!("Configuration loaded with:");
    println!("- {} integer range(s)", validation_configuration.get_integer_ranges().len());
    println!("- {} integer-string rule(s)", validation_configuration.get_integer_string_rules().len());

    // Prompt for configuration export
    prompt_for_configuration_export(&validation_configuration)?;

    // Create the validation engine from the configuration
    let validation_engine = InputValidationEngine::from_configuration(&validation_configuration);

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
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_integer_validation_range_json_serialization() {
        let range = IntegerValidationRange::new(1, 10);
        let json = range.to_json_string();
        let parsed_range = IntegerValidationRange::from_json_string(&json).unwrap();
        
        assert_eq!(range, parsed_range);
    }

    #[test]
    fn test_integer_string_validation_rule_json_serialization() {
        let range = IntegerValidationRange::new(5, 15);
        let rule = IntegerStringValidationRule::new(range, 20);
        let json = rule.to_json_string();
        let parsed_rule = IntegerStringValidationRule::from_json_string(&json).unwrap();
        
        assert_eq!(rule.get_integer_range().get_minimum_value(), parsed_rule.get_integer_range().get_minimum_value());
        assert_eq!(rule.get_integer_range().get_maximum_value(), parsed_rule.get_integer_range().get_maximum_value());
        assert_eq!(rule.get_maximum_string_length(), parsed_rule.get_maximum_string_length());
    }

    #[test]
    fn test_validation_configuration_file_operations() -> Result<(), ValidationError> {
        // Create a temporary configuration
        let integer_ranges = vec![IntegerValidationRange::new(1, 5)];
        let integer_string_rules = vec![IntegerStringValidationRule::new(
            IntegerValidationRange::new(10, 20),
            15,
        )];
        let config = ValidationConfiguration::new(
            integer_ranges,
            integer_string_rules,
            Some("Test Configuration".to_string()),
        );

        // Create a temporary file path
        let mut temp_path = std::env::temp_dir();
        temp_path.push("test_validation_config.json");

        // Export the configuration
        config.export_to_file(&temp_path)?;

        // Import the configuration
        let imported_config = ValidationConfiguration::import_from_file(&temp_path)?;

        // Verify the imported configuration
        assert_eq!(config.get_configuration_name(), imported_config.get_configuration_name());
        assert_eq!(config.get_integer_ranges().len(), imported_config.get_integer_ranges().len());
        assert_eq!(config.get_integer_string_rules().len(), imported_config.get_integer_string_rules().len());

        // Clean up
        if temp_path.exists() {
            fs::remove_file(&temp_path).ok();
        }

        Ok(())
    }

    #[test]
    fn test_validation_engine_from_configuration() {
        let integer_ranges = vec![IntegerValidationRange::new(1, 3)];
        let string_rules = vec![IntegerStringValidationRule::new(
            IntegerValidationRange::new(10, 20),
            10,
        )];
        
        let config = ValidationConfiguration::new(
            integer_ranges,
            string_rules,
            Some("Test Config".to_string()),
        );
        
        let engine = InputValidationEngine::from_configuration(&config);
        
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
        
        let empty_inputs = parse_comma_separated_inputs(",,,");
        assert!(empty_inputs.is_empty());
    }
}
