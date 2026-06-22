use ink_parser::ir::*;

#[test]
fn test_identifier_creation() {
    let id = Identifier::new("test_knot", SourceLocation::new("test.ink", 1, 1));
    assert_eq!(id.name, "test_knot");
    assert_eq!(id.location.line, 1);
}

#[test]
fn test_source_location_display() {
    let loc = SourceLocation::new("story.ink", 42, 10);
    assert_eq!(format!("{}", loc), "story.ink:42:10");
}

#[test]
fn test_ink_error_display() {
    let err = InkError::error("something broke", SourceLocation::new("test.ink", 5, 3));
    assert_eq!(format!("{}", err), "ERROR at test.ink:5:3: something broke");
}

#[test]
fn test_ink_error_warning() {
    let warn = InkError::warning("just a heads up", SourceLocation::new("test.ink", 1, 1));
    assert!(!warn.is_error());
}
