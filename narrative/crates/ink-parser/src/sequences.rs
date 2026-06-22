use crate::parser::Parser;
use crate::ir::sequence::*;
use crate::ir::content::*;
use crate::ir::story::*;

/// Parse a sequence: {! cycle | ~ shuffle | & once | stopping (default)}
/// Note: sequences are parsed when we see { at statement level.
/// The character after { determines the type:
///   {! -> cycle (repeats forever, cycling through elements)
///   {~ -> shuffle (random order)
///   {& -> once (each element shown once, then stops)
///   {  -> stopping (default, stop at last element)
pub fn parse_sequence(p: &mut Parser) -> Option<StoryNode> {
    let loc = p.current_source_location();

    if p.parse_string("{").is_none() {
        return None;
    }

    // Determine sequence type
    let sequence_type = {
        if p.parse_string("!").is_some() {
            SequenceType::Cycle
        } else if p.parse_string("~").is_some() {
            SequenceType::Shuffle
        } else if p.parse_string("&").is_some() {
            SequenceType::Once
        } else {
            SequenceType::Stopping
        }
    };

    // Parse elements separated by |
    let mut elements = Vec::new();

    loop {
        p.parse_whitespace();

        // Check for end
        if p.peek() == Some('}') {
            p.advance(1);
            break;
        }

        // Parse element content
        let element = parse_sequence_element(p)?;
        elements.push(element);

        p.parse_whitespace();

        // Check for pipe separator
        if p.peek() == Some('|') {
            p.advance(1);
            continue;
        }

        // Check for closing brace
        if p.peek() == Some('}') {
            p.advance(1);
            break;
        }

        // Blank element (double pipe)
        if p.peek() == Some('}') || p.peek() == Some('|') {
            continue;
        }

        break;
    }

    if elements.is_empty() {
        p.error("Sequence requires at least one element");
        return None;
    }

    Some(StoryNode::Sequence(Sequence {
        elements,
        sequence_type,
        location: loc,
    }))
}

/// Parse one element of a sequence
fn parse_sequence_element(p: &mut Parser) -> Option<SequenceElement> {
    let loc = p.current_source_location();

    // Check for condition [condition]
    let condition = None; // TODO: parse inline conditions

    // Parse content for this element
    let mut content_items = Vec::new();

    loop {
        if p.is_end() || p.peek() == Some('\n') || p.peek() == Some('\r') {
            break;
        }
        if p.peek() == Some('|') || p.peek() == Some('}') {
            break;
        }

        p.parse_whitespace();

        // Parse content text (no special processing for now)
        let text = parse_sequence_text(p)?;
        content_items.push(ContentItem::Text(text));

        // Check for glue
        if p.peek_str("<>") {
            p.advance(2);
            content_items.push(ContentItem::Glue);
        }
    }

    if content_items.is_empty() {
        // Return empty content list
        return Some(SequenceElement {
            content: ContentList::empty(loc),
            condition: None,
        });
    }

    Some(SequenceElement {
        content: ContentList::from_items(content_items, loc),
        condition,
    })
}

/// Parse text content within a sequence element
fn parse_sequence_text(p: &mut Parser) -> Option<Text> {
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
        if ch == '{' || ch == '}' || ch == '|' || ch == '\n' || ch == '\r' || ch == '#' || ch == '[' {
            break;
        }

        // Handle escape
        if ch == '\\' {
            p.advance(1);
            if let Some(escaped) = p.parse_single_character() {
                result.push(escaped);
            }
            continue;
        }

        // Check for divert or glue
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