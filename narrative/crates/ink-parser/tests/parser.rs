use ink_parser::parser::Parser;

#[test]
fn test_parser_parse_string_match() {
    let mut p = Parser::new("hello world", "test.ink");
    assert!(p.parse_string("hello").is_some());
    assert_eq!(p.position(), 5);
}

#[test]
fn test_parser_parse_string_no_match() {
    let mut p = Parser::new("hello world", "test.ink");
    assert!(p.parse_string("world").is_none());
    assert_eq!(p.position(), 0);
}

#[test]
fn test_parser_parse_int() {
    let mut p = Parser::new("42 rest", "test.ink");
    let val = p.parse_int();
    assert_eq!(val, Some(42));
    assert_eq!(p.position(), 2);
}

#[test]
fn test_parser_parse_negative_int() {
    let mut p = Parser::new("-10", "test.ink");
    assert_eq!(p.parse_int(), Some(-10));
}

#[test]
fn test_parser_parse_int_no_match() {
    let mut p = Parser::new("not a number", "test.ink");
    assert_eq!(p.parse_int(), None);
}

#[test]
fn test_parser_parse_newline() {
    let mut p = Parser::new("line1\nline2", "test.ink");
    assert!(p.parse_string("line1").is_some());
    assert!(p.parse_newline());
    assert!(p.parse_string("line2").is_some());
}

#[test]
fn test_parser_peek() {
    let p = Parser::new("abc", "test.ink");
    assert_eq!(p.peek(), Some('a'));
    assert_eq!(p.position(), 0);
}

#[test]
fn test_parser_end_of_input() {
    let mut p = Parser::new("ab", "test.ink");
    p.advance(2);
    assert!(p.is_end());
    assert_eq!(p.peek(), None);
}

#[test]
fn test_parser_begin_rule_succeed() {
    let mut p = Parser::new("hello world", "test.ink");
    let rule_id = p.begin_rule();
    assert!(p.parse_string("hello").is_some());
    p.succeed_rule(rule_id);
    assert_eq!(p.position(), 5);
}

#[test]
fn test_parser_begin_rule_rollback() {
    let mut p = Parser::new("hello world", "test.ink");
    let rule_id = p.begin_rule();
    assert!(p.parse_string("hello").is_some());
    p.fail_rule(rule_id);
    assert_eq!(p.position(), 0);
}

#[test]
fn test_parser_error_collection() {
    let mut p = Parser::new("test", "test.ink");
    p.error("something went wrong");
    assert_eq!(p.errors().len(), 1);
    assert_eq!(p.errors()[0].message, "something went wrong");
}

#[test]
fn test_parser_whitespace() {
    let mut p = Parser::new("  \t  hello", "test.ink");
    p.parse_whitespace();
    assert_eq!(p.position(), 5);
    assert!(p.parse_string("hello").is_some());
}

#[test]
fn test_parser_identifier() {
    let mut p = Parser::new("my_knot_name rest", "test.ink");
    let id = p.parse_identifier();
    assert_eq!(id, Some("my_knot_name".to_string()));
}

#[test]
fn test_parser_identifier_no_match() {
    let mut p = Parser::new("123", "test.ink");
    assert!(p.parse_identifier().is_none());
}

#[test]
fn test_parser_source_location_tracking() {
    let mut p = Parser::new("line1\nline2\nline3", "test.ink");
    assert_eq!(p.current_source_location().line, 1);
    p.parse_until_newline();
    p.parse_newline();
    assert_eq!(p.current_source_location().line, 2);
    p.parse_until_newline();
    p.parse_newline();
    assert_eq!(p.current_source_location().line, 3);
}

#[test]
fn test_parser_float() {
    let mut p = Parser::new("3.14 rest", "test.ink");
    assert_eq!(p.parse_float(), Some(3.14));
}

#[test]
fn test_parser_parse_until_newline() {
    let mut p = Parser::new("some text\nnext line", "test.ink");
    let text = p.parse_until_newline();
    assert_eq!(text, "some text");
    assert!(p.parse_newline());
}
