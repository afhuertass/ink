use crate::parser::Parser;
use crate::ir::base::*;
use crate::ir::list_def::*;
use crate::ir::story::StoryNode;

/// Parse a LIST declaration: LIST name = item1, item2, item3
pub fn parse_list_declaration(p: &mut Parser) -> Option<StoryNode> {
    p.parse_whitespace();
    if p.parse_string("LIST").is_none() {
        return None;
    }
    p.parse_whitespace();

    // parse_identifier returns Option<String>, ? unwraps to String
    let name_str = p.parse_identifier()?;
    let identifier = Identifier::new(&name_str, p.current_source_location());

    p.parse_whitespace();
    let eq_rule_id = p.begin_rule();
    if p.parse_string("=").is_none() {
        p.fail_rule(eq_rule_id);
        p.error("Expected = for list");
        return None;
    }
    p.succeed_rule(eq_rule_id);

    p.parse_whitespace();

    let mut items = Vec::new();
    let mut item_value: i32 = 1;

    loop {
        p.parse_whitespace();
        if p.is_end() {
            break;
        }
        if p.peek() == Some('\n') || p.peek() == Some('\r') {
            break;
        }
        if p.peek() == Some(')') || p.peek() == Some('}') {
            break;
        }

        // Try to parse item name (may fail on empty items)
        let start = p.position();
        let item_name_str = p.parse_identifier()?;

        // Handle empty item (double comma)
        if item_name_str.is_empty() {
            // Roll back and just consume comma
            p.advance(start - p.position());
            if p.parse_string(",").is_some() {
                p.parse_whitespace();
                item_value += 1;
                continue;
            }
            break;
        }

        p.parse_whitespace();

        let mut explicit_value: Option<i32> = None;
        if p.parse_string(":").is_some() {
            p.parse_whitespace();
            if let Some(val) = p.parse_int() {
                explicit_value = Some(val as i32);
                item_value = val as i32 + 1;
            }
        }

        let item = ListItemDeclaration {
            name: Identifier::new(&item_name_str, p.current_source_location()),
            value: explicit_value.or(Some(item_value)),
        };
        items.push(item);
        item_value += 1;

        p.parse_whitespace();
        if p.parse_string(",").is_some() {
            p.parse_whitespace();
            continue;
        }
        break;
    }

    p.parse_newline();

    Some(StoryNode::ListDeclaration(ListDeclaration {
        identifier,
        items,
        location: p.current_source_location(),
    }))
}