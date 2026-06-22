use crate::ir::base::*;
use crate::ir::expression::Expression;

#[derive(Debug, Clone)]
pub struct Divert {
    pub target: DivertTarget,
    pub is_tunnel: bool,
    pub is_thread: bool,
    pub is_conditional: bool,
    pub arguments: Vec<Expression>,
    pub is_empty: bool,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub enum DivertTarget {
    Path(InkPath),
    Variable(String),
}

#[derive(Debug, Clone)]
pub struct InkPath {
    pub components: Vec<PathComponent>,
}

#[derive(Debug, Clone)]
pub enum PathComponent {
    Name(String),
    Parent,
    Index(usize),
}

impl InkPath {
    pub fn from_names(names: Vec<String>) -> Self {
        Self {
            components: names.into_iter().map(PathComponent::Name).collect(),
        }
    }

    pub fn to_string_path(&self) -> String {
        self.components
            .iter()
            .map(|c| match c {
                PathComponent::Name(n) => n.clone(),
                PathComponent::Parent => "^".to_string(),
                PathComponent::Index(i) => i.to_string(),
            })
            .collect::<Vec<_>>()
            .join(".")
    }
}

#[derive(Debug, Clone)]
pub struct TunnelOnwards {
    pub divert_after: Option<Box<Divert>>,
    pub location: SourceLocation,
}
