use crate::parser::Parser;
use crate::ir::base::*;
use crate::ir::expression::*;
use crate::ir::variable::*;

/// Parse a logic line starting with ~
pub fn parse_logic_line(p: &mut Parser) -> Option<crate::ir::story::StoryNode> {
    p.parse_whitespace();
    if p.parse_string("~").is_none() {
        return None;
    }
    p.parse_whitespace();

    let loc = p.current_source_location();

    // Try variable declaration/assignment
    // ~ temp x = 5
    // ~ x = 10
    // ~ x++
    // ~ x--
    // ~ function_call()

    // Try TEMP declaration
    if p.parse_string("temp").is_some() {
        p.parse_whitespace();
        let name = p.parse_identifier()?;
        let identifier = Identifier::new(&name, p.current_source_location());
        p.parse_whitespace();

        let value = if p.parse_string("=").is_some() {
            p.parse_whitespace();
            Some(parse_expression(p)?)
        } else {
            None
        };

        p.parse_newline();
        return Some(crate::ir::story::StoryNode::VariableAssignment(VariableAssignment {
            identifier,
            value,
            is_new_declaration: true,
            is_global: false,
            location: loc,
        }));
    }

    // Try variable assignment or increment
    let name = p.parse_identifier()?;
    let identifier = Identifier::new(&name, p.current_source_location());
    p.parse_whitespace();

    // Increment/decrement
    if p.parse_string("++").is_some() {
        p.parse_newline();
        return Some(crate::ir::story::StoryNode::VariableAssignment(VariableAssignment {
            identifier,
            value: None, // increment is special-cased in codegen
            is_new_declaration: false,
            is_global: false,
            location: loc,
        }));
    }
    if p.parse_string("--").is_some() {
        p.parse_newline();
        return Some(crate::ir::story::StoryNode::VariableAssignment(VariableAssignment {
            identifier,
            value: None, // decrement is special-cased in codegen
            is_new_declaration: false,
            is_global: false,
            location: loc,
        }));
    }

    // Assignment
    if p.parse_string("=").is_some() {
        p.parse_whitespace();
        let value = parse_expression(p)?;
        p.parse_newline();
        return Some(crate::ir::story::StoryNode::VariableAssignment(VariableAssignment {
            identifier,
            value: Some(value),
            is_new_declaration: false,
            is_global: false,
            location: loc,
        }));
    }

    // Function call on logic line
    p.parse_newline();
    Some(crate::ir::story::StoryNode::Logic(crate::ir::story::LogicBlock {
        content: vec![],
        location: loc,
    }))
}

/// Parse a VAR global declaration
pub fn parse_var_declaration(p: &mut Parser) -> Option<crate::ir::story::StoryNode> {
    p.parse_whitespace();
    if p.parse_string("VAR").is_none() {
        return None;
    }
    p.parse_whitespace();

    let name = p.parse_identifier()?;
    let identifier = Identifier::new(&name, p.current_source_location());
    let id_loc = identifier.location.clone();
    p.parse_whitespace();

    p.expect(|p| p.parse_string("="), "= for variable assignment")?;
    p.parse_whitespace();

    let value = parse_expression(p)?;
    p.parse_newline();

    Some(crate::ir::story::StoryNode::VariableAssignment(VariableAssignment {
        identifier,
        value: Some(value),
        is_new_declaration: true,
        is_global: true,
        location: id_loc,
    }))
}

/// Parse a CONST declaration
pub fn parse_const_declaration(p: &mut Parser) -> Option<crate::ir::story::StoryNode> {
    p.parse_whitespace();
    if p.parse_string("CONST").is_none() {
        return None;
    }
    p.parse_whitespace();

    let name = p.parse_identifier()?;
    let identifier = Identifier::new(&name, p.current_source_location());
    p.parse_whitespace();

    p.expect(|p| p.parse_string("="), "= for constant assignment")?;
    p.parse_whitespace();

    let expr = parse_expression(p)?;
    let value = extract_expression_value(&expr)?;

    p.parse_newline();

    Some(crate::ir::story::StoryNode::ConstDeclaration(ConstDeclaration {
        identifier,
        value,
        location: p.current_source_location(),
    }))
}

