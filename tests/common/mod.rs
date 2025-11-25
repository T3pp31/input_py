//! Common test utilities and mock implementations
//!
//! This module provides shared mock implementations for testing
//! the input_py library without requiring actual I/O operations.

#![allow(dead_code)]

use input_py::{InputReader, OutputWriter};
use std::io;

/// Mock reader that returns predefined input
pub struct MockReader {
    input: String,
    read_count: usize,
}

impl MockReader {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            read_count: 0,
        }
    }
}

impl InputReader for MockReader {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        if self.read_count > 0 {
            return Ok(0); // EOF after first read
        }
        self.read_count += 1;
        buf.push_str(&self.input);
        Ok(self.input.len())
    }
}

/// Mock reader that always fails
pub struct FailingReader;

impl InputReader for FailingReader {
    fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::UnexpectedEof, "test error"))
    }
}

/// Mock writer that captures output
pub struct MockWriter {
    buffer: Vec<u8>,
    pub flush_count: usize,
}

impl MockWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            flush_count: 0,
        }
    }

    pub fn output(&self) -> String {
        String::from_utf8_lossy(&self.buffer).to_string()
    }
}

impl Default for MockWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputWriter for MockWriter {
    fn write_str(&mut self, s: &str) -> io::Result<()> {
        self.buffer.extend_from_slice(s.as_bytes());
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush_count += 1;
        Ok(())
    }
}

/// Mock writer that fails on write/flush
pub struct FailingWriter;

impl OutputWriter for FailingWriter {
    fn write_str(&mut self, _s: &str) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::BrokenPipe, "write failed"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::BrokenPipe, "flush failed"))
    }
}
