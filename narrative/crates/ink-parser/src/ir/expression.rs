use crate::ir::base::*;
use crate::ir::divert::InkPath;
use crate::ir::variable::VariableReference;

#[derive(Debug, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Literal(ExpressionValue),
    VariableRef(VariableReference),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    FunctionCall(FunctionCall),
    List(ListExpression),
    InkListLiteral(InkListLiteral),
    DivertTarget(InkPath),
    MultipleConditions(MultipleConditionExpression),
}

#[derive(Debug, Clone)]
pub enum ExpressionValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    DivertTarget(InkPath),
    VariablePointer(String),
    InkList(InkListLiteral),
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub left: Box<Expression>,
    pub op: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
    Min,
    Max,
}

#[derive(Debug, Clone)]
pub struct UnaryOp {
    pub op: UnaryOperator,
    pub inner: Box<Expression>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: Identifier,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct MultipleConditionExpression {
    pub conditions: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct ListExpression {
    pub items: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct InkListLiteral {
    pub items: Vec<InkListItem>,
}

#[derive(Debug, Clone)]
pub struct InkListItem {
    pub origin: Option<String>,
    pub name: String,
}
