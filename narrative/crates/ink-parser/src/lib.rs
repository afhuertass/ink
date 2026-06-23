pub mod ir;
pub mod parser;
pub mod whitespace;
pub mod content;
pub mod statements;
pub mod knot;
pub mod choices;
pub mod divert;
pub mod logic;
pub mod conditionals;
pub mod sequences;
pub mod tags;
pub mod include;
pub mod lists;
pub mod directives;

use ir::story::ParsedStory;
use parser::Parser;

/// Parse an ink source string into a ParsedStory.
pub fn parse_story(source: &str, filename: &str) -> ParsedStory {
    let mut p = Parser::new(source, filename);
    let content = statements::parse_statements_at_level(&mut p, statements::StatementLevel::Top);
    let errors = p.take_errors();

    ParsedStory {
        content,
        global_variables: Vec::new(),
        list_declarations: Vec::new(),
        external_declarations: Vec::new(),
        errors,
        is_include: false,
    }
}