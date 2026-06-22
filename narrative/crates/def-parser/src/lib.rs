//! Definitions parser for `.inkdef.yaml` files.

pub mod types;
pub mod validate;
pub mod error;

use types::Definitions;
use error::DefinitionError;

/// Parse a `.inkdef.yaml` source string into a Definitions struct.
/// Returns the parsed definitions or validation errors.
pub fn parse_definitions(source: &str, filename: &str) -> Result<Definitions, Vec<DefinitionError>> {
    let defs: Definitions = serde_yaml::from_str(source)
        .map_err(|e| vec![DefinitionError::YamlError {
            message: e.to_string(),
            filename: filename.to_string(),
        }])?;

    let errors = validate::validate(&defs, filename);
    if errors.iter().any(|e| e.is_error()) {
        return Err(errors);
    }

    Ok(defs)
}

/// Parse YAML without running validation. Useful for testing or when
/// you want to collect all parse errors first.
pub fn parse_definitions_unvalidated(source: &str, filename: &str) -> Result<Definitions, DefinitionError> {
    serde_yaml::from_str(source).map_err(|e| DefinitionError::YamlError {
        message: e.to_string(),
        filename: filename.to_string(),
    })
}