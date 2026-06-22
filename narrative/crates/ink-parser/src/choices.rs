use crate::parser::Parser;
use crate::ir::base::*;
use crate::ir::choice::*;
use crate::ir::content::*;
use crate::ir::story::*;

/// Parse a choice: * or + bullets
pub fn parse_choice_full(p: &mut Parser) -> Option<StoryNode> {
    let loc = p.current_source_location();

    // Determine bullet type and count
    let (once_only, indentation_depth) = {
        let rule_id = p.begin_rule();
        let count = count_bullets(p, "*");
        if count > 0 {
            p.succeed_rule(rule_id);
            (true, count)
        } else {
            p.fail_rule(rule_id);
            let count = count_bullets(p, "+");
            if count > 0 {
                (false, count)
            } else {
                return None;
            }
        }
    };

    // Optional bracketed name (name)
    let identifier = parse_bracketed_name(p);

    p.parse_whitespace();

    // Optional newline after name
    if identifier.is_some() {
        p.parse_newline();
    }

    // Optional condition {expr}
    let condition = None; // TODO: parse choice condition when expression parsing is ready

    p.parse_whitespace();

    // Parse choice content
    let start_content = {
        let items = crate::content::parse_line_of_mixed_text_and_logic(p)?;
        Some(ContentList::from_items(items, loc.clone()))
    };

    let mut option_only_content = None;
    let mut inner_content = ContentList::empty(loc.clone());
    let mut has_brackets = false;

    // Check for weave-style brackets [...]
    if p.parse_string("[").is_some() {
        has_brackets = true;

        // Parse option-only content (what the player sees)
        let opt_items = crate::content::parse_line_of_mixed_text_and_logic(p);
        if let Some(items) = opt_items {
            option_only_content = Some(ContentList::from_items(items, loc.clone()));
        }

        p.expect(|p| p.parse_string("]"), "closing ']' for weave-style option");

        // Parse inner content (what happens after choosing)
        let inner_items = crate::content::parse_line_of_mixed_text_and_logic(p);
        if let Some(items) = inner_items {
            inner_content = ContentList::from_items(items, loc.clone());
        }
    }

    p.parse_whitespace();

    // Consume end of line
    p.parse_newline();

    let empty_content = start_content.is_none() && inner_content.items.is_empty() && option_only_content.is_none();
    if empty_content {
        p.warning("Choice is completely empty. Interpreting as a default fallback choice. Add a divert arrow to remove this warning: * ->");
    }

    Some(StoryNode::Choice(Choice {
        start_content,
        option_only_content,
        inner_content,
        condition,
        once_only,
        is_invisible_default: empty_content,
        indentation_depth,
        identifier,
        has_brackets,
        location: loc,
    }))
}

fn count_bullets(p: &mut Parser, bullet: &str) -> usize {
    let mut count = 0;
    loop {
        p.parse_whitespace();
        if p.parse_string(bullet).is_some() {
            count += 1;
        } else {
            break;
        }
    }
    count
}

/// Parse a gather: - dashes
pub fn parse_gather(p: &mut Parser) -> Option<StoryNode> {
    let loc = p.current_source_location();

    let depth = count_gather_dashes(p)?;
    if depth == 0 {
        return None;
    }

    // Optional bracketed name
    let identifier = parse_bracketed_name(p);

    // Optional newline before content
    p.parse_newline();

    Some(StoryNode::Gather(Gather {
        identifier,
        depth,
        content: None,
        location: loc,
    }))
}

fn count_gather_dashes(p: &mut Parser) -> Option<usize> {
    p.parse_whitespace();
    let mut count = 0;
    while parse_dash_not_arrow(p) {
        count += 1;
        p.parse_whitespace();
    }
    if count > 0 {
        Some(count)
    } else {
        None
    }
}

/// Parse a dash that is NOT part of a divert arrow (->)
fn parse_dash_not_arrow(p: &mut Parser) -> bool {
    let rule_id = p.begin_rule();
    if p.parse_string("->").is_none() && p.parse_single_character() == Some('-') {
        p.succeed_rule(rule_id);
        true
    } else {
        p.fail_rule(rule_id);
        false
    }
}

fn parse_bracketed_name(p: &mut Parser) -> Option<Identifier> {
    if p.parse_string("(").is_none() {
        return None;
    }
    p.parse_whitespace();
    let name = p.parse_identifier()?;
    let loc = p.current_source_location();
    p.parse_whitespace();
    p.expect(|p| p.parse_string(")"), "closing ')' for bracketed name");
    Some(Identifier::new(&name, loc))
}
