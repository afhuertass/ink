# Narrative — Ink Compiler & Story Framework

A Rust rewrite of the [ink](https://github.com/inkle/ink) narrative scripting compiler, extended with a structured directive system for story-driven/text games.

## Status

**Phase 1 complete** — the Rust ink compiler produces ink JSON (version 21) compatible with existing runtimes (inkjs, ink-unity-integration). 160 tests passing.

## Quick Start

```bash
# Build
cargo build --release

# Compile ink to JSON
narrative compile story.ink
narrative compile story.ink --pretty
narrative compile story.ink -o output.json

# Check for errors
narrative check story.ink
narrative check story.ink --warnings

# Validate definitions
narrative check-defs story.inkdef.yaml

# Play interactively (via inkjs/Node.js)
narrative play story.ink

# Version
narrative --version
```

## Project Structure

```
narrative/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── ink-parser/               # Ink language parser (IR generation)
│   │   ├── src/
│   │   │   ├── lib.rs            #   Public API: parse_story()
│   │   │   ├── parser.rs         #   Parser engine (cursor, backtracking, primitives)
│   │   │   ├── statements.rs     #   Top-level statement dispatch
│   │   │   ├── content.rs        #   Text lines, mixed content parsing
│   │   │   ├── whitespace.rs     #   Whitespace, line/block comments
│   │   │   ├── knot.rs           #   Knot (===) and stitch (==) definitions
│   │   │   ├── choices.rs        #   Choice (*) and gather (-) parsing
│   │   │   ├── divert.rs         #   Diverts (->), tunnels (->->), threads (<-)
│   │   │   ├── logic.rs          #   Logic lines (~), VAR, CONST, EXTERNAL
│   │   │   ├── conditionals.rs   #   Inline {cond: | else} and multiline conditionals
│   │   │   ├── sequences.rs      #   {!cycle | ~shuffle | &once | stopping} sequences
│   │   │   ├── tags.rs           #   # tag parsing
│   │   │   ├── include.rs        #   INCLUDE statement + FileHandler trait
│   │   │   ├── lists.rs          #   LIST declarations with values
│   │   │   └── ir/               #   Intermediate representation types
│   │   │       ├── mod.rs        #     Re-exports all IR types
│   │   │       ├── base.rs       #     SourceLocation, Identifier, InkError
│   │   │       ├── story.rs      #     ParsedStory, StoryNode enum
│   │   │       ├── knot.rs       #     Knot, Stitch, FlowArgument
│   │   │       ├── choice.rs     #     Choice, Gather
│   │   │       ├── content.rs    #     Text, ContentList, ContentItem
│   │   │       ├── divert.rs     #     Divert, InkPath, DivertTarget, PathComponent
│   │   │       ├── expression.rs #     Expression, ExpressionKind, BinaryOp, UnaryOp, etc.
│   │   │       ├── conditional.rs#     Conditional, ConditionalBranch
│   │   │       ├── sequence.rs   #     Sequence, SequenceType, SequenceElement
│   │   │       ├── tag.rs        #     Tag
│   │   │       ├── variable.rs   #     VariableAssignment, VariableReference, ConstDeclaration
│   │   │       ├── list_def.rs   #     ListDeclaration, ListItemDeclaration
│   │   │       └── external.rs   #     ExternalDeclaration
│   │   └── tests/                #   80 unit tests
│   │       ├── basic.rs
│   │       ├── parser.rs
│   │       ├── story_parsing.rs
│   │       ├── knot.rs
│   │       ├── choices.rs
│   │       ├── divert.rs
│   │       ├── logic.rs
│   │       └── features.rs
│   │
│   ├── def-parser/               # Definitions parser (.inkdef.yaml)
│   │   ├── src/
│   │   │   ├── lib.rs            #   Public API: parse_definitions()
│   │   │   ├── types.rs          #   Definitions, Asset, Character, Scene, Action, StateVar, Event
│   │   │   ├── validate.rs       #   Cross-reference validation
│   │   │   └── error.rs          #   DefinitionError types
│   │   └── tests/
│   │       ├── parse_tests.rs    #   YAML deserialization tests
│   │       ├── validation_tests.rs # Validation error tests
│   │       └── integration.rs    # End-to-end integration tests
│   │
│   ├── compiler/                 # Compiler (IR → runtime objects → JSON)
│   │   ├── src/
│   │   │   ├── lib.rs            #   Public API: compile_ink()
│   │   │   ├── codegen.rs        #   ParsedStory → Story conversion
│   │   │   ├── runtime_types.rs  #   Story, Container, Divert, ChoicePoint, Tag, etc.
│   │   │   ├── json_output.rs    #   JSON serialization (ink runtime format v21)
│   │   │   ├── resolve.rs        #   Path resolution (stub)
│   │   │   └── error.rs          #   Error types (stub)
│   │   └── tests/                #   86 tests
│   │       ├── conformance.rs    #     64 compile-time conformance tests
│   │       └── integration.rs    #     16 end-to-end integration tests
│   │
│   └── cli/                      # Command-line interface
│       └── src/
│           └── main.rs           #   compile, check, play subcommands
```

## Compilation Pipeline

```
ink source (.ink)
    │
    ▼
┌──────────────┐
│  ink-parser   │  parse_story(source, filename) → ParsedStory
└──────────────┘
    │                    IR (StoryNode tree)
    ▼
┌──────────────┐
│   compiler    │  codegen::compile(parsed) → Story
└──────────────┘
    │                    Runtime objects
    ▼
┌──────────────┐
│  json_output  │  story_to_json(story) → JSON string
└──────────────┘
    │                    ink JSON (version 21)
    ▼
  ink runtime (inkjs, ink-unity, etc.)
```

## Ink Syntax Support

| Feature | Syntax | Status |
|---------|--------|--------|
| Text content | `Hello, world!` | ✅ |
| Knots | `=== knot ===` | ✅ |
| Stitches | `== stitch ==` | ✅ |
| Choices (once-only) | `* Choice text` | ✅ |
| Choices (sticky) | `+ Choice text` | ✅ |
| Bracket notation | `* Hello[.] World` | ✅ |
| Named choices | `* (name) Choice` | ✅ |
| Gathers | `- Gather text` | ✅ |
| Diverts | `-> target` | ✅ |
| Tunnels | `-> tunnel ->` / `->->` | ✅ |
| Threads | `<- knot` | ✅ |
| Variables | `VAR x = 5` | ✅ |
| Constants | `CONST MAX = 100` | ✅ |
| Temp variables | `~ temp x = 5` | ✅ |
| Assignments | `~ x = 5`, `~ x++`, `~ x--` | ✅ |
| External functions | `EXTERNAL greet(name)` | ✅ |
| Expressions | `{ 2 + 3 }`, `{ x > 5 }` | ✅ |
| Inline conditionals | `{ true: Yes \| No }` | ✅ |
| Multiline conditionals | `{ - content }` | ✅ |
| Cycle sequences | `{! A \| B \| C }` | ✅ |
| Shuffle sequences | `{~ A \| B \| C }` | ✅ |
| Once sequences | `{& A \| B \| C }` | ✅ |
| Stopping sequences | `{ A \| B \| C }` | ✅ |
| Tags | `# tag_name` | ✅ |
| List declarations | `LIST colors = red, green, blue` | ✅ |
| Includes | `INCLUDE other.ink` | ✅ |
| Comments | `// line`, `/* block */` | ✅ |
| Glue | `A~ B` | ✅ |
| Function knots | `=== function add(a,b) ===` | ✅ |

## Known Limitations

- **Knot separation after choices**: Knots defined after choice/gather blocks may be merged into the preceding knot's container. This is a parser bug that needs fixing.
- **JSON runtime compatibility**: The compiler produces structurally valid ink JSON but it's not yet fully runtime-compatible — inkjs can execute simple stories but not complex ones with choices, diverts, or conditionals.
- **Tag text not stored**: The IR `Tag` struct has `is_start`/`in_choice`/`location` but doesn't preserve the actual tag text content.
- **Expression codegen**: Binary operators, unary operators, and function calls are parsed but the codegen for them needs refinement to match the exact ink runtime format.

## Roadmap

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Rust ink compiler (compatible JSON output) | ✅ Complete |
| 2 | Definitions parser (`.inkdef.yaml`) | ✅ Complete |
| 3 | Directives (`@` prefix) + validation | Planned |
| 4 | LSP with validation + autocompletion | Planned |
| 5 | Godot SDK | Planned |

## Design Documents

- **Spec**: `docs/superpowers/specs/2026-06-21-narrative-framework-design.md`
- **Implementation plan**: `docs/superpowers/plans/2026-06-21-phase1-rust-ink-compiler.md`

## Reference Sources

- **C# ink parser**: `compiler/InkParser/` (16 parser modules)
- **C# runtime types**: `ink-engine-runtime/`
- **C# test suite**: `tests/Tests.cs` (180 tests, ~4259 lines)
- **Ink JSON format**: `Documentation/ink_JSON_runtime_format.md`

## License

This project is a Rust rewrite of [inkle/ink](https://github.com/inkle/ink), which is licensed under the MIT License.
