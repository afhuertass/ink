use crate::parser::Parser;
use crate::ir::divert::*;
use crate::ir::story::*;

/// Parse diverts, tunnels, and threads
/// Supports: -> target, -> tunnel ->, ->->, <- thread
pub fn parse_multi_divert(p: &mut Parser) -> Option<StoryNode> {
    let loc = p.current_source_location();

    // Try thread first
    if let Some(node) = parse_start_thread(p) {
        return Some(node);
    }

    // Parse arrows and divert targets
    let first_arrow = parse_divert_arrow_or_tunnel_onwards(p)?;
    let mut arrows_and_targets: Vec<(String, Option<Divert>)> = vec![(first_arrow, None)];

    // Interleave arrows and targets
    loop {
        // Try to parse a divert identifier
        if let Some(divert) = parse_divert_identifier_with_arguments(p) {
            // Attach to last arrow
            arrows_and_targets.last_mut().unwrap().1 = Some(divert);
        } else {
            break;
        }

        // Try another arrow
        if let Some(arrow) = parse_divert_arrow_or_tunnel_onwards(p) {
            arrows_and_targets.push((arrow, None));
        } else {
            break;
        }
    }

    // Build result from arrows and targets
    let mut diverts = Vec::new();

    for (i, (arrow, target)) in arrows_and_targets.iter().enumerate() {
        if arrow == "->->" {
            // Tunnel onwards
            let mut tunnel_onwards = TunnelOnwards {
                divert_after: None,
                location: loc.clone(),
            };
            // If there's a target after, it's the divert after tunnel onwards
            if let Some(div) = target {
                tunnel_onwards.divert_after = Some(Box::new(div.clone()));
            }
            diverts.push(StoryNode::Divert(Divert {
                target: DivertTarget::Path(InkPath::from_names(vec!["->->".to_string()])),
                is_tunnel: false,
                is_thread: false,
                is_conditional: false,
                arguments: Vec::new(),
                is_empty: false,
                location: loc.clone(),
            }));
            // For now, return a simplified version
            break;
        }

        if let Some(div) = target {
            let is_tunnel = i < arrows_and_targets.len() - 1;
            let mut d = div.clone();
            d.is_tunnel = is_tunnel;
            diverts.push(StoryNode::Divert(d));
        }
    }

    // Single empty -> (for default choices)
    if diverts.is_empty() && arrows_and_targets.len() == 1 && arrows_and_targets[0].0 == "->" {
        diverts.push(StoryNode::Divert(Divert {
            target: DivertTarget::Path(InkPath::from_names(vec![])),
            is_tunnel: false,
            is_thread: false,
            is_conditional: false,
            arguments: Vec::new(),
            is_empty: true,
            location: loc.clone(),
        }));
    }

    if diverts.is_empty() {
        return None;
    }

    // Return the first divert (simplified for now — full multi-divert will be handled in codegen)
    Some(diverts.remove(0))
}

fn parse_start_thread(p: &mut Parser) -> Option<StoryNode> {
    p.parse_whitespace();
    if p.parse_string("<-").is_none() {
        return None;
    }
    p.parse_whitespace();

    let divert = parse_divert_identifier_with_arguments(p)?;
    let mut thread_divert = divert;
    thread_divert.is_thread = true;

    Some(StoryNode::Divert(thread_divert))
}

fn parse_divert_identifier_with_arguments(p: &mut Parser) -> Option<Divert> {
    p.parse_whitespace();

    let components = parse_dot_separated_path(p)?;
    let loc = p.current_source_location();

    p.parse_whitespace();

    // Optional function call arguments
    let arguments = parse_expression_function_call_arguments(p).unwrap_or_default();

    let target_path = InkPath { components };

    Some(Divert {
        target: DivertTarget::Path(target_path),
        is_tunnel: false,
        is_thread: false,
        is_conditional: false,
        arguments,
        is_empty: false,
        location: loc,
    })
}

fn parse_dot_separated_path(p: &mut Parser) -> Option<Vec<PathComponent>> {
    let first = p.parse_identifier()?;
    let mut components = vec![PathComponent::Name(first)];

    while p.parse_string(".").is_some() {
        // Try identifier
        if let Some(name) = p.parse_identifier() {
            components.push(PathComponent::Name(name));
        } else {
            // Could be parent (^)
            if p.parse_string("^").is_some() {
                components.push(PathComponent::Parent);
            } else {
                p.error("Expected identifier or ^ after '.' in path");
                break;
            }
        }
    }

    Some(components)
}

fn parse_expression_function_call_arguments(p: &mut Parser) -> Option<Vec<crate::ir::expression::Expression>> {
    // TODO: implement expression parsing for function call arguments
    if p.parse_string("(").is_none() {
        return None;
    }
    // For now, just consume until closing paren
    let mut depth = 1;
    while !p.is_end() && depth > 0 {
        if p.peek() == Some('(') {
            depth += 1;
        } else if p.peek() == Some(')') {
            depth -= 1;
        }
        if depth > 0 {
            p.advance(1);
        }
    }
    p.parse_string(")");
    Some(Vec::new()) // Empty args for now
}

fn parse_divert_arrow_or_tunnel_onwards(p: &mut Parser) -> Option<String> {
    let mut num_arrows = 0;
    while p.parse_string("->").is_some() {
        num_arrows += 1;
    }
    match num_arrows {
        0 => None,
        1 => Some("->".to_string()),
        2 => Some("->->".to_string()),
        _ => {
            p.error("Unexpected number of arrows in divert. Should only have '->' or '->->'");
            Some("->->".to_string())
        }
    }
}

/// Parse a single simple divert (used at end of content lines)
pub fn parse_single_divert(p: &mut Parser) -> Option<StoryNode> {
    let node = parse_multi_divert(p)?;

    // Only return if it's a simple single divert (not tunnel, not multi)
    if let StoryNode::Divert(ref d) = node {
        if d.is_tunnel || d.is_thread || d.is_empty {
            return None;
        }
    }

    Some(node)
}
