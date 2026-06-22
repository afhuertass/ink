use crate::ir::base::*;
use crate::ir::content::ContentList;
use crate::ir::expression::Expression;

#[derive(Debug, Clone)]
pub struct Sequence {
    pub elements: Vec<SequenceElement>,
    pub sequence_type: SequenceType,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceType {
    Cycle,
    Shuffle,
    Once,
    Stopping,
}

#[derive(Debug, Clone)]
pub struct SequenceElement {
    pub content: ContentList,
    pub condition: Option<Expression>,
}
