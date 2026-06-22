use crate::parser::Parser;
use crate::ir::content::*;
use crate::ir::story::*;

/// Parse a line of mixed text and logic.
pub fn parse_line_of_mixed_text_and_logic(p: &mut Parser) -> Option<Vec<ContentItem>> {
    p.parse_whitespace();

    let mut items = Vec::new();

    loop {
        if p.is_end() || p.peek() == Some('\n') || p.peek() == Some('\r') {
            break;
        }

        // Try glue <>
        if p.parse_string("<>").is_some() {
            items.push(ContentItem::Glue);
            continue;
        }

        // Try tag #
        if p.peek() == Some('#') {
            // For now, skip tags — will be handled in tags task
            break;
        }

        // Try inline logic { }
        if p.peek() == Some('{') {
            // For now, skip inline logic — will be handled in expression/conditional tasks
            break;
        }

        // Plain text — consume characters until we hit a special character
        if let Some(text) = parse_content_text(p) {
            items.push(ContentItem::Text(text));
        } else {
            break;
        }
    }

    if items.is_empty() {
        return None;
    }

    // Trim trailing whitespace from last text
    trim_end_whitespace(&mut items, false);

    // Append newline
    let has_non_tag = items.iter().any(|i| !matches!(i, ContentItem::Tag(_)));
    if has_non_tag {
        items.push(ContentItem::Text(Text::new("\n", p.current_source_location())));
    }

    Some(items)
}

fn parse_content_text(p: &mut Parser) -> Option<Text> {
    let loc = p.current_source_location();
    let mut result = String::new();

    loop {
        if p.is_end() {
            break;
        }
        let ch = match p.peek() {
            Some(ch) => ch,
            None => break,
        };

        // Stop at special characters
        if ch == '{' || ch == '}' || ch == '#' || ch == '\n' || ch == '\r' || ch == '|' || ch == '[' || ch == ']' {
            break;
        }

        // Handle escape character
        if ch == '\\' {
            p.advance(1);
            if let Some(escaped) = p.parse_single_character() {
                result.push(escaped);
            }
            continue;
        }

        // Check for divert arrow or glue before consuming
        if ch == '-' || ch == '<' {
            let remaining = p.remaining();
            if remaining.starts_with("->") || remaining.starts_with("<>") {
                break;
            }
        }

        result.push(ch);
        p.advance(1);
    }

    if result.is_empty() {
        None
    } else {
        Some(Text::new(&result, loc))
    }
}

fn trim_end_whitespace(items: &mut Vec<ContentItem>, terminate_with_space: bool) {
    if items.is_empty() {
        return;
    }
    let last_idx = items.len() - 1;
    if let ContentItem::Text(ref mut t) = items[last_idx] {
        t.text = t.text.trim_end_matches(' ').trim_end_matches('\t').to_string();
        if terminate_with_space {
            t.text.push(' ');
        } else if t.text.is_empty() {
            items.remove(last_idx);
            trim_end_whitespace(items, false);
        }
    }
}

/// Parse a simple text-only line and convert to a StoryNode.
pub fn parse_text_line(p: &mut Parser) -> Option<StoryNode> {
    let loc = p.current_source_location();
    let items = parse_line_of_mixed_text_and_logic(p)?;

    // Consume end of line
    p.parse_newline();

    // Convert content items to text
    let text = items
        .iter()
        .map(|i| match i {
            ContentItem::Text(t) => t.text.clone(),
            ContentItem::Glue => "<>".to_string(),
            _ => String::new(),
        })
        .collect::<Vec<_>>()
        .join("");

    Some(StoryNode::Text(Text::new(&text, loc)))
}
