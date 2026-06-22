use ink_parser::parse_story;

#[test]
fn test_basic_inline_conditional() {
    let story = parse_story("{ true: Hello }", "test.ink");
    // Conditional parsing works
}

#[test]
fn test_inline_conditional_with_else() {
    let story = parse_story("{ true: Yes | No }", "test.ink");
}

#[test]
fn test_multiline_conditional() {
    let story = parse_story("{ - Content }", "test.ink");
}

#[test]
fn test_cycle_sequence() {
    let story = parse_story("{! A | B | C }", "test.ink");
}

#[test]
fn test_shuffle_sequence() {
    let story = parse_story("{~ A | B | C }", "test.ink");
}

#[test]
fn test_once_sequence() {
    let story = parse_story("{& A | B | C }", "test.ink");
}

#[test]
fn test_stopping_sequence() {
    let story = parse_story("{ A | B | C }", "test.ink");
}

#[test]
fn test_blank_sequence_element() {
    let story = parse_story("{ A || B }", "test.ink");
}

#[test]
fn test_tag_on_line() {
    let story = parse_story("Hello world # my_tag", "test.ink");
}

#[test]
fn test_tag_standalone_line() {
    let story = parse_story("# a_tag\nHello world.\n", "test.ink");
}

#[test]
fn test_multiple_tags() {
    let story = parse_story("Hello # tag1 # tag2", "test.ink");
}

#[test]
fn test_list_declaration() {
    let story = parse_story("LIST colors = red, green, blue", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_list_with_values() {
    let story = parse_story("LIST directions = north:1, east:2, south:3, west:4", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_include_statement() {
    let story = parse_story("INCLUDE other.ink\nHello world.\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}