//! Test suite for input_py library
//!
//! Test categories:
//! - process_input: Pure function tests (no I/O)
//! - read_input_with_io: I/O abstracted tests with mocked reader/writer
//! - InputError: Error handling tests
//! - config: Configuration module tests

#[cfg(test)]
mod tests {
    use input_py::config;
    use input_py::{
        process_input, read_input_with_io, BufReaderInput, GenericWriter, InputError,
        InputReader, OutputWriter,
    };
    use std::io::{self, Cursor};

    // ==========================================================
    // Mock implementations for testing
    // ==========================================================

    /// Mock reader that returns predefined input
    struct MockReader {
        input: String,
        read_count: usize,
    }

    impl MockReader {
        fn new(input: &str) -> Self {
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
    struct FailingReader;

    impl InputReader for FailingReader {
        fn read_line(&mut self, _buf: &mut String) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "test error"))
        }
    }

    /// Mock writer that captures output
    struct MockWriter {
        buffer: Vec<u8>,
        flush_count: usize,
    }

    impl MockWriter {
        fn new() -> Self {
            Self {
                buffer: Vec::new(),
                flush_count: 0,
            }
        }

        fn output(&self) -> String {
            String::from_utf8_lossy(&self.buffer).to_string()
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

    /// Mock writer that fails on flush
    struct FailingWriter;

    impl OutputWriter for FailingWriter {
        fn write_str(&mut self, _s: &str) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "write failed"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "flush failed"))
        }
    }

    // ==========================================================
    // process_input tests (pure function, no I/O)
    // ==========================================================
    mod process_input_tests {
        use super::*;

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
    }

    // ==========================================================
    // read_input_with_io tests (with mocked I/O)
    // ==========================================================
    mod read_input_with_io_tests {
        use super::*;

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
            let result =
                read_input_with_io("Port", Some("8080"), true, true, &mut reader, &mut writer);

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
            let result =
                read_input_with_io("Port", Some("8080"), true, true, &mut reader, &mut writer);

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
            let result =
                read_input_with_io("Prompt", None, true, false, &mut reader, &mut writer);

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
            let result =
                read_input_with_io("Name", Some(""), true, true, &mut reader, &mut writer);

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
            let result =
                read_input_with_io("Prompt", None, true, true, &mut reader, &mut writer);

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
            let result =
                read_input_with_io("Prompt", None, true, true, &mut reader, &mut writer);

            // Then: FlushError should be returned
            assert!(result.is_err());
            match result.unwrap_err() {
                InputError::FlushError(_) => {} // Expected
                _ => panic!("Expected FlushError"),
            }
        }

        #[test]
        fn test_trim_disabled_preserves_whitespace() {
            // Given: Input with whitespace
            let mut reader = MockReader::new("  spaced  \n");
            let mut writer = MockWriter::new();

            // When: Reading with trim disabled
            let result =
                read_input_with_io("Text", None, false, true, &mut reader, &mut writer);

            // Then: Whitespace should be preserved (except newline)
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "  spaced  ");
        }
    }

    // ==========================================================
    // BufReaderInput and GenericWriter tests
    // ==========================================================
    mod io_abstraction_tests {
        use super::*;

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
    }

    // ==========================================================
    // InputError tests
    // ==========================================================
    mod error_tests {
        use super::*;

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
    }

    // ==========================================================
    // Config module tests
    // ==========================================================
    mod config_tests {
        use super::*;

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
    }

    // ==========================================================
    // Integration tests with full I/O simulation
    // ==========================================================
    mod integration_tests {
        use super::*;

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
    }
}
