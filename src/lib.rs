/// # input_py
///
/// you can use input like python3
///
/// # usage
/// ```
/// use input_py::input;
/// fn main() {
///     let comment = "test";
///     let input_data = input(&comment);
/// }
/// ```

use std::io::{self, Write};
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
