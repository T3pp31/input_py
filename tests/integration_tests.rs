//! Integration tests with full I/O simulation
//!
//! These tests verify end-to-end behavior using the I/O abstractions.

mod common;

use common::MockWriter;
use input_py::{read_input_with_io, BufReaderInput, GenericWriter};
use std::io::Cursor;

#[test]
fn test_full_input_flow_with_default() {
    // Given: Complete I/O simulation
    let cursor = Cursor::new("\n");
    let mut reader = BufReaderInput::new(cursor);
    let output: Vec<u8> = Vec::new();
    let mut writer = GenericWriter::new(output);

    // When: Simulating input with default
    let result = read_input_with_io(
        "Enter port",
        Some("8080"),
        true,
        true,
        &mut reader,
        &mut writer,
    );

    // Then: Default should be returned
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "8080");

    // And prompt should contain default
    let binding = writer.into_inner();
    let output = String::from_utf8_lossy(&binding);
    assert!(output.contains("[8080]"));
}

#[test]
fn test_full_input_flow_with_user_value() {
    // Given: User provides custom value
    let cursor = Cursor::new("3000\n");
    let mut reader = BufReaderInput::new(cursor);
    let output: Vec<u8> = Vec::new();
    let mut writer = GenericWriter::new(output);

    // When: Reading input
    let result = read_input_with_io(
        "Enter port",
        Some("8080"),
        true,
        true,
        &mut reader,
        &mut writer,
    );

    // Then: User value should override default
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "3000");
}

#[test]
fn test_sequential_reads() {
    // Given: Multiple lines of input
    let cursor = Cursor::new("line1\nline2\n");
    let mut reader = BufReaderInput::new(cursor);
    let mut writer = MockWriter::new();

    // When: Reading first line
    let result1 = read_input_with_io("First", None, true, false, &mut reader, &mut writer);

    // Then: First line should be returned
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), "line1");

    // When: Reading second line
    let result2 = read_input_with_io("Second", None, true, false, &mut reader, &mut writer);

    // Then: Second line should be returned
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), "line2");
}
