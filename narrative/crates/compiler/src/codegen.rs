//! Code generation: converting ParsedStory IR → runtime Story objects.
//!
//! The compilation pipeline:
//! 1. Parse ink source → ParsedStory (IR)
//! 2. Compile ParsedStory → Story (runtime)
//! 3. Serialize Story → JSON

use ink_parser::ir::story::{ParsedStory, StoryNode};
use ink_parser::ir::knot::Knot;
use ink_parser::ir::choice::{Choice, Gather};
use ink_parser::ir::content::ContentItem;
use ink_parser::ir::divert::{InkPath, DivertTarget, PathComponent};
use ink_parser::ir::variable::{VariableAssignment as IrVarAssignment, VariableReference as IrVarRef};
use ink_parser::ir::conditional::Conditional;
use ink_parser::ir::sequence::Sequence;
use ink_parser::ir::expression::{
    Expression, ExpressionKind, ExpressionValue, BinaryOperator
};

use crate::runtime_types::*;

/// Compile a ParsedStory IR into a runtime Story.
pub fn compile(story: &ParsedStory) -> Story {
    let mut result = Story::new();

    // Add global variables
    for var in &story.global_variables {
        result.add_variable(&var.identifier.name);
    }

    // Process top-level content
    for node in &story.content {
        compile_story_node(node, &mut result);
    }

    // Add list declarations
    for list in &story.list_declarations {
        let def = ListDefinition {
            name: list.identifier.name.clone(),
            items: list.items.iter().map(|item| ListItem {
                name: item.name.name.clone(),
                value: item.value.unwrap_or(0),
            }).collect(),
        };
        result.add_list(def);
    }

    result
}

fn compile_story_node(node: &StoryNode, story: &mut Story) {
    match node {
        StoryNode::Knot(knot) => compile_knot(knot, story),
        StoryNode::Text(t) => {
            story.root.push(InkObject::String(t.text.clone()));
        }
        StoryNode::Divert(d) => {
            story.root.push(compile_divert(d));
        }
        StoryNode::Conditional(cond) => {
            compile_conditional(cond, &mut story.root);
        }
        StoryNode::VariableAssignment(va) => {
            story.root.push(compile_var_assignment(va));
        }
        StoryNode::ConstDeclaration(_) => {}
        StoryNode::ListDeclaration(list) => {
            let def = ListDefinition {
                name: list.identifier.name.clone(),
                items: list.items.iter().map(|item| ListItem {
                    name: item.name.name.clone(),
                    value: item.value.unwrap_or(0),
                }).collect(),
            };
            story.add_list(def);
        }
        StoryNode::Tag(_tag) => {
            story.root.push(InkObject::Tag(Tag::new("")));
        }
        StoryNode::Logic(logic) => {
            for node in &logic.content {
                compile_story_node(node, story);
            }
        }
        StoryNode::AuthorWarning(_) => {}
        StoryNode::Sequence(seq) => {
            compile_sequence(seq, &mut story.root);
        }
                StoryNode::Choice(_) | StoryNode::Gather(_) | StoryNode::Include(_) | StoryNode::Directive(_) => {}
    }
}

/// Compile a knot definition into a named container.
fn compile_knot(knot: &Knot, story: &mut Story) {
    let knot_name = normalize_name(&knot.identifier.name);
    let mut knot_container = Container::new();
    knot_container.set_name(&knot_name);

    // Process knot content
    compile_knot_content(&knot.content, &mut knot_container);

    // End with tunnel return or function return
    if knot.is_function {
        knot_container.push(InkObject::ControlCommand(ControlCommand::FuncRet));
    } else {
        knot_container.push(InkObject::ControlCommand(ControlCommand::TunnelRet));
    }

    story.add_knot(&knot_name, knot_container);
}

/// Normalize a name: replace spaces with underscores.
fn normalize_name(name: &str) -> String {
    name.replace(' ', "_")
}

/// Convert InkPath to a dot-separated string.
fn ink_path_to_string(path: &InkPath) -> String {
    path.components.iter().map(|c| match c {
        PathComponent::Name(n) => n.clone(),
        PathComponent::Index(i) => i.to_string(),
        PathComponent::Parent => "^".to_string(),
    }).collect::<Vec<_>>().join(".")
}

