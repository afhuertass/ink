pub mod codegen;
pub mod resolve;
pub mod runtime_types;
pub mod json_output;
pub mod error;

pub use runtime_types::*;
pub use json_output::{story_to_json, serialize_story};
pub use codegen::compile;

/// Compile ink source to JSON.
pub fn compile_ink(source: &str, filename: &str) -> Result<String, Vec<String>> {
    // Parse
    let parsed = ink_parser::parse_story(source, filename);
    if parsed.has_errors() {
        let errors: Vec<String> = parsed.errors.iter().map(|e| e.to_string()).collect();
        return Err(errors);
    }

    // Compile to runtime
    let story = codegen::compile(&parsed);

    // Serialize to JSON
    let json = json_output::story_to_json(&story);
    Ok(json)
}

/// Compile ink source with definitions validation.
/// Returns the ink JSON, or errors from parsing/compiling/validation.
pub fn compile_ink_with_definitions(
    ink_source: &str,
    ink_filename: &str,
    definitions_yaml: &str,
    definitions_filename: &str,
) -> Result<String, Vec<String>> {
    // Parse definitions
    let _defs = def_parser::parse_definitions(definitions_yaml, definitions_filename)
        .map_err(|errors: Vec<def_parser::error::DefinitionError>| -> Vec<String> { errors.iter().map(|e| e.to_string()).collect() })?;

    // Compile ink (directives will be validated against definitions in Phase 3)
    compile_ink(ink_source, ink_filename)
}