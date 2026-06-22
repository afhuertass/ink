use crate::parser::Parser;
use crate::ir::base::*;
use crate::ir::knot::*;
use crate::ir::story::*;
use crate::statements::{parse_statements_at_level, StatementLevel};

/// Parse a knot definition: === knot_name ===
pub fn parse_knot_definition(p: &mut Parser) -> Option<StoryNode> {
    let decl = parse_knot_declaration(p)?;
    p.parse_newline();

    let content = parse_statements_at_level(p, StatementLevel::Knot);

    Some(StoryNode::Knot(Knot {
        identifier: decl.name,
        content,
        arguments: decl.arguments,
        is_function: decl.is_function,
        location: decl.location,
    }))
}

struct FlowDecl {
    name: Identifier,
    arguments: Vec<FlowArgument>,
    is_function: bool,
    location: SourceLocation,
}

fn parse_knot_declaration(p: &mut Parser) -> Option<FlowDecl> {
    p.parse_whitespace();

    let loc = p.current_source_location();
    let equals = parse_equals(p)?;
    if equals < 2 {
        return None;
    }

    p.parse_whitespace();

    // Check for "function" keyword
    let is_function = {
        let rule_id = p.begin_rule();
        let id = p.parse_identifier();
        if id.as_deref() == Some("function") {
            p.succeed_rule(rule_id);
            true
        } else {
            p.fail_rule(rule_id);
            false
        }
    };

    if is_function {
        p.parse_whitespace();
    }

    let name_str = p.parse_identifier();
    if name_str.is_none() {
        p.error("Expected the name of the knot");
        return None;
    }
    let identifier = Identifier::new(&name_str.unwrap(), p.current_source_location());

    p.parse_whitespace();

    // Parse optional parameters
    let arguments = parse_bracketed_arguments(p).unwrap_or_default();

    p.parse_whitespace();

    // Optional trailing equals
    parse_equals(p);

    Some(FlowDecl {
        name: identifier,
        arguments,
        is_function,
        location: loc,
    })
}

/// Parse stitch definition: = stitch_name
pub fn parse_stitch_definition(p: &mut Parser) -> Option<StoryNode> {
    let decl = parse_stitch_declaration(p)?;
    p.parse_newline();

    let content = parse_statements_at_level(p, StatementLevel::Stitch);

    Some(StoryNode::Knot(Knot {
        identifier: decl.name,
        content,
        arguments: decl.arguments,
        is_function: decl.is_function,
        location: decl.location,
    }))
}

fn parse_stitch_declaration(p: &mut Parser) -> Option<FlowDecl> {
    p.parse_whitespace();

    let loc = p.current_source_location();

    // Single equals for stitch
    if p.parse_string("=").is_none() {
        return None;
    }
    // If there's a second equals, that's a knot — fail this rule
    if p.peek() == Some('=') {
        return None;
    }

    p.parse_whitespace();

    // Stitches aren't allowed to be functions, but parse and report error
    let is_function = p.parse_string("function").is_some();
    if is_function {
        p.parse_whitespace();
        p.error("Stitches cannot be functions");
    }

    let name_str = p.parse_identifier();
    if name_str.is_none() {
        return None;
    }
    let identifier = Identifier::new(&name_str.unwrap(), p.current_source_location());

    p.parse_whitespace();
    let arguments = parse_bracketed_arguments(p).unwrap_or_default();
    p.parse_whitespace();

    Some(FlowDecl {
        name: identifier,
        arguments,
        is_function,
        location: loc,
    })
}

fn parse_equals(p: &mut Parser) -> Option<usize> {
    let mut count = 0;
    while p.parse_string("=").is_some() {
        count += 1;
    }
    if count > 0 {
        Some(count)
    } else {
        None
    }
}

fn parse_bracketed_arguments(p: &mut Parser) -> Option<Vec<FlowArgument>> {
    if p.parse_string("(").is_none() {
        return None;
    }

    let mut args = Vec::new();
    loop {
        p.parse_whitespace();
        if p.parse_string(")").is_some() {
            break;
        }
        if !args.is_empty() {
            if p.parse_string(",").is_none() {
                p.error("Expected ',' between arguments");
                // Try to continue by looking for closing paren
                p.parse_whitespace();
                if p.parse_string(")").is_some() {
                    break;
                }
            }
            p.parse_whitespace();
        }

        // Parse argument: possibly "ref", possibly "->" for divert target
        let first_id = p.parse_identifier();
        p.parse_whitespace();
        let is_divert = p.parse_string("->").is_some();
        p.parse_whitespace();
        let second_id = p.parse_identifier();

        let arg = if first_id.as_deref() == Some("ref") {
            FlowArgument {
                identifier: Identifier::new(
                    second_id.as_deref().unwrap_or(""),
                    p.current_source_location(),
                ),
                is_by_reference: true,
                is_divert_target: is_divert,
            }
        } else if is_divert {
            FlowArgument {
                identifier: Identifier::new(
                    second_id.as_deref().unwrap_or(""),
                    p.current_source_location(),
                ),
                is_by_reference: false,
                is_divert_target: true,
            }
        } else {
            FlowArgument {
                identifier: Identifier::new(
                    first_id.as_deref().unwrap_or(""),
                    p.current_source_location(),
                ),
                is_by_reference: false,
                is_divert_target: false,
            }
        };
        args.push(arg);
    }

    Some(args)
}

/// Check if current position starts a knot declaration
pub fn at_knot_declaration(p: &Parser) -> bool {
    p.remaining().starts_with("==")
}
