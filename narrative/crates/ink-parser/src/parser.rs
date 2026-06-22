use crate::ir::base::{ErrorType, InkError, SourceLocation};

/// Core parser engine: a cursor over input text with backtracking,
/// error collection, and primitive parse methods.
pub struct Parser {
    input: String,
    filename: String,
    pos: usize,
    line: usize,
    column: usize,
    errors: Vec<InkError>,
    rule_stack: Vec<RuleState>,
}

#[derive(Clone)]
struct RuleState {
    pos: usize,
    line: usize,
    column: usize,
}

impl Parser {
    pub fn new(input: &str, filename: &str) -> Self {
        Self {
            input: input.to_string(),
            filename: filename.to_string(),
            pos: 0,
            line: 1,
            column: 1,
            errors: Vec::new(),
            rule_stack: Vec::new(),
        }
    }

    // ---- Cursor state ----

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn is_end(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    /// Look ahead multiple characters without advancing.
    pub fn peek_str(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    pub fn advance(&mut self, count: usize) {
        let end = (self.pos + count).min(self.input.len());
        for ch in self.input[self.pos..end].chars() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.pos = end;
    }

    fn advance_one(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.advance(1);
        Some(ch)
    }

    pub fn current_source_location(&self) -> SourceLocation {
        SourceLocation::new(&self.filename, self.line, self.column)
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Get remaining input from current position.
    pub fn remaining(&self) -> &str {
        &self.input[self.pos..]
    }

    // ---- Rule system (backtracking) ----

    pub fn begin_rule(&mut self) -> usize {
        let id = self.rule_stack.len();
        self.rule_stack.push(RuleState {
            pos: self.pos,
            line: self.line,
            column: self.column,
        });
        id
    }

    pub fn succeed_rule(&mut self, _rule_id: usize) {
        self.rule_stack.pop();
    }

    pub fn fail_rule(&mut self, rule_id: usize) {
        if let Some(state) = self.rule_stack.get(rule_id).cloned() {
            self.pos = state.pos;
            self.line = state.line;
            self.column = state.column;
        }
        self.rule_stack.truncate(rule_id);
    }

    // ---- Primitive parsers ----

    /// Try to parse a specific string literal. Returns Some(()) on match.
    pub fn parse_string(&mut self, s: &str) -> Option<()> {
        if self.input[self.pos..].starts_with(s) {
            self.advance(s.len());
            Some(())
        } else {
            None
        }
    }

    /// Parse a single character, returning it.
    pub fn parse_single_character(&mut self) -> Option<char> {
        self.advance_one()
    }

    /// Parse an integer, returning its value.
    pub fn parse_int(&mut self) -> Option<i64> {
        let start = self.pos;
        let start_line = self.line;
        let start_col = self.column;
        let has_negative = self.parse_string("-").is_some();
        let digits_start = self.pos;
        while !self.is_end() && self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance(1);
        }
        if self.pos == digits_start {
            if has_negative {
                self.pos = start;
                self.line = start_line;
                self.column = start_col;
            }
            return None;
        }
        let num_str = &self.input[digits_start..self.pos];
        let val: i64 = num_str.parse().ok()?;
        Some(if has_negative { -val } else { val })
    }

    /// Parse a float, returning its value.
    pub fn parse_float(&mut self) -> Option<f64> {
        let start = self.pos;
        let start_line = self.line;
        let start_col = self.column;
        let _has_negative = self.parse_string("-").is_some();
        let num_start = self.pos;
        while !self.is_end() && self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance(1);
        }
        if self.parse_string(".").is_some() {
            while !self.is_end() && self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.advance(1);
            }
        }
        if self.pos == num_start {
            self.pos = start;
            self.line = start_line;
            self.column = start_col;
            return None;
        }
        let num_str = &self.input[start..self.pos];
        num_str.parse().ok()
    }

    /// Parse a newline character (either \r\n or \n).
    pub fn parse_newline(&mut self) -> bool {
        if self.parse_string("\r\n").is_some() || self.parse_string("\n").is_some() {
            true
        } else {
            false
        }
    }

    /// Parse optional whitespace (spaces and tabs, not newlines).
    pub fn parse_whitespace(&mut self) -> bool {
        let mut found = false;
        while !self.is_end() {
            match self.peek() {
                Some(' ') | Some('\t') => {
                    self.advance(1);
                    found = true;
                }
                _ => break,
            }
        }
        found
    }