/// Process knot content (can include nested Knots for stitches).
fn compile_knot_content(nodes: &[StoryNode], container: &mut Container) {
    for node in nodes {
        match node {
            StoryNode::Knot(stitch) => {
                let name = normalize_name(&stitch.identifier.name);
                let mut sub = Container::new();
                sub.set_name(&name);
                sub.flags = ContainerFlags::from_bits(
                    ContainerFlags::VISITS | ContainerFlags::TURNS | ContainerFlags::COUNT_START_ONLY
                );
                compile_knot_content(&stitch.content, &mut sub);
                sub.push(InkObject::ControlCommand(ControlCommand::End));
                container.add_named(&name, sub);
            }
            StoryNode::Choice(c) => {
                compile_choice(c, container);
            }
            StoryNode::Gather(g) => {
                compile_gather(g, container);
            }
            StoryNode::Text(t) => {
                container.push(InkObject::String(t.text.clone()));
            }
            StoryNode::Divert(d) => {
                container.push(compile_divert(d));
            }
            StoryNode::Conditional(cond) => {
                compile_conditional(cond, container);
            }
            StoryNode::Sequence(seq) => {
                compile_sequence(seq, container);
            }
            StoryNode::Tag(_tag) => {
                container.push(InkObject::Tag(Tag::new("")));
            }
            StoryNode::VariableAssignment(va) => {
                container.push(compile_var_assignment(va));
            }
            StoryNode::Logic(logic) => {
                for node in &logic.content {
                    match node {
                        StoryNode::Text(t) => container.push(InkObject::String(t.text.clone())),
                        StoryNode::Divert(d) => container.push(compile_divert(d)),
                        _ => {}
                    }
                }
            }
            StoryNode::AuthorWarning(_) | StoryNode::ConstDeclaration(_)
            | StoryNode::ListDeclaration(_) | StoryNode::Include(_) | StoryNode::Directive(_) => {}
        }
    }
}

/// Compile a choice into a ChoicePoint with associated containers.
fn compile_choice(choice: &Choice, container: &mut Container) {
    static mut CHOICE_COUNTER: usize = 0;
    unsafe {
        CHOICE_COUNTER += 1;
        let counter = CHOICE_COUNTER;

        let choice_name = choice.identifier.as_ref()
            .map(|n| normalize_name(&n.name))
            .unwrap_or_else(|| format!("c_{}", counter));
        let start_name = format!("s_{}", counter);

        // Determine flags
        let mut flags_val = if choice.once_only { 0x10 } else { 0 };
        if choice.condition.is_some() { flags_val |= 0x1; }
        if choice.start_content.is_some() || !choice.inner_content.items.is_empty() { flags_val |= 0x2; }
        if choice.option_only_content.is_some() || choice.has_brackets { flags_val |= 0x4; }
        if choice.is_invisible_default { flags_val |= 0x8; }
        let flags = ChoiceFlags::from_bits(flags_val);

        // Start content container
        let start_path = format!(".^.{}", start_name);
        let mut start_container = Container::new();
        start_container.set_name(&start_name);
        start_container.flags = ContainerFlags::from_bits(
            ContainerFlags::VISITS | ContainerFlags::TURNS | ContainerFlags::COUNT_START_ONLY
        );
        if let Some(ref sc) = choice.start_content {
            compile_content_items(&sc.items, &mut start_container);
        } else {
            compile_content_items(&choice.inner_content.items, &mut start_container);
        }

        // Choice-only content container
        let choice_path = format!(".^.{}", choice_name);
        let mut choice_container = Container::new();
        choice_container.set_name(&choice_name);
        choice_container.flags = ContainerFlags::from_bits(
            ContainerFlags::VISITS | ContainerFlags::TURNS | ContainerFlags::COUNT_START_ONLY
        );
        if let Some(ref oc) = choice.option_only_content {
            compile_content_items(&oc.items, &mut choice_container);
        }

        // Outer container with choice evaluation
        let mut outer = Container::new();

        // Evaluate start content as string
        outer.push(InkObject::ControlCommand(ControlCommand::EvalStart));
        outer.push(InkObject::ControlCommand(ControlCommand::StringEvalStart));
        outer.push(InkObject::Divert(crate::runtime_types::Divert::function(&start_path)));
        outer.push(InkObject::ControlCommand(ControlCommand::StringEvalEnd));
        outer.push(InkObject::ControlCommand(ControlCommand::EvalEnd));
        outer.push(InkObject::ControlCommand(ControlCommand::Out));

        // ChoicePoint object
        let cp = ChoicePoint {
            target_path: choice_path,
            flags,
            start_content_path: Some(start_path),
            choice_only_content_path: Some(format!(".^.{}", choice_name)),
            original_text_hash: None,
        };
        outer.push(InkObject::ChoicePoint(cp));

        outer.add_named(&start_name, start_container);
        outer.add_named(&choice_name, choice_container);

        container.push(InkObject::Container(outer));
    }
}

