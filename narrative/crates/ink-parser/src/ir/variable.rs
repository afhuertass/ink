use crate::ir::base::*;
use crate::ir::divert::InkPath;
use crate::ir::expression::Expression;

#[derive(Debug, Clone)]
pub struct VariableAssignment {
    pub identifier: Identifier,
    pub value: Option<Expression>,
    pub is_new_declaration: bool,
    pub is_global: bool,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct VariableReference {
    pub name: String,
    pub is_divert_target: bool,
    pub is_read_count: bool,
    pub read_count_path: Option<InkPath>,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct ConstDeclaration {
    pub identifier: Identifier,
    pub value: ExpressionValue,
    pub location: SourceLocation,
}

use crate::ir::expression::ExpressionValue;
