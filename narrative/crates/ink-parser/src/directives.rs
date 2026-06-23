use crate::parser::Parser;
use crate::ir::base::*;
use crate::ir::directive::*;

pub fn parse_directive(p: &mut Parser) -> Option<Directive> {
    let loc = p.current_source_location();

    if p.parse_string("@").is_none() {
        return None;
    }
    p.parse_whitespace();

    let type_str = p.parse_identifier()?;
    let directive_type = DirectiveType::from_str(&type_str)?;
    p.parse_whitespace();

    if p.parse_string(":").is_none() {
        p.error("Expected ':'");
        return None;
    }
    p.parse_whitespace();

    let name = p.parse_identifier().unwrap_or_default();
    
    Some(Directive {
        directive_type,
        name,
        args: Vec::new(),
        modifiers: Vec::new(),
        location: loc,
    })
}