fn compile_gather(gather: &Gather, container: &mut Container) {
    let mut gather_container = Container::new();
    gather_container.flags = ContainerFlags::from_bits(
        ContainerFlags::VISITS | ContainerFlags::TURNS | ContainerFlags::COUNT_START_ONLY
    );

    if let Some(ref content) = gather.content {
        compile_content_items(&content.items, &mut gather_container);
    }

    gather_container.push(InkObject::ControlCommand(ControlCommand::End));
    container.push(InkObject::Container(gather_container));
}

/// Compile a divert from the parser's Divert to a runtime Divert.
fn compile_divert(d: &ink_parser::ir::divert::Divert) -> InkObject {
    let target_str = match &d.target {
        DivertTarget::Path(p) => ink_path_to_string(p),
        DivertTarget::Variable(v) => v.clone(),
    };

    let mut div = if d.is_thread {
        crate::runtime_types::Divert::tunnel(&target_str)
    } else if d.is_tunnel {
        crate::runtime_types::Divert::tunnel(&target_str)
    } else {
        crate::runtime_types::Divert::new(&target_str)
    };

    div.is_conditional = d.is_conditional;
    // Note: function call handling would need more work
    // For now, treat function calls as regular diverts

    InkObject::Divert(div)
}

fn compile_var_assignment(va: &IrVarAssignment) -> InkObject {
    let is_reassign = va.is_new_declaration;
    let assignment = VariableAssignment::global(&va.identifier.name, is_reassign);
    InkObject::VariableAssignment(assignment)
}

fn _compile_var_reference(vr: &IrVarRef) -> InkObject {
    if vr.is_read_count {
        let path_str = match vr.read_count_path {
            Some(ref path) => ink_path_to_string(path),
            None => vr.name.clone(),
        };
        InkObject::VariableReference(VariableReference::read_count(&path_str))
    } else {
        InkObject::VariableReference(VariableReference::new(&vr.name))
    }
}

/// Compile an expression onto the evaluation stack.
fn compile_expression_to_stack(expr: &Expression) -> Vec<InkObject> {
    let mut result = Vec::new();
    result.push(InkObject::ControlCommand(ControlCommand::EvalStart));
    compile_expr_value(&expr.kind, &mut result);
    result.push(InkObject::ControlCommand(ControlCommand::EvalEnd));
    result
}

fn compile_expr_value(kind: &ExpressionKind, out: &mut Vec<InkObject>) {
    match kind {
        ExpressionKind::Literal(v) => {
            match v {
                ExpressionValue::Int(n) => out.push(InkObject::Int(*n)),
                ExpressionValue::Float(f) => out.push(InkObject::Float(*f)),
                ExpressionValue::String(s) => out.push(InkObject::String(s.clone())),
                ExpressionValue::Bool(b) => out.push(InkObject::Int(if *b { 1 } else { 0 })),
                ExpressionValue::DivertTarget(path) => {
                    out.push(InkObject::DivertTarget(ink_path_to_string(path)));
                }
                ExpressionValue::VariablePointer(name) => {
                    out.push(InkObject::VariablePointer { varname: name.clone(), context_index: -1 });
                }
                ExpressionValue::InkList(_) => {}
            }
        }
        ExpressionKind::VariableRef(vr) => {
            if vr.is_read_count {
                let path_str = match vr.read_count_path {
                    Some(ref path) => ink_path_to_string(path),
                    None => vr.name.clone(),
                };
                out.push(InkObject::VariableReference(VariableReference::read_count(&path_str)));
            } else {
                out.push(InkObject::VariableReference(VariableReference::new(&vr.name)));
            }
        }
        ExpressionKind::BinaryOp(binary_op) => {
            compile_expr_value(&binary_op.left.kind, out);
            compile_expr_value(&binary_op.right.kind, out);
            if let Some(nf) = binary_operator_to_native_func(&binary_op.op) {
                out.push(InkObject::NativeFuncCall(nf));
            }
        }
        ExpressionKind::UnaryOp(unary_op) => {
            compile_expr_value(&unary_op.inner.kind, out);
            if let Some(nf) = unary_operator_to_native_func(&unary_op.op) {
                out.push(InkObject::NativeFuncCall(nf));
            }
        }
        ExpressionKind::FunctionCall(fc) => {
            // External function call
            out.push(InkObject::Divert(crate::runtime_types::Divert::external(
                &fc.name.name, fc.arguments.len() as i32
            )));
        }
        ExpressionKind::List(_) | ExpressionKind::InkListLiteral(_) 
        | ExpressionKind::DivertTarget(_) | ExpressionKind::MultipleConditions(_) => {}
    }
}

