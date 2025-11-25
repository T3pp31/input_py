//! Tests for the process_input pure function
//!
//! These tests verify the input processing logic without any I/O operations.

use input_py::process_input;

#[test]
fn test_normal_input_with_trim() {
    // Given: A normal input string with trailing newline
    let input = "hello\n".to_string();

    // When: Processing with trim enabled
    let result = process_input(input, None, true);

    // Then: Whitespace should be trimmed
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello");
}

#[test]
fn test_input_with_surrounding_whitespace_trim_enabled() {
    // Given: Input with leading and trailing whitespace
    let input = "  hello world  \n".to_string();

    // When: Processing with trim enabled
    let result = process_input(input, None, true);

    // Then: All surrounding whitespace should be removed
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello world");
}

#[test]
fn test_input_with_surrounding_whitespace_trim_disabled() {
    // Given: Input with leading and trailing whitespace
    let input = "  hello world  \n".to_string();

    // When: Processing with trim disabled
    let result = process_input(input, None, false);

    // Then: Only trailing newline should be removed
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "  hello world  ");
}

#[test]
fn test_empty_input_with_default() {
    // Given: Empty input string with newline
    let input = "\n".to_string();
    let default = "default_value";

    // When: Processing with trim enabled and default value
    let result = process_input(input, Some(default), true);

    // Then: Default value should be returned
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "default_value");
}

#[test]
fn test_whitespace_only_input_with_default() {
    // Given: Whitespace-only input
    let input = "   \n".to_string();
    let default = "fallback";

    // When: Processing with trim enabled and default value
    let result = process_input(input, Some(default), true);

    // Then: Default value should be returned
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "fallback");
}

#[test]
fn test_empty_input_without_default() {
    // Given: Empty input with no default
    let input = "\n".to_string();

    // When: Processing with trim enabled
    let result = process_input(input, None, true);

    // Then: Empty string should be returned
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_crlf_newline_handling() {
    // Given: Input with Windows-style CRLF newline
    let input = "test\r\n".to_string();

    // When: Processing with trim disabled
    let result = process_input(input, None, false);

    // Then: Both CR and LF should be removed
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_input_without_newline() {
    // Given: Input without trailing newline
    let input = "no newline".to_string();

    // When: Processing with trim disabled
    let result = process_input(input, None, false);

    // Then: String should remain unchanged
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "no newline");
}

#[test]
fn test_japanese_input() {
    // Given: Japanese input
    let input = "こんにちは\n".to_string();

    // When: Processing with trim enabled
    let result = process_input(input, None, true);

    // Then: Japanese characters should be preserved
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "こんにちは");
}

#[test]
fn test_completely_empty_input() {
    // Given: Completely empty string (boundary case)
    let input = "".to_string();

    // When: Processing with trim enabled
    let result = process_input(input, None, true);

    // Then: Empty string should be returned
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_user_input_overrides_default() {
    // Given: Non-empty input with default value available
    let input = "user_value\n".to_string();
    let default = "default";

    // When: Processing with trim enabled
    let result = process_input(input, Some(default), true);

    // Then: User input should override default
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "user_value");
}

#[test]
fn test_empty_default_value() {
    // Given: Empty input with empty default
    let input = "\n".to_string();
    let default = "";

    // When: Processing with trim enabled
    let result = process_input(input, Some(default), true);

    // Then: Empty default should be returned
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_tabs_and_spaces_mixed() {
    // Given: Input with mixed whitespace
    let input = "\t  hello \t \n".to_string();

    // When: Processing with trim enabled
    let result = process_input(input, None, true);

    // Then: All whitespace should be trimmed
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello");
}

#[test]
fn test_only_newline_trim_disabled() {
    // Given: Only newline character
    let input = "\n".to_string();

    // When: Processing with trim disabled
    let result = process_input(input, None, false);

    // Then: Empty string after newline removal
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_special_characters() {
    // Given: Input with special characters
    let input = "hello@world#123$%\n".to_string();

    // When: Processing with trim enabled
    let result = process_input(input, None, true);

    // Then: Special characters should be preserved
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello@world#123$%");
}
