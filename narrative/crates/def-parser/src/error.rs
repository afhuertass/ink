use std::fmt;

/// A definition validation error.
#[derive(Debug, Clone)]
pub enum DefinitionError {
    YamlError {
        message: String,
        filename: String,
    },
    UndefinedReference {
        kind: String,
        name: String,
        referenced_by: String,
        filename: String,
    },
    TypeMismatch {
        expected: String,
        actual: String,
        name: String,
        referenced_by: String,
        filename: String,
    },
    MissingRequiredField {
        definition_kind: String,
        definition_name: String,
        field: String,
        filename: String,
    },
    DuplicateName {
        kind: String,
        name: String,
        filename: String,
    },
    InvalidSchemaVersion {
        version: i64,
        filename: String,
    },
}

impl DefinitionError {
    pub fn is_error(&self) -> bool {
        true
    }
}

impl fmt::Display for DefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DefinitionError::YamlError { message, filename } => {
                write!(f, "YAML error in {}: {}", filename, message)
            }
            DefinitionError::UndefinedReference { kind, name, referenced_by, filename } => {
                write!(f, "ERROR in {}: Undefined {} '{}' referenced by {}", filename, kind, name, referenced_by)
            }
            DefinitionError::TypeMismatch { expected, actual, name, referenced_by, filename } => {
                write!(f, "ERROR in {}: Type mismatch for '{}': expected {} but got {}, referenced by {}", filename, name, expected, actual, referenced_by)
            }
            DefinitionError::MissingRequiredField { definition_kind, definition_name, field, filename } => {
                write!(f, "ERROR in {}: {} '{}' missing required field '{}'", filename, definition_kind, definition_name, field)
            }
            DefinitionError::DuplicateName { kind, name, filename } => {
                write!(f, "ERROR in {}: Duplicate {} name '{}'", filename, kind, name)
            }
            DefinitionError::InvalidSchemaVersion { version, filename } => {
                write!(f, "ERROR in {}: Invalid schema version {} (expected 1)", filename, version)
            }
        }
    }
}