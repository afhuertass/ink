use crate::ir::base::*;
use crate::ir::divert::Divert;
use crate::ir::expression::Expression;
use crate::ir::tag::Tag;

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub location: SourceLocation,
}

impl Text {
    pub fn new(text: &str, location: SourceLocation) -> Self {
        Self {
            text: text.to_string(),
            location,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContentList {
    pub items: Vec<ContentItem>,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub enum ContentItem {
    Text(Text),
    Expression(Expression),
    Divert(Divert),
    Tag(Tag),
    Glue,
}

impl ContentList {
    pub fn empty(location: SourceLocation) -> Self {
        Self {
            items: Vec::new(),
            location,
        }
    }

    pub fn from_items(items: Vec<ContentItem>, location: SourceLocation) -> Self {
        Self { items, location }
    }
}
