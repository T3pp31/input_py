//! Configuration constants for the input_py library and demo application.
//! All hardcoded values should be defined here to maintain code maintainability.

/// Demo application configuration
pub mod demo {
    /// Demo title displayed at startup
    pub const TITLE: &str = "=== input_py Demo ===";

    /// Default port value for the demo
    pub const DEFAULT_PORT: &str = "8080";

    /// Prompt messages for each demo section
    pub mod prompts {
        pub const NAME: &str = "Enter your name";
        pub const PORT: &str = "Enter port";
        pub const TEXT_PRESERVED: &str = "Enter text (whitespace preserved)";
        pub const TEXT_TRIMMED: &str = "Enter text (whitespace trimmed)";
        pub const EMPTY_PROMPT: &str = "";
    }

    /// Output messages
    pub mod messages {
        pub const NO_NAME_ENTERED: &str = "No name entered!";
        pub const DEMO_COMPLETED: &str = "Demo completed successfully!";
    }
}

/// Format strings used in prompts
pub mod format {
    /// Prompt suffix without default value: ":"
    pub const PROMPT_SUFFIX: &str = ":";
}

/// Error message prefixes
pub mod errors {
    pub const WRITE_ERROR_PREFIX: &str = "Failed to write to stdout";
    pub const FLUSH_ERROR_PREFIX: &str = "Failed to flush stdout";
    pub const READ_ERROR_PREFIX: &str = "Failed to read from stdin";
}
