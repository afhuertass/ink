use crate::ir::base::*;

#[derive(Debug, Clone)]
pub struct Directive {
    pub directive_type: DirectiveType,
    pub name: String,
    pub args: Vec<DirectiveArg>,
    pub modifiers: Vec<DirectiveModifier>,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectiveType {
    Action,
    Scene,
    Character,
    State,
    Event,
    Asset,
}

impl DirectiveType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "action" => Some(DirectiveType::Action),
            "scene" => Some(DirectiveType::Scene),
            "character" => Some(DirectiveType::Character),
            "state" => Some(DirectiveType::State),
            "event" => Some(DirectiveType::Event),
            "asset" => Some(DirectiveType::Asset),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DirectiveType::Action => "action",
            DirectiveType::Scene => "scene",
            DirectiveType::Character => "character",
            DirectiveType::State => "state",
            DirectiveType::Event => "event",
            DirectiveType::Asset => "asset",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirectiveArg {
    pub value: String,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct DirectiveModifier {
    pub key: String,
    pub value: String,
    pub location: SourceLocation,
}