fn binary_operator_to_native_func(op: &BinaryOperator) -> Option<NativeFuncCall> {
    match op {
        BinaryOperator::Add => Some(NativeFuncCall::Add),
        BinaryOperator::Sub => Some(NativeFuncCall::Subtract),
        BinaryOperator::Mul => Some(NativeFuncCall::Multiply),
        BinaryOperator::Div => Some(NativeFuncCall::Divide),
        BinaryOperator::Mod => Some(NativeFuncCall::Modulo),
        BinaryOperator::Equal => Some(NativeFuncCall::Equal),
        BinaryOperator::NotEqual => Some(NativeFuncCall::NotEqual),
        BinaryOperator::Greater => Some(NativeFuncCall::Greater),
        BinaryOperator::Less => Some(NativeFuncCall::Less),
        BinaryOperator::GreaterEqual => Some(NativeFuncCall::GreaterEq),
        BinaryOperator::LessEqual => Some(NativeFuncCall::LessEq),
        BinaryOperator::And => Some(NativeFuncCall::And),
        BinaryOperator::Or => Some(NativeFuncCall::Or),
        BinaryOperator::Min => Some(NativeFuncCall::Min),
        BinaryOperator::Max => Some(NativeFuncCall::Max),
    }
}

fn unary_operator_to_native_func(op: &ink_parser::ir::expression::UnaryOperator) -> Option<NativeFuncCall> {
    match op {
        ink_parser::ir::expression::UnaryOperator::Negate => Some(NativeFuncCall::Negate),
        ink_parser::ir::expression::UnaryOperator::Not => Some(NativeFuncCall::Not),
    }
}

fn compile_conditional(cond: &Conditional, container: &mut Container) {
    for (i, branch) in cond.branches.iter().enumerate() {
        if i > 0 {
            container.push(InkObject::ControlCommand(ControlCommand::Pop));
        }

        if let Some(ref expr) = branch.condition {
            for obj in compile_expression_to_stack(expr) {
                container.push(obj);
            }
        }

        for node in &branch.content {
            match node {
                StoryNode::Text(t) => container.push(InkObject::String(t.text.clone())),
                StoryNode::Divert(d) => container.push(compile_divert(d)),
                StoryNode::Gather(g) => compile_gather(g, container),
                StoryNode::Choice(c) => compile_choice(c, container),
                _ => {}
            }
        }
    }
}

fn compile_sequence(seq: &Sequence, container: &mut Container) {
    container.push(InkObject::ControlCommand(ControlCommand::EvalStart));
    container.push(InkObject::Int(seq.elements.len() as i64));
    container.push(InkObject::ControlCommand(ControlCommand::Sequence));
    container.push(InkObject::ControlCommand(ControlCommand::EvalEnd));

    for element in &seq.elements {
        let mut elem_container = Container::new();
        elem_container.flags = ContainerFlags::from_bits(
            ContainerFlags::VISITS | ContainerFlags::TURNS | ContainerFlags::COUNT_START_ONLY
        );
        compile_content_items(&element.content.items, &mut elem_container);
        elem_container.push(InkObject::ControlCommand(ControlCommand::End));
        container.push(InkObject::Container(elem_container));
    }
}

fn compile_content_items(items: &[ContentItem], container: &mut Container) {
    for item in items {
        match item {
            ContentItem::Text(t) => {
                container.push(InkObject::String(t.text.clone()));
            }
            ContentItem::Expression(expr) => {
                for obj in compile_expression_to_stack(expr) {
                    container.push(obj);
                }
            }
            ContentItem::Divert(d) => {
                container.push(compile_divert(d));
            }
            ContentItem::Tag(_tag) => {
                container.push(InkObject::Tag(Tag::new("")));
            }
            ContentItem::Glue => {
                container.push(InkObject::ControlCommand(ControlCommand::NoOp));
            }
        }
    }
}