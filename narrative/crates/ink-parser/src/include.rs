use std::collections::HashMap;
use crate::parser::Parser;

/// Trait for file resolution and reading (used by INCLUDE)
pub trait FileHandler: Send + Sync {
    fn resolve(&self, filename: &str, current_file: &str) -> String;
    fn read(&self, resolved_path: &str) -> Result<String, String>;
}

/// Default file handler for filesystem-based resolution
pub struct DefaultFileHandler {
    #[allow(dead_code)]
    base_dir: String,
}

impl DefaultFileHandler {
    pub fn new(base_dir: &str) -> Self {
        Self {
            base_dir: base_dir.to_string(),
        }
    }
}

impl FileHandler for DefaultFileHandler {
    fn resolve(&self, filename: &str, current_file: &str) -> String {
        if let Some(last_slash) = current_file.rfind('/') {
            let current_dir = &current_file[..last_slash + 1];
            format!("{}{}", current_dir, filename)
        } else {
            filename.to_string()
        }
    }

    fn read(&self, resolved_path: &str) -> Result<String, String> {
        std::fs::read_to_string(resolved_path).map_err(|e| e.to_string())
    }
}

/// Memory file handler for testing
pub struct MemoryFileHandler {
    files: HashMap<String, String>,
}

impl MemoryFileHandler {
    pub fn new(files: HashMap<String, String>) -> Self {
        Self { files }
    }
}

impl FileHandler for MemoryFileHandler {
    fn resolve(&self, filename: &str, _current_file: &str) -> String {
        filename.to_string()
    }

    fn read(&self, resolved_path: &str) -> Result<String, String> {
        self.files
            .get(resolved_path)
            .cloned()
            .ok_or_else(|| format!("File not found: {}", resolved_path))
    }
}

/// Parse an INCLUDE statement: INCLUDE filename.ink
pub fn parse_include(p: &mut Parser) -> Option<String> {
    p.parse_whitespace();
    if p.parse_string("INCLUDE").is_none() {
        return None;
    }
    p.parse_whitespace();

    // Get raw string first
    let raw_filename = p.parse_identifier()?;
    p.parse_newline();

    Some(raw_filename)
}