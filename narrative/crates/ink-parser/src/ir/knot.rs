use crate::ir::base::*;

#[derive(Debug, Clone)]
pub struct Knot {
    pub identifier: Identifier,
    pub content: Vec<crate::ir::story::StoryNode>,
    pub arguments: Vec<FlowArgument>,
    pub is_function: bool,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct Stitch {
    pub identifier: Identifier,
    pub content: Vec<crate::ir::story::StoryNode>,
    pub arguments: Vec<FlowArgument>,
    pub is_function: bool,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct FlowArgument {
    pub identifier: Identifier,
    pub is_by_reference: bool,
    pub is_divert_target: bool,
}
