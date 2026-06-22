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