use crate::parser::Parser;
use crate::ir::base::*;
use crate::ir::conditional::*;
use crate::ir::story::*;

/// Parse a conditional block (inline or multiline)
pub fn parse_conditional(p: &mut Parser) -> Option<StoryNode> {
    let loc = p.current_source_location();

    // Must start with {
    if p.parse_string("{").is_none() {
        return None;
    }

    p.parse_whitespace();

    // Check if this is multiline (might start with -)
    if p.peek() == Some('-') {
        // Multiline conditional
        return parse_multiline_conditional(p, loc);
    }

    // Inline conditional
    return parse_inline_conditional(p, loc);
}

fn parse_inline_conditional(p: &mut Parser, loc: SourceLocation) -> Option<StoryNode> {
    // Parse condition
    let condition = parse_conditional_expression(p)?;

    p.parse_whitespace();

    // Parse branches
    let mut branches = Vec::new();

    // First branch (required)
    p.expect(|p| p.parse_string(":"), "':' for conditional branch")?;
    p.parse_whitespace();

    let first_content = parse_conditional_content(p)?;
    branches.push(ConditionalBranch {
        condition: Some(condition),
        content: first_content,
        is_else: false,
        location: loc.clone(),
    });

    // Additional branches (else if, else)
    loop {
        p.parse_whitespace();

        // Check for else if
        if p.peek() == Some('|') {
            p.advance(1);
            p.parse_whitespace();

            if p.parse_string(":").is_some() {
                // Pure else branch
                p.parse_whitespace();
                let else_content = parse_conditional_content(p)?;
                branches.push(ConditionalBranch {
                    condition: None,
                    content: else_content,
                    is_else: true,
                    location: loc.clone(),
                });
                break;
            } else if p.peek() == Some('{') {
                // Nested conditional — stop here (can't backtrack, just break)
                break;
            } else {
                // Else-if with condition
                let else_if_condition = parse_conditional_expression(p)?;
                p.parse_whitespace();
                p.expect(|p| p.parse_string(":"), "':' for else-if branch")?;
                p.parse_whitespace();
                let else_if_content = parse_conditional_content(p)?;
                branches.push(ConditionalBranch {
                    condition: Some(else_if_condition),
                    content: else_if_content,
                    is_else: false,
                    location: loc.clone(),
                });
            }
        } else {
            break;
        }
    }

    p.expect(|p| p.parse_string("}"), "closing '}' for conditional");

    Some(StoryNode::Conditional(Conditional {
        branches,
        location: loc,
    }))
}

fn parse_multiline_conditional(p: &mut Parser, loc: SourceLocation) -> Option<StoryNode> {
    // We're at the - of a multiline conditional
    p.parse_string("-"); // consume the -

    let mut branches = Vec::new();

    loop {
        p.parse_whitespace();

        // Check for closing }
        if p.parse_string("}").is_some() {
            break;
        }

        // Else branch
        if p.peek() == Some('|') {
            p.advance(1);
            p.parse_whitespace();
            p.expect(|p| p.parse_string("-"), "'-' for else branch")?;
            p.parse_whitespace();
            // Content until next branch or closing
            let content = parse_statements_at_level_inner(p)?;
            branches.push(ConditionalBranch {
                condition: None,
                content,
                is_else: true,
                location: loc.clone(),
            });
            continue;
        }

        // Conditional branch (no leading -, that's handled above)
        // Actually, multiline conditionals don't have condition keywords, just -
        // The first - is the opener, then subsequent - are else-if
        // Let me check: { x: content - else: content }
        // or { - content - else - content }

        // For now, let's treat this as: parse content, then check for else branches
        let content = parse_statements_at_level_inner(p)?;

        // Check for else marker
        p.parse_whitespace();
        if p.peek() == Some('|') {
            p.advance(1);
            p.parse_whitespace();
            p.expect(|p| p.parse_string("-"), "'-' for else branch")?;
            p.parse_whitespace();
            let else_content = parse_statements_at_level_inner(p)?;
            branches.push(ConditionalBranch {
                condition: None,
                content,
                is_else: false,
                location: loc.clone(),
            });
            branches.push(ConditionalBranch {
                condition: None,
                content: else_content,
                is_else: true,
                location: loc.clone(),
            });
        } else {
            branches.push(ConditionalBranch {
                condition: None,
                content,
                is_else: false,
                location: loc.clone(),
            });
        }

        // Check for closing
        if p.parse_string("}").is_some() {
            break;
        }
    }

    Some(StoryNode::Conditional(Conditional {
        branches,
        location: loc,
    }))
}

