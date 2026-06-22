use crate::ir::base::*;

#[derive(Debug, Clone)]
pub struct Tag {
    pub is_start: bool,
    pub in_choice: bool,
    pub location: SourceLocation,
}
