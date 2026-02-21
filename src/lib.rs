use std::io::{self, BufRead, Write};

pub mod config;

/// Errors that can occur during input operations
#[derive(Debug)]
pub enum InputError {
    /// Failed to write to stdout
    WriteError(io::Error),
    /// Failed to flush stdout
    FlushError(io::Error),
    /// Failed to read from stdin
    ReadError(io::Error),
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::WriteError(e) => {
                write!(f, "{}: {e}", config::errors::WRITE_ERROR_PREFIX)
            }
            InputError::FlushError(e) => {
                write!(f, "{}: {e}", config::errors::FLUSH_ERROR_PREFIX)
            }
            InputError::ReadError(e) => {
                write!(f, "{}: {e}", config::errors::READ_ERROR_PREFIX)
            }
        }
    }
}

impl std::error::Error for InputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InputError::WriteError(e) => Some(e),
            InputError::FlushError(e) => Some(e),
            InputError::ReadError(e) => Some(e),
        }
    }
}

/// Trait for abstracting input operations (enables testing)
pub trait InputReader {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize>;
}

/// Trait for abstracting output operations (enables testing)
pub trait OutputWriter {
    fn write_str(&mut self, s: &str) -> io::Result<()>;
    fn flush(&mut self) -> io::Result<()>;
}

/// Standard stdin implementation
pub struct StdinReader;

impl InputReader for StdinReader {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        io::stdin().read_line(buf)
    }
}

/// Standard stdout implementation
pub struct StdoutWriter;

impl OutputWriter for StdoutWriter {
    fn write_str(&mut self, s: &str) -> io::Result<()> {
        write!(io::stdout(), "{}", s)
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

/// Generic reader from BufRead (for testing)
pub struct BufReaderInput<R: BufRead> {
    reader: R,
}

impl<R: BufRead> BufReaderInput<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BufRead> InputReader for BufReaderInput<R> {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        self.reader.read_line(buf)
    }
}

/// Generic writer to Write (for testing)
pub struct GenericWriter<W: Write> {
    writer: W,
}

impl<W: Write> GenericWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W: Write> OutputWriter for GenericWriter<W> {
    fn write_str(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_all(s.as_bytes())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// Internal helper function to read input with various options
/// This version accepts generic reader/writer for testing
///
/// # Arguments
/// * `prompt` - The prompt text to display
/// * `default_value` - Optional default value to return if input is empty
/// * `trim_whitespace` - Whether to trim leading/trailing whitespace
/// * `show_prompt` - Whether to display the prompt
/// * `reader` - Input reader implementation
/// * `writer` - Output writer implementation
pub fn read_input_with_io<R: InputReader, W: OutputWriter>(
    prompt: &str,
    default_value: Option<&str>,
    trim_whitespace: bool,
    show_prompt: bool,
    reader: &mut R,
    writer: &mut W,
) -> Result<String, InputError> {
    // Display prompt if needed
    if show_prompt && !prompt.is_empty() {
        let prompt_text = if let Some(default) = default_value {
            if !default.is_empty() {
                format!("{prompt} [{default}]{}", config::format::PROMPT_SUFFIX)
            } else {
                format!("{prompt}{}", config::format::PROMPT_SUFFIX)
            }
        } else {
            format!("{prompt}{}", config::format::PROMPT_SUFFIX)
        };
        writer
            .write_str(&prompt_text)
            .map_err(InputError::WriteError)?;
        writer.flush().map_err(InputError::FlushError)?;
    }

    // Read input from reader
    let mut buf = String::new();
    reader.read_line(&mut buf).map_err(InputError::ReadError)?;

    // Process the input based on options
    Ok(process_input(buf, default_value, trim_whitespace))
}

/// Process input string based on options
/// This is a pure function that can be tested independently
pub fn process_input(
    mut input: String,
    default_value: Option<&str>,
    trim_whitespace: bool,
) -> String {
    if trim_whitespace {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            if let Some(default) = default_value {
                return default.to_string();
            }
        }
        trimmed.to_string()
    } else {
        // Remove only the trailing newline characters
        if input.ends_with('\n') {
            input.pop();
            if input.ends_with('\r') {
                input.pop();
            }
        }
        // Apply default value when input is empty even with trim disabled
        if input.is_empty() {
            if let Some(default) = default_value {
                return default.to_string();
            }
        }
        input
    }
}

/// Builder for configuring and reading user input
///
/// Provides a fluent API for combining options like default values,
/// trimming behavior, and prompt visibility.
///
/// # Examples
/// ```no_run
/// use input_py::Input;
///
/// // Basic usage
/// let name = Input::new("Enter your name").read().unwrap();
///
/// // With default value and no trimming
/// let port = Input::new("Enter port")
///     .default("8080")
///     .trim(false)
///     .read()
///     .unwrap();
/// ```
pub struct Input<'a> {
    prompt: &'a str,
    default_value: Option<&'a str>,
    trim_whitespace: bool,
    show_prompt: bool,
}

impl<'a> Input<'a> {
    /// Create a new Input builder with the given prompt text.
    /// If the prompt is empty, the prompt will not be displayed.
    pub fn new(prompt: &'a str) -> Self {
        Self {
            prompt,
            default_value: None,
            trim_whitespace: true,
            show_prompt: !prompt.is_empty(),
        }
    }

