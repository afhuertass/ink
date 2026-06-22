use crate::ir::base::*;

#[derive(Debug, Clone)]
pub struct ExternalDeclaration {
    pub identifier: Identifier,
    pub argument_names: Vec<String>,
    pub location: SourceLocation,
}
