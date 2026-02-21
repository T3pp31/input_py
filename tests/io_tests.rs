//! Tests for I/O operations with mocked reader/writer
//!
//! These tests verify the read_input_with_io function and I/O abstractions.

mod common;

use common::{FailingReader, FailingWriter, MockReader, MockWriter};
use input_py::{read_input_with_io, BufReaderInput, GenericWriter, InputError, InputReader};
use std::io::Cursor;

// ==========================================================
// read_input_with_io tests
// ==========================================================

#[test]
fn test_basic_input_with_prompt() {
    // Given: A mock reader with input and writer
    let mut reader = MockReader::new("test_input\n");
    let mut writer = MockWriter::new();

    // When: Reading input with prompt
    let result = read_input_with_io("Enter name", None, true, true, &mut reader, &mut writer);

    // Then: Input should be returned and prompt should be written
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test_input");
    assert!(writer.output().contains("Enter name:"));
    assert_eq!(writer.flush_count, 1);
}

#[test]
fn test_input_with_default_value_prompt() {
    // Given: Mock I/O with default value
    let mut reader = MockReader::new("custom\n");
    let mut writer = MockWriter::new();

    // When: Reading with default value
    let result = read_input_with_io("Port", Some("8080"), true, true, &mut reader, &mut writer);

    // Then: Prompt should include default value
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "custom");
    assert!(writer.output().contains("Port [8080]:"));
}

#[test]
fn test_empty_input_returns_default() {
    // Given: Empty input with default value
    let mut reader = MockReader::new("\n");
    let mut writer = MockWriter::new();

    // When: Reading empty input
    let result = read_input_with_io("Port", Some("8080"), true, true, &mut reader, &mut writer);

    // Then: Default value should be returned
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "8080");
}

#[test]
fn test_no_prompt_when_show_prompt_false() {
    // Given: show_prompt set to false
    let mut reader = MockReader::new("test\n");
    let mut writer = MockWriter::new();

    // When: Reading with show_prompt=false
    let result = read_input_with_io("Prompt", None, true, false, &mut reader, &mut writer);

    // Then: No prompt should be written
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test");
    assert!(writer.output().is_empty());
    assert_eq!(writer.flush_count, 0);
}

#[test]
fn test_empty_prompt_no_output() {
    // Given: Empty prompt string
    let mut reader = MockReader::new("data\n");
    let mut writer = MockWriter::new();

    // When: Reading with empty prompt
    let result = read_input_with_io("", None, true, true, &mut reader, &mut writer);

    // Then: No prompt should be written
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "data");
    assert!(writer.output().is_empty());
}

#[test]
fn test_empty_default_not_shown_in_prompt() {
    // Given: Empty default value
    let mut reader = MockReader::new("input\n");
    let mut writer = MockWriter::new();

    // When: Reading with empty default
    let result = read_input_with_io("Name", Some(""), true, true, &mut reader, &mut writer);

    // Then: Prompt should not show empty brackets
    assert!(result.is_ok());
    let output = writer.output();
    assert!(output.contains("Name:"));
    assert!(!output.contains("[]"));
}

#[test]
fn test_read_error_handling() {
    // Given: A failing reader
    let mut reader = FailingReader;
    let mut writer = MockWriter::new();

    // When: Attempting to read
    let result = read_input_with_io("Prompt", None, true, true, &mut reader, &mut writer);

    // Then: ReadError should be returned
    assert!(result.is_err());
    match result.unwrap_err() {
        InputError::ReadError(_) => {} // Expected
        _ => panic!("Expected ReadError"),
    }
}

#[test]
fn test_write_error_handling() {
    // Given: A failing writer
    let mut reader = MockReader::new("test\n");
    let mut writer = FailingWriter;

    // When: Attempting to write prompt
    let result = read_input_with_io("Prompt", None, true, true, &mut reader, &mut writer);

    // Then: WriteError should be returned
    assert!(result.is_err());
    match result.unwrap_err() {
        InputError::WriteError(_) => {} // Expected
        _ => panic!("Expected WriteError"),
    }
}

#[test]
fn test_trim_disabled_preserves_whitespace() {
    // Given: Input with whitespace
    let mut reader = MockReader::new("  spaced  \n");
    let mut writer = MockWriter::new();

    // When: Reading with trim disabled
    let result = read_input_with_io("Text", None, false, true, &mut reader, &mut writer);

    // Then: Whitespace should be preserved (except newline)
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "  spaced  ");
}

// ==========================================================
// BufReaderInput and GenericWriter tests
// ==========================================================

#[test]
fn test_buf_reader_input() {
    // Given: A cursor with test data
    let data = "test line\n";
    let cursor = Cursor::new(data);
    let mut reader = BufReaderInput::new(cursor);

    // When: Reading a line
    let mut buf = String::new();
    let result = reader.read_line(&mut buf);

    // Then: Line should be read successfully
    assert!(result.is_ok());
    assert_eq!(buf, "test line\n");
}

#[test]
fn test_generic_writer() {
    // Given: A vec writer
    let vec: Vec<u8> = Vec::new();
    let mut writer = GenericWriter::new(vec);

    // When: Writing a string
    use input_py::OutputWriter;
    let result = writer.write_str("test output");

    // Then: String should be written
    assert!(result.is_ok());
    let inner = writer.into_inner();
    assert_eq!(String::from_utf8_lossy(&inner), "test output");
}

#[test]
fn test_generic_writer_flush() {
    // Given: A vec writer
    let vec: Vec<u8> = Vec::new();
    let mut writer = GenericWriter::new(vec);

    // When: Flushing
    use input_py::OutputWriter;
    let result = writer.flush();

    // Then: Flush should succeed
    assert!(result.is_ok());
}

#[test]
fn test_buf_reader_eof() {
    // Given: An empty cursor
    let cursor = Cursor::new("");
    let mut reader = BufReaderInput::new(cursor);

    // When: Reading from empty input
    let mut buf = String::new();
    let result = reader.read_line(&mut buf);

    // Then: Should return 0 bytes (EOF)
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
    assert!(buf.is_empty());
}
