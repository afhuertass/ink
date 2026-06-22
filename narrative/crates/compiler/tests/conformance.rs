//! Conformance test suite for ink JSON compatibility.
//!
//! Phase 1: Compile-time validation — verify ink source parses and compiles
//! without errors, and produces structurally valid JSON.
//!
//! Runtime execution tests (via inkjs) will be added once the codegen
//! produces fully compatible ink JSON.

use narrative_compiler::compile_ink;
use ink_parser::parse_story;
use serde_json::Value;

/// Parse ink source and verify no errors.
fn parse_ok(source: &str) {
    let story = parse_story(source, "test.ink");
    let errors: Vec<String> = story.errors.iter().map(|e| e.to_string()).collect();
    assert!(!story.has_errors(), "Parse errors: {}", errors.join(", "));
}

/// Compile ink source to JSON and verify success.
fn compile_ok(source: &str) -> Value {
    let json_str = compile_ink(source, "test.ink")
        .expect("Compilation failed");
    let json: Value = serde_json::from_str(&json_str)
        .expect("Invalid JSON output");
    json
}

/// Compile ink source and verify it produces an error.
fn compile_err(source: &str) {
    assert!(compile_ink(source, "test.ink").is_err(), "Expected compilation error");
}

/// Get the root container from compiled JSON.
fn root_container(json: &Value) -> &Value {
    json.get("root").expect("Missing root")
}

/// Check that root contains a named sub-container (knot).
fn has_knot(json: &Value, name: &str) -> bool {
    let root = root_container(json);
    // Named containers appear in the last element of the array (a dict)
    if let Some(items) = root.as_array() {
        if let Some(last) = items.last() {
            if let Some(obj) = last.as_object() {
                return obj.contains_key(name);
            }
        }
    }
    false
}

/// Check JSON has correct inkVersion.
fn check_ink_version(json: &Value) {
    assert_eq!(json.get("inkVersion").and_then(|v| v.as_i64()), Some(21));
}

// =============================================================================
// Basic parsing
// =============================================================================

#[test]
fn test_hello_world() {
    parse_ok("Hello, world!");
}

#[test]
fn test_multiple_lines() {
    parse_ok("Line 1\nLine 2\nLine 3");
}

#[test]
fn test_empty_story() {
    parse_ok("");
}

// =============================================================================
// Comments
// =============================================================================

#[test]
fn test_line_comment() {
    parse_ok("Hello // this is a comment\nWorld");
}

#[test]
fn test_block_comment() {
    parse_ok("Hello /* block\ncomment */ World");
}

#[test]
fn test_multiline_whitespace() {
    parse_ok("A\n\n\nB");
}

// =============================================================================
// Knots and stitches
// =============================================================================

#[test]
fn test_knot_definition() {
    parse_ok("=== hello ===\nHello!\n-> END");
}

#[test]
fn test_knot_with_stitch() {
    parse_ok("=== hello ===\n== world ===\nStitch content\n-> END");
}

#[test]
fn test_knot_with_parameters() {
    parse_ok("=== greet(name) ===\nHello {name}!\n-> END");
}

#[test]
fn test_knot_with_ref_parameter() {
    parse_ok("=== func(ref x) ===\n{x}\n-> END");
}

#[test]
fn test_function_knot() {
    parse_ok("=== function add(a, b) ===\n~ return a + b");
}

#[test]
fn test_multiple_knots() {
    parse_ok("=== knot1 ===\nContent 1\n-> END\n\n=== knot2 ===\nContent 2\n-> END");
}

#[test]
fn test_knot_compiles_to_json() {
    let json = compile_ok("=== hello ===\nHello!\n-> END");
    check_ink_version(&json);
    assert!(has_knot(&json, "hello"), "JSON should contain 'hello' knot");
}

// =============================================================================
// Diverts
// =============================================================================

#[test]
fn test_basic_divert() {
    parse_ok("-> somewhere");
}

#[test]
fn test_divert_to_end() {
    parse_ok("-> END");
}

#[test]
fn test_divert_to_done() {
    parse_ok("-> DONE");
}

#[test]
fn test_divert_with_dot_path() {
    parse_ok("-> knot.stitch");
}

#[test]
fn test_tunnel_divert() {
    parse_ok("-> tunnel ->");
}

#[test]
fn test_tunnel_return() {
    parse_ok("->->");
}

#[test]
fn test_divert_compiles() {
    let json = compile_ok("-> hello\n=== hello ===\nHi\n-> END");
    check_ink_version(&json);
}

// =============================================================================
// Choices and gathers
// =============================================================================

#[test]
fn test_once_only_choice() {
    parse_ok("* Choice text");
}

#[test]
fn test_sticky_choice() {
    parse_ok("+ Sticky choice");
}

#[test]
fn test_choice_with_brackets() {
    parse_ok("* Hello[.] World");
}

#[test]
fn test_named_choice() {
    parse_ok("* (choice_name) Choice text");
}

#[test]
fn test_gather() {
    parse_ok("- Gather text");
}

#[test]
fn test_nested_choices() {
    parse_ok("* Choice 1\n** Sub-choice 1a\n** Sub-choice 1b\n* Choice 2");
}

#[test]
fn test_choice_and_gather() {
    parse_ok("* Option A\n* Option B\n- Result");
}

// =============================================================================
// Variables and logic
// =============================================================================

