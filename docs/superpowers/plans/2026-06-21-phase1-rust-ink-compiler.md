# Phase 1: Rust Ink Compiler — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust compiler that produces the same ink JSON runtime format (version 21) as the C# compiler, serving as a drop-in replacement for `inklecate`.

**Architecture:** Rust workspace with three crates: `ink-parser` (hand-written recursive descent parser producing a typed IR), `compiler` (IR → ink JSON code generation + reference resolution), and `cli` (command-line interface). The parser mirrors the C# `InkParser` structure with partial-class-like modules.

**Tech Stack:** Rust (edition 2021), `serde` + `serde_json` for JSON output, `clap` for CLI argument parsing.

---

## File Structure

```
narrative/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── ink-parser/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs             # Re-exports, ParsedStory top-level type
│   │       ├── parser.rs          # Core parser engine (cursor, error recovery)
│   │       ├── statements.rs      # Statement-level parsing dispatch
│   │       ├── content.rs         # Mixed text and logic lines
│   │       ├── knot.rs            # Knot/stitch definitions
│   │       ├── choices.rs         # Choice and gather parsing
│   │       ├── divert.rs          # Divert, tunnel, thread parsing
│   │       ├── logic.rs           # Logic lines (~), variable assignment
│   │       ├── expressions.rs     # Expression parser (arithmetic, boolean, function calls)
│   │       ├── conditionals.rs    # If/else/else if blocks
│   │       ├── sequences.rs       # Cycles, shuffles, once, etc.
│   │       ├── tags.rs            # # tag parsing
│   │       ├── include.rs         # INCLUDE statements
│   │       ├── whitespace.rs      # Whitespace and newline handling
│   │       ├── lists.rs           # LIST declarations
│   │       ├── character_ranges.rs # Unicode identifier ranges
│   │       └── ir/                # Parsed IR types
│   │           ├── mod.rs         # Re-exports
│   │           ├── story.rs       # ParsedStory, includes, global declarations
│   │           ├── knot.rs        # Knot, Stitch, FlowBase, Argument
│   │           ├── choice.rs      # Choice, Gather
│   │           ├── content.rs      # Text, ContentList
│   │           ├── divert.rs      # Divert, TunnelOnwards, Path
│   │           ├── variable.rs    # VariableAssignment, VariableReference, ConstDeclaration
│   │           ├── expression.rs  # Expression types, FunctionCall, BinaryOp, UnaryOp
│   │           ├── conditional.rs # Conditional, ConditionalSingleBranch
│   │           ├── sequence.rs    # Sequence
│   │           ├── tag.rs         # Tag
│   │           ├── list_def.rs    # ListDefinition, ListValue
│   │           ├── external.rs    # ExternalDeclaration
│   │           └── base.rs       # Object base, Identifier, DebugMetadata, SourceLocation
│   ├── compiler/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs             # Re-exports, top-level compile() function
│   │       ├── codegen.rs         # IR → runtime object hierarchy
│   │       ├── resolve.rs         # Reference resolution pass
│   │       ├── json_output.rs     # Runtime hierarchy → ink JSON format
│   │       ├── runtime_types.rs   # Runtime type definitions (Container, Value, ControlCommand, etc.)
│   │       └── error.rs           # Compilation errors
│   └── cli/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs            # CLI entry point (compile, play, check modes)
└── tests/
    ├── conformance/               # Tests ported from C# test suite
    │   ├── mod.rs
    │   ├── basic.rs                # Hello world, empty, basic text
    │   ├── choices.rs              # Choice parsing and output
    │   ├── diverts.rs              # Diverts, tunnels, threads
    │   ├── variables.rs            # Variables, logic, state
    │   ├── conditionals.rs         # If/else/conditional choices
    │   ├── functions.rs            # Function calls, external bindings
    │   ├── lists.rs                # List operations
    │   ├── sequences.rs            # Sequence types
    │   ├── glue.rs                 # Glue (<>), whitespace
    │   ├── tags.rs                 # Tags
    │   ├── includes.rs             # Multi-file includes
    │   ├── multi_flow.rs           # Parallel flows
    │   ├── save_load.rs            # State save/load
    │   └── error_checks.rs         # Error and warning detection
    └── fixtures/                   # .ink test files
        └── (test .ink files copied from C# test suite)
```

---

### Task 1: Workspace Scaffolding

**Files:**
- Create: `narrative/Cargo.toml`
- Create: `narrative/crates/ink-parser/Cargo.toml`
- Create: `narrative/crates/compiler/Cargo.toml`
- Create: `narrative/crates/cli/Cargo.toml`
- Create: `narrative/crates/ink-parser/src/lib.rs`
- Create: `narrative/crates/compiler/src/lib.rs`
- Create: `narrative/crates/cli/src/main.rs`

- [ ] **Step 1: Create the workspace root Cargo.toml**

```toml
[workspace]
members = [
    "crates/ink-parser",
    "crates/compiler",
    "crates/cli",
]
resolver = "2"
```

- [ ] **Step 2: Create ink-parser Cargo.toml**

```toml
[package]
name = "ink-parser"
version = "0.1.0"
edition = "2021"

[dependencies]
```

- [ ] **Step 3: Create compiler Cargo.toml**

```toml
[package]
name = "narrative-compiler"
version = "0.1.0"
edition = "2021"

[dependencies]
ink-parser = { path = "../ink-parser" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

- [ ] **Step 4: Create cli Cargo.toml**

```toml
[package]
name = "narrative-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
narrative-compiler = { path = "../compiler" }
clap = { version = "4", features = ["derive"] }
```

- [ ] **Step 5: Create minimal lib.rs and main.rs stubs**

`crates/ink-parser/src/lib.rs`:
```rust
pub mod ir;
pub mod parser;
```

`crates/compiler/src/lib.rs`:
```rust
pub mod codegen;
pub mod resolve;
pub mod runtime_types;
pub mod json_output;
pub mod error;
```

`crates/cli/src/main.rs`:
```rust
fn main() {
    println!("narrative-cli: not yet implemented");
}
```

- [ ] **Step 6: Verify workspace builds**

Run: `cd narrative && cargo build`
Expected: Compiles successfully with no errors

- [ ] **Step 7: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: scaffold Rust workspace with ink-parser, compiler, and cli crates"
```

---

### Task 2: IR Base Types

**Files:**
- Create: `crates/ink-parser/src/ir/mod.rs`
- Create: `crates/ink-parser/src/ir/base.rs`

- [ ] **Step 1: Write the test for IR base types**

Create `crates/ink-parser/src/ir/mod.rs`:
```rust
pub mod base;
pub mod story;
pub mod knot;
pub mod choice;
pub mod content;
pub mod divert;
pub mod variable;
pub mod expression;
pub mod conditional;
pub mod sequence;
pub mod tag;
pub mod list_def;
pub mod external;

pub use base::*;
pub use story::*;
pub use knot::*;
pub use choice::*;
pub use content::*;
pub use divert::*;
pub use variable::*;
pub use expression::*;
pub use conditional::*;
pub use sequence::*;
pub use tag::*;
pub use list_def::*;
pub use external::*;
```

Create `tests/conformance/mod.rs`:
```rust
mod basic;
```

Create `tests/conformance/basic.rs`:
```rust
use ink_parser::ir::*;

#[test]
fn test_identifier_creation() {
    let id = Identifier::new("test_knot", SourceLocation::new("test.ink", 1, 1));
    assert_eq!(id.name, "test_knot");
    assert_eq!(id.location.line, 1);
}

#[test]
fn test_source_location_display() {
    let loc = SourceLocation::new("story.ink", 42, 10);
    assert_eq!(format!("{}", loc), "story.ink:42:10");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL — modules not yet created

- [ ] **Step 3: Implement IR base types**

Create `crates/ink-parser/src/ir/base.rs`:
```rust
use std::fmt;

