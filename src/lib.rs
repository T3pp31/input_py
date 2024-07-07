use std::io::{self, Write};


/// # input_py
///
/// you can use input like python3
///
/// # Args:
/// * comment: &str - display text in terminal
/// if set comment like "test" display like
///
/// test:
///
/// # Returns:
/// Result<String, String>
///
/// # usage
/// ```
/// use input_py::input;
/// fn main() {
///     let comment = "test";
///     let input_data = input(&comment);
/// }
/// ```
pub fn input(comment: &str) -> Result<String, String> {
    let mut buf = String::new();
    print!("{}:", comment);
    io::stdout().flush().expect("Failed to flush stdout");

    if io::stdin().read_line(&mut buf).is_ok() {
        let trimmed = buf.trim().to_string();
        Ok(trimmed)
    } else {
        Err("Failed to read input".to_string())
    }
}
