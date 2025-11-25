//! Tests for InputError handling
//!
//! These tests verify error types, display formatting, and trait implementations.

use input_py::{config, InputError};
use std::io;

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

#[test]
fn test_input_error_debug() {
    // Given: An InputError
    let error = InputError::FlushError(io::Error::new(io::ErrorKind::BrokenPipe, "test"));

    // When: Formatting with Debug
    let debug_str = format!("{:?}", error);

    // Then: Should contain variant name
    assert!(debug_str.contains("FlushError"));
}

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

    // When/Then: All should be wrappable in InputError
    for kind in error_kinds {
        let flush_err = InputError::FlushError(io::Error::new(kind, "test"));
        let read_err = InputError::ReadError(io::Error::new(kind, "test"));
        assert!(!flush_err.to_string().is_empty());
        assert!(!read_err.to_string().is_empty());
    }
}