/// Source location for error reporting and debug metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    pub fn new(file: &str, line: usize, column: usize) -> Self {
        Self {
            file: file.to_string(),
            line,
            column,
        }
    }

    pub fn unknown() -> Self {
        Self {
            file: String::new(),
            line: 0,
            column: 0,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// An ink identifier (knot name, variable name, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub location: SourceLocation,
}

impl Identifier {
    pub fn new(name: &str, location: SourceLocation) -> Self {
        Self {
            name: name.to_string(),
            location,
        }
    }
}

/// Base trait for all parsed IR nodes.
pub trait InkNode {
    fn as_any(&self) -> &dyn std::any::Any;
    fn source_location(&self) -> &SourceLocation;
}

/// Error severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    Error,
    Warning,
    AuthorMessage,
}

/// A parse or compilation error/warning.
#[derive(Debug, Clone)]
pub struct InkError {
    pub message: String,
    pub error_type: ErrorType,
    pub location: SourceLocation,
}

impl InkError {
    pub fn error(message: &str, location: SourceLocation) -> Self {
        Self {
            message: message.to_string(),
            error_type: ErrorType::Error,
            location,
        }
    }

    pub fn warning(message: &str, location: SourceLocation) -> Self {
        Self {
            message: message.to_string(),
            error_type: ErrorType::Warning,
            location,
        }
    }
}

impl fmt::Display for InkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.error_type {
            ErrorType::Error => "ERROR",
            ErrorType::Warning => "WARNING",
            ErrorType::AuthorMessage => "TODO",
        };
        write!(f, "{} at {}: {}", prefix, self.location, self.message)
    }
}
```

Create stub files for each IR module (empty struct/type placeholders):

`crates/ink-parser/src/ir/story.rs`:
```rust
use crate::ir::base::*;

/// The top-level parsed story.
#[derive(Debug, Clone)]
pub struct ParsedStory {
    pub content: Vec<StoryNode>,
    pub global_variables: Vec<VariableDeclaration>,
    pub list_declarations: Vec<ListDeclaration>,
    pub external_declarations: Vec<ExternalDeclaration>,
    pub errors: Vec<InkError>,
    pub is_include: bool,
}

/// Any node at the top level of a story.
#[derive(Debug, Clone)]
pub enum StoryNode {
    Knot(Knot),
    Text(Text),
    Choice(Choice),
    Gather(Gather),
    Divert(Divert),
    Conditional(Conditional),
    VariableAssignment(VariableAssignment),
    ConstDeclaration(ConstDeclaration),
    Tag(Tag),
    Logic(LogicBlock),
    AuthorWarning(String),
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub identifier: Identifier,
    pub initial_value: Option<ExpressionValue>,
}

#[derive(Debug, Clone)]
pub struct LogicBlock {
    pub content: Vec<StoryNode>,
    pub location: SourceLocation,
}
```

`crates/ink-parser/src/ir/knot.rs`:
```rust
use crate::ir::base::*;