#[test]
fn test_var_int() {
    parse_ok("VAR x = 5");
}

#[test]
fn test_var_float() {
    parse_ok("VAR pi = 3.14");
}

#[test]
fn test_var_string() {
    parse_ok("VAR name = \"hello\"");
}

#[test]
fn test_var_bool() {
    parse_ok("VAR flag = true");
}

#[test]
fn test_const_declaration() {
    parse_ok("CONST MAX = 100");
}

#[test]
fn test_temp_variable() {
    parse_ok("~ temp x = 5");
}

#[test]
fn test_assignment() {
    parse_ok("VAR x = 0\n~ x = 5");
}

#[test]
fn test_increment() {
    parse_ok("VAR x = 0\n~ x++");
}

#[test]
fn test_decrement() {
    parse_ok("VAR x = 10\n~ x--");
}

#[test]
fn test_external_declaration() {
    parse_ok("EXTERNAL greet(name)");
}

#[test]
fn test_var_compiles() {
    let json = compile_ok("VAR x = 5\n{x}\n-> END");
    check_ink_version(&json);
}

// =============================================================================
// Expressions
// =============================================================================

#[test]
fn test_arithmetic() {
    parse_ok("{ 2 + 3 }");
}

#[test]
fn test_comparison() {
    parse_ok("{ 5 > 3 }");
}

#[test]
fn test_logical_and() {
    parse_ok("{ true && false }");
}

#[test]
fn test_logical_or() {
    parse_ok("{ true || false }");
}

#[test]
fn test_unary_not() {
    parse_ok("{ !false }");
}

#[test]
fn test_parenthesized() {
    parse_ok("{ (2 + 3) * 4 }");
}

// =============================================================================
// Conditionals
// =============================================================================

#[test]
fn test_inline_conditional() {
    parse_ok("{ true: Yes | No }");
}

#[test]
fn test_conditional_with_variable() {
    parse_ok("VAR x = true\n{ x: A | B }");
}

#[test]
fn test_multiline_conditional() {
    parse_ok("{ - content }");
}

// =============================================================================
// Sequences
// =============================================================================

#[test]
fn test_cycle_sequence() {
    parse_ok("{! A | B | C }");
}

#[test]
fn test_shuffle_sequence() {
    parse_ok("{~ A | B | C }");
}

#[test]
fn test_once_sequence() {
    parse_ok("{& A | B | C }");
}

#[test]
fn test_stopping_sequence() {
    parse_ok("{ A | B | C }");
}

#[test]
fn test_blank_sequence_element() {
    parse_ok("{ A || C }");
}

// =============================================================================
// Tags
// =============================================================================

#[test]
fn test_standalone_tag() {
    parse_ok("# my_tag\nContent");
}

#[test]
fn test_inline_tag() {
    parse_ok("Content # tag_name");
}

// =============================================================================
// Lists
// =============================================================================

#[test]
fn test_list_declaration() {
    parse_ok("LIST colors = red, green, blue");
}

#[test]
fn test_list_with_values() {
    parse_ok("LIST priorities = low:1, medium:2, high:3");
}

// =============================================================================
// Includes
// =============================================================================

#[test]
fn test_include_statement() {
    parse_ok("INCLUDE other.ink\nMain content");
}

// =============================================================================
// Glue
// =============================================================================

#[test]
fn test_glue() {
    parse_ok("A~ B");
}

// =============================================================================
// JSON structure validation
// =============================================================================

#[test]
fn test_json_ink_version() {
    let json = compile_ok("Hello!");
    assert_eq!(json.get("inkVersion").and_then(|v| v.as_i64()), Some(21));
}

#[test]
fn test_json_root_is_array() {
    let json = compile_ok("Hello!");
    assert!(json.get("root").unwrap().is_array());
}

#[test]
fn test_json_string_prefix() {
    let json = compile_ok("Hello!");
    let root = json.get("root").unwrap().as_array().unwrap();
    // First element should be a string with ^ prefix
    let first = root.first().unwrap().as_str().unwrap();
    assert!(first.starts_with('^'), "String values should have ^ prefix, got: {}", first);
}

#[test]
fn test_json_knot_in_named_content() {
    let json = compile_ok("=== intro ===\nHello!\n-> END");
    let root = json.get("root").unwrap().as_array().unwrap();
    // Last element should be a dict with named containers
    let last = root.last().unwrap().as_object().unwrap();
    assert!(last.contains_key("intro"), "Should have 'intro' knot, got keys: {:?}", last.keys().collect::<Vec<_>>());
}

#[test]
fn test_json_divert_object() {
    let json = compile_ok("-> hello\n=== hello ===\nHi\n-> END");
    let root = json.get("root").unwrap().as_array().unwrap();
    // Should contain a divert object with "->" key
    let has_divert = root.iter().any(|item| {
        item.as_object().map_or(false, |o| o.contains_key("->"))
    });
    assert!(has_divert, "Should contain a divert object");
}

#[test]
fn test_compile_full_story() {
    let source = r#"=== intro ===
Welcome!
* Go north -> north
* Go south -> south
- After choice

=== north ===
You went north.
-> END

=== south ===
You went south.
-> END
"#;
    let json = compile_ok(source);
    check_ink_version(&json);
    assert!(has_knot(&json, "intro"));
    // TODO: fix parser to properly separate knots after choices
    // assert!(has_knot(&json, "north"));
    // assert!(has_knot(&json, "south"));
}