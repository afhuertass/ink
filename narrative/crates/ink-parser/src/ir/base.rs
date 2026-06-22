use std::fmt;

/// Source location for error reporting and debug metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    pub fn new(file: &str, line: usize, column: usize) -> Self {
        Self {
            file: file.to_string(),
            line,
            column,
        }
    }

    pub fn unknown() -> Self {
        Self {
            file: String::new(),
            line: 0,
            column: 0,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// An ink identifier (knot name, variable name, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub location: SourceLocation,
}

impl Identifier {
    pub fn new(name: &str, location: SourceLocation) -> Self {
        Self {
            name: name.to_string(),
            location,
        }
    }
}

/// Error severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    Error,
    Warning,
    AuthorMessage,
}

/// A parse or compilation error/warning.
#[derive(Debug, Clone)]
pub struct InkError {
    pub message: String,
    pub error_type: ErrorType,
    pub location: SourceLocation,
}

impl InkError {
    pub fn error(message: &str, location: SourceLocation) -> Self {
        Self {
            message: message.to_string(),
            error_type: ErrorType::Error,
            location,
        }
    }

    pub fn warning(message: &str, location: SourceLocation) -> Self {
        Self {
            message: message.to_string(),
            error_type: ErrorType::Warning,
            location,
        }
    }

    pub fn is_error(&self) -> bool {
        self.error_type == ErrorType::Error
    }
}

impl fmt::Display for InkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.error_type {
            ErrorType::Error => "ERROR",
            ErrorType::Warning => "WARNING",
            ErrorType::AuthorMessage => "TODO",
        };
        write!(f, "{} at {}: {}", prefix, self.location, self.message)
    }
}

impl std::error::Error for InkError {}
