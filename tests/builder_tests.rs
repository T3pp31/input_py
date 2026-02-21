//! Tests for the Input builder pattern
//!
//! These tests verify the builder API for configuring and reading user input.

mod common;

use common::{FailingReader, FailingWriter, MockReader, MockWriter};
use input_py::{Input, InputError};

// ==========================================================
// Basic builder usage tests
// ==========================================================

#[test]
fn test_builder_basic_input() {
    // Given: A builder with a prompt
    let mut reader = MockReader::new("hello\n");
    let mut writer = MockWriter::new();

    // When: Reading input via builder
    let result = Input::new("Name").read_with_io(&mut reader, &mut writer);

    // Then: Input should be returned with prompt displayed
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello");
    assert!(writer.output().contains("Name:"));
}

#[test]
fn test_builder_empty_prompt() {
    // Given: A builder with empty prompt
    let mut reader = MockReader::new("data\n");
    let mut writer = MockWriter::new();

    // When: Reading input
    let result = Input::new("").read_with_io(&mut reader, &mut writer);

    // Then: No prompt should be displayed
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "data");
    assert!(writer.output().is_empty());
}

// ==========================================================
// Default value tests
// ==========================================================

#[test]
fn test_builder_with_default_empty_input() {
    // Given: Builder with default value and empty input
    let mut reader = MockReader::new("\n");
    let mut writer = MockWriter::new();

    // When: Reading empty input
    let result = Input::new("Port")
        .default("8080")
        .read_with_io(&mut reader, &mut writer);

    // Then: Default value should be returned
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "8080");
    assert!(writer.output().contains("[8080]"));
}

#[test]
fn test_builder_with_default_user_override() {
    // Given: Builder with default value and user input
    let mut reader = MockReader::new("3000\n");
    let mut writer = MockWriter::new();

    // When: Reading user input
    let result = Input::new("Port")
        .default("8080")
        .read_with_io(&mut reader, &mut writer);

    // Then: User input should override default
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "3000");
}

#[test]
fn test_builder_with_empty_default() {
    // Given: Builder with empty default value
    let mut reader = MockReader::new("\n");
    let mut writer = MockWriter::new();

    // When: Reading empty input
    let result = Input::new("Name")
        .default("")
        .read_with_io(&mut reader, &mut writer);

    // Then: Empty default should be returned
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
    // Empty default should not show brackets
    assert!(!writer.output().contains("[]"));
}

// ==========================================================
// Trim control tests
// ==========================================================

#[test]
fn test_builder_trim_enabled() {
    // Given: Builder with trim enabled (default)
    let mut reader = MockReader::new("  hello  \n");
    let mut writer = MockWriter::new();

    // When: Reading input
    let result = Input::new("Text").read_with_io(&mut reader, &mut writer);

    // Then: Whitespace should be trimmed
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello");
}

#[test]
fn test_builder_trim_disabled() {
    // Given: Builder with trim disabled
    let mut reader = MockReader::new("  hello  \n");
    let mut writer = MockWriter::new();

    // When: Reading input with trim disabled
    let result = Input::new("Text")
        .trim(false)
        .read_with_io(&mut reader, &mut writer);

    // Then: Whitespace should be preserved (except newline)
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "  hello  ");
}

// ==========================================================
// Combined options tests (Issue #3 regression)
// ==========================================================

#[test]
fn test_builder_default_and_trim_disabled_empty_input() {
    // Given: Builder with default and trim disabled, empty input
    let mut reader = MockReader::new("\n");
    let mut writer = MockWriter::new();

    // When: Reading empty input
    let result = Input::new("Port")
        .default("8080")
        .trim(false)
        .read_with_io(&mut reader, &mut writer);

    // Then: Default value should be returned even with trim disabled
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "8080");
}

#[test]
fn test_builder_default_and_trim_disabled_with_input() {
    // Given: Builder with default and trim disabled, user provides input
    let mut reader = MockReader::new("  3000  \n");
    let mut writer = MockWriter::new();

    // When: Reading user input
    let result = Input::new("Port")
        .default("8080")
        .trim(false)
        .read_with_io(&mut reader, &mut writer);

    // Then: User input preserved with whitespace
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "  3000  ");
}

// ==========================================================
// show_prompt control tests
// ==========================================================