#[derive(Debug, Clone)]
pub struct Knot {
    pub identifier: Identifier,
    pub content: Vec<StoryNode>,
    pub arguments: Vec<FlowArgument>,
    pub is_function: bool,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct Stitch {
    pub identifier: Identifier,
    pub content: Vec<StoryNode>,
    pub arguments: Vec<FlowArgument>,
    pub is_function: bool,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct FlowArgument {
    pub identifier: Identifier,
    pub is_by_reference: bool,
    pub is_divert_target: bool,
}

// Re-export StoryNode from story.rs
pub use crate::ir::story::StoryNode;
```

`crates/ink-parser/src/ir/choice.rs`:
```rust
use crate::ir::base::*;

#[derive(Debug, Clone)]
pub struct Choice {
    pub start_content: Option<ContentList>,
    pub option_only_content: Option<ContentList>,
    pub inner_content: ContentList,
    pub condition: Option<Expression>,
    pub once_only: bool,
    pub is_invisible_default: bool,
    pub indentation_depth: usize,
    pub identifier: Option<Identifier>,
    pub has_brackets: bool,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct Gather {
    pub identifier: Option<Identifier>,
    pub depth: usize,
    pub content: Option<ContentList>,
    pub location: SourceLocation,
}
```

`crates/ink-parser/src/ir/content.rs`:
```rust
use crate::ir::base::*;

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
```

`crates/ink-parser/src/ir/divert.rs`:
```rust
use crate::ir::base::*;

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
    Path( InkPath),
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
}

#[derive(Debug, Clone)]
pub struct TunnelOnwards {
    pub divert_after: Option<Box<Divert>>,
    pub location: SourceLocation,
}
```

`crates/ink-parser/src/ir/variable.rs`:
```rust
use crate::ir::base::*;

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

use crate::ir::divert::InkPath;

#[derive(Debug, Clone)]
pub struct ConstDeclaration {
    pub identifier: Identifier,
    pub value: ExpressionValue,
    pub location: SourceLocation,
}
```

`crates/ink-parser/src/ir/expression.rs`:
```rust
use crate::ir::base::*;
use crate::ir::divert::InkPath;

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
    Add, Sub, Mul, Div, Mod,
    Equal, NotEqual, Greater, Less, GreaterEqual, LessEqual,
    And, Or,
    Min, Max,
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

use crate::ir::variable::VariableReference;
```

`crates/ink-parser/src/ir/conditional.rs`:
```rust
use crate::ir::base::*;
use crate::ir::story::StoryNode;

#[derive(Debug, Clone)]
pub struct Conditional {
    pub branches: Vec<ConditionalBranch>,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct ConditionalBranch {
    pub condition: Option<Expression>,
    pub content: Vec<StoryNode>,
    pub is_else: bool,
    pub location: SourceLocation,
}

use crate::ir::expression::Expression;
```

`crates/ink-parser/src/ir/sequence.rs`:
```rust
use crate::ir::base::*;
use crate::ir::content::ContentList;

#[derive(Debug, Clone)]
pub struct Sequence {
    pub elements: Vec<SequenceElement>,
    pub sequence_type: SequenceType,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceType {
    Cycle,
    Shuffle,
    Once,
    Stopping,
}

#[derive(Debug, Clone)]
pub struct SequenceElement {
    pub content: ContentList,
    pub condition: Option<Expression>,
}

use crate::ir::expression::Expression;
```

`crates/ink-parser/src/ir/tag.rs`:
```rust
use crate::ir::base::*;

#[derive(Debug, Clone)]
pub struct Tag {
    pub is_start: bool,
    pub in_choice: bool,
    pub location: SourceLocation,
}
```

`crates/ink-parser/src/ir/list_def.rs`:
```rust
use crate::ir::base::*;

#[derive(Debug, Clone)]
pub struct ListDeclaration {
    pub identifier: Identifier,
    pub items: Vec<ListItemDeclaration>,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct ListItemDeclaration {
    pub name: Identifier,
    pub value: Option<i32>,
}
```

`crates/ink-parser/src/ir/external.rs`:
```rust
use crate::ir::base::*;

#[derive(Debug, Clone)]
pub struct ExternalDeclaration {
    pub identifier: Identifier,
    pub argument_names: Vec<String>,
    pub location: SourceLocation,
}
```

Add test infrastructure to workspace root `Cargo.toml`:

Update `narrative/Cargo.toml`:
```toml
[workspace]
members = [
    "crates/ink-parser",
    "crates/compiler",
    "crates/cli",
]
resolver = "2"
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: PASS — `test_identifier_creation` and `test_source_location_display` pass

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: add IR base types (Identifier, SourceLocation, InkError) and story node stubs"
```

---

### Task 3: Parser Engine — Core Cursor and Error Recovery

**Files:**
- Create: `crates/ink-parser/src/parser.rs`

This is the foundation of the parser: a cursor over the input string with backtracking support, error collection, and the primitive parsing methods (parse_string, parse_int, etc.) that mirror the C# `StringParser`.

- [ ] **Step 1: Write the failing tests for the parser cursor**

Add to `tests/conformance/basic.rs`:
```rust
use ink_parser::parser::Parser;

#[test]
fn test_parser_parse_string_match() {
    let mut p = Parser::new("hello world", "test.ink");
    assert!(p.parse_string("hello").is_some());
    assert_eq!(p.position(), 5);
}

#[test]
fn test_parser_parse_string_no_match() {
    let mut p = Parser::new("hello world", "test.ink");
    assert!(p.parse_string("world").is_none());
    assert_eq!(p.position(), 0); // no advancement on failure
}

#[test]
fn test_parser_parse_int() {
    let mut p = Parser::new("42 rest", "test.ink");
    let val = p.parse_int();
    assert_eq!(val, Some(42));
    assert_eq!(p.position(), 2);
}

#[test]
fn test_parser_parse_int_no_match() {
    let mut p = Parser::new("not a number", "test.ink");
    assert_eq!(p.parse_int(), None);
}

#[test]
fn test_parser_parse_newline() {
    let mut p = Parser::new("line1\nline2", "test.ink");
    assert!(p.parse_string("line1").is_some());
    assert!(p.parse_newline());
    assert!(p.parse_string("line2").is_some());
}

#[test]
fn test_parser_peek_char() {
    let mut p = Parser::new("abc", "test.ink");
    assert_eq!(p.peek(), Some('a'));
    assert_eq!(p.position(), 0); // peek doesn't advance
}

#[test]
fn test_parser_end_of_input() {
    let mut p = Parser::new("ab", "test.ink");
    p.advance(2);
    assert!(p.is_end());
    assert_eq!(p.peek(), None);
}

#[test]
fn test_parser_begin_rule_succeed_fail() {
    let mut p = Parser::new("hello world", "test.ink");
    let rule_id = p.begin_rule();
    assert!(p.parse_string("hello").is_some());
    p.succeed_rule(rule_id);
    assert_eq!(p.position(), 5);
}

#[test]
fn test_parser_begin_rule_rollback() {
    let mut p = Parser::new("hello world", "test.ink");
    let rule_id = p.begin_rule();
    assert!(p.parse_string("hello").is_some());
    p.fail_rule(rule_id);
    assert_eq!(p.position(), 0); // rolled back
}

#[test]
fn test_parser_error_collection() {
    let mut p = Parser::new("test", "test.ink");
    p.error("something went wrong");
    assert_eq!(p.errors().len(), 1);
    assert_eq!(p.errors()[0].message, "something went wrong");
}

#[test]
fn test_parser_whitespace() {
    let mut p = Parser::new("  \t  hello", "test.ink");
    p.parse_whitespace();
    assert_eq!(p.position(), 5);
    assert!(p.parse_string("hello").is_some());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL — `Parser` type doesn't exist yet

- [ ] **Step 3: Implement the parser engine**

Create `crates/ink-parser/src/parser.rs`:
```rust
use crate::ir::base::{InkError, ErrorType, SourceLocation};

/// Core parser engine: a cursor over input text with backtracking,
/// error collection, and primitive parse methods.
pub struct Parser {
    input: String,
    filename: String,
    pos: usize,
    line: usize,
    column: usize,
    errors: Vec<InkError>,

    // Rule stack for backtracking
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

    pub fn advance(&mut self, count: usize) {
        for ch in self.input[self.pos..self.pos + count.min(self.input.len() - self.pos)].chars() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.pos += count;
    }

    fn advance_one(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.advance(1);
        Some(ch)
    }

    pub fn current_source_location(&self) -> SourceLocation {
        SourceLocation::new(&self.filename, self.line, self.column)
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
        // Pop but don't restore — keep current position
        self.rule_stack.pop();
    }

    pub fn fail_rule(&mut self, rule_id: usize) {
        // Restore to state when rule began
        let state = self.rule_stack.get(rule_id).cloned();
        if let Some(s) = state {
            self.pos = s.pos;
            self.line = s.line;
            self.column = s.column;
        }
        // Pop everything from rule_id onward
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
        let has_negative = self.parse_string("-").is_some();
        let digits_start = self.pos;
        while !self.is_end() && self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance(1);
        }
        if self.pos == digits_start {
            if has_negative {
                // No digits after minus, roll back
                self.pos = start;
                self.column -= 1; // rough approximation
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
        let _neg = self.parse_string("-").is_some();
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
            return None;
        }
        let num_str = &self.input[start..self.pos];
        num_str.parse().ok()
    }

    /// Parse a newline character.
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

    /// Parse until end of line.
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

    /// Parse an ink identifier: starts with a letter or underscore,
    /// followed by letters, digits, or underscores.
    /// Also supports Unicode character ranges (like the C# version).
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
        ch.is_ascii_alphabetic() || ch == '_' || ch.is_cjk() || ch.is_hiragana() || ch.is_katakana()
    }

    fn is_identifier_continue(ch: char) -> bool {
        Self::is_identifier_start(ch) || ch.is_ascii_digit()
    }

    // ---- Parsing combinators (to be expanded) ----

    /// Parse optional content, returning None if the rule fails.
    pub fn optional<T>(&mut self, rule: impl FnMut(&mut Parser) -> Option<T>) -> Option<T> {
        rule(self)
    }

    /// Parse content expecting it to succeed; report an error if it doesn't.
    pub fn expect<T>(&mut self, rule: impl FnMut(&mut Parser) -> Option<T>, description: &str) -> Option<T> {
        match rule(self) {
            Some(val) => Some(val),
            None => {
                self.error(&format!("Expected {}", description));
                None
            }
        }
    }
}

// Helper trait for CJK/Hiragana/Katakana detection
trait CharExt {
    fn is_cjk(&self) -> bool;
    fn is_hiragana(&self) -> bool;
    fn is_katakana(&self) -> bool;
}

impl CharExt for char {
    fn is_cjk(&self) -> bool {
        ('\u{4E00}'..='\u{9FFF}').contains(self)
    }
    fn is_hiragana(&self) -> bool {
        ('\u{3040}'..='\u{309F}').contains(self)
    }
    fn is_katakana(&self) -> bool {
        ('\u{30A0}'..='\u{30FF}').contains(self)
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: All tests pass

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement parser engine with cursor, backtracking, and primitive parse methods"
```

---

### Task 4: Content Parsing — Text, Whitespace, Comments

**Files:**
- Create: `crates/ink-parser/src/whitespace.rs`
- Create: `crates/ink-parser/src/content.rs`
- Modify: `crates/ink-parser/src/lib.rs` — add new modules

This task implements the simplest ink features: plain text content, whitespace handling, comment elimination, and the `MixedTextAndLogic` parser. These are the most fundamental building blocks.

- [ ] **Step 1: Write the failing tests**

Add to `tests/conformance/basic.rs`:
```rust
use ink_parser::parser::Parser;
use ink_parser::parse_story;

#[test]
fn test_hello_world() {
    let story = parse_story("Hello world!", "test.ink");
    assert!(!story.has_errors());
    // Story should have one text node: "Hello world!\n"
    assert_eq!(story.content.len(), 1);
}

#[test]
fn test_multiple_lines() {
    let story = parse_story("Hello world!\nHello?\n", "test.ink");
    assert!(!story.has_errors());
    assert_eq!(story.content.len(), 2); // two text lines
}

#[test]
fn test_comment_elimination() {
    let story = parse_story("Hello // this is a comment\nworld!", "test.ink");
    assert!(!story.has_errors());
    // Comment should be stripped; "Hello " + newline + "world!"
}

#[test]
fn test_block_comment() {
    let story = parse_story("Hello /* removed */ world!", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_empty_story() {
    let story = parse_story("", "test.ink");
    // Empty story should parse without errors (may produce no content)
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL — `parse_story` function doesn't exist yet

- [ ] **Step 3: Implement whitespace/comment handling**

Create `crates/ink-parser/src/whitespace.rs`:
```rust
use crate::parser::Parser;

/// Parse a line comment (// ... until newline)
pub fn parse_line_comment(p: &mut Parser) -> Option<()> {
    if p.parse_string("//").is_none() {
        return None;
    }
    // Skip until end of line
    p.parse_until_newline();
    Some(())
}

/// Parse a block comment (/* ... */)
pub fn parse_block_comment(p: &mut Parser) -> Option<()> {
    if p.parse_string("/*").is_none() {
        return None;
    }
    loop {
        if p.is_end() {
            p.error("Unclosed block comment");
            break;
        }
        if p.parse_string("*/").is_some() {
            break;
        }
        p.advance(1);
    }
    Some(())
}

/// Parse a TODO/author warning (not a standard comment — produces author messages)
pub fn parse_author_warning(p: &mut Parser) -> Option<String> {
    p.parse_whitespace();
    if p.parse_string("TODO:").is_none() {
        return None;
    }
    p.parse_whitespace();
    let text = p.parse_until_newline();
    Some(text)
}

/// Parse multiline whitespace (spaces, tabs, newlines, blank lines)
pub fn parse_multiline_whitespace(p: &mut Parser) -> bool {
    let mut found = false;
    loop {
        let had_whitespace = p.parse_whitespace();
        let had_newline = p.parse_newline();
        if !had_whitespace && !had_newline {
            break;
        }
        found = true;
    }
    found
}
```

- [ ] **Step 4: Implement content parsing**

Create `crates/ink-parser/src/content.rs`:
```rust
use crate::parser::Parser;
use crate::ir::base::*;
use crate::ir::content::*;
use crate::ir::story::*;
use crate::whitespace::*;

/// Parse a line of mixed text and logic.
/// This handles plain text, inline expressions { }, glue <>, and diverts.
pub fn parse_line_of_mixed_text_and_logic(p: &mut Parser) -> Option<Vec<ContentItem>> {
    p.parse_whitespace();

    let mut items = Vec::new();

    // Parse mixed content until end of line
    loop {
        if p.is_end() || p.peek() == Some('\n') || p.peek() == Some('\r') {
            break;
        }

        // Try glue
        if p.parse_string("<>").is_some() {
            items.push(ContentItem::Glue);
            continue;
        }

        // Try inline logic { }
        if p.peek() == Some('{') {
            // TODO: parse inline logic — will be implemented in expression task
            // For now, skip the brace content
            break;
        }

        // Try tag #
        if p.peek() == Some('#') {
            // TODO: parse tag — will be implemented in tags task
            break;
        }

        // Plain text — consume characters until we hit a special character
        let text = parse_content_text(p);
        if let Some(t) = text {
            items.push(ContentItem::Text(t));
        } else {
            break;
        }
    }

    if items.is_empty() {
        return None;
    }

    // Trim trailing whitespace from last text
    trim_end_whitespace(&mut items, false);

    // Append newline unless the line is purely tags
    let has_non_tag = items.iter().any(|i| !matches!(i, ContentItem::Tag(_)));
    if has_non_tag {
        items.push(ContentItem::Text(Text::new("\n", p.current_source_location())));
    }

    Some(items)
}

fn parse_content_text(p: &mut Parser) -> Option<Text> {
    let start = p.position();
    let loc = p.current_source_location();
    let mut result = String::new();

    loop {
        if p.is_end() {
            break;
        }
        let ch = p.peek()?;
        // Stop at special characters
        if ch == '{' || ch == '}' || ch == '#' || ch == '\n' || ch == '\r' || ch == '|' {
            break;
        }
        // Handle escape character
        if ch == '\\' {
            p.advance(1);
            if let Some(escaped) = p.parse_single_character() {
                result.push(escaped);
            }
            continue;
        }
        // Handle potential divert arrow or glue
        if ch == '-' || ch == '<' {
            // Check if it's -> or <>
            let remaining = &p.input[p.position()..];
            if remaining.starts_with("->") || remaining.starts_with("<>") {
                break;
            }
        }
        result.push(ch);
        p.advance(1);
    }

    if result.is_empty() {
        None
    } else {
        Some(Text::new(&result, loc))
    }
}

fn trim_end_whitespace(items: &mut Vec<ContentItem>, terminate_with_space: bool) {
    if items.is_empty() {
        return;
    }
    let last_idx = items.len() - 1;
    if let ContentItem::Text(ref mut t) = items[last_idx] {
        t.text = t.text.trim_end_matches(' ').trim_end_matches('\t').to_string();
        if terminate_with_space {
            t.text.push(' ');
        } else if t.text.is_empty() {
            items.remove(last_idx);
            trim_end_whitespace(items, false);
        }
    }
}

/// Parse a simple text-only line (no inline logic).
pub fn parse_simple_text_line(p: &mut Parser) -> Option<StoryNode> {
    let loc = p.current_source_location();
    let items = parse_line_of_mixed_text_and_logic(p)?;

    // Consume end of line
    p.parse_newline();

    Some(StoryNode::Text(Text::new(
        &items.iter().map(|i| match i {
            ContentItem::Text(t) => t.text.clone(),
            ContentItem::Glue => "<>".to_string(),
            _ => String::new(),
        }).collect::<Vec<_>>().join(""),
        loc,
    )))
}
```

- [ ] **Step 5: Implement the top-level parse_story function and statement dispatch**

Create `crates/ink-parser/src/statements.rs`:
```rust
use crate::parser::Parser;
use crate::ir::base::*;
use crate::ir::story::*;
use crate::whitespace::*;

/// Statement level context
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatementLevel {
    InnerBlock,
    Stitch,
    Knot,
    Top,
}

/// Parse statements at a given level
pub fn parse_statements_at_level(p: &mut Parser, level: StatementLevel) -> Vec<StoryNode> {
    let mut content = Vec::new();

    loop {
        p.parse_whitespace();

        if p.is_end() {
            break;
        }

        // Try to parse a statement at this level
        let before_pos = p.position();
        let stmt = parse_statement_at_level(p, level);

        if let Some(node) = stmt {
            content.push(node);
            continue;
        }

        // Try to skip blank lines and whitespace
        if parse_multiline_whitespace(p) {
            continue;
        }

        // Check if we should break for a higher-level construct
        if should_break_for_level(p, level) {
            break;
        }

        // No progress — skip this line and report error
        if p.position() == before_pos {
            let bad_line = p.parse_until_newline();
            p.parse_newline();
            p.error(&format!("Unexpected content: {}", &bad_line[..bad_line.len().min(50)]));
        }
    }

    content
}

fn parse_statement_at_level(p: &mut Parser, level: StatementLevel) -> Option<StoryNode> {
    // Try each statement type in priority order

    // Knot definition (only at top level)
    if level >= StatementLevel::Top {
        // TODO: implement knot parsing (Task 5)
    }

    // Choice
    // TODO: implement choice parsing (Task 6)

    // Author warning / TODO
    let loc = p.current_source_location();
    if let Some(msg) = crate::whitespace::parse_author_warning(p) {
        p.parse_newline();
        return Some(StoryNode::AuthorWarning(msg));
    }

    // Gather (not in inner blocks)
    if level > StatementLevel::InnerBlock {
        // TODO: implement gather parsing (Task 6)
    }

    // Stitch definition (at knot level and above)
    if level >= StatementLevel::Knot {
        // TODO: implement stitch parsing (Task 5)
    }

    // Variable declarations, constants, externals, lists
    // TODO: implement these in logic/variable tasks

    // Logic line (~ ...)
    if p.peek() == Some('~') {
        // TODO: implement logic line parsing (Task 8)
    }

    // Line of mixed text and logic
    let items = crate::content::parse_line_of_mixed_text_and_logic(p)?;
    p.parse_newline();

    // Convert content items to a text StoryNode (simplified for now)
    let text = items.iter().map(|i| match i {
        crate::ir::content::ContentItem::Text(t) => t.text.clone(),
        crate::ir::content::ContentItem::Glue => "<>".to_string(),
        _ => String::new(),
    }).collect::<Vec<_>>().join("");

    Some(StoryNode::Text(crate::ir::content::Text::new(&text, loc)))
}

fn should_break_for_level(p: &mut Parser, level: StatementLevel) -> bool {
    // Check for constructs that break the current level
    if level <= StatementLevel::Knot {
        // Knot declaration (===) breaks current knot/stitch
        if p.input[p.position()..].starts_with("===") {
            return true;
        }
    }
    if level <= StatementLevel::Stitch {
        // Stitch declaration (=) breaks current stitch
        if p.input[p.position()..].starts_with("=") && !p.input[p.position()..].starts_with("==") {
            return true;
        }
    }
    if level <= StatementLevel::InnerBlock {
        if p.peek() == Some('}') {
            return true;
        }
    }
    false
}
```

Update `crates/ink-parser/src/lib.rs`:
```rust
pub mod ir;
pub mod parser;
pub mod whitespace;
pub mod content;
pub mod statements;

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
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: Basic content tests pass

- [ ] **Step 7: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement content parsing, whitespace, comments, and top-level statement dispatch"
```

---

### Task 5: Knot and Stitch Parsing

**Files:**
- Create: `crates/ink-parser/src/knot.rs`
- Modify: `crates/ink-parser/src/statements.rs` — wire in knot/stitch parsing
- Modify: `crates/ink-parser/src/lib.rs` — add module

- [ ] **Step 1: Write the failing tests**

Add to `tests/conformance/basic.rs`:
```rust
#[test]
fn test_knot_definition() {
    let story = parse_story("=== knot_name ===\nHello world.\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_knot_with_stitch() {
    let story = parse_story(
        "=== my_knot ===\nHello.\n= my_stitch\nWorld.\n",
        "test.ink",
    );
    assert!(!story.has_errors());
}

#[test]
fn test_function_definition() {
    let story = parse_story(
        "=== function multiply(x,y) ===\n~ return x * y\n",
        "test.ink",
    );
    assert!(!story.has_errors());
}

#[test]
fn test_multiple_knots() {
    let story = parse_story(
        "=== first ===\nHello.\n=== second ===\nWorld.\n",
        "test.ink",
    );
    assert!(!story.has_errors());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL — knot parsing not implemented yet

- [ ] **Step 3: Implement knot and stitch parsing**

Create `crates/ink-parser/src/knot.rs`:
```rust
use crate::parser::Parser;
use crate::ir::base::*;
use crate::ir::knot::*;
use crate::ir::story::*;
use crate::statements::{parse_statements_at_level, StatementLevel};

/// Parse a knot definition: === knot_name ===
pub fn parse_knot_definition(p: &mut Parser) -> Option<StoryNode> {
    let decl = parse_knot_declaration(p)?;
    p.parse_newline(); // consume end of declaration line

    let content = parse_statements_at_level(p, StatementLevel::Knot);

    Some(StoryNode::Knot(Knot {
        identifier: decl.name,
        content,
        arguments: decl.arguments,
        is_function: decl.is_function,
        location: decl.location,
    }))
}

struct FlowDecl {
    name: Identifier,
    arguments: Vec<FlowArgument>,
    is_function: bool,
    location: SourceLocation,
}

fn parse_knot_declaration(p: &mut Parser) -> Option<FlowDecl> {
    p.parse_whitespace();

    // Must have 2+ equals signs
    let loc = p.current_source_location();
    let equals = parse_equals(p)?;
    if equals < 2 {
        return None;
    }

    p.parse_whitespace();

    // Check for "function" keyword
    let is_function = {
        let id = p.parse_identifier();
        id.as_deref() == Some("function")
    };

    if is_function {
        p.parse_whitespace();
    }

    let name = p.parse_identifier()?;
    let identifier = Identifier::new(&name, p.current_source_location());

    p.parse_whitespace();

    // Parse optional parameters
    let arguments = parse_bracketed_arguments(p).unwrap_or_default();

    p.parse_whitespace();

    // Optional trailing equals
    parse_equals(p);

    Some(FlowDecl {
        name: identifier,
        arguments,
        is_function,
        location: loc,
    })
}

/// Parse stitch definition: = stitch_name
pub fn parse_stitch_definition(p: &mut Parser) -> Option<StoryNode> {
    let decl = parse_stitch_declaration(p)?;
    p.parse_newline();

    let content = parse_statements_at_level(p, StatementLevel::Stitch);

    Some(StoryNode::Knot(Knot {
        // Stitches are represented as Knots with is_function=false at a lower level
        identifier: decl.name,
        content,
        arguments: decl.arguments,
        is_function: decl.is_function,
        location: decl.location,
    }))
}

fn parse_stitch_declaration(p: &mut Parser) -> Option<FlowDecl> {
    p.parse_whitespace();

    let loc = p.current_source_location();

    // Single equals for stitch
    if p.parse_string("=").is_none() {
        return None;
    }
    // If there's a second equals, that's a knot — fail this rule
    if p.peek() == Some('=') {
        // Roll back
        return None;
    }

    p.parse_whitespace();

    // Stitches aren't allowed to be functions, but parse and report error
    let is_function = p.parse_string("function").is_some();
    if is_function {
        p.parse_whitespace();
        p.error("Stitches cannot be functions");
    }

    let name = p.parse_identifier()?;
    let identifier = Identifier::new(&name, p.current_source_location());

    p.parse_whitespace();
    let arguments = parse_bracketed_arguments(p).unwrap_or_default();
    p.parse_whitespace();

    Some(FlowDecl {
        name: identifier,
        arguments,
        is_function,
        location: loc,
    })
}

fn parse_equals(p: &mut Parser) -> Option<usize> {
    let mut count = 0;
    while p.parse_string("=").is_some() {
        count += 1;
    }
    if count > 0 { Some(count) } else { None }
}

fn parse_bracketed_arguments(p: &mut Parser) -> Option<Vec<FlowArgument>> {
    if p.parse_string("(").is_none() {
        return None;
    }

    let mut args = Vec::new();
    loop {
        p.parse_whitespace();
        if p.parse_string(")").is_some() {
            break;
        }
        if !args.is_empty() {
            if p.parse_string(",").is_none() {
                p.error("Expected ',' between arguments");
                break;
            }
            p.parse_whitespace();
        }

        // Parse argument: possibly "ref", possibly "->" for divert target
        let first_id = p.parse_identifier();
        p.parse_whitespace();
        let is_divert = p.parse_string("->").is_some();
        p.parse_whitespace();
        let second_id = p.parse_identifier();

        let arg = if first_id.as_deref() == Some("ref") {
            FlowArgument {
                identifier: Identifier::new(second_id.as_deref().unwrap_or(""), p.current_source_location()),
                is_by_reference: true,
                is_divert_target: is_divert,
            }
        } else if is_divert {
            FlowArgument {
                identifier: Identifier::new(second_id.as_deref().unwrap_or(""), p.current_source_location()),
                is_by_reference: false,
                is_divert_target: true,
            }
        } else {
            FlowArgument {
                identifier: Identifier::new(first_id.as_deref().unwrap_or(""), p.current_source_location()),
                is_by_reference: false,
                is_divert_target: false,
            }
        };
        args.push(arg);
    }

    Some(args)
}

/// Check if current position starts a knot declaration
pub fn at_knot_declaration(p: &Parser) -> bool {
    let remaining = &p.input[p.position()..];
    remaining.starts_with("==")
}
```

Wire knot parsing into `statements.rs` — update `parse_statement_at_level` to call `knot::parse_knot_definition` and `knot::parse_stitch_definition` at appropriate levels.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: Knot and stitch tests pass

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement knot and stitch definition parsing"
```

---

### Task 6: Choice and Gather Parsing

**Files:**
- Create: `crates/ink-parser/src/choices.rs`
- Modify: `crates/ink-parser/src/statements.rs` — wire in choice/gather parsing

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn test_basic_choice() {
    let story = parse_story("* Hello world\n- After choice\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_sticky_choice() {
    let story = parse_story("+ Hello again\n- Repeated\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_choice_with_brackets() {
    let story = parse_story("* Hello[.] world\n- Continued\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_nested_choices() {
    let story = parse_story(
        "* First choice\n  ** Second choice\n  -- After second\n- After first\n",
        "test.ink",
    );
    assert!(!story.has_errors());
}

#[test]
fn test_gather() {
    let story = parse_story("- Gathered text\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_conditional_choice() {
    let story = parse_story("{true} * Conditional choice\n- After\n", "test.ink");
    // This test will need expression parsing — may defer
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL

- [ ] **Step 3: Implement choice and gather parsing**

Create `crates/ink-parser/src/choices.rs` with the full choice parsing logic mirroring `InkParser_Choices.cs`: bullet parsing (*, +), bracket notation ([...]), start/option/inner content splitting, gather dashes, named choices with (name).

The key complexity:
- Bullets (`*` for once-only, `+` for sticky) determine indentation depth
- `[bracket]` notation splits choice text into start content, option-only content, and inner content
- Gathers use `-` dashes (but not `->` which is a divert)
- Named choices use `(name)` after the bullet

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: Choice and gather tests pass

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement choice and gather parsing with bracket notation"
```

---

### Task 7: Divert, Tunnel, Thread Parsing

**Files:**
- Create: `crates/ink-parser/src/divert.rs`
- Modify: `crates/ink-parser/src/statements.rs` — wire in divert parsing

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn test_basic_divert() {
    let story = parse_story("-> target\n=== target ===\nHello.\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_tunnel() {
    let story = parse_story("-> tunnel ->\nDone.\n=== tunnel ===\nInside\n->->\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_tunnel_onwards() {
    let story = parse_story("->->\n", "test.ink");
    // ->-> at top level is an error, but should parse
}

#[test]
fn test_thread() {
    let story = parse_story("<- concurrent_thread\n=== concurrent_thread ===\nThread content\n-> DONE\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_divert_to_done() {
    let story = parse_story("-> DONE\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_divert_to_end() {
    let story = parse_story("-> END\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_variable_divert_target() {
    let story = parse_story("VAR target = -> somewhere\n=== somewhere ===\nHello\n-> END\n", "test.ink");
    // Needs variable parsing first
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL

- [ ] **Step 3: Implement divert parsing**

Create `crates/ink-parser/src/divert.rs` mirroring `InkParser_Divert.cs`:
- Multi-divert: `-> target`, `-> tunnel ->`, `->->` tunnel onwards
- Thread: `<- target`
- Divert paths: dot-separated identifiers
- Function call arguments on diverts

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: Divert tests pass

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement divert, tunnel, and thread parsing"
```

---

### Task 8: Variables, Logic, and Expressions

**Files:**
- Create: `crates/ink-parser/src/logic.rs`
- Create: `crates/ink-parser/src/expressions.rs`
- Modify: `crates/ink-parser/src/statements.rs` — wire in logic/expression parsing

This is the most complex parser task. The expression parser handles arithmetic, boolean logic, function calls, string literals, ternary operators, and ink-specific features like divert targets and list operations.

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn test_variable_declaration() {
    let story = parse_story("VAR x = 5\n{x}\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_temp_variable() {
    let story = parse_story("=== knot ===\n~ temp y = 10\n{y}\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_arithmetic() {
    let story = parse_story("{ 2 * 3 + 5 * 6 }\n", "test.ink");
    // Expression parsing must handle operator precedence
}

#[test]
fn test_string_literal() {
    let story = parse_story("VAR x = \"Hello world\"\n{x}\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_function_call() {
    let story = parse_story(
        "=== function add(a,b) ===\n~ return a + b\n{add(1,2)}\n",
        "test.ink",
    );
    assert!(!story.has_errors());
}

#[test]
fn test_const_declaration() {
    let story = parse_story("CONST PI = 3\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_increment() {
    let story = parse_story("VAR x = 0\n~ x++\n~ x--\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_external_declaration() {
    let story = parse_story("EXTERNAL playSound(name)\n", "test.ink");
    assert!(!story.has_errors());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL

- [ ] **Step 3: Implement expression parser**

Create `crates/ink-parser/src/expressions.rs` — a recursive descent expression parser with:
- Operator precedence (following C# `InkParser_Expressions.cs`)
- Binary operators: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `>`, `<`, `>=`, `<=`, `&&`, `||`
- Unary operators: `-`, `!`
- Ternary: `? :`
- Function calls: `func(arg1, arg2)`
- String literals: `"hello"`
- List operations: `LIST_COUNT`, `LIST_MIN`, `LIST_MAX`, `LIST_ALL`, `LIST_INVERT`, `LIST_RANDOM`
- Type coercion functions: `int()`, `float()`, `string()`

Create `crates/ink-parser/src/logic.rs` for:
- `~` logic lines (variable assignment, function calls)
- `VAR` global variable declarations
- `CONST` constant declarations
- `~ temp` temporary variable declarations
- `EXTERNAL` declarations
- Increment/decrement (`++`, `--`)

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: Variable, logic, and expression tests pass

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement expression parser with operator precedence and logic lines"
```

---

### Task 9: Conditionals

**Files:**
- Create: `crates/ink-parser/src/conditionals.rs`

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn test_basic_if() {
    let story = parse_story("{ x > 5: Big }\n", "test.ink");
    // Inline conditional
}

#[test]
fn test_if_else() {
    let story = parse_story("{ x > 5: Big | else: Small }\n", "test.ink");
}

#[test]
fn test_multiline_conditional() {
    let story = parse_story(
        "{ x > 5:\n  Big\n- else:\n  Small\n}\n",
        "test.ink",
    );
    assert!(!story.has_errors());
}

#[test]
fn test_conditional_choices() {
    let story = parse_story(
        "{ x > 5:\n  * Big choice\n}\n",
        "test.ink",
    );
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL

- [ ] **Step 3: Implement conditional parsing**

Create `crates/ink-parser/src/conditionals.rs` mirroring `InkParser_Conditional.cs`:
- Inline conditionals: `{ condition: content | alternative }`
- Multiline conditionals with `{`, `-`, `}` delimiters
- Else-if chains
- Conditional choices

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: Conditional tests pass

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement inline and multiline conditional parsing"
```

---

### Task 10: Sequences

**Files:**
- Create: `crates/ink-parser/src/sequences.rs`

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn test_cycle_sequence() {
    let story = parse_story("{! Red | Blue | Green }\n", "test.ink");
}

#[test]
fn test_stopping_sequence() {
    let story = parse_story("{ Red | Blue | Green }\n", "test.ink");
}

#[test]
fn test_shuffle_sequence() {
    let story = parse_story("{~ Red | Blue | Green }\n", "test.ink");
}

#[test]
fn test_once_sequence() {
    let story = parse_story("{& Red | Blue | Green }\n", "test.ink");
}

#[test]
fn test_sequence_with_blank() {
    let story = parse_story("{ Red || Blue }\n", "test.ink");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL

- [ ] **Step 3: Implement sequence parsing**

Create `crates/ink-parser/src/sequences.rs` mirroring `InkParser_Sequences.cs`:
- Cycle `!`, Shuffle `~`, Once `&`, Stopping (default)
- Pipe-separated elements
- Blank elements (double pipe)
- Conditional elements within sequences

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: Sequence tests pass

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement sequence parsing (cycle, shuffle, once, stopping)"
```

---

### Task 11: Tags

**Files:**
- Create: `crates/ink-parser/src/tags.rs` (parser-level tag handling)

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn test_line_tag() {
    let story = parse_story("Hello world # my_tag\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_multiple_tags() {
    let story = parse_story("Hello # tag1 # tag2\n", "test.ink");
}

#[test]
fn test_knot_tag() {
    let story = parse_story("=== my_knot ===\n# location: Germany\nHello.\n", "test.ink");
}

#[test]
fn test_above_line_tag() {
    let story = parse_story("# a tag\nThis is the line.\n", "test.ink");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL

- [ ] **Step 3: Implement tag parsing**

Wire tag handling into content parsing and statement dispatch. Tags use `#` prefix and are stored as string metadata attached to content nodes.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: Tag tests pass

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement tag parsing (# prefix, line and knot tags)"
```

---

### Task 12: Includes and Multi-File Support

**Files:**
- Create: `crates/ink-parser/src/include.rs`
- Modify: `crates/ink-parser/src/lib.rs` — add file handler abstraction

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn test_include_statement() {
    // Requires a file handler that can resolve includes
    let story = parse_story("INCLUDE other_file.ink\nHello.\n", "test.ink");
    // Include should be parsed; resolution depends on file handler
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL

- [ ] **Step 3: Implement include parsing and file handler**

Create `crates/ink-parser/src/include.rs`:
- Parse `INCLUDE filename.ink` statements
- Define a `FileHandler` trait for resolving and reading included files
- Implement `DefaultFileHandler` for filesystem-based resolution
- Implement `MemoryFileHandler` for testing (files stored in a HashMap)

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: Include tests pass

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement INCLUDE parsing and file handler abstraction"
```

---

### Task 13: List Declarations and Operations

**Files:**
- Create: `crates/ink-parser/src/lists.rs`

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn test_list_declaration() {
    let story = parse_story("LIST colors = red, green, blue\n", "test.ink");
    assert!(!story.has_errors());
}

#[test]
fn test_list_assignment() {
    let story = parse_story("LIST colors = red, green, blue\nVAR x = (red)\n", "test.ink");
}

#[test]
fn test_list_operations() {
    let story = parse_story(
        "LIST colors = red, green, blue\n~ x = (red, green)\n~ y = x - (red)\n",
        "test.ink",
    );
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL

- [ ] **Step 3: Implement list parsing**

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: List tests pass

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement LIST declarations and list operations parsing"
```

---

### Task 14: Runtime Types and Code Generation

**Files:**
- Create: `crates/compiler/src/runtime_types.rs`
- Create: `crates/compiler/src/codegen.rs`

This is where we convert the parsed IR into the ink runtime format — the hierarchy of containers, values, and control commands that gets serialized to JSON.

- [ ] **Step 1: Write the failing test**

```rust
use narrative_compiler::compile_string;

#[test]
fn test_codegen_hello_world() {
    let json = compile_string("Hello world!", None).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["inkVersion"], 21);
    assert!(parsed["root"].is_array());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL

- [ ] **Step 3: Implement runtime types**

Create `crates/compiler/src/runtime_types.rs` mirroring the C# runtime types:
- `RuntimeContainer` (array + named content dictionary + flags)
- `RuntimeValue` (string with `^` prefix, int, float, divert target, variable pointer, void)
- `RuntimeControlCommand` (all command types from the spec)
- `RuntimeDivert` (standard, function call, tunnel, external, variable target)
- `RuntimeChoicePoint` (with flags)
- `RuntimeVariableAssignment` (global vs temp, new vs reassignment)
- `RuntimeVariableReference`
- `RuntimeNativeFunctionCall`
- `RuntimePath`

- [ ] **Step 4: Implement code generation**

Create `crates/compiler/src/codegen.rs` — walk the `ParsedStory` IR and produce the runtime hierarchy:
- `Text` → `RuntimeValue::String`
- `Knot/Stitch` → `RuntimeContainer` with named content
- `Choice` → `RuntimeContainer` with `RuntimeChoicePoint`
- `Divert` → `RuntimeDivert`
- `VariableAssignment` → `RuntimeVariableAssignment`
- `Expression` → evaluation stack operations (`ev`/`/ev` blocks)
- `Conditional` → branch containers with diverts
- `Sequence` → shuffled/sequential containers
- `Tag` → `BeginTag`/`EndTag` control commands

Each parsed node's `GenerateRuntimeObject` equivalent produces the runtime objects.

- [ ] **Step 5: Implement reference resolution**

Create `crates/compiler/src/resolve.rs` — second pass that:
- Resolves divert target names to runtime paths
- Resolves variable references
- Validates that all divert targets exist
- Reports unresolved references as errors

- [ ] **Step 6: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: Codegen test passes, produces valid JSON structure

- [ ] **Step 7: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement runtime types, code generation, and reference resolution"
```

---

### Task 15: JSON Output — Ink Runtime Format

**Files:**
- Create: `crates/compiler/src/json_output.rs`

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn test_json_output_structure() {
    let json = compile_string("Hello world!", None).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["inkVersion"], 21);
    // Root should be an array (container)
    assert!(parsed["root"].is_array());
    // First element should be a container containing the text
    let root = &parsed["root"];
    // Find the string value with ^ prefix
    let found_text = root.as_array().unwrap().iter().any(|item| {
        item.as_str().map_or(false, |s| s.starts_with("^Hello world"))
    });
    assert!(found_text);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd narrative && cargo test`
Expected: FAIL

- [ ] **Step 3: Implement JSON serialization**

Create `crates/compiler/src/json_output.rs` — serialize the runtime hierarchy into the ink JSON format documented in `ink_JSON_runtime_format.md`:
- Containers → arrays with optional trailing dictionary for named content/flags
- String values → `^` prefix
- Control commands → string identifiers (`"ev"`, `"/ev"`, `"out"`, etc.)
- Diverts → objects with `->`, `f()`, `->t->`, `x()` keys
- Variable operations → objects with `VAR=`, `temp=`, `VAR?` keys
- ChoicePoints → objects with `*` and `flg` keys
- Native functions → string identifiers (`"+"`, `"-"`, etc.)

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd narrative && cargo test`
Expected: JSON output test passes

- [ ] **Step 5: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement ink JSON runtime format serialization (version 21)"
```

---

### Task 16: Conformance Test Suite — Port from C#

**Files:**
- Create: `tests/conformance/choices.rs`
- Create: `tests/conformance/diverts.rs`
- Create: `tests/conformance/variables.rs`
- Create: `tests/conformance/conditionals.rs`
- Create: `tests/conformance/functions.rs`
- Create: `tests/conformance/lists.rs`
- Create: `tests/conformance/sequences.rs`
- Create: `tests/conformance/glue.rs`
- Create: `tests/conformance/tags.rs`
- Create: `tests/conformance/includes.rs`
- Create: `tests/conformance/multi_flow.rs`
- Create: `tests/conformance/save_load.rs`
- Create: `tests/conformance/error_checks.rs`

This is the critical validation gate: port the 180 C# tests and verify the Rust compiler produces the same output.

- [ ] **Step 1: Create test helper infrastructure**

Create a test helper that compiles an ink string and returns the story output (text + choices):

```rust
/// Compile an ink string and return the full text output
fn compile_and_run(ink_source: &str) -> String {
    let json = narrative_compiler::compile_string(ink_source, None).unwrap();
    // Use a minimal Rust runtime to execute the compiled story
    // (or compare JSON output directly with C# compiler output)
    todo!("Implement after runtime is available or use JSON comparison")
}
```

**Important decision:** At this stage, we don't have a Rust ink runtime. We have two options:
1. **JSON comparison** — Compile with both C# and Rust compilers, compare the JSON outputs semantically. This validates the compiler without needing a runtime.
2. **Build a minimal runtime** — Implement enough of the ink runtime in Rust to execute stories and compare text output.

Option 1 is faster for conformance testing. Option 2 is needed eventually (for Phase 5: Godot SDK). For Phase 1, we'll use JSON comparison.

- [ ] **Step 2: Port the first 30 core tests (basic content, choices, diverts)**

Each test follows the pattern:
1. Define ink source
2. Compile with Rust compiler
3. Compare JSON output with C# compiler output (or validate structure)

Port from `tests/Tests.cs`:
- TestHelloWorld
- TestEmpty
- TestBasicStringLiterals
- TestArithmetic
- TestBasicTunnel
- TestCommentEliminator
- TestChoiceCount
- TestChoiceDivertsToDone
- TestConditionalChoices
- TestConditionals
- TestConst
- TestDefaultChoices
- TestDivertNotFoundError
- TestEnd
- TestExternalBinding
- TestFactorialRecursive
- TestHelloWorld
- TestImplicitInlineGlue
- TestInclude
- TestLogicInChoices
- TestMultipleConstantReferences
- TestOnceOnlyChoicesCanLinkBackToSelf
- TestPaths
- TestPrintNum
- TestSimpleGlue
- TestStringsInChoices
- TestTags
- TestVariableGetSetAPI
- TestWeaveGathers
- TestWeaveOptions

- [ ] **Step 3: Run tests and fix failures iteratively**

Run: `cd narrative && cargo test conformance`
Expected: Some pass, some fail. Fix the parser/codegen for each failure.

- [ ] **Step 4: Port remaining 150 tests in batches**

Port in batches of ~30, fixing issues as they arise. Group by feature area for efficiency.

- [ ] **Step 5: Achieve full test suite pass rate**

Run: `cd narrative && cargo test conformance`
Expected: All 180 tests pass

- [ ] **Step 6: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: port full C# test suite (180 tests) as conformance tests"
```

---

### Task 17: CLI — Compile and Play Modes

**Files:**
- Modify: `crates/cli/src/main.rs`

- [ ] **Step 1: Implement CLI with clap**

```rust
use clap::{Parser, Subcommand};
use narrative_compiler::{compile_string, CompileOptions};

#[derive(Parser)]
#[command(name = "narrative", version, about = "Ink narrative compiler")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile an ink file to JSON
    Compile {
        /// Input .ink file
        input: String,
        /// Output JSON file (default: same name .ink.json)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Compile and play an ink file interactively
    Play {
        /// Input .ink file
        input: String,
    },
    /// Check an ink file for errors without compiling
    Check {
        /// Input .ink file
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Compile { input, output } => {
            let source = std::fs::read_to_string(&input).expect("Failed to read input file");
            let options = CompileOptions {
                source_filename: Some(input.clone()),
                ..Default::default()
            };
            match compile_string(&source, Some(options)) {
                Ok(json) => {
                    let out_path = output.unwrap_or_else(|| input + ".json");
                    std::fs::write(&out_path, json).expect("Failed to write output");
                }
                Err(errors) => {
                    for e in &errors {
                        eprintln!("{}", e);
                    }
                    std::process::exit(1);
                }
            }
        }
        Commands::Play { input } => {
            // Requires a runtime — will be basic at first
            eprintln!("Play mode not yet implemented");
            std::process::exit(1);
        }
        Commands::Check { input } => {
            let source = std::fs::read_to_string(&input).expect("Failed to read input file");
            let options = CompileOptions {
                source_filename: Some(input.clone()),
                ..Default::default()
            };
            match compile_string(&source, Some(options)) {
                Ok(_) => println!("No errors found."),
                Err(errors) => {
                    for e in &errors {
                        eprintln!("{}", e);
                    }
                    std::process::exit(1);
                }
            }
        }
    }
}
```

- [ ] **Step 2: Build and test the CLI**

Run: `cd narrative && cargo build --release`
Run: `./target/release/narrative compile test.ink`
Expected: Produces `test.ink.json`

- [ ] **Step 3: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: implement CLI with compile, play, and check subcommands"
```

---

### Task 18: Final Integration Test — Round-Trip with C# Runtime

**Files:**
- Create: `tests/integration/roundtrip.rs`

This is the ultimate validation: compile with Rust, run with the C# runtime (or inkjs), verify the output matches what the C# compiler produces.

- [ ] **Step 1: Write integration test comparing Rust and C# JSON output**

```rust
#[test]
fn test_roundtrip_hello_world() {
    let ink = "Hello world!";
    let rust_json = narrative_compiler::compile_string(ink, None).unwrap();
    // Parse and validate structure
    let parsed: serde_json::Value = serde_json::from_str(&rust_json).unwrap();
    assert_eq!(parsed["inkVersion"], 21);
}

#[test]
fn test_roundtrip_complex_story() {
    let ink = r#"
=== intro ===
Hello, world!
* [Go to town]
    -> town
* [Stay home]
    -> END

=== town ===
You arrive at the town square.
"#;
    let rust_json = narrative_compiler::compile_string(ink, None).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&rust_json).unwrap();
    // Verify knots exist in named content
    assert!(parsed["root"].is_array());
}
```

- [ ] **Step 2: Run and verify all conformance tests still pass**

Run: `cd narrative && cargo test`
Expected: All tests pass

- [ ] **Step 3: Commit**

```bash
cd narrative
git add -A
git commit -m "feat: add round-trip integration tests validating Rust compiler output"
```

---

## Self-Review

**1. Spec coverage:** This plan covers Phase 1 of the spec — the Rust ink compiler that produces compatible ink JSON. Phases 2-5 (definitions parser, directives, LSP, Godot SDK) will each get their own plan when Phase 1 is complete.

**2. Placeholder scan:** Tasks 6-16 contain some abbreviated implementation steps ("implement choice parsing mirroring InkParser_Choices.cs") rather than full inline code. This is because these tasks each involve 200-400 lines of parser code that would make the plan unreadably long. The key structures and test cases are specified; the implementation must follow the C# reference. Future plan revisions can expand these into fully-detailed steps.

**3. Type consistency:** All IR types are defined in Task 2 and referenced consistently across subsequent tasks. The `StoryNode` enum, `ParsedStory` struct, and `Parser` cursor API are established early and used throughout.

---

Plan complete and saved to `docs/superpowers/plans/2026-06-21-phase1-rust-ink-compiler.md`. Two execution options:

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

Which approach?
