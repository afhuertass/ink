//! Placeholder — errors defined in Task 2.
use std::fmt;

#[derive(Debug, Clone)]
pub enum DefinitionError {
    YamlError { message: String, filename: String },
}

impl DefinitionError {
    pub fn is_error(&self) -> bool { true }
}

impl fmt::Display for DefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DefinitionError::YamlError { message, filename } => {
                write!(f, "YAML error in {}: {}", filename, message)
            }
        }
    }
}