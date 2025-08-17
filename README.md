# input_py

[![Crates.io](https://img.shields.io/crates/v/input_py.svg)](https://crates.io/crates/input_py)
[![Documentation](https://docs.rs/input_py/badge.svg)](https://docs.rs/input_py)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**input_py** is a simple Rust library that provides Python-like input functionality for reading user input from the terminal.

## Features

- 🐍 **Python-like syntax**: Familiar `input()` function similar to Python
- 🛡️ **Robust error handling**: Proper error types instead of panics
- 🎯 **Default values**: Support for default values when no input is provided
- ✂️ **Configurable trimming**: Control whitespace handling behavior
- 📝 **Rich documentation**: Comprehensive examples and documentation
- ✅ **Well tested**: Extensive test suite with 14+ tests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
input_py = "1.0.0"
```

## Quick Start

```rust
use input_py::input;

fn main() {
    match input("Enter your name") {
        Ok(name) => println!("Hello, {}!", name),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**Terminal output:**

```
Enter your name: Alice
Hello, Alice!
```

## API Reference

### `input(comment: &str)`

Basic input function that prompts the user and returns the trimmed input.

```rust
use input_py::input;

// Basic usage
let name = input("What's your name")?;
println!("Hello, {}!", name);

// Empty prompt
let data = input("")?;  // No prompt displayed
```

### `input_with_default(comment: &str, default: &str)`

Input with a default value that's used when the user enters nothing.

```rust
use input_py::input_with_default;

// With default value
let port = input_with_default("Enter port", "8080")?;
println!("Using port: {}", port);

// User sees: "Enter port [8080]:"
// Pressing Enter uses "8080"
```

### `input_trim(comment: &str, trim_whitespace: bool)`

Input with configurable whitespace trimming behavior.

```rust
use input_py::input_trim;

// Preserve whitespace
let raw_text = input_trim("Enter text", false)?;
println!("Raw: '{}'", raw_text);

// Trim whitespace (default behavior)
let clean_text = input_trim("Enter text", true)?;
println!("Clean: '{}'", clean_text);
```

## Error Handling

All functions return `Result<String, InputError>` for robust error handling:

```rust
use input_py::{input, InputError};

match input("Enter something") {
    Ok(value) => {
        if value.is_empty() {
            println!("Nothing entered!");
        } else {
            println!("You entered: {}", value);
        }
    }
    Err(InputError::FlushError(e)) => {
        eprintln!("Failed to flush stdout: {}", e);
    }
    Err(InputError::ReadError(e)) => {
        eprintln!("Failed to read from stdin: {}", e);
    }
}
```

## Complete Example

```rust
use input_py::{input, input_with_default, input_trim};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== User Registration ===");

    // Required field
    let name = input("Full name")?;
    if name.is_empty() {
        eprintln!("Name is required!");
        return Ok(());
    }

    // Optional field with default
    let age = input_with_default("Age", "18")?;

    // Preserve formatting for addresses
    let address = input_trim("Address (with spacing)", false)?;

    println!("\n--- Registration Complete ---");
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Address: '{}'", address);

    Ok(())
}
```

## Testing

Run the test suite:

```bash
cargo test
```

The library includes comprehensive tests covering:

- ✅ Normal input scenarios
- ❌ Error conditions
- 🔧 Internal logic verification
- 🎯 Edge cases and special characters

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### v1.0.0 🎉
- 🚀 **Stable release** - Ready for production use
- ✨ Added `input_with_default()` function for default value support
- ✨ Added `input_trim()` function for configurable whitespace handling
- 🛡️ Improved error handling with custom `InputError` type (breaking change)
- 📚 Enhanced documentation with comprehensive examples and API reference
- ✅ Added extensive test suite (14+ tests) with full coverage
- 🔧 Fixed clippy warnings for better code quality
- 📖 Complete README with examples, comparison table, and usage guide

### v0.2.1
- Basic input functionality similar to Python's input()
