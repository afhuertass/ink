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

fn parse_statement_at_level(p: &mut Parser, level: StatementLevel) -> Option<StoryNode> {
    // Knot definition (only at top level)
    if level >= StatementLevel::Top && crate::knot::at_knot_declaration(p) {
        return crate::knot::parse_knot_definition(p);
    }

    // Stitch definition (at knot level and above)
    if level >= StatementLevel::Knot {
        let rule_id = p.begin_rule();
        if let Some(node) = crate::knot::parse_stitch_definition(p) {
            p.succeed_rule(rule_id);
            return Some(node);
        }
        p.fail_rule(rule_id);
    }

    // Choice (* or + bullets)
    if p.peek() == Some('*') || p.peek() == Some('+') {
        let rule_id = p.begin_rule();
        if let Some(node) = crate::choices::parse_choice_full(p) {
            p.succeed_rule(rule_id);
            return Some(node);
        }
        p.fail_rule(rule_id);
    }

    // Divert (-> target) — standalone divert line
    if p.peek_str("->") {
        let rule_id = p.begin_rule();
        if let Some(node) = crate::divert::parse_multi_divert(p) {
            p.succeed_rule(rule_id);
            p.parse_newline();
            return Some(node);
        }
        p.fail_rule(rule_id);
    }

    // Gather (dashes, not in inner blocks)
    if level > StatementLevel::InnerBlock {
        let rule_id = p.begin_rule();
        if let Some(node) = crate::choices::parse_gather(p) {
            p.succeed_rule(rule_id);
            return Some(node);
        }
        p.fail_rule(rule_id);
    }

    // VAR global declaration
    {
        let rule_id = p.begin_rule();
        if let Some(node) = crate::logic::parse_var_declaration(p) {
            p.succeed_rule(rule_id);
            return Some(node);
        }
        p.fail_rule(rule_id);
    }

    // CONST declaration
    {
        let rule_id = p.begin_rule();
        if let Some(node) = crate::logic::parse_const_declaration(p) {
            p.succeed_rule(rule_id);
            return Some(node);
        }
        p.fail_rule(rule_id);
    }

    // EXTERNAL declaration
    {
        let rule_id = p.begin_rule();
        if let Some(node) = crate::logic::parse_external_declaration(p) {
            p.succeed_rule(rule_id);
            return Some(node);
        }
        p.fail_rule(rule_id);
    }

    // LIST declaration
    {
        let rule_id = p.begin_rule();
        if let Some(node) = crate::lists::parse_list_declaration(p) {
            p.succeed_rule(rule_id);
            return Some(node);
        }
        p.fail_rule(rule_id);
    }

    // INCLUDE statement
    {
        let rule_id = p.begin_rule();
        if let Some(filename) = crate::include::parse_include(p) {
            p.succeed_rule(rule_id);
            return Some(StoryNode::Include(filename));
        }
        p.fail_rule(rule_id);
    }

    // Logic line (~ ...)
    if p.peek() == Some('~') {
        let rule_id = p.begin_rule();
        if let Some(node) = crate::logic::parse_logic_line(p) {
            p.succeed_rule(rule_id);
            return Some(node);
        }
        p.fail_rule(rule_id);
    }

    // Sequence ({! | {~ | {& | { }) or Conditional ({ ... })
    if p.peek() == Some('{') {
        let remaining = p.remaining();
        if remaining.starts_with("{!") || remaining.starts_with("{~") || remaining.starts_with("{&") {
            // Likely a sequence
            let rule_id = p.begin_rule();
            if let Some(node) = crate::sequences::parse_sequence(p) {
                p.succeed_rule(rule_id);
                return Some(node);
            }
            p.fail_rule(rule_id);
        } else if remaining.starts_with("{") {
            // Could be conditional or sequence (plain { for stopping type)
            // Try sequence first since it's clearer
            let rule_id = p.begin_rule();
            if let Some(node) = crate::sequences::parse_sequence(p) {
                p.succeed_rule(rule_id);
                return Some(node);
            }
            p.fail_rule(rule_id);

            // Fall back to conditional
            let rule_id = p.begin_rule();
            if let Some(node) = crate::conditionals::parse_conditional(p) {
                p.succeed_rule(rule_id);
                p.parse_newline();
                return Some(node);
            }
            p.fail_rule(rule_id);
        }
    }

    // Author warning / TODO
    if let Some(msg) = whitespace::parse_author_warning(p) {
        p.parse_newline();
        return Some(StoryNode::AuthorWarning(msg));
    }

    // Tag (# tag_name)
    if p.peek() == Some('#') {
        let rule_id = p.begin_rule();
        if let Some(tag) = crate::tags::parse_tag(p) {
            p.succeed_rule(rule_id);
            p.parse_newline();
            return Some(StoryNode::Tag(tag));
        }
        p.fail_rule(rule_id);
    }

    // Directive (@ type: name)
    if p.peek() == Some('@') {
        let rule_id = p.begin_rule();
        if let Some(directive) = crate::directives::parse_directive(p) {
            p.succeed_rule(rule_id);
            p.parse_newline();
            return Some(StoryNode::Directive(directive));
        }
        p.fail_rule(rule_id);
    }

    // Line of mixed text and logic
    crate::content::parse_text_line(p)
}

fn should_break_for_level(p: &mut Parser, level: StatementLevel) -> bool {
    if p.is_end() {
        return true;
    }
    if level <= StatementLevel::Knot {
        if p.remaining().starts_with("===") {
            return true;
        }
    }
    if level <= StatementLevel::Stitch {
        if p.peek() == Some('=') && !p.remaining().starts_with("==") {
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
