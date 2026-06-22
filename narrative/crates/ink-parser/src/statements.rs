use crate::parser::Parser;
use crate::ir::story::*;
use crate::whitespace;

/// Statement level context
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatementLevel {
    InnerBlock,
    Stitch,
    Knot,
    Top,
}

/// Parse statements at a given level
pub fn parse_statements_at_level(p: &mut Parser, level: StatementLevel) -> Vec<StoryNode> {
    let mut content = Vec::new();

    loop {
        p.parse_whitespace();

        if p.is_end() {
            break;
        }

        let before_pos = p.position();

        // Skip blank lines and comments
        if whitespace::parse_multiline_whitespace(p) {
            continue;
        }

        if p.is_end() {
            break;
        }

        // Try to parse a statement at this level
        if let Some(node) = parse_statement_at_level(p, level) {
            content.push(node);
            continue;
        }

        // Check if we should break for a higher-level construct
        if should_break_for_level(p, level) {
            break;
        }

        // No progress — skip this line and report error
        if p.position() == before_pos {
            let bad_line = p.parse_until_newline();
            p.parse_newline();
            if !bad_line.trim().is_empty() {
                p.error(&format!("Unexpected content: {}", &bad_line[..bad_line.len().min(50)]));
            }
        }
    }

    content
}

fn parse_statement_at_level(p: &mut Parser, _level: StatementLevel) -> Option<StoryNode> {
    // Author warning / TODO
    if let Some(msg) = whitespace::parse_author_warning(p) {
        p.parse_newline();
        return Some(StoryNode::AuthorWarning(msg));
    }

    // Line of mixed text and logic
    crate::content::parse_text_line(p)
}

fn should_break_for_level(p: &mut Parser, level: StatementLevel) -> bool {
    if p.is_end() {
        return true;
    }
    // Check for constructs that break the current level
    if level <= StatementLevel::Knot {
        if p.peek_str("===") {
            return true;
        }
    }
    if level <= StatementLevel::Stitch {
        // Single = for stitch, but not == (which is knot)
        if p.peek() == Some('=') && !p.peek_str("==") {
            return true;
        }
    }
    if level <= StatementLevel::InnerBlock {
        if p.peek() == Some('}') {
            return true;
        }
    }
    false
}
