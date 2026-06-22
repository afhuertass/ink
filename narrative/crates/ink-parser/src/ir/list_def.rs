use crate::ir::base::*;

#[derive(Debug, Clone)]
pub struct ListDeclaration {
    pub identifier: Identifier,
    pub items: Vec<ListItemDeclaration>,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct ListItemDeclaration {
    pub name: Identifier,
    pub value: Option<i32>,
}
