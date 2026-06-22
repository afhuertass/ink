use crate::ir::base::*;
use crate::ir::expression::Expression;
use crate::ir::story::StoryNode;

#[derive(Debug, Clone)]
pub struct Conditional {
    pub branches: Vec<ConditionalBranch>,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct ConditionalBranch {
    pub condition: Option<Expression>,
    pub content: Vec<StoryNode>,
    pub is_else: bool,
    pub location: SourceLocation,
}
