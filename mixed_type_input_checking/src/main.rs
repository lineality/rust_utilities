//! Input Validation System with Configuration Import/Export and Overlap Detection
//! 
//! This module provides a comprehensive input validation system that can validate
//! integers within specified ranges and integer-string pairs with length constraints.
//! The system supports importing and exporting validation configurations to/from JSON files,
//! and includes comprehensive overlap detection to ensure validation rules are unambiguous.

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
    /// Error occurred due to overlapping ranges in configuration
    OverlapError(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::ParseError(message) => write!(formatter, "Parse error: {}", message),
            ValidationError::IoError(message) => write!(formatter, "I/O error: {}", message),
            ValidationError::ConfigurationError(message) => write!(formatter, "Configuration error: {}", message),
            ValidationError::FileError(message) => write!(formatter, "File error: {}", message),
            ValidationError::JsonError(message) => write!(formatter, "JSON error: {}", message),
            ValidationError::OverlapError(message) => write!(formatter, "Range overlap error: {}", message),
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

/// Detailed information about a detected range overlap
/// 
/// This struct provides comprehensive information about where and how
/// ranges overlap, making it easier to identify and resolve conflicts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeOverlapDetails {
    /// Description of the type of overlap detected
    overlap_description: String,
    /// The first range involved in the overlap
    first_range_description: String,
    /// The second range involved in the overlap
    second_range_description: String,
    /// The overlapping portion (minimum value of the overlap)
    overlap_start_value: i32,
    /// The overlapping portion (maximum value of the overlap)
    overlap_end_value: i32,
}

impl RangeOverlapDetails {
    /// Creates a new range overlap details instance
    /// 
    /// # Arguments
    /// * `overlap_description` - Description of the type of overlap
    /// * `first_range_description` - Description of the first overlapping range
    /// * `second_range_description` - Description of the second overlapping range
    /// * `overlap_start_value` - The start of the overlapping portion
    /// * `overlap_end_value` - The end of the overlapping portion
    /// 
    /// # Returns
    /// A new `RangeOverlapDetails` instance
    pub fn new(
        overlap_description: String,
        first_range_description: String,
        second_range_description: String,
        overlap_start_value: i32,
        overlap_end_value: i32,
    ) -> Self {
        Self {
            overlap_description,
            first_range_description,
            second_range_description,
            overlap_start_value,
            overlap_end_value,
        }
    }

    /// Gets the overlap description
    /// 
    /// # Returns
    /// A reference to the overlap description string
    pub fn get_overlap_description(&self) -> &str {
        &self.overlap_description
    }

    /// Gets the first range description
    /// 
    /// # Returns
    /// A reference to the first range description string
    pub fn get_first_range_description(&self) -> &str {
        &self.first_range_description
    }

    /// Gets the second range description
    /// 
    /// # Returns
    /// A reference to the second range description string
    pub fn get_second_range_description(&self) -> &str {
        &self.second_range_description
    }

    /// Gets the start value of the overlap
    /// 
    /// # Returns
    /// The minimum value where the ranges overlap
    pub fn get_overlap_start_value(&self) -> i32 {
        self.overlap_start_value
    }

    /// Gets the end value of the overlap
    /// 
    /// # Returns
    /// The maximum value where the ranges overlap
    pub fn get_overlap_end_value(&self) -> i32 {
        self.overlap_end_value
    }
}

