//! Tests for InputError handling
//!
//! These tests verify error types, display formatting, and trait implementations.

use input_py::{config, InputError};
use std::error::Error;
use std::io;

// ==========================================================
// Display formatting tests
// ==========================================================

#[test]
fn test_write_error_display() {
    // Given: A WriteError
    let error = InputError::WriteError(io::Error::new(io::ErrorKind::BrokenPipe, "pipe broke"));

    // When: Converting to string
    let display = error.to_string();

    // Then: Should contain expected prefix
    assert!(display.contains(config::errors::WRITE_ERROR_PREFIX));
    assert!(display.contains("pipe broke"));
}

#[test]
fn test_flush_error_display() {
    // Given: A FlushError
    let error = InputError::FlushError(io::Error::new(io::ErrorKind::BrokenPipe, "pipe broke"));

    // When: Converting to string
    let display = error.to_string();

    // Then: Should contain expected prefix
    assert!(display.contains(config::errors::FLUSH_ERROR_PREFIX));
    assert!(display.contains("pipe broke"));
}

#[test]
fn test_read_error_display() {
    // Given: A ReadError
    let error = InputError::ReadError(io::Error::new(io::ErrorKind::UnexpectedEof, "eof"));

    // When: Converting to string
    let display = error.to_string();

    // Then: Should contain expected prefix
    assert!(display.contains(config::errors::READ_ERROR_PREFIX));
    assert!(display.contains("eof"));
}

// ==========================================================
// Debug formatting tests
// ==========================================================

#[test]
fn test_write_error_debug() {
    // Given: A WriteError
    let error = InputError::WriteError(io::Error::new(io::ErrorKind::BrokenPipe, "test"));

    // When: Formatting with Debug
    let debug_str = format!("{:?}", error);

    // Then: Should contain variant name
    assert!(debug_str.contains("WriteError"));
}

#[test]
fn test_flush_error_debug() {
    // Given: A FlushError
    let error = InputError::FlushError(io::Error::new(io::ErrorKind::BrokenPipe, "test"));

    // When: Formatting with Debug
    let debug_str = format!("{:?}", error);

    // Then: Should contain variant name
    assert!(debug_str.contains("FlushError"));
}

#[test]
fn test_read_error_debug() {
    // Given: A ReadError
    let error = InputError::ReadError(io::Error::new(io::ErrorKind::UnexpectedEof, "test"));

    // When: Formatting with Debug
    let debug_str = format!("{:?}", error);

    // Then: Should contain variant name
    assert!(debug_str.contains("ReadError"));
}

// ==========================================================
// Trait implementation tests
// ==========================================================

#[test]
fn test_input_error_implements_error_trait() {
    // Given: An InputError
    let error = InputError::FlushError(io::Error::new(io::ErrorKind::BrokenPipe, "test"));

    // When/Then: Should be usable as dyn Error
    let _: &dyn std::error::Error = &error;
}

#[test]
fn test_input_error_send_sync() {
    // Given/When/Then: InputError should be Send + Sync
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<InputError>();
}

// ==========================================================
// source() method tests
// ==========================================================

#[test]
fn test_write_error_source() {
    // Given: A WriteError wrapping an io::Error
    let io_err = io::Error::new(io::ErrorKind::BrokenPipe, "write failed");
    let error = InputError::WriteError(io_err);

    // When: Getting the source
    let source = error.source();

    // Then: Source should be present and contain the original message
    assert!(source.is_some());
    assert!(source.unwrap().to_string().contains("write failed"));
}

#[test]
fn test_flush_error_source() {
    // Given: A FlushError wrapping an io::Error
    let io_err = io::Error::new(io::ErrorKind::BrokenPipe, "flush failed");
    let error = InputError::FlushError(io_err);

    // When: Getting the source
    let source = error.source();

    // Then: Source should be present and contain the original message
    assert!(source.is_some());
    assert!(source.unwrap().to_string().contains("flush failed"));
}

#[test]
fn test_read_error_source() {
    // Given: A ReadError wrapping an io::Error
    let io_err = io::Error::new(io::ErrorKind::UnexpectedEof, "read failed");
    let error = InputError::ReadError(io_err);

    // When: Getting the source
    let source = error.source();

    // Then: Source should be present and contain the original message
    assert!(source.is_some());
    assert!(source.unwrap().to_string().contains("read failed"));
}

// ==========================================================
// Various IO error kinds tests
// ==========================================================

#[test]
fn test_various_io_error_kinds() {
    // Given: Various IO error kinds
    let error_kinds = vec![
        io::ErrorKind::NotFound,
        io::ErrorKind::PermissionDenied,
        io::ErrorKind::ConnectionRefused,
        io::ErrorKind::ConnectionReset,
        io::ErrorKind::ConnectionAborted,
        io::ErrorKind::NotConnected,
        io::ErrorKind::BrokenPipe,
        io::ErrorKind::AlreadyExists,
        io::ErrorKind::InvalidInput,
        io::ErrorKind::InvalidData,
        io::ErrorKind::TimedOut,
        io::ErrorKind::Interrupted,
        io::ErrorKind::UnexpectedEof,
    ];

    // When/Then: All should be wrappable in all InputError variants
    for kind in error_kinds {
        let write_err = InputError::WriteError(io::Error::new(kind, "test"));
        let flush_err = InputError::FlushError(io::Error::new(kind, "test"));
        let read_err = InputError::ReadError(io::Error::new(kind, "test"));
        assert!(!write_err.to_string().is_empty());
        assert!(!flush_err.to_string().is_empty());
        assert!(!read_err.to_string().is_empty());
    }
}
