use crate::parser::Parser;
use crate::ir::tag::*;

/// Parse a tag (# tag_name)
/// Tags can appear:
/// - On their own line (above a content line)
/// - At the end of a content line
/// - At the start of a knot declaration
/// For now, we parse tags that appear in content lines
pub fn parse_tag(p: &mut Parser) -> Option<Tag> {
    let loc = p.current_source_location();

    if p.parse_string("#").is_none() {
        return None;
    }

    p.parse_whitespace();

    // Parse the rest of the line as the tag content
    p.parse_until_newline();

    Some(Tag {
        is_start: true,  // Tags in content are treated as start tags
        in_choice: false,
        location: loc,
    })
}

/// Parse all tags at the current position (can be multiple # tags)
pub fn parse_tags(p: &mut Parser) -> Option<Vec<Tag>> {
    let mut tags = Vec::new();
    let loc = p.current_source_location();

    loop {
        p.parse_whitespace();
        if p.parse_string("#").is_none() {
            break;
        }
        p.parse_until_newline();

        tags.push(Tag {
            is_start: true,
            in_choice: false,
            location: loc.clone(),
        });
    }

    if tags.is_empty() {
        None
    } else {
        Some(tags)
    }
}