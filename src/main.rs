use input_py::{input, input_with_default, input_trim};

fn main() {
    println!("=== input_py Demo ===\n");

    // Basic input example with error handling
    println!("1. Basic input example:");
    match input("Enter your name") {
        Ok(name) => {
            if name.is_empty() {
                println!("No name entered!");
            } else {
                println!("Hello, {name}!");
            }
        }
        Err(e) => {
            eprintln!("Error reading input: {e}");
            return;
        }
    }

    // Input with default value
    println!("\n2. Input with default value:");
    match input_with_default("Enter port", "8080") {
        Ok(port) => println!("Using port: {port}"),
        Err(e) => {
            eprintln!("Error reading port: {e}");
            return;
        }
    }

    // Input with whitespace preservation
    println!("\n3. Input with preserved whitespace:");
    match input_trim("Enter text (whitespace preserved)", false) {
        Ok(text) => println!("Raw input: '{text}'"),
        Err(e) => {
            eprintln!("Error reading text: {e}");
            return;
        }
    }

    // Input with trimming (default behavior)
    println!("\n4. Input with trimming:");
    match input_trim("Enter text (whitespace trimmed)", true) {
        Ok(text) => println!("Trimmed input: '{text}'"),
        Err(e) => {
            eprintln!("Error reading text: {e}");
            return;
        }
    }

    // Empty prompt example
    println!("\n5. Empty prompt example:");
    match input("") {
        Ok(data) => println!("You entered: '{data}'"),
        Err(e) => {
            eprintln!("Error reading input: {e}");
            return;
        }
    }

    println!("\nDemo completed successfully!");
}
