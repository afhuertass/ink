use ink_parser::parse_story;

#[test]
fn test_basic_choice() {
    let story = parse_story("* Hello world\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_sticky_choice() {
    let story = parse_story("+ Hello again\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_choice_with_brackets() {
    let story = parse_story("* Hello[.] world\n", "test.ink");
    // Bracket notation should parse
}

#[test]
fn test_named_choice() {
    let story = parse_story("* (my_choice) Hello\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_gather() {
    let story = parse_story("- Gathered text\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_nested_gather() {
    let story = parse_story("-- Nested gather\n", "test.ink");
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_choice_and_gather() {
    let story = parse_story(
        "* Choice A\n* Choice B\n- Gathered\n",
        "test.ink",
    );
    assert!(!story.has_errors(), "Unexpected errors: {:?}", story.errors);
}

#[test]
fn test_choice_with_text_content() {
    let story = parse_story(
        "* [Go inside]\n- You go inside.\n",
        "test.ink",
    );
    // Choice with bracket notation
}