    /// Parse until end of line (not consuming the newline).
    pub fn parse_until_newline(&mut self) -> String {
        let start = self.pos;
        while !self.is_end() && self.peek() != Some('\n') && self.peek() != Some('\r') {
            self.advance(1);
        }
        self.input[start..self.pos].to_string()
    }

    /// Check if we're at end of line (or end of input).
    pub fn at_end_of_line(&mut self) -> bool {
        self.is_end() || self.peek() == Some('\n') || self.peek() == Some('\r')
    }

    /// Parse until encountering one of the given characters, returning the text consumed.
    pub fn parse_until_char(&mut self, end_chars: &[char]) -> Option<String> {
        let start = self.pos;
        while !self.is_end() {
            if let Some(ch) = self.peek() {
                if end_chars.contains(&ch) {
                    break;
                }
                self.advance(1);
            }
        }
        if self.pos > start {
            Some(self.input[start..self.pos].to_string())
        } else {
            None
        }
    }

    /// Parse characters from a given set until none remain.
    pub fn parse_characters_from_string(&mut self, char_set: &str) -> Option<String> {
        let start = self.pos;
        while !self.is_end() {
            if let Some(ch) = self.peek() {
                if char_set.contains(ch) {
                    self.advance(1);
                } else {
                    break;
                }
            }
        }
        if self.pos > start {
            Some(self.input[start..self.pos].to_string())
        } else {
            None
        }
    }

    // ---- Error handling ----

    pub fn error(&mut self, message: &str) {
        self.errors.push(InkError::error(message, self.current_source_location()));
    }

    pub fn warning(&mut self, message: &str) {
        self.errors.push(InkError::warning(message, self.current_source_location()));
    }

    pub fn errors(&self) -> &[InkError] {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|e| e.error_type == ErrorType::Error)
    }

    pub fn take_errors(&mut self) -> Vec<InkError> {
        std::mem::take(&mut self.errors)
    }

    // ---- Identifier parsing ----

    /// Parse an ink identifier: starts with a letter, underscore, or CJK character,
    /// followed by letters, digits, underscores, or CJK characters.
    pub fn parse_identifier(&mut self) -> Option<String> {
        self.parse_whitespace();
        if self.is_end() {
            return None;
        }
        let start = self.pos;
        let first = self.peek()?;
        if !Self::is_identifier_start(first) {
            return None;
        }
        self.advance(1);
        while !self.is_end() {
            let ch = self.peek()?;
            if Self::is_identifier_continue(ch) {
                self.advance(1);
            } else {
                break;
            }
        }
        Some(self.input[start..self.pos].to_string())
    }

    fn is_identifier_start(ch: char) -> bool {
        ch.is_ascii_alphabetic()
            || ch == '_'
            || ('\u{4E00}'..='\u{9FFF}').contains(&ch)   // CJK Unified Ideographs
            || ('\u{3040}'..='\u{309F}').contains(&ch)    // Hiragana
            || ('\u{30A0}'..='\u{30FF}').contains(&ch)    // Katakana
    }

    fn is_identifier_continue(ch: char) -> bool {
        Self::is_identifier_start(ch) || ch.is_ascii_digit()
    }

    // ---- Convenience ----

    /// Parse optional content, returning None if the rule fails.
    pub fn optional<T>(&mut self, mut rule: impl FnMut(&mut Parser) -> Option<T>) -> Option<T> {
        rule(self)
    }

    /// Parse content expecting it to succeed; report an error if it doesn't.
    pub fn expect<T>(&mut self, mut rule: impl FnMut(&mut Parser) -> Option<T>, description: &str) -> Option<T> {
        match rule(self) {
            Some(val) => Some(val),
            None => {
                self.error(&format!("Expected {}", description));
                None
            }
        }
    }

    /// Try multiple parse rules in order, returning the first that succeeds.
    pub fn one_of<T>(&mut self, rules: &mut [impl FnMut(&mut Parser) -> Option<T>]) -> Option<T> {
        for rule in rules.iter_mut() {
            let rule_id = self.begin_rule();
            if let Some(result) = rule(self) {
                self.succeed_rule(rule_id);
                return Some(result);
            }
            self.fail_rule(rule_id);
        }
        None
    }
}
