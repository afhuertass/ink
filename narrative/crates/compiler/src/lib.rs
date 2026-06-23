pub mod codegen;
pub mod resolve;
pub mod runtime_types;
pub mod json_output;
pub mod error;
pub mod directives_manifest;
pub mod schema_output;

pub use runtime_types::*;
pub use json_output::{story_to_json, serialize_story};
pub use codegen::compile;
pub use directives_manifest::generate_manifest;
pub use schema_output::generate_schema;

use ink_parser::ir::story::ParsedStory;

/// Compile ink source to JSON (standard ink JSON, directives stripped).
pub fn compile_ink(source: &str, filename: &str) -> Result<String, Vec<String>> {
    let parsed = ink_parser::parse_story(source, filename);
    if parsed.has_errors() {
        let errors: Vec<String> = parsed.errors.iter().map(|e| e.to_string()).collect();
        return Err(errors);
    }

    let story = codegen::compile(&parsed);
    let json = json_output::story_to_json(&story);
    Ok(json)
}

/// Full compilation with definitions: produces all three outputs.
pub fn compile_full(
    ink_source: &str,
    ink_filename: &str,
    definitions_yaml: &str,
    definitions_filename: &str,
) -> Result<CompilationOutput, Vec<String>> {
    // Parse ink
    let parsed = ink_parser::parse_story(ink_source, ink_filename);
    if parsed.has_errors() {
        let errors: Vec<String> = parsed.errors.iter().map(|e| e.to_string()).collect();
        return Err(errors);
    }

    // Parse and validate definitions
    let defs = def_parser::parse_definitions(definitions_yaml, definitions_filename)
        .map_err(|errors: Vec<def_parser::error::DefinitionError>| -> Vec<String> { errors.iter().map(|e| e.to_string()).collect() })?;

    // Validate directives against definitions
    let validation_errors = validate_directives(&parsed, &defs, ink_filename);
    if !validation_errors.is_empty() {
        return Err(validation_errors);
    }

    // Generate ink JSON (directives stripped)
    let runtime_story = codegen::compile(&parsed);
    let ink_json = json_output::story_to_json(&runtime_story);

    // Generate directives manifest
    let manifest = directives_manifest::generate_manifest(&parsed, ink_filename);

    // Generate definitions schema
    let schema = schema_output::generate_schema(&defs);

    Ok(CompilationOutput {
        ink_json,
        directives_manifest: serde_json::to_string_pretty(&manifest).unwrap(),
        definitions_schema: serde_json::to_string_pretty(&schema).unwrap(),
    })
}

/// Validate @ directives in the parsed story against definitions.
fn validate_directives(
    story: &ParsedStory,
    defs: &def_parser::types::Definitions,
    filename: &str,
) -> Vec<String> {
    let mut errors = Vec::new();
    validate_directives_in_nodes(&story.content, defs, filename, &mut errors);
    errors
}

fn validate_directives_in_nodes(
    nodes: &[ink_parser::ir::story::StoryNode],
    defs: &def_parser::types::Definitions,
    filename: &str,
    errors: &mut Vec<String>,
) {
    for node in nodes {
        match node {
            ink_parser::ir::story::StoryNode::Directive(dir) => {
                validate_directive(dir, defs, filename, errors);
            }
            ink_parser::ir::story::StoryNode::Knot(k) => {
                validate_directives_in_nodes(&k.content, defs, filename, errors);
            }
            ink_parser::ir::story::StoryNode::Logic(l) => {
                validate_directives_in_nodes(&l.content, defs, filename, errors);
            }
            ink_parser::ir::story::StoryNode::Conditional(c) => {
                for branch in &c.branches {
                    validate_directives_in_nodes(&branch.content, defs, filename, errors);
                }
            }
            _ => {}
        }
    }
}

fn validate_directive(
    dir: &ink_parser::ir::directive::Directive,
    defs: &def_parser::types::Definitions,
    filename: &str,
    errors: &mut Vec<String>,
) {
    use ink_parser::ir::directive::DirectiveType;

    match dir.directive_type {
        DirectiveType::Action => {
            if !defs.actions.contains_key(&dir.name) {
                errors.push(format!("Undefined action '{}' at {}:{}", dir.name, filename, dir.location.line));
            } else if let Some(action) = defs.actions.get(&dir.name) {
                // Check required params
                let provided_names: Vec<&str> = dir.args.iter().map(|a| a.value.as_str()).collect();
                for param in &action.params {
                    if param.required && !provided_names.contains(&param.name.as_str()) {
                        errors.push(format!(
                            "Missing required param '{}' on action '{}' at {}:{}",
                            param.name, dir.name, filename, dir.location.line
                        ));
                    }
                }
            }
        }
        DirectiveType::Scene => {
            if !defs.scenes.contains_key(&dir.name) {
                errors.push(format!("Undefined scene '{}' at {}:{}", dir.name, filename, dir.location.line));
            }
        }
        DirectiveType::Character => {
            if !defs.characters.contains_key(&dir.name) {
                errors.push(format!("Undefined character '{}' at {}:{}", dir.name, filename, dir.location.line));
            }
        }
        DirectiveType::State => {
            // State assignments: extract variable name from "var = ..."
            if let Some(eq_pos) = dir.name.find('=') {
                let var_name = dir.name[..eq_pos].trim();
                if !defs.state.contains_key(var_name) {
                    errors.push(format!("Undefined state variable '{}' at {}:{}", var_name, filename, dir.location.line));
                }
            }
        }
        DirectiveType::Event => {
            if !defs.events.contains_key(&dir.name) {
                errors.push(format!("Undefined event '{}' at {}:{}", dir.name, filename, dir.location.line));
            }
        }
        DirectiveType::Asset => {
            for arg in &dir.args {
                if !defs.assets.contains_key(&arg.value) {
                    errors.push(format!("Undefined asset '{}' at {}:{}", arg.value, filename, dir.location.line));
                }
            }
            if dir.args.is_empty() && !defs.assets.contains_key(&dir.name) {
                errors.push(format!("Undefined asset '{}' at {}:{}", dir.name, filename, dir.location.line));
            }
        }
    }
}

/// The three compiled output artifacts.
#[derive(Debug)]
pub struct CompilationOutput {
    pub ink_json: String,
    pub directives_manifest: String,
    pub definitions_schema: String,
}