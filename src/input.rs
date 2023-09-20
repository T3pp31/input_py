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
