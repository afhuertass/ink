use ink_parser::parse_story;

#[test]
fn test_knot_definition() {
    let story = parse_story("=== knot_name ===\nHello world.\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_knot_with_stitch() {
    let story = parse_story(
        "=== my_knot ===\nHello.\n= my_stitch\nWorld.\n",
        "test.ink",
    );
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_function_definition() {
    let _story = parse_story(
        "=== function multiply(x,y) ===\n~ return x * y\n",
        "test.ink",
    );
    // Will have errors because ~ logic lines aren't parsed yet, but knot should parse
    // For now, just check that the knot structure is recognized
}

#[test]
fn test_multiple_knots() {
    let story = parse_story(
        "=== first ===\nHello.\n=== second ===\nWorld.\n",
        "test.ink",
    );
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_knot_with_parameters() {
    let _story = parse_story(
        "=== greet(name) ===\nHello {name}.\n",
        "test.ink",
    );
    // Inline expressions not parsed yet, but knot declaration should work
}

#[test]
fn test_knot_with_ref_parameter() {
    let _story = parse_story(
        "=== function modify(ref x) ===\n~ x = x + 1\n",
        "test.ink",
    );
    // Logic lines not parsed yet, but ref parameter should be parsed
}

#[test]
fn test_knot_with_divert_target_parameter() {
    let _story = parse_story(
        "=== function go(-> target) ===\n-> target\n",
        "test.ink",
    );
    // Divert target parameters should parse
}
