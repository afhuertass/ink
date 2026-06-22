use crate::parser::Parser;

/// Parse a line comment (// ... until newline)
pub fn parse_line_comment(p: &mut Parser) -> Option<()> {
    if p.parse_string("//").is_none() {
        return None;
    }
    p.parse_until_newline();
    Some(())
}

/// Parse a block comment (/* ... */)
pub fn parse_block_comment(p: &mut Parser) -> Option<()> {
    if p.parse_string("/*").is_none() {
        return None;
    }
    loop {
        if p.is_end() {
            p.error("Unclosed block comment");
            break;
        }
        if p.parse_string("*/").is_some() {
            break;
        }
        p.advance(1);
    }
    Some(())
}

/// Parse a TODO/author warning
pub fn parse_author_warning(p: &mut Parser) -> Option<String> {
    p.parse_whitespace();
    if p.parse_string("TODO:").is_none() {
        return None;
    }
    p.parse_whitespace();
    let text = p.parse_until_newline();
    Some(text)
}

/// Parse multiline whitespace (spaces, tabs, newlines, blank lines)
pub fn parse_multiline_whitespace(p: &mut Parser) -> bool {
    let mut found = false;
    loop {
        let had_whitespace = p.parse_whitespace();
        let had_newline = p.parse_newline();
        // Skip comments as whitespace
        let had_comment = parse_line_comment(p).is_some() || parse_block_comment(p).is_some();
        if !had_whitespace && !had_newline && !had_comment {
            break;
        }
        found = true;
        // After comment, consume the rest of the line
        if had_comment {
            p.parse_until_newline();
        }
    }
    found
}