impl fmt::Display for RangeOverlapDetails {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}: {} overlaps with {} in range [{}, {}]",
            self.overlap_description,
            self.first_range_description,
            self.second_range_description,
            self.overlap_start_value,
            self.overlap_end_value
        )
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

    /// Checks if this range overlaps with another integer validation range
    /// 
    /// Two ranges overlap if they share any common integer values.
    /// This method uses inclusive bounds, so ranges like [1,5] and [5,10] do overlap at value 5.
    /// 
    /// # Arguments
    /// * `other_range` - The other range to check for overlap with
    /// 
    /// # Returns
    /// `Some(RangeOverlapDetails)` if the ranges overlap, `None` if they don't overlap
    /// 
    /// # Examples
    /// ```
    /// let range1 = IntegerValidationRange::new(1, 10);
    /// let range2 = IntegerValidationRange::new(5, 15);
    /// assert!(range1.check_overlap_with_integer_range(&range2).is_some());
    /// 
    /// let range3 = IntegerValidationRange::new(20, 30);
    /// assert!(range1.check_overlap_with_integer_range(&range3).is_none());
    /// ```
    pub fn check_overlap_with_integer_range(&self, other_range: &IntegerValidationRange) -> Option<RangeOverlapDetails> {
        // Calculate the overlap boundaries
        let overlap_start = std::cmp::max(self.minimum_value, other_range.minimum_value);
        let overlap_end = std::cmp::min(self.maximum_value, other_range.maximum_value);

        // Check if there's actually an overlap (start <= end means there's at least one overlapping value)
        if overlap_start <= overlap_end {
            Some(RangeOverlapDetails::new(
                "Integer range overlap detected".to_string(),
                format!("integer range [{}, {}]", self.minimum_value, self.maximum_value),
                format!("integer range [{}, {}]", other_range.minimum_value, other_range.maximum_value),
                overlap_start,
                overlap_end,
            ))
        } else {
            None
        }
    }

    /// Checks if this range overlaps with an integer-string validation rule's integer range
    /// 
    /// This method checks if the integer part of an integer-string rule overlaps
    /// with this standalone integer range, which would create ambiguous validation.
    /// 
    /// # Arguments
    /// * `integer_string_rule` - The integer-string rule to check for overlap with
    /// 
    /// # Returns
    /// `Some(RangeOverlapDetails)` if the ranges overlap, `None` if they don't overlap
    /// 
    /// # Examples
    /// ```
    /// let int_range = IntegerValidationRange::new(1, 10);
    /// let string_rule = IntegerStringValidationRule::new(
    ///     IntegerValidationRange::new(5, 15), 
    ///     20
    /// );
    /// assert!(int_range.check_overlap_with_integer_string_rule(&string_rule).is_some());
    /// ```
    pub fn check_overlap_with_integer_string_rule(&self, integer_string_rule: &IntegerStringValidationRule) -> Option<RangeOverlapDetails> {
        let other_range = integer_string_rule.get_integer_range();
        
        // Calculate the overlap boundaries
        let overlap_start = std::cmp::max(self.minimum_value, other_range.minimum_value);
        let overlap_end = std::cmp::min(self.maximum_value, other_range.maximum_value);

        // Check if there's actually an overlap
        if overlap_start <= overlap_end {
            Some(RangeOverlapDetails::new(
                "Cross-type range overlap detected".to_string(),
                format!("standalone integer range [{}, {}]", self.minimum_value, self.maximum_value),
                format!("integer-string rule integer range [{}, {}] (max string length: {})",
                    other_range.minimum_value, 
                    other_range.maximum_value,
                    integer_string_rule.get_maximum_string_length()
                ),
                overlap_start,
                overlap_end,
            ))
        } else {
            None
        }
    }

    /// Creates a human-readable description of this range for error reporting
    /// 
    /// # Returns
    /// A string describing this range in a user-friendly format
    pub fn create_range_description(&self) -> String {
        format!("integer range [{}, {}]", self.minimum_value, self.maximum_value)
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

    /// Checks if this integer-string rule's integer range overlaps with another integer-string rule
    /// 
    /// Two integer-string rules overlap if their integer ranges share any common values.
    /// This creates ambiguous validation because the same integer could match multiple rules
    /// with potentially different string length constraints.
    /// 
    /// # Arguments
    /// * `other_rule` - The other integer-string rule to check for overlap with
    /// 
    /// # Returns
    /// `Some(RangeOverlapDetails)` if the integer ranges overlap, `None` if they don't overlap
    /// 
    /// # Examples
    /// ```
    /// let rule1 = IntegerStringValidationRule::new(
    ///     IntegerValidationRange::new(1, 10), 
    ///     5
    /// );
    /// let rule2 = IntegerStringValidationRule::new(
    ///     IntegerValidationRange::new(8, 15), 
    ///     10
    /// );
    /// assert!(rule1.check_overlap_with_integer_string_rule(&rule2).is_some());
    /// ```
    pub fn check_overlap_with_integer_string_rule(&self, other_rule: &IntegerStringValidationRule) -> Option<RangeOverlapDetails> {
        let other_range = other_rule.get_integer_range();
        
        // Calculate the overlap boundaries
        let overlap_start = std::cmp::max(self.integer_range.minimum_value, other_range.minimum_value);
        let overlap_end = std::cmp::min(self.integer_range.maximum_value, other_range.maximum_value);

        // Check if there's actually an overlap
        if overlap_start <= overlap_end {
            Some(RangeOverlapDetails::new(
                "Integer-string rule overlap detected".to_string(),
                format!("integer-string rule with range [{}, {}] (max string length: {})",
                    self.integer_range.minimum_value, 
                    self.integer_range.maximum_value,
                    self.maximum_string_length
                ),
                format!("integer-string rule with range [{}, {}] (max string length: {})",
                    other_range.minimum_value, 
                    other_range.maximum_value,
                    other_rule.maximum_string_length
                ),
                overlap_start,
                overlap_end,
            ))
        } else {
            None
        }
    }

    /// Creates a human-readable description of this rule for error reporting
    /// 
    /// # Returns
    /// A string describing this rule in a user-friendly format
    pub fn create_rule_description(&self) -> String {
        format!(
            "integer-string rule with range [{}, {}] and max string length {}",
            self.integer_range.minimum_value,
            self.integer_range.maximum_value,
            self.maximum_string_length
        )
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

/// Comprehensive overlap validation utility for validation configurations
/// 
/// This struct provides methods to detect and report all types of range overlaps
/// that could cause ambiguous validation behavior in the system.
#[derive(Debug)]
pub struct ValidationRangeOverlapDetector;

impl ValidationRangeOverlapDetector {
    /// Performs comprehensive overlap detection on a complete validation configuration
    /// 
    /// This method checks for all possible types of overlaps:
    /// 1. Integer range to integer range overlaps
    /// 2. Integer-string rule to integer-string rule overlaps (based on integer ranges)
    /// 3. Cross-type overlaps between integer ranges and integer-string rule ranges
    /// 
    /// # Arguments
    /// * `integer_ranges` - Vector of standalone integer validation ranges
    /// * `integer_string_rules` - Vector of integer-string validation rules
    /// 
    /// # Returns
    /// `Ok(())` if no overlaps are detected, or `Err(ValidationError::OverlapError)` with detailed information
    /// 
    /// # Examples
    /// ```
    /// let int_ranges = vec![IntegerValidationRange::new(1, 5)];
    /// let string_rules = vec![IntegerStringValidationRule::new(
    ///     IntegerValidationRange::new(10, 15), 
    ///     20
    /// )];
    /// 
    /// // This should pass - no overlaps
    /// assert!(ValidationRangeOverlapDetector::detect_all_range_overlaps(&int_ranges, &string_rules).is_ok());
    /// ```
    pub fn detect_all_range_overlaps(
        integer_ranges: &[IntegerValidationRange],
        integer_string_rules: &[IntegerStringValidationRule],
    ) -> Result<(), ValidationError> {
        let mut detected_overlaps = Vec::new();

        // Check for overlaps between standalone integer ranges
        let integer_range_overlaps = Self::detect_integer_range_to_integer_range_overlaps(integer_ranges);
        detected_overlaps.extend(integer_range_overlaps);

        // Check for overlaps between integer-string rules (based on their integer ranges)
        let integer_string_rule_overlaps = Self::detect_integer_string_rule_to_integer_string_rule_overlaps(integer_string_rules);
        detected_overlaps.extend(integer_string_rule_overlaps);

        // Check for cross-type overlaps (integer ranges vs integer-string rule ranges)
        let cross_type_overlaps = Self::detect_cross_type_range_overlaps(integer_ranges, integer_string_rules);
        detected_overlaps.extend(cross_type_overlaps);

        // If any overlaps were detected, return a comprehensive error
        if !detected_overlaps.is_empty() {
            let overlap_summary = Self::create_overlap_error_summary(&detected_overlaps);
            return Err(ValidationError::OverlapError(overlap_summary));
        }

        Ok(())
    }

    /// Detects overlaps between standalone integer validation ranges
    /// 
    /// This method checks all pairs of integer ranges to identify any overlapping values
    /// that would cause ambiguous validation behavior.
    /// 
    /// # Arguments
    /// * `integer_ranges` - Vector of integer validation ranges to check
    /// 
    /// # Returns
    /// Vector of `RangeOverlapDetails` for each detected overlap
    fn detect_integer_range_to_integer_range_overlaps(
        integer_ranges: &[IntegerValidationRange]
    ) -> Vec<RangeOverlapDetails> {
        let mut detected_overlaps = Vec::new();

        // Check each pair of integer ranges for overlaps
        for (first_index, first_range) in integer_ranges.iter().enumerate() {
            for (second_index, second_range) in integer_ranges.iter().enumerate() {
                // Only check each pair once (avoid duplicate checks)
                if first_index < second_index {
                    if let Some(overlap_details) = first_range.check_overlap_with_integer_range(second_range) {
                        detected_overlaps.push(overlap_details);
                    }
                }
            }
        }

        detected_overlaps
    }

    /// Detects overlaps between integer-string validation rules based on their integer ranges
    /// 
    /// This method checks all pairs of integer-string rules to identify any overlapping
    /// integer ranges that would cause ambiguous validation behavior.
    /// 
    /// # Arguments
    /// * `integer_string_rules` - Vector of integer-string validation rules to check
    /// 
    /// # Returns
    /// Vector of `RangeOverlapDetails` for each detected overlap
    fn detect_integer_string_rule_to_integer_string_rule_overlaps(
        integer_string_rules: &[IntegerStringValidationRule]
    ) -> Vec<RangeOverlapDetails> {
        let mut detected_overlaps = Vec::new();

        // Check each pair of integer-string rules for overlaps in their integer ranges
        for (first_index, first_rule) in integer_string_rules.iter().enumerate() {
            for (second_index, second_rule) in integer_string_rules.iter().enumerate() {
                // Only check each pair once (avoid duplicate checks)
                if first_index < second_index {
                    if let Some(overlap_details) = first_rule.check_overlap_with_integer_string_rule(second_rule) {
                        detected_overlaps.push(overlap_details);
                    }
                }
            }
        }

        detected_overlaps
    }

    /// Detects cross-type overlaps between integer ranges and integer-string rule ranges
    /// 
    /// This method identifies cases where a standalone integer range overlaps with
    /// the integer range of an integer-string rule, which creates ambiguous validation.
    /// 
    /// # Arguments
    /// * `integer_ranges` - Vector of standalone integer validation ranges
    /// * `integer_string_rules` - Vector of integer-string validation rules
    /// 
    /// # Returns
    /// Vector of `RangeOverlapDetails` for each detected cross-type overlap
    fn detect_cross_type_range_overlaps(
        integer_ranges: &[IntegerValidationRange],
        integer_string_rules: &[IntegerStringValidationRule],
    ) -> Vec<RangeOverlapDetails> {
        let mut detected_overlaps = Vec::new();

        // Check each integer range against each integer-string rule's integer range
        for integer_range in integer_ranges {
            for integer_string_rule in integer_string_rules {
                if let Some(overlap_details) = integer_range.check_overlap_with_integer_string_rule(integer_string_rule) {
                    detected_overlaps.push(overlap_details);
                }
            }
        }

        detected_overlaps
    }

    /// Creates a comprehensive error summary from detected overlaps
    /// 
    /// This method takes all detected overlaps and formats them into a single,
    /// comprehensive error message that clearly explains all the conflicts.
    /// 
    /// # Arguments
    /// * `detected_overlaps` - Vector of all detected range overlaps
    /// 
    /// # Returns
    /// A formatted string summarizing all detected overlaps
    fn create_overlap_error_summary(detected_overlaps: &[RangeOverlapDetails]) -> String {
        let mut error_message = format!(
            "Configuration contains {} range overlap(s) that would cause ambiguous validation:\n\n",
            detected_overlaps.len()
        );

        for (overlap_index, overlap_details) in detected_overlaps.iter().enumerate() {
            error_message.push_str(&format!(
                "{}. {}\n   Overlapping values: {} to {}\n\n",
                overlap_index + 1,
                overlap_details,
                overlap_details.overlap_start_value,
                overlap_details.overlap_end_value
            ));
        }

        error_message.push_str("Please modify your ranges to eliminate these overlaps before proceeding.");
        error_message
    }
}

/// Configuration structure that holds all validation rules with overlap validation
/// 
/// This struct can be serialized to and deserialized from JSON format
/// for easy import/export of validation configurations. It includes
/// comprehensive overlap detection to ensure validation rules are unambiguous.
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
    /// Creates a new validation configuration with overlap validation
    /// 
    /// This constructor automatically validates that the provided ranges do not overlap,
    /// ensuring that the resulting configuration will produce unambiguous validation results.
    /// 
    /// # Arguments
    /// * `integer_ranges` - Vector of integer validation ranges
    /// * `integer_string_rules` - Vector of integer-string validation rules
    /// * `configuration_name` - Optional name for this configuration
    /// 
    /// # Returns
    /// `Ok(ValidationConfiguration)` if no overlaps are detected, or `Err(ValidationError::OverlapError)`
    /// 
    /// # Examples
    /// ```
    /// let int_ranges = vec![IntegerValidationRange::new(1, 5)];
    /// let string_rules = vec![IntegerStringValidationRule::new(
    ///     IntegerValidationRange::new(10, 15), 
    ///     20
    /// )];
    /// 
    /// let config = ValidationConfiguration::new(int_ranges, string_rules, None)?;
    /// ```
    pub fn new(
        integer_ranges: Vec<IntegerValidationRange>,
        integer_string_rules: Vec<IntegerStringValidationRule>,
        configuration_name: Option<String>,
    ) -> Result<Self, ValidationError> {
        // Validate that there are no overlapping ranges
        ValidationRangeOverlapDetector::detect_all_range_overlaps(&integer_ranges, &integer_string_rules)?;

        Ok(Self {
            integer_ranges,
            integer_string_rules,
            configuration_name,
        })
    }

    /// Creates a new validation configuration without overlap validation (for internal use)
    /// 
    /// This method is used internally when we know the ranges are already validated,
    /// such as during JSON deserialization where we validate separately.
    /// 
    /// # Arguments
    /// * `integer_ranges` - Vector of integer validation ranges
    /// * `integer_string_rules` - Vector of integer-string validation rules
    /// * `configuration_name` - Optional name for this configuration
    /// 
    /// # Returns
    /// A new `ValidationConfiguration` instance without overlap validation
    fn new_without_overlap_validation(
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

    /// Validates the current configuration for range overlaps
    /// 
    /// This method can be called to re-validate a configuration after it has been
    /// modified or loaded from an external source.
    /// 
    /// # Returns
    /// `Ok(())` if no overlaps are detected, or `Err(ValidationError::OverlapError)`
    pub fn validate_configuration_for_overlaps(&self) -> Result<(), ValidationError> {
        ValidationRangeOverlapDetector::detect_all_range_overlaps(&self.integer_ranges, &self.integer_string_rules)
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

    /// Imports a configuration from a JSON file with overlap validation
    /// 
    /// This method loads a configuration from a JSON file and automatically
    /// validates it for range overlaps before returning it.
    /// 
    /// # Arguments
    /// * `file_path` - The absolute path to the configuration file to load
    /// 
    /// # Returns
    /// Result containing the loaded and validated configuration or an error
    pub fn import_from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, ValidationError> {
        let json_content = fs::read_to_string(file_path)
            .map_err(|error| ValidationError::FileError(format!("Failed to read configuration file: {}", error)))?;
        
        let configuration = Self::from_json_string(&json_content)?;
        
        // Validate the imported configuration for overlaps
        configuration.validate_configuration_for_overlaps()?;
        
        Ok(configuration)
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

    /// Creates a ValidationConfiguration from a JSON string without overlap validation
    /// 
    /// This method is used internally during import to create the configuration
    /// before separate overlap validation is performed.
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

        Ok(Self::new_without_overlap_validation(integer_ranges, integer_string_rules, configuration_name))
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
/// validate individual inputs and batches of inputs. It ensures that
/// all rules are non-overlapping for unambiguous validation.
#[derive(Debug)]
pub struct InputValidationEngine {
    /// List of valid integer ranges for standalone integer validation
    integer_validation_ranges: Vec<IntegerValidationRange>,
    /// List of validation rules for integer-string pairs
    integer_string_validation_rules: Vec<IntegerStringValidationRule>,
}

impl InputValidationEngine {
    /// Creates a new validation engine with the specified rules and overlap validation
    /// 
    /// This constructor automatically validates that the provided ranges do not overlap,
    /// ensuring that the resulting engine will produce unambiguous validation results.
    /// 
    /// # Arguments
    /// * `integer_validation_ranges` - Vector of valid integer ranges
    /// * `integer_string_validation_rules` - Vector of integer-string validation rules
    /// 
    /// # Returns
    /// `Ok(InputValidationEngine)` if no overlaps are detected, or `Err(ValidationError::OverlapError)`
    pub fn new(
        integer_validation_ranges: Vec<IntegerValidationRange>,
        integer_string_validation_rules: Vec<IntegerStringValidationRule>,
    ) -> Result<Self, ValidationError> {
        // Validate that there are no overlapping ranges
        ValidationRangeOverlapDetector::detect_all_range_overlaps(&integer_validation_ranges, &integer_string_validation_rules)?;

        Ok(Self {
            integer_validation_ranges,
            integer_string_validation_rules,
        })
    }

    /// Creates a new validation engine from a configuration
    /// 
    /// Since the configuration has already been validated for overlaps,
    /// this method can safely create the engine without additional validation.
    /// 
    /// # Arguments
    /// * `configuration` - The validation configuration to use
    /// 
    /// # Returns
    /// A new `InputValidationEngine` instance
    pub fn from_configuration(configuration: &ValidationConfiguration) -> Self {
        Self {
            integer_validation_ranges: configuration.integer_ranges.clone(),
            integer_string_validation_rules: configuration.integer_string_rules.clone(),
        }
    }

    /// Gets the current configuration from this engine
    /// 
    /// # Arguments
    /// * `configuration_name` - Optional name for the configuration
    /// 
    /// # Returns
    /// Result containing a `ValidationConfiguration` representing the current engine state
    pub fn to_configuration(&self, configuration_name: Option<String>) -> Result<ValidationConfiguration, ValidationError> {
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

/// Collects integer validation ranges from user input with overlap checking
/// 
/// This function collects ranges one by one and checks for overlaps as they are added,
/// providing immediate feedback to the user if conflicts are detected.
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
        loop {
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
                println!("Error: Minimum value cannot be greater than maximum value. Please try again.\n");
                continue;
            }

            let new_range = IntegerValidationRange::new(minimum_value, maximum_value);
            
            // Check for overlaps with existing ranges
            let mut has_overlap = false;
            for existing_range in &validation_ranges {
                if let Some(overlap_details) = new_range.check_overlap_with_integer_range(existing_range) {
                    println!("Error: {}", overlap_details);
                    println!("Please enter a different range that doesn't overlap.\n");
                    has_overlap = true;
                    break;
                }
            }

            if !has_overlap {
                validation_ranges.push(new_range);
                println!("Range [{}, {}] added successfully.\n", minimum_value, maximum_value);
                break;
            }
        }
    }

    Ok(validation_ranges)
}

/// Collects integer-string validation rules from user input with overlap checking
/// 
/// This function collects rules one by one and checks for overlaps as they are added,
/// providing immediate feedback to the user if conflicts are detected.
/// 
/// # Arguments
/// * `existing_integer_ranges` - Previously defined integer ranges to check for cross-type overlaps
/// 
/// # Returns
/// A vector of `IntegerStringValidationRule` instances or an error
fn collect_integer_string_validation_rules_from_user(
    existing_integer_ranges: &[IntegerValidationRange]
) -> Result<Vec<IntegerStringValidationRule>, ValidationError> {
    let mut validation_rules = Vec::new();
    
    println!("Enter the number of integer ranges with string constraints you want to add:");
    io::stdout().flush()?;
    
    let mut number_of_rules_input = String::new();
    io::stdin().read_line(&mut number_of_rules_input)?;
    
    let number_of_rules: usize = number_of_rules_input.trim().parse()
        .map_err(|_| ValidationError::ParseError("Please enter a valid number".to_string()))?;

    for rule_index in 0..number_of_rules {
        loop {
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
                println!("Error: Minimum value cannot be greater than maximum value. Please try again.\n");
                continue;
            }

            println!("Enter the maximum string length for range {}:", rule_index + 1);
            io::stdout().flush()?;
            
            let mut maximum_string_length_input = String::new();
            io::stdin().read_line(&mut maximum_string_length_input)?;
            
            let maximum_string_length: usize = maximum_string_length_input.trim().parse()
                .map_err(|_| ValidationError::ParseError("Please enter a valid number".to_string()))?;

            let integer_range = IntegerValidationRange::new(minimum_value, maximum_value);
            let new_rule = IntegerStringValidationRule::new(integer_range, maximum_string_length);
            
            let mut has_overlap = false;
            
            // Check for overlaps with existing integer-string rules
            for existing_rule in &validation_rules {
                if let Some(overlap_details) = new_rule.check_overlap_with_integer_string_rule(existing_rule) {
                    println!("Error: {}", overlap_details);
                    println!("Please enter a different range that doesn't overlap.\n");
                    has_overlap = true;
                    break;
                }
            }

            // Check for cross-type overlaps with existing integer ranges
            if !has_overlap {
                for existing_integer_range in existing_integer_ranges {
                    if let Some(overlap_details) = existing_integer_range.check_overlap_with_integer_string_rule(&new_rule) {
                        println!("Error: {}", overlap_details);
                        println!("Please enter a different range that doesn't overlap.\n");
                        has_overlap = true;
                        break;
                    }
                }
            }

            if !has_overlap {
                validation_rules.push(new_rule);
                println!("Integer-string rule with range [{}, {}] and max string length {} added successfully.\n", 
                    minimum_value, maximum_value, maximum_string_length);
                break;
            }
        }
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
            println!("\n=== Manual Configuration Setup with Overlap Detection ===");
            println!("Note: The system will automatically detect and prevent overlapping ranges.\n");
            
            // Collect integer validation ranges from user with overlap checking
            let integer_validation_ranges = collect_integer_validation_ranges_from_user()?;

            // Collect integer-string validation rules from user with overlap checking
            let integer_string_validation_rules = collect_integer_string_validation_rules_from_user(&integer_validation_ranges)?;

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

            // Since we've been checking for overlaps during input, this should succeed
            ValidationConfiguration::new(
                integer_validation_ranges,
                integer_string_validation_rules,
                config_name,
            )
        }
        ConfigurationSource::File => {
            println!("\n=== Import Configuration from File with Overlap Validation ===");
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

/// Main function that orchestrates the input validation system with overlap detection
/// 
/// This function:
/// 1. Creates or imports a validation configuration with overlap detection
/// 2. Creates a validation engine with those rules
/// 3. Optionally exports the configuration
/// 4. Continuously accepts input and validates it
/// 5. Displays structured validation results
fn main() -> Result<(), ValidationError> {
    println!("=== Input Validation System with Configuration Import/Export and Overlap Detection ===\n");

    // Create or import validation configuration with overlap detection
    let validation_configuration = create_validation_configuration()?;

    // Display configuration info
    if let Some(name) = validation_configuration.get_configuration_name() {
        println!("\nLoaded configuration: '{}'", name);
    }
    println!("Configuration loaded successfully with:");
    println!("- {} integer range(s)", validation_configuration.get_integer_ranges().len());
    println!("- {} integer-string rule(s)", validation_configuration.get_integer_string_rules().len());
    println!("- No overlapping ranges detected ✓");

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

    #[test]
    fn test_integer_range_overlap_detection() {
        let range1 = IntegerValidationRange::new(1, 10);
        let range2 = IntegerValidationRange::new(5, 15);
        let range3 = IntegerValidationRange::new(20, 30);

        // Should detect overlap between range1 and range2
        assert!(range1.check_overlap_with_integer_range(&range2).is_some());
        
        // Should not detect overlap between range1 and range3
        assert!(range1.check_overlap_with_integer_range(&range3).is_none());
    }

    #[test]
    fn test_integer_string_rule_overlap_detection() {
        let rule1 = IntegerStringValidationRule::new(
            IntegerValidationRange::new(1, 10),
            5
        );
        let rule2 = IntegerStringValidationRule::new(
            IntegerValidationRange::new(8, 15),
            10
        );
        let rule3 = IntegerStringValidationRule::new(
            IntegerValidationRange::new(20, 30),
            15
        );

        // Should detect overlap between rule1 and rule2
        assert!(rule1.check_overlap_with_integer_string_rule(&rule2).is_some());
        
        // Should not detect overlap between rule1 and rule3
        assert!(rule1.check_overlap_with_integer_string_rule(&rule3).is_none());
    }

    #[test]
    fn test_cross_type_overlap_detection() {
        let int_range = IntegerValidationRange::new(1, 10);
        let string_rule = IntegerStringValidationRule::new(
            IntegerValidationRange::new(5, 15),
            20
        );
        let non_overlapping_rule = IntegerStringValidationRule::new(
            IntegerValidationRange::new(20, 30),
            20
        );

        // Should detect cross-type overlap
        assert!(int_range.check_overlap_with_integer_string_rule(&string_rule).is_some());
        
        // Should not detect overlap with non-overlapping rule
        assert!(int_range.check_overlap_with_integer_string_rule(&non_overlapping_rule).is_none());
    }

    #[test]
    fn test_validation_configuration_overlap_rejection() {
        let overlapping_ranges = vec![
            IntegerValidationRange::new(1, 10),
            IntegerValidationRange::new(5, 15),  // Overlaps with first range
        ];
        let rules = vec![];

        // Should reject configuration with overlapping ranges
        assert!(ValidationConfiguration::new(overlapping_ranges, rules, None).is_err());
    }

    #[test]
    fn test_validation_configuration_overlap_acceptance() -> Result<(), ValidationError> {
        let non_overlapping_ranges = vec![
            IntegerValidationRange::new(1, 10),
            IntegerValidationRange::new(20, 30),  // Does not overlap
        ];
        let rules = vec![
            IntegerStringValidationRule::new(
                IntegerValidationRange::new(100, 200),  // Does not overlap with ranges
                15
            )
        ];

        // Should accept configuration with non-overlapping ranges
        let config = ValidationConfiguration::new(non_overlapping_ranges, rules, None)?;
        assert_eq!(config.get_integer_ranges().len(), 2);
        assert_eq!(config.get_integer_string_rules().len(), 1);
        
        Ok(())
    }

    #[test]
    fn test_comprehensive_overlap_detection() {
        let int_ranges = vec![
            IntegerValidationRange::new(1, 5),
            IntegerValidationRange::new(3, 8),  // Overlaps with first
        ];
        let string_rules = vec![
            IntegerStringValidationRule::new(
                IntegerValidationRange::new(7, 12),  // Overlaps with second int range
                10
            ),
        ];

        // Should detect multiple overlaps
        let result = ValidationRangeOverlapDetector::detect_all_range_overlaps(&int_ranges, &string_rules);
        assert!(result.is_err());
        
        if let Err(ValidationError::OverlapError(message)) = result {
            // Should mention multiple overlaps
            assert!(message.contains("2 range overlap(s)"));
        }
    }

    #[test]
    fn test_validation_engine_creation_with_overlaps() {
        let overlapping_ranges = vec![
            IntegerValidationRange::new(1, 10),
            IntegerValidationRange::new(5, 15),  // Overlaps
        ];
        let rules = vec![];

        // Should reject engine creation with overlapping ranges
        assert!(InputValidationEngine::new(overlapping_ranges, rules).is_err());
    }

    #[test]
    fn test_range_overlap_details_display() {
        let overlap = RangeOverlapDetails::new(
            "Test overlap".to_string(),
            "range A".to_string(),
            "range B".to_string(),
            5,
            10
        );

        let display_string = format!("{}", overlap);
        assert!(display_string.contains("Test overlap"));
        assert!(display_string.contains("range A"));
        assert!(display_string.contains("range B"));
        assert!(display_string.contains("[5, 10]"));
    }

    #[test]
    fn test_edge_case_touching_ranges() {
        let range1 = IntegerValidationRange::new(1, 5);
        let range2 = IntegerValidationRange::new(5, 10);  // Touches at value 5

        // Touching ranges should be considered overlapping (inclusive bounds)
        assert!(range1.check_overlap_with_integer_range(&range2).is_some());
    }

    #[test]
    fn test_edge_case_adjacent_ranges() {
        let range1 = IntegerValidationRange::new(1, 5);
        let range2 = IntegerValidationRange::new(6, 10);  // Adjacent but not touching

        // Adjacent ranges should not be considered overlapping
        assert!(range1.check_overlap_with_integer_range(&range2).is_none());
    }
}