    /// Set a default value to return when the user enters nothing.
    pub fn default(mut self, value: &'a str) -> Self {
        self.default_value = Some(value);
        self
    }

    /// Control whether leading/trailing whitespace is trimmed.
    /// Defaults to `true`.
    pub fn trim(mut self, trim: bool) -> Self {
        self.trim_whitespace = trim;
        self
    }

    /// Control whether the prompt is displayed.
    /// Defaults to `true` when the prompt is non-empty.
    pub fn show_prompt(mut self, show: bool) -> Self {
        self.show_prompt = show;
        self
    }

    /// Read input from standard stdin/stdout.
    pub fn read(self) -> Result<String, InputError> {
        read_input_internal(
            self.prompt,
            self.default_value,
            self.trim_whitespace,
            self.show_prompt,
        )
    }

    /// Read input using custom reader/writer implementations.
    /// Useful for testing without actual I/O.
    pub fn read_with_io<R: InputReader, W: OutputWriter>(
        self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<String, InputError> {
        read_input_with_io(
            self.prompt,
            self.default_value,
            self.trim_whitespace,
            self.show_prompt,
            reader,
            writer,
        )
    }
}

/// Internal helper function to read input with various options
/// Uses standard stdin/stdout
fn read_input_internal(
    prompt: &str,
    default_value: Option<&str>,
    trim_whitespace: bool,
    show_prompt: bool,
) -> Result<String, InputError> {
    let mut reader = StdinReader;
    let mut writer = StdoutWriter;
    read_input_with_io(
        prompt,
        default_value,
        trim_whitespace,
        show_prompt,
        &mut reader,
        &mut writer,
    )
}

/// Reads a line of input from stdin with a prompt, similar to Python's input() function.
///
/// # Arguments
/// * `comment` - The prompt text to display before the colon. If empty, no prompt is shown.
///
/// # Returns
/// * `Ok(String)` - The input string with leading/trailing whitespace removed
/// * `Err(InputError)` - An error if stdout write/flush or stdin read fails
///
/// # Examples
/// ```no_run
/// use input_py::input;
///
/// // Basic usage with prompt
/// match input("Enter your name") {
///     Ok(name) => println!("Hello, {}!", name),
///     Err(e) => eprintln!("Error: {}", e),
/// }
///
/// // Empty prompt
/// match input("") {
///     Ok(data) => println!("You entered: {}", data),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn input(comment: &str) -> Result<String, InputError> {
    Input::new(comment).read()
}

/// Reads a line of input with a default value if nothing is entered.
///
/// # Arguments
/// * `comment` - The prompt text to display
/// * `default` - The default value to return if the user enters nothing
///
/// # Returns
/// * `Ok(String)` - The input string or default value
/// * `Err(InputError)` - An error if stdout write/flush or stdin read fails
///
/// # Examples
/// ```no_run
/// use input_py::input_with_default;
///
/// match input_with_default("Enter port", "8080") {
///     Ok(port) => println!("Using port: {}", port),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn input_with_default(comment: &str, default: &str) -> Result<String, InputError> {
    Input::new(comment).default(default).read()
}

/// Reads a line of input with configurable trimming behavior.
///
/// # Arguments
/// * `comment` - The prompt text to display
/// * `trim_whitespace` - Whether to trim leading/trailing whitespace
///
/// # Returns
/// * `Ok(String)` - The input string (trimmed or not based on setting)
/// * `Err(InputError)` - An error if stdout write/flush or stdin read fails
///
/// # Examples
/// ```no_run
/// use input_py::input_trim;
///
/// // Preserve whitespace
/// match input_trim("Enter text", false) {
///     Ok(text) => println!("Raw input: '{}'", text),
///     Err(e) => eprintln!("Error: {}", e),
/// }
///
/// // Trim whitespace (default behavior)
/// match input_trim("Enter text", true) {
///     Ok(text) => println!("Trimmed input: '{}'", text),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn input_trim(comment: &str, trim_whitespace: bool) -> Result<String, InputError> {
    Input::new(comment).trim(trim_whitespace).read()
}
