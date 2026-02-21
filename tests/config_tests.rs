//! Tests for the config module
//!
//! These tests verify that all configuration constants are properly defined.

use input_py::config;

#[test]
fn test_demo_constants_exist() {
    // Given/When/Then: Demo constants should be accessible
    assert!(!config::demo::TITLE.is_empty());
    assert!(!config::demo::DEFAULT_PORT.is_empty());
}

#[test]
fn test_prompt_constants_exist() {
    // Given/When/Then: Prompt constants should be accessible
    assert!(!config::demo::prompts::NAME.is_empty());
    assert!(!config::demo::prompts::PORT.is_empty());
    assert!(!config::demo::prompts::TEXT_PRESERVED.is_empty());
    assert!(!config::demo::prompts::TEXT_TRIMMED.is_empty());
    // EMPTY_PROMPT is intentionally empty
    assert!(config::demo::prompts::EMPTY_PROMPT.is_empty());
}

#[test]
fn test_message_constants_exist() {
    // Given/When/Then: Message constants should be accessible
    assert!(!config::demo::messages::NO_NAME_ENTERED.is_empty());
    assert!(!config::demo::messages::DEMO_COMPLETED.is_empty());
}

#[test]
fn test_format_constants_exist() {
    // Given/When/Then: Format constants should be accessible
    assert!(!config::format::PROMPT_SUFFIX.is_empty());
}

#[test]
fn test_error_prefix_constants_exist() {
    // Given/When/Then: Error prefix constants should be accessible
    assert!(!config::errors::WRITE_ERROR_PREFIX.is_empty());
    assert!(!config::errors::FLUSH_ERROR_PREFIX.is_empty());
    assert!(!config::errors::READ_ERROR_PREFIX.is_empty());
}

#[test]
fn test_default_port_is_valid_number() {
    // Given: Default port constant
    let port = config::demo::DEFAULT_PORT;

    // When: Parsing as number
    let parsed: Result<u16, _> = port.parse();

    // Then: Should be valid port number
    assert!(parsed.is_ok());
    let port_num = parsed.unwrap();
    assert!(port_num > 0); // u16 is already <= 65535
}
