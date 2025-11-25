use std::io::{self, Write};

/// Errors that can occur during input operations
#[derive(Debug)]
pub enum InputError {
    /// Failed to flush stdout
    FlushError(io::Error),
    /// Failed to read from stdin
    ReadError(io::Error),
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::FlushError(e) => write!(f, "Failed to flush stdout: {e}"),
            InputError::ReadError(e) => write!(f, "Failed to read from stdin: {e}"),
        }
    }
}

impl std::error::Error for InputError {}

/// Internal helper function to read input with various options
///
/// # Arguments
/// * `prompt` - The prompt text to display
/// * `default_value` - Optional default value to return if input is empty
/// * `trim_whitespace` - Whether to trim leading/trailing whitespace
/// * `show_prompt` - Whether to display the prompt
fn read_input_internal(
    prompt: &str,
    default_value: Option<&str>,
    trim_whitespace: bool,
    show_prompt: bool,
) -> Result<String, InputError> {
    // Display prompt if needed
    if show_prompt && !prompt.is_empty() {
        if let Some(default) = default_value {
            if !default.is_empty() {
                print!("{prompt} [{default}]:");
            } else {
                print!("{prompt}:");
            }
        } else {
            print!("{prompt}:");
        }
        io::stdout().flush().map_err(InputError::FlushError)?;
    }

    // Read input from stdin
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .map_err(InputError::ReadError)?;

    // Process the input based on options
    if trim_whitespace {
        let trimmed = buf.trim();
        if trimmed.is_empty() {
            if let Some(default) = default_value {
                return Ok(default.to_string());
            }
        }
        Ok(trimmed.to_string())
    } else {
        // Remove only the trailing newline characters
        if buf.ends_with('\n') {
            buf.pop();
            if buf.ends_with('\r') {
                buf.pop();
            }
        }
        Ok(buf)
    }
}

/// Reads a line of input from stdin with a prompt, similar to Python's input() function.
///
/// # Arguments
/// * `comment` - The prompt text to display before the colon. If empty, no prompt is shown.
///
/// # Returns
/// * `Ok(String)` - The input string with leading/trailing whitespace removed
/// * `Err(InputError)` - An error if stdout flush or stdin read fails
///
/// # Examples
/// ```
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
    read_input_internal(comment, None, true, !comment.is_empty())
}

/// Reads a line of input with a default value if nothing is entered.
///
/// # Arguments
/// * `comment` - The prompt text to display
/// * `default` - The default value to return if the user enters nothing
///
/// # Returns
/// * `Ok(String)` - The input string or default value
/// * `Err(InputError)` - An error if stdout flush or stdin read fails
///
/// # Examples
/// ```
/// use input_py::input_with_default;
///
/// match input_with_default("Enter port", "8080") {
///     Ok(port) => println!("Using port: {}", port),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn input_with_default(comment: &str, default: &str) -> Result<String, InputError> {
    read_input_internal(comment, Some(default), true, true)
}

/// Reads a line of input with configurable trimming behavior.
///
/// # Arguments
/// * `comment` - The prompt text to display
/// * `trim_whitespace` - Whether to trim leading/trailing whitespace
///
/// # Returns
/// * `Ok(String)` - The input string (trimmed or not based on setting)
/// * `Err(InputError)` - An error if stdout flush or stdin read fails
///
/// # Examples
/// ```
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
    read_input_internal(comment, None, trim_whitespace, !comment.is_empty())
}