/// Parse an EXTERNAL declaration
pub fn parse_external_declaration(p: &mut Parser) -> Option<crate::ir::story::StoryNode> {
    p.parse_whitespace();
    let first = p.parse_identifier()?;
    if first != "EXTERNAL" {
        return None;
    }
    p.parse_whitespace();

    let name = p.parse_identifier()?;
    let identifier = Identifier::new(&name, p.current_source_location());
    p.parse_whitespace();

    // Parse argument list
    let mut arg_names = Vec::new();
    if p.parse_string("(").is_some() {
        loop {
            p.parse_whitespace();
            if p.parse_string(")").is_some() {
                break;
            }
            if !arg_names.is_empty() {
                p.expect(|p| p.parse_string(","), "',' between arguments");
                p.parse_whitespace();
            }
            if let Some(arg) = p.parse_identifier() {
                arg_names.push(arg);
            }
        }
    }

    p.parse_newline();

    Some(crate::ir::story::StoryNode::ConstDeclaration(ConstDeclaration {
        // Use ConstDeclaration as a carrier — EXTERNALs are special
        identifier,
        value: ExpressionValue::String(format!("EXTERNAL({})", arg_names.join(","))),
        location: p.current_source_location(),
    }))
}

fn extract_expression_value(expr: &Expression) -> Option<ExpressionValue> {
    match &expr.kind {
        ExpressionKind::Literal(v) => Some(v.clone()),
        _ => None,
    }
}

/// Parse an expression (simplified for now — full expression parsing is complex)
pub fn parse_expression(p: &mut Parser) -> Option<Expression> {
    let loc = p.current_source_location();

    // Try number literal
    if let Some(val) = p.parse_int() {
        return Some(Expression {
            kind: ExpressionKind::Literal(ExpressionValue::Int(val)),
            location: loc.clone(),
        });
    }

    // Try float literal
    if let Some(val) = p.parse_float() {
        return Some(Expression {
            kind: ExpressionKind::Literal(ExpressionValue::Float(val)),
            location: loc.clone(),
        });
    }

    // Try string literal
    if p.peek() == Some('"') {
        return parse_string_literal(p);
    }

    // Try boolean
    {
        let rule_id = p.begin_rule();
        if p.parse_string("true").is_some() {
            p.succeed_rule(rule_id);
            return Some(Expression {
                kind: ExpressionKind::Literal(ExpressionValue::Bool(true)),
                location: loc,
            });
        }
        p.fail_rule(rule_id);
    }
    {
        let rule_id = p.begin_rule();
        if p.parse_string("false").is_some() {
            p.succeed_rule(rule_id);
            return Some(Expression {
                kind: ExpressionKind::Literal(ExpressionValue::Bool(false)),
                location: loc,
            });
        }
        p.fail_rule(rule_id);
    }

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

    None
}

fn parse_string_literal(p: &mut Parser) -> Option<Expression> {
    let loc = p.current_source_location();
    if p.parse_string("\"").is_none() {
        return None;
    }

    let mut result = String::new();
    loop {
        if p.is_end() {
            p.error("Unclosed string literal");
            break;
        }
        if p.peek() == Some('"') {
            p.advance(1);
            break;
        }
        if p.peek() == Some('\\') {
            p.advance(1);
            if let Some(ch) = p.parse_single_character() {
                result.push(ch);
            }
        } else if let Some(ch) = p.parse_single_character() {
            result.push(ch);
        }
    }

    Some(Expression {
        kind: ExpressionKind::Literal(ExpressionValue::String(result)),
        location: loc,
    })
}
