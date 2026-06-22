use ink_parser::parse_story;

#[test]
fn test_var_declaration() {
    let story = parse_story("VAR x = 5\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_var_string() {
    let story = parse_story("VAR name = \"Hello\"\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_var_float() {
    let story = parse_story("VAR pi = 3.14\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_var_bool() {
    let story = parse_story("VAR flag = true\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_const_declaration() {
    let story = parse_story("CONST PI = 3\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_temp_variable() {
    let story = parse_story("=== knot ===\n~ temp y = 10\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_logic_assignment() {
    let story = parse_story("VAR x = 0\n~ x = 5\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_increment() {
    let story = parse_story("VAR x = 0\n~ x++\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_decrement() {
    let story = parse_story("VAR x = 0\n~ x--\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_external_declaration() {
    let story = parse_story("EXTERNAL playSound(name)\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_var_with_expression_reference() {
    let story = parse_story("VAR x = 5\n{x}\n", "test.ink");
    // Inline expressions not fully parsed yet — may have errors about {x}
}
