use crate::Backend;
use tower_lsp::lsp_types::*;
use ink_parser::parse_story;

pub async fn get_diagnostics(uri: &Url, source: &str) -> Vec<Diagnostic> {
    let filename = uri.path();
    let story = parse_story(source, filename);
    
    story.errors.iter().map(|err| {
        let range = Range {
            start: Position { line: err.location.line as u32 - 1, character: err.location.column as u32 - 1 },
            end: Position { line: err.location.line as u32 - 1, character: err.location.column as u32 },
        };
        Diagnostic {
            range,
            severity: Some(if err.is_error() { DiagnosticSeverity::ERROR } else { DiagnosticSeverity::WARNING }),
            message: err.message.clone(),
            ..Default::default()
        }
    }).collect()
}