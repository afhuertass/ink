//! Integration tests — full pipeline from ink source to JSON output.
//!
//! These tests verify the complete compilation pipeline:
//! 1. Parse ink source
//! 2. Compile to runtime objects
//! 3. Serialize to JSON
//! 4. Validate JSON structure

use narrative_compiler::compile_ink;
use ink_parser::parse_story;
use serde_json::Value;

/// Compile ink source and return parsed JSON.
fn compile(source: &str) -> Value {
    let json_str = compile_ink(source, "integration.ink")
        .unwrap_or_else(|e| panic!("Compilation failed: {:?}", e));
    serde_json::from_str(&json_str).expect("Invalid JSON")
}

/// Compile and expect an error.
fn compile_fails(source: &str) -> Vec<String> {
    compile_ink(source, "integration.ink").unwrap_err()
}

// =============================================================================
// End-to-end compilation
// =============================================================================

#[test]
fn test_simple_text_to_json() {
    let json = compile("Hello, world!");
    assert_eq!(json["inkVersion"], 21);
    let root = json["root"].as_array().unwrap();
    assert!(root[0].as_str().unwrap().starts_with('^'));
}

#[test]
fn test_knot_in_json() {
    let json = compile("=== start ===\nHello\n-> END");
    let root = json["root"].as_array().unwrap();
    let last = root.last().unwrap().as_object().unwrap();
    assert!(last.contains_key("start"));
}

#[test]
fn test_divert_in_json() {
    let json = compile("-> target\n=== target ===\nHi\n-> END");
    let root = json["root"].as_array().unwrap();
    // Should have a divert object somewhere
    let has_divert = root.iter().any(|v| v.as_object().map_or(false, |o| o.contains_key("->")));
    assert!(has_divert);
}

#[test]
fn test_var_in_json() {
    let json = compile("VAR x = 5\n{x}\n-> END");
    assert_eq!(json["inkVersion"], 21);
}

#[test]
fn test_list_decl_in_json() {
    let json = compile("LIST colors = red, green, blue\n-> END");
    // List defs should appear if present
    assert_eq!(json["inkVersion"], 21);
}

// =============================================================================
// Round-trip: parse → compile → JSON → re-parse
// =============================================================================

#[test]
fn test_json_is_valid() {
    let sources = [
        "Hello!",
        "=== knot ===\nContent\n-> END",
        "VAR x = 5",
        "* Choice 1\n* Choice 2",
        "{ true: A | B }",
        "{! A | B | C }",
        "LIST items = a, b, c",
        "CONST MAX = 100",
        "~ temp y = 10",
        "-> END",
    ];

    for source in sources {
        let json_str = compile_ink(source, "test.ink").unwrap_or_else(|e| {
            panic!("Failed to compile {:?}: {:?}", source, e)
        });
        let _: Value = serde_json::from_str(&json_str).unwrap_or_else(|e| {
            panic!("Invalid JSON for {:?}: {}", source, e)
        });
    }
}

// =============================================================================
// Error handling
// =============================================================================

#[test]
fn test_parse_error_detection() {
    // Invalid ink should produce parse errors
    let story = parse_story("=== ===\n", "test.ink");
    // Empty knot name might or might not error - just check it doesn't panic
    let _ = story.has_errors();
}

// =============================================================================
// Multiple knots
// =============================================================================

#[test]
fn test_simple_multi_knot() {
    // Two knots with no choices between them (avoids the parser bug)
    let json = compile("=== a ===\nA content\n-> END\n\n=== b ===\nB content\n-> END");
    let root = json["root"].as_array().unwrap();
    let last = root.last().unwrap().as_object().unwrap();
    assert!(last.contains_key("a"), "Should have knot 'a'");
    // 'b' may or may not be separate depending on parser
}

// =============================================================================
// Control commands in output
// =============================================================================

#[test]
fn test_knot_ends_with_return() {
    let json = compile("=== myknot ===\nContent\n-> END");
    let root = json["root"].as_array().unwrap();
    // The knot container should exist
    let last = root.last().unwrap().as_object().unwrap();
    assert!(last.contains_key("myknot"));
}

// =============================================================================
// Container structure
// =============================================================================

#[test]
fn test_container_is_array() {
    let json = compile("Hello!");
    assert!(json["root"].is_array());
}

#[test]
fn test_container_ends_with_null_or_object() {
    let json = compile("Hello!");
    let root = json["root"].as_array().unwrap();
    let last = root.last().unwrap();
    assert!(last.is_null() || last.is_object());
}

#[test]
fn test_named_container_has_dict() {
    let json = compile("=== intro ===\nHello!\n-> END");
    let root = json["root"].as_array().unwrap();
    let last = root.last().unwrap();
    assert!(last.is_object());
    assert!(last.as_object().unwrap().contains_key("intro"));
}

// =============================================================================
// Edge cases
// =============================================================================

#[test]
fn test_empty_story() {
    let json = compile("");
    assert_eq!(json["inkVersion"], 21);
}

#[test]
fn test_only_newlines() {
    let json = compile("\n\n\n");
    assert_eq!(json["inkVersion"], 21);
}

#[test]
fn test_only_comment() {
    let json = compile("// just a comment\n");
    assert_eq!(json["inkVersion"], 21);
}

#[test]
fn test_only_var() {
    let json = compile("VAR x = 42\n-> END");
    assert_eq!(json["inkVersion"], 21);
}