/// Parse content inside a conditional branch
fn parse_conditional_content(p: &mut Parser) -> Option<Vec<StoryNode>> {
    let mut content = Vec::new();

    loop {
        if p.is_end() || p.peek() == Some('\n') || p.peek() == Some('\r') {
            break;
        }
        if p.peek() == Some('|') || p.peek() == Some('}') {
            break;
        }

        p.parse_whitespace();

        // Parse inline content
        let items = crate::content::parse_line_of_mixed_text_and_logic(p)?;
        if items.is_empty() {
            break;
        }

        let text = items
            .iter()
            .filter_map(|i| match i {
                crate::ir::content::ContentItem::Text(t) => Some(t.text.clone()),
                crate::ir::content::ContentItem::Glue => Some("<>".to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");

        if !text.is_empty() && text != "\n" {
            content.push(crate::ir::story::StoryNode::Text(crate::ir::content::Text::new(
                &text,
                p.current_source_location(),
            )));
        }

        // Check for inline divert
        if p.peek_str("->") {
            if let Some(divert) = crate::divert::parse_single_divert(p) {
                content.push(divert);
            }
        }
    }

    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}

/// Parse the condition expression for a conditional
fn parse_conditional_expression(p: &mut Parser) -> Option<crate::ir::expression::Expression> {
    use crate::ir::expression::*;
    use crate::ir::variable::VariableReference;

    let loc = p.current_source_location();

    // Try variable reference
    if let Some(name) = p.parse_identifier() {
        return Some(Expression {
            kind: ExpressionKind::VariableRef(VariableReference {
                name,
                is_divert_target: false,
                is_read_count: false,
                read_count_path: None,
                location: loc.clone(),
            }),
            location: loc,
        });
    }

    // Try boolean literal
    if p.parse_string("true").is_some() {
        return Some(Expression {
            kind: ExpressionKind::Literal(ExpressionValue::Bool(true)),
            location: loc,
        });
    }
    if p.parse_string("false").is_some() {
        return Some(Expression {
            kind: ExpressionKind::Literal(ExpressionValue::Bool(false)),
            location: loc,
        });
    }

    // Try integer (for truthy test)
    if let Some(val) = p.parse_int() {
        return Some(Expression {
            kind: ExpressionKind::Literal(ExpressionValue::Int(val)),
            location: loc,
        });
    }

    // For now, return a "true" expression as fallback
    Some(Expression {
        kind: ExpressionKind::Literal(ExpressionValue::Bool(true)),
        location: loc,
    })
}

/// Parse statements at inner level (for multiline conditionals)
fn parse_statements_at_level_inner(p: &mut Parser) -> Option<Vec<StoryNode>> {
    let mut content = Vec::new();

    loop {
        p.parse_whitespace();
        if p.is_end() {
            break;
        }

        // Check for closing or branch
        if p.peek() == Some('}') || p.peek() == Some('|') {
            break;
        }

        // Parse line
        if let Some(items) = crate::content::parse_line_of_mixed_text_and_logic(p) {
            p.parse_newline();

            let text = items
                .iter()
                .filter_map(|i| match i {
                    crate::ir::content::ContentItem::Text(t) => Some(t.text.clone()),
                    crate::ir::content::ContentItem::Glue => Some("<>".to_string()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("");

            if !text.is_empty() {
                content.push(crate::ir::story::StoryNode::Text(crate::ir::content::Text::new(
                    &text,
                    p.current_source_location(),
                )));
            }
        }
    }

    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}