use crate::ir::base::*;

/// The top-level parsed story.
#[derive(Debug, Clone)]
pub struct ParsedStory {
    pub content: Vec<StoryNode>,
    pub global_variables: Vec<VariableDeclaration>,
    pub list_declarations: Vec<ListDeclaration>,
    pub external_declarations: Vec<ExternalDeclaration>,
    pub errors: Vec<InkError>,
    pub is_include: bool,
}

impl ParsedStory {
    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|e| e.is_error())
    }
}

/// Any node at the top level of a story.
#[derive(Debug, Clone)]
pub enum StoryNode {
    Knot(Knot),
    Text(Text),
    Choice(Choice),
    Gather(Gather),
    Divert(Divert),
    Conditional(Conditional),
    VariableAssignment(VariableAssignment),
    ConstDeclaration(ConstDeclaration),
    ListDeclaration(ListDeclaration),
    Tag(Tag),
    Logic(LogicBlock),
    AuthorWarning(String),
    Sequence(Sequence),
    Include(String),
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub identifier: Identifier,
    pub initial_value: Option<ExpressionValue>,
}

#[derive(Debug, Clone)]
pub struct LogicBlock {
    pub content: Vec<StoryNode>,
    pub location: SourceLocation,
}

use crate::ir::knot::Knot;
use crate::ir::choice::{Choice, Gather};
use crate::ir::content::Text;
use crate::ir::divert::Divert;
use crate::ir::variable::{VariableAssignment, ConstDeclaration};
use crate::ir::expression::ExpressionValue;
use crate::ir::conditional::Conditional;
use crate::ir::sequence::Sequence;
use crate::ir::tag::Tag;
use crate::ir::list_def::ListDeclaration;
use crate::ir::external::ExternalDeclaration;