#[test]
fn test_builder_show_prompt_false() {
    // Given: Builder with show_prompt disabled
    let mut reader = MockReader::new("test\n");
    let mut writer = MockWriter::new();

    // When: Reading with hidden prompt
    let result = Input::new("Secret")
        .show_prompt(false)
        .read_with_io(&mut reader, &mut writer);

    // Then: No prompt should be displayed
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test");
    assert!(writer.output().is_empty());
    assert_eq!(writer.flush_count, 0);
}

#[test]
fn test_builder_show_prompt_true_on_empty() {
    // Given: Builder with empty prompt and show_prompt forced true
    let mut reader = MockReader::new("data\n");
    let mut writer = MockWriter::new();

    // When: Reading with empty prompt but show_prompt=true
    let result = Input::new("")
        .show_prompt(true)
        .read_with_io(&mut reader, &mut writer);

    // Then: No prompt written (empty prompt is still empty regardless of show_prompt)
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "data");
    assert!(writer.output().is_empty());
}

// ==========================================================
// Error handling tests
// ==========================================================

#[test]
fn test_builder_read_error() {
    // Given: A failing reader
    let mut reader = FailingReader;
    let mut writer = MockWriter::new();

    // When: Attempting to read
    let result = Input::new("Prompt").read_with_io(&mut reader, &mut writer);

    // Then: ReadError should be returned
    assert!(result.is_err());
    match result.unwrap_err() {
        InputError::ReadError(_) => {} // Expected
        other => panic!("Expected ReadError, got: {other}"),
    }
}

#[test]
fn test_builder_write_error() {
    // Given: A failing writer
    let mut reader = MockReader::new("test\n");
    let mut writer = FailingWriter;

    // When: Attempting to write prompt
    let result = Input::new("Prompt").read_with_io(&mut reader, &mut writer);

    // Then: WriteError should be returned
    assert!(result.is_err());
    match result.unwrap_err() {
        InputError::WriteError(_) => {} // Expected
        other => panic!("Expected WriteError, got: {other}"),
    }
}

#[test]
fn test_builder_no_write_error_when_prompt_hidden() {
    // Given: A failing writer but prompt is hidden
    let mut reader = MockReader::new("test\n");
    let mut writer = FailingWriter;

    // When: Reading with show_prompt=false
    let result = Input::new("Prompt")
        .show_prompt(false)
        .read_with_io(&mut reader, &mut writer);

    // Then: Should succeed because no write is attempted
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test");
}

// ==========================================================
// Japanese input tests
// ==========================================================

#[test]
fn test_builder_japanese_input() {
    // Given: Japanese input via builder
    let mut reader = MockReader::new("こんにちは\n");
    let mut writer = MockWriter::new();

    // When: Reading Japanese input
    let result = Input::new("名前").read_with_io(&mut reader, &mut writer);

    // Then: Japanese characters should be preserved
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "こんにちは");
    assert!(writer.output().contains("名前:"));
}

#[test]
fn test_builder_japanese_default() {
    // Given: Japanese default value with empty input
    let mut reader = MockReader::new("\n");
    let mut writer = MockWriter::new();

    // When: Reading with Japanese default
    let result = Input::new("都市")
        .default("東京")
        .read_with_io(&mut reader, &mut writer);

    // Then: Japanese default should be returned
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "東京");
    assert!(writer.output().contains("[東京]"));
}

// ==========================================================
// Method chaining order tests
// ==========================================================

#[test]
fn test_builder_chaining_all_options() {
    // Given: Builder with all options chained
    let mut reader = MockReader::new("\n");
    let mut writer = MockWriter::new();

    // When: Using all builder options
    let result = Input::new("Config")
        .default("default_val")
        .trim(true)
        .show_prompt(true)
        .read_with_io(&mut reader, &mut writer);

    // Then: All options should be applied correctly
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "default_val");
    assert!(writer.output().contains("Config [default_val]:"));
}

#[test]
fn test_builder_chaining_reverse_order() {
    // Given: Builder with options in reverse order
    let mut reader = MockReader::new("\n");
    let mut writer = MockWriter::new();

    // When: Chaining in different order
    let result = Input::new("Config")
        .show_prompt(true)
        .trim(true)
        .default("reverse_val")
        .read_with_io(&mut reader, &mut writer);

    // Then: Options should still work regardless of order
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "reverse_val");
    assert!(writer.output().contains("[reverse_val]"));
}
