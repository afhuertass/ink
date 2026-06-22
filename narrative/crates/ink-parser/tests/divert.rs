use ink_parser::parse_story;

#[test]
fn test_basic_divert() {
    let story = parse_story("-> target\n=== target ===\nHello.\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_divert_to_done() {
    let story = parse_story("=== knot ===\nHello.\n-> DONE\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_divert_to_end() {
    let story = parse_story("=== knot ===\nHello.\n-> END\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_divert_with_dot_path() {
    let story = parse_story("-> knot.stitch\n=== knot ===\n= stitch\nHello.\n-> DONE\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_tunnel() {
    let story = parse_story(
        "-> tunnel ->\nAfter tunnel.\n=== tunnel ===\nInside tunnel.\n->->\n",
        "test.ink",
    );
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}
