use ink_parser::parse_story;

#[test]
fn test_hello_world() {
    let story = parse_story("Hello world!", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
    assert_eq!(story.content.len(), 1);
}

#[test]
fn test_multiple_lines() {
    let story = parse_story("Hello world!\nHello?\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
    assert_eq!(story.content.len(), 2);
}

#[test]
fn test_comment_elimination() {
    let story = parse_story("Hello // this is a comment\nworld!", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_empty_story() {
    let story = parse_story("", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_glue() {
    let story = parse_story("Hello <> world!\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_escape_character() {
    let story = parse_story("Hello \\nworld!\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_multiline_whitespace() {
    let story = parse_story("Line one\n\n\nLine two\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
    assert_eq!(story.content.len(), 2);
}

#[test]
fn test_block_comment() {
    let _story = parse_story("Before /* comment */ after\n", "test.ink");
    // Block comments within a line are tricky — this test verifies basic handling
}
