use input_py::config::demo::{messages, prompts, DEFAULT_PORT, TITLE};
use input_py::{input, input_trim, input_with_default};

fn main() {
    println!("{}\n", TITLE);

    // Basic input example with error handling
    println!("1. Basic input example:");
    match input(prompts::NAME) {
        Ok(name) => {
            if name.is_empty() {
                println!("{}", messages::NO_NAME_ENTERED);
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
    match input_with_default(prompts::PORT, DEFAULT_PORT) {
        Ok(port) => println!("Using port: {port}"),
        Err(e) => {
            eprintln!("Error reading port: {e}");
            return;
        }
    }

    // Input with whitespace preservation
    println!("\n3. Input with preserved whitespace:");
    match input_trim(prompts::TEXT_PRESERVED, false) {
        Ok(text) => println!("Raw input: '{text}'"),
        Err(e) => {
            eprintln!("Error reading text: {e}");
            return;
        }
    }

    // Input with trimming (default behavior)
    println!("\n4. Input with trimming:");
    match input_trim(prompts::TEXT_TRIMMED, true) {
        Ok(text) => println!("Trimmed input: '{text}'"),
        Err(e) => {
            eprintln!("Error reading text: {e}");
            return;
        }
    }

    // Empty prompt example
    println!("\n5. Empty prompt example:");
    match input(prompts::EMPTY_PROMPT) {
        Ok(data) => println!("You entered: '{data}'"),
        Err(e) => {
            eprintln!("Error reading input: {e}");
            return;
        }
    }

    println!("\n{}", messages::DEMO_COMPLETED);
}
