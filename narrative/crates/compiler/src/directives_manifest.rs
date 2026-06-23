//! Directives manifest generator.
//! Produces the directives JSON keyed by ink runtime paths.

use ink_parser::ir::story::ParsedStory;
use ink_parser::ir::directive::*;
use serde_json::{Map, Value};

/// Generate the directives manifest JSON from a ParsedStory.
/// Directives are keyed by their ink runtime path.
pub fn generate_manifest(story: &ParsedStory, source_filename: &str) -> Value {
    let mut directives_map = Map::new();
    let mut current_path = String::new();

    collect_directives(&story.content, &mut current_path, &mut directives_map);

    let mut root = Map::new();
    root.insert("version".to_string(), Value::Number(1.into()));
    root.insert("source".to_string(), Value::String(source_filename.to_string()));
    root.insert("directives".to_string(), Value::Object(directives_map));
    Value::Object(root)
}

/// Recursively collect directives from story nodes, tracking the current ink path.
fn collect_directives(
    nodes: &[ink_parser::ir::story::StoryNode],
    current_path: &mut String,
    directives_map: &mut Map<String, Value>,
) {
    for (i, node) in nodes.iter().enumerate() {
        match node {
            ink_parser::ir::story::StoryNode::Knot(knot) => {
                let prev_len = current_path.len();
                if !current_path.is_empty() {
                    current_path.push('.');
                }
                current_path.push_str(&knot.identifier.name.replace(' ', "_"));

                // Collect directives within this knot
                collect_directives_from_content(&knot.content, current_path, directives_map);

                current_path.truncate(prev_len);
            }
            ink_parser::ir::story::StoryNode::Directive(dir) => {
                let path = if current_path.is_empty() {
                    format!("{}", i)
                } else {
                    format!("{}.{}", current_path, i)
                };
                let directive_val = serialize_directive(dir);
                if let Some(existing) = directives_map.get_mut(&path) {
                    if let Some(arr) = existing.as_array_mut() {
                        arr.push(directive_val);
                    }
                } else {
                    directives_map.insert(path, Value::Array(vec![directive_val]));
                }
            }
            // Other node types may contain nested directives
            ink_parser::ir::story::StoryNode::Logic(logic) => {
                collect_directives(&logic.content, current_path, directives_map);
            }
            _ => {}
        }
    }
}

/// Collect directives from knot content (which uses StoryNode directly).
fn collect_directives_from_content(
    nodes: &[ink_parser::ir::story::StoryNode],
    current_path: &mut String,
    directives_map: &mut Map<String, Value>,
) {
    for (i, node) in nodes.iter().enumerate() {
        match node {
            ink_parser::ir::story::StoryNode::Knot(stitch) => {
                let prev_len = current_path.len();
                current_path.push('.');
                current_path.push_str(&stitch.identifier.name.replace(' ', "_"));
                collect_directives_from_content(&stitch.content, current_path, directives_map);
                current_path.truncate(prev_len);
            }
            ink_parser::ir::story::StoryNode::Directive(dir) => {
                let path = format!("{}.{}", current_path, i);
                let directive_val = serialize_directive(dir);
                if let Some(existing) = directives_map.get_mut(&path) {
                    if let Some(arr) = existing.as_array_mut() {
                        arr.push(directive_val);
                    }
                } else {
                    directives_map.insert(path, Value::Array(vec![directive_val]));
                }
            }
            ink_parser::ir::story::StoryNode::Logic(logic) => {
                collect_directives_from_content(&logic.content, current_path, directives_map);
            }
            ink_parser::ir::story::StoryNode::Conditional(cond) => {
                for (bi, branch) in cond.branches.iter().enumerate() {
                    collect_directives_from_content(&branch.content, current_path, directives_map);
                    let _ = bi;
                }
            }
            _ => {}
        }
    }
}

/// Serialize a single Directive to JSON.
fn serialize_directive(dir: &Directive) -> Value {
    let mut obj = Map::new();
    obj.insert("type".to_string(), Value::String(dir.directive_type.as_str().to_string()));
    obj.insert("name".to_string(), Value::String(dir.name.clone()));

    if !dir.args.is_empty() {
        let args_obj: Map<String, Value> = dir.args.iter().enumerate().map(|(i, arg)| {
            (format!("arg{}", i), Value::String(arg.value.clone()))
        }).collect();
        obj.insert("args".to_string(), Value::Object(args_obj));
    }

    if !dir.modifiers.is_empty() {
        let mods_obj: Map<String, Value> = dir.modifiers.iter().map(|m| {
            (m.key.clone(), Value::String(m.value.clone()))
        }).collect();
        obj.insert("modifiers".to_string(), Value::Object(mods_obj));
    }

    obj.insert(
        "source".to_string(),
        Value::Object({
            let mut src = Map::new();
            src.insert("file".to_string(), Value::String(dir.location.file.clone()));
            src.insert("line".to_string(), Value::Number(dir.location.line.into()));
            src.insert("column".to_string(), Value::Number(dir.location.column.into()));
            src
        }),
    );

    Value::Object(obj)
}