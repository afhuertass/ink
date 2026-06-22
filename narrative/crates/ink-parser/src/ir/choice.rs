use crate::ir::base::*;
use crate::ir::content::ContentList;
use crate::ir::expression::Expression;

#[derive(Debug, Clone)]
pub struct Choice {
    pub start_content: Option<ContentList>,
    pub option_only_content: Option<ContentList>,
    pub inner_content: ContentList,
    pub condition: Option<Expression>,
    pub once_only: bool,
    pub is_invisible_default: bool,
    pub indentation_depth: usize,
    pub identifier: Option<Identifier>,
    pub has_brackets: bool,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct Gather {
    pub identifier: Option<Identifier>,
    pub depth: usize,
    pub content: Option<ContentList>,
    pub location: SourceLocation,
}
