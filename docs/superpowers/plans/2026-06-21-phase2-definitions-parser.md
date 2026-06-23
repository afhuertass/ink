# Phase 2: Definitions Parser Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Parse and validate `.inkdef.yaml` definition files, producing a typed `Definitions` struct with internal cross-reference validation and clear error diagnostics.

**Architecture:** New `def-parser` crate using `serde_yaml` for deserialization into typed Rust structs. A separate validation pass walks the `Definitions` and checks referential integrity (character portrait → image asset, scene background → image asset, etc.). Errors reuse the existing `InkError` / `SourceLocation` pattern from `ink-parser`.

**Tech Stack:** Rust, `serde` + `serde_yaml` for deserialization, existing `InkError` type for diagnostics.

---

## File Structure

```
narrative/
├── crates/
│   ├── def-parser/                    ← NEW CRATE
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                 Public API: parse_definitions(), Definitions
│   │       ├── types.rs               All definition types (Asset, Character, Scene, etc.)
│   │       ├── validate.rs            Internal cross-reference validation
│   │       └── error.rs               Definition-specific errors
│   │
│   ├── compiler/                      ← MODIFY
│   │   ├── src/lib.rs                 Add compile_with_definitions()
│   │   └── Cargo.toml                 Add def-parser dependency
│   │
│   └── cli/                           ← MODIFY
│       └── src/main.rs                Add check-defs subcommand
```

---

### Task 1: Scaffold `def-parser` crate

**Files:**
- Create: `crates/def-parser/Cargo.toml`
- Create: `crates/def-parser/src/lib.rs`
- Modify: `Cargo.toml` (workspace members)

- [ ] **Step 1: Create the crate directory and Cargo.toml**

```toml
[package]
name = "def-parser"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
```

- [ ] **Step 2: Create lib.rs with stub API**

```rust
//! Definitions parser for `.inkdef.yaml` files.

pub mod types;
pub mod validate;
pub mod error;

use types::Definitions;
use error::DefinitionError;

/// Parse a `.inkdef.yaml` source string into a Definitions struct.
/// Returns the parsed definitions and any errors found during validation.
pub fn parse_definitions(source: &str, filename: &str) -> Result<Definitions, Vec<DefinitionError>> {
    let defs: Definitions = serde_yaml::from_str(source)
        .map_err(|e| vec![DefinitionError::YamlError {
            message: e.to_string(),
            filename: filename.to_string(),
        }])?;
    let errors = validate::validate(&defs, filename);
    if errors.iter().any(|e| e.is_error()) {
        return Err(errors);
    }
    Ok(defs)
}
```

- [ ] **Step 3: Add to workspace Cargo.toml**

Add `"crates/def-parser"` to the `members` array in `narrative/Cargo.toml`.

- [ ] **Step 4: Build and verify it compiles**

Run: `cargo build`
Expected: Compiles (with warnings about unused stubs)

- [ ] **Step 5: Commit**

```bash
git add crates/def-parser/ Cargo.toml
git commit -m "feat: scaffold def-parser crate"
```

---

### Task 2: Definition types (core structs)

**Files:**
- Create: `crates/def-parser/src/types.rs`
- Create: `crates/def-parser/src/error.rs`

- [ ] **Step 1: Write the error types**

Create `error.rs`:

```rust
use std::fmt;

/// A definition validation error.
#[derive(Debug, Clone)]
pub enum DefinitionError {
    YamlError {
        message: String,
        filename: String,
    },
    UndefinedReference {
        kind: String,      // "character", "asset", etc.
        name: String,      // the undefined name
        referenced_by: String, // "scene 'haunted_manor'.background"
        filename: String,
    },
    TypeMismatch {
        expected: String,  // "image"
        actual: String,    // "audio"
        name: String,
        referenced_by: String,
        filename: String,
    },
    MissingRequiredField {
        definition_kind: String,
        definition_name: String,
        field: String,
        filename: String,
    },
    DuplicateName {
        kind: String,
        name: String,
        filename: String,
    },
    InvalidSchemaVersion {
        version: i64,
        filename: String,
    },
}

impl DefinitionError {
    pub fn is_error(&self) -> bool {
        true // All definition errors are errors for now
    }
}

impl fmt::Display for DefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DefinitionError::YamlError { message, filename } => {
                write!(f, "YAML error in {}: {}", filename, message)
            }
            DefinitionError::UndefinedReference { kind, name, referenced_by, filename } => {
                write!(f, "ERROR in {}: Undefined {} '{}' referenced by {}", filename, kind, name, referenced_by)
            }
            DefinitionError::TypeMismatch { expected, actual, name, referenced_by, filename } => {
                write!(f, "ERROR in {}: Type mismatch for '{}': expected {} but got {}, referenced by {}", filename, name, expected, actual, referenced_by)
            }
            DefinitionError::MissingRequiredField { definition_kind, definition_name, field, filename } => {
                write!(f, "ERROR in {}: {} '{}' missing required field '{}'", filename, definition_kind, definition_name, field)
            }
            DefinitionError::DuplicateName { kind, name, filename } => {
                write!(f, "ERROR in {}: Duplicate {} name '{}'", filename, kind, name)
            }
            DefinitionError::InvalidSchemaVersion { version, filename } => {
                write!(f, "ERROR in {}: Invalid schema version {} (expected 1)", filename, version)
            }
        }
    }
}
```

- [ ] **Step 2: Write the definition types**

Create `types.rs`:

```rust
use serde::Deserialize;
use std::collections::HashMap;

/// Top-level definitions structure from `.inkdef.yaml`.
#[derive(Debug, Clone, Deserialize)]
pub struct Definitions {
    /// Schema version (must be 1).
    pub version: i64,

    #[serde(default)]
    pub assets: HashMap<String, Asset>,

    #[serde(default)]
    pub characters: HashMap<String, Character>,

    #[serde(default)]
    pub scenes: HashMap<String, Scene>,

    #[serde(default)]
    pub actions: HashMap<String, Action>,

    #[serde(default)]
    pub state: HashMap<String, StateVar>,

    #[serde(default)]
    pub events: HashMap<String, Event>,
}

/// Asset definition (audio, image, etc.).
#[derive(Debug, Clone, Deserialize)]
pub struct Asset {
    #[serde(rename = "type")]
    pub asset_type: String,   // "audio", "image"
    pub path: String,

    // Optional metadata
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(default)]
    pub width: Option<i64>,
    #[serde(default)]
    pub height: Option<i64>,
    #[serde(default)]
    pub r#loop: Option<bool>,
}

/// Character definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Character {
    pub display_name: String,

    #[serde(default)]
    pub portrait: Option<String>,

    #[serde(default)]
    pub color: Option<String>,

    #[serde(default)]
    pub style: Option<String>,
}

/// Scene definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Scene {
    #[serde(default)]
    pub background: Option<String>,

    #[serde(default)]
    pub ambient: Option<String>,

    #[serde(default)]
    pub characters: Vec<String>,
}

/// Action definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Action {
    #[serde(default)]
    pub params: Vec<ActionParam>,

    #[serde(default)]
    pub returns: String,
}

/// Action parameter.
#[derive(Debug, Clone, Deserialize)]
pub struct ActionParam {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,

    #[serde(default)]
    pub required: bool,

    #[serde(default)]
    pub default: Option<serde_yaml::Value>,
}

/// State variable definition.
#[derive(Debug, Clone, Deserialize)]
pub struct StateVar {
    #[serde(rename = "type")]
    pub var_type: String,

    #[serde(default)]
    pub default: Option<serde_yaml::Value>,

    #[serde(default)]
    pub min: Option<i64>,

    #[serde(default)]
    pub max: Option<i64>,
}

/// Event definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Event {
    #[serde(default)]
    pub params: Vec<ActionParam>,
}
```

- [ ] **Step 3: Build and verify**

Run: `cargo build -p def-parser`
Expected: Compiles successfully

- [ ] **Step 4: Commit**

```bash
git add crates/def-parser/
git commit -m "feat: add definition types and error types for def-parser"
```

---

### Task 3: YAML deserialization tests

**Files:**
- Create: `crates/def-parser/tests/parse_tests.rs`

- [ ] **Step 1: Write deserialization tests**

```rust
use def_parser::types::*;
use def_parser::parse_definitions;

#[test]
fn test_parse_minimal_definitions() {
    let yaml = "version: 1\n";
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_ok());
    let defs = result.unwrap();
    assert_eq!(defs.version, 1);
    assert!(defs.assets.is_empty());
    assert!(defs.characters.is_empty());
}

#[test]
fn test_parse_with_assets() {
    let yaml = r#"
version: 1
assets:
  creak_door:
    type: audio
    path: audio/creak_door.ogg
    duration: 2.5
  haunted_manor:
    type: image
    path: backgrounds/haunted_manor.png
    width: 1920
    height: 1080
"#;
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_ok());
    let defs = result.unwrap();

    let door = defs.assets.get("creak_door").unwrap();
    assert_eq!(door.asset_type, "audio");
    assert_eq!(door.path, "audio/creak_door.ogg");
    assert_eq!(door.duration, Some(2.5));

    let manor = defs.assets.get("haunted_manor").unwrap();
    assert_eq!(manor.asset_type, "image");
    assert_eq!(manor.width, Some(1920));
}

#[test]
fn test_parse_with_characters() {
    let yaml = r#"
version: 1
characters:
  elara:
    display_name: "Elara"
    portrait: elara_portrait
    color: "#4a9eff"
  narrator:
    display_name: ""
    style: italic
"#;
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    // May fail validation (portrait ref) but should parse
    let defs = result.unwrap_or_else(|_| {
        // Validation errors are OK for this test; re-parse without validation
        serde_yaml::from_str(yaml).unwrap()
    });
    let elara = defs.characters.get("elara").unwrap();
    assert_eq!(elara.display_name, "Elara");
    assert_eq!(elara.portrait, Some("elara_portrait".to_string()));
}

#[test]
fn test_parse_with_scenes() {
    let yaml = r#"
version: 1
scenes:
  haunted_manor:
    background: haunted_manor_bg
    ambient: eerie_ambient
    characters: [elara]
"#;
    let defs: Definitions = serde_yaml::from_str(yaml).unwrap();
    let scene = defs.scenes.get("haunted_manor").unwrap();
    assert_eq!(scene.background, Some("haunted_manor_bg".to_string()));
    assert_eq!(scene.characters, vec!["elara"]);
}

#[test]
fn test_parse_with_actions() {
    let yaml = r#"
version: 1
actions:
  play_sound:
    params:
      - name: asset
        type: audio
        required: true
    returns: void
  shake_screen:
    params:
      - name: intensity
        type: float
        required: false
        default: 0.5
    returns: void
"#;
    let defs: Definitions = serde_yaml::from_str(yaml).unwrap();
    let action = defs.actions.get("play_sound").unwrap();
    assert_eq!(action.params.len(), 1);
    assert_eq!(action.params[0].name, "asset");
    assert_eq!(action.params[0].param_type, "audio");
    assert!(action.params[0].required);
    assert_eq!(action.returns, "void");
}

#[test]
fn test_parse_with_state() {
    let yaml = r#"
version: 1
state:
  suspicion_level:
    type: int
    default: 0
    min: 0
    max: 100
  has_map:
    type: bool
    default: false
"#;
    let defs: Definitions = serde_yaml::from_str(yaml).unwrap();
    let suspicion = defs.state.get("suspicion_level").unwrap();
    assert_eq!(suspicion.var_type, "int");
    assert_eq!(suspicion.min, Some(0));
    assert_eq!(suspicion.max, Some(100));
}

#[test]
fn test_parse_with_events() {
    let yaml = r#"
version: 1
events:
  on_enter_scene:
    params:
      - name: scene
        type: scene
        required: true
"#;
    let defs: Definitions = serde_yaml::from_str(yaml).unwrap();
    let event = defs.events.get("on_enter_scene").unwrap();
    assert_eq!(event.params.len(), 1);
    assert_eq!(event.params[0].param_type, "scene");
}

#[test]
fn test_parse_invalid_yaml() {
    let yaml = "version: 1\nassets: [\n"; // malformed YAML
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_err());
}

#[test]
fn test_parse_wrong_version() {
    let yaml = "version: 99\n";
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| matches!(e, def_parser::error::DefinitionError::InvalidSchemaVersion { .. })));
}
```

- [ ] **Step 2: Run tests and verify they pass**

Run: `cargo test -p def-parser`
Expected: All 8 tests pass

- [ ] **Step 3: Commit**

```bash
git add crates/def-parser/
git commit -m "test: add YAML deserialization tests for def-parser"
```

---

### Task 4: Cross-reference validation

**Files:**
- Create: `crates/def-parser/src/validate.rs`
- Modify: `crates/def-parser/src/lib.rs`

- [ ] **Step 1: Implement the validation module**

Create `validate.rs`:

```rust
use crate::types::*;
use crate::error::DefinitionError;

/// Validate internal referential integrity of a Definitions struct.
/// Returns a list of errors (empty if valid).
pub fn validate(defs: &Definitions, filename: &str) -> Vec<DefinitionError> {
    let mut errors = Vec::new();

    // Schema version must be 1
    if defs.version != 1 {
        errors.push(DefinitionError::InvalidSchemaVersion {
            version: defs.version,
            filename: filename.to_string(),
        });
        return errors; // Can't validate further with unknown schema
    }

    // Validate character portraits reference existing image assets
    for (char_name, character) in &defs.characters {
        if let Some(ref portrait) = character.portrait {
            match defs.assets.get(portrait) {
                Some(asset) => {
                    if asset.asset_type != "image" {
                        errors.push(DefinitionError::TypeMismatch {
                            expected: "image".to_string(),
                            actual: asset.asset_type.to_string(),
                            name: portrait.clone(),
                            referenced_by: format!("character '{}'.portrait", char_name),
                            filename: filename.to_string(),
                        });
                    }
                }
                None => {
                    errors.push(DefinitionError::UndefinedReference {
                        kind: "asset".to_string(),
                        name: portrait.clone(),
                        referenced_by: format!("character '{}'.portrait", char_name),
                        filename: filename.to_string(),
                    });
                }
            }
        }
    }

    // Validate scene backgrounds reference existing image assets
    for (scene_name, scene) in &defs.scenes {
        if let Some(ref bg) = scene.background {
            match defs.assets.get(bg) {
                Some(asset) => {
                    if asset.asset_type != "image" {
                        errors.push(DefinitionError::TypeMismatch {
                            expected: "image".to_string(),
                            actual: asset.asset_type.to_string(),
                            name: bg.clone(),
                            referenced_by: format!("scene '{}'.background", scene_name),
                            filename: filename.to_string(),
                        });
                    }
                }
                None => {
                    errors.push(DefinitionError::UndefinedReference {
                        kind: "asset".to_string(),
                        name: bg.clone(),
                        referenced_by: format!("scene '{}'.background", scene_name),
                        filename: filename.to_string(),
                    });
                }
            }
        }

        // Validate scene ambient references existing audio assets
        if let Some(ref ambient) = scene.ambient {
            match defs.assets.get(ambient) {
                Some(asset) => {
                    if asset.asset_type != "audio" {
                        errors.push(DefinitionError::TypeMismatch {
                            expected: "audio".to_string(),
                            actual: asset.asset_type.to_string(),
                            name: ambient.clone(),
                            referenced_by: format!("scene '{}'.ambient", scene_name),
                            filename: filename.to_string(),
                        });
                    }
                }
                None => {
                    errors.push(DefinitionError::UndefinedReference {
                        kind: "asset".to_string(),
                        name: ambient.clone(),
                        referenced_by: format!("scene '{}'.ambient", scene_name),
                        filename: filename.to_string(),
                    });
                }
            }
        }

        // Validate scene characters reference existing characters
        for char_ref in &scene.characters {
            if !defs.characters.contains_key(char_ref) {
                errors.push(DefinitionError::UndefinedReference {
                    kind: "character".to_string(),
                    name: char_ref.clone(),
                    referenced_by: format!("scene '{}'.characters", scene_name),
                    filename: filename.to_string(),
                });
            }
        }
    }

    // Validate action param types reference known types or definitions
    for (action_name, action) in &defs.actions {
        for param in &action.params {
            validate_param_type(&param.param_type, &format!("action '{}'.param '{}'", action_name, param.name), filename, &mut errors);
        }
    }

    // Validate event param types
    for (event_name, event) in &defs.events {
        for param in &event.params {
            validate_param_type(&param.param_type, &format!("event '{}'.param '{}'", event_name, param.name), filename, &mut errors);
        }
    }

    errors
}

/// Known parameter types in the definitions type system.
const VALID_PARAM_TYPES: &[&str] = &[
    "string", "int", "float", "bool",
    "audio", "image", "scene", "character", "asset",
    "void",
];

fn validate_param_type(
    param_type: &str,
    context: String,
    filename: &str,
    errors: &mut Vec<DefinitionError>,
) {
    if !VALID_PARAM_TYPES.contains(&param_type) {
        errors.push(DefinitionError::TypeMismatch {
            expected: "valid type".to_string(),
            actual: param_type.to_string(),
            name: param_type.to_string(),
            referenced_by: context,
            filename: filename.to_string(),
        });
    }
}
```

- [ ] **Step 2: Update lib.rs to use validation**

Update `crates/def-parser/src/lib.rs`:

```rust
//! Definitions parser for `.inkdef.yaml` files.

pub mod types;
pub mod validate;
pub mod error;

use types::Definitions;
use error::DefinitionError;

/// Parse a `.inkdef.yaml` source string into a Definitions struct.
/// Returns the parsed definitions or validation errors.
pub fn parse_definitions(source: &str, filename: &str) -> Result<Definitions, Vec<DefinitionError>> {
    let defs: Definitions = serde_yaml::from_str(source)
        .map_err(|e| vec![DefinitionError::YamlError {
            message: e.to_string(),
            filename: filename.to_string(),
        }])?;

    let errors = validate::validate(&defs, filename);
    if errors.iter().any(|e| e.is_error()) {
        return Err(errors);
    }

    Ok(defs)
}

/// Parse YAML without running validation. Useful for testing or when
/// you want to collect all parse errors first.
pub fn parse_definitions_unvalidated(source: &str, filename: &str) -> Result<Definitions, DefinitionError> {
    serde_yaml::from_str(source).map_err(|e| DefinitionError::YamlError {
        message: e.to_string(),
        filename: filename.to_string(),
    })
}
```

- [ ] **Step 3: Build and run existing tests**

Run: `cargo test -p def-parser`
Expected: All tests still pass

- [ ] **Step 4: Commit**

```bash
git add crates/def-parser/
git commit -m "feat: implement cross-reference validation for definitions"
```

---

### Task 5: Validation tests

**Files:**
- Create: `crates/def-parser/tests/validation_tests.rs`

- [ ] **Step 1: Write validation tests**

```rust
use def_parser::parse_definitions;
use def_parser::error::DefinitionError;

#[test]
fn test_valid_full_definitions() {
    let yaml = r#"
version: 1
assets:
  creak_door:
    type: audio
    path: audio/creak_door.ogg
  elara_portrait:
    type: image
    path: portraits/elara.png
  haunted_manor_bg:
    type: image
    path: bg/haunted_manor.png
  eerie_ambient:
    type: audio
    path: audio/eerie_loop.ogg
characters:
  elara:
    display_name: "Elara"
    portrait: elara_portrait
    color: "#4a9eff"
scenes:
  haunted_manor:
    background: haunted_manor_bg
    ambient: eerie_ambient
    characters: [elara]
actions:
  play_sound:
    params:
      - name: asset
        type: audio
        required: true
    returns: void
state:
  suspicion_level:
    type: int
    default: 0
    min: 0
    max: 100
events:
  on_enter_scene:
    params:
      - name: scene
        type: scene
        required: true
"#;
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_ok(), "Expected valid, got errors: {:?}", result.err());
}

#[test]
fn test_undefined_character_portrait() {
    let yaml = r#"
version: 1
characters:
  elara:
    display_name: "Elara"
    portrait: nonexistent_asset
"#;
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| matches!(e, DefinitionError::UndefinedReference { kind, .. } if kind == "asset")));
}

#[test]
fn test_portrait_not_image_type() {
    let yaml = r#"
version: 1
assets:
  creak_door:
    type: audio
    path: audio/creak_door.ogg
characters:
  elara:
    display_name: "Elara"
    portrait: creak_door
"#;
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| matches!(e, DefinitionError::TypeMismatch { expected, .. } if expected == "image")));
}

#[test]
fn test_undefined_scene_background() {
    let yaml = r#"
version: 1
scenes:
  haunted_manor:
    background: nonexistent_bg
"#;
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| matches!(e, DefinitionError::UndefinedReference { kind, .. } if kind == "asset")));
}

#[test]
fn test_scene_ambient_not_audio() {
    let yaml = r#"
version: 1
assets:
  portrait:
    type: image
    path: img/portrait.png
scenes:
  scene1:
    ambient: portrait
"#;
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| matches!(e, DefinitionError::TypeMismatch { expected, .. } if expected == "audio")));
}

#[test]
fn test_undefined_scene_character() {
    let yaml = r#"
version: 1
scenes:
  scene1:
    characters: [ghost]
"#;
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| matches!(e, DefinitionError::UndefinedReference { kind, .. } if kind == "character")));
}

#[test]
fn test_invalid_param_type() {
    let yaml = r#"
version: 1
actions:
  bad_action:
    params:
      - name: x
        type: nonexistent_type
        required: true
    returns: void
"#;
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_err());
}

#[test]
fn test_wrong_schema_version() {
    let yaml = "version: 2\n";
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| matches!(e, DefinitionError::InvalidSchemaVersion { .. })));
}
```

- [ ] **Step 2: Run all def-parser tests**

Run: `cargo test -p def-parser`
Expected: All 16 tests pass (8 parse + 8 validation)

- [ ] **Step 3: Commit**

```bash
git add crates/def-parser/
git commit -m "test: add cross-reference validation tests for def-parser"
```

---

### Task 6: Wire into compiler

**Files:**
- Modify: `crates/compiler/Cargo.toml`
- Modify: `crates/compiler/src/lib.rs`

- [ ] **Step 1: Add def-parser dependency to compiler**

Add to `crates/compiler/Cargo.toml`:

```toml
def-parser = { path = "../def-parser" }
```

- [ ] **Step 2: Add compile_with_definitions to compiler lib.rs**

Add to `crates/compiler/src/lib.rs`:

```rust
/// Compile ink source with definitions validation.
/// Returns the ink JSON, or errors from parsing/compiling/validation.
pub fn compile_ink_with_definitions(
    ink_source: &str,
    ink_filename: &str,
    definitions_yaml: &str,
    definitions_filename: &str,
) -> Result<String, Vec<String>> {
    // Parse definitions
    let defs = def_parser::parse_definitions(definitions_yaml, definitions_filename)
        .map_err(|errors| errors.iter().map(|e| e.to_string()).collect())?;

    // Compile ink (directives will be validated against definitions in Phase 3)
    let _ = defs; // Used in Phase 3 for cross-reference validation
    compile_ink(ink_source, ink_filename)
}
```

- [ ] **Step 3: Build and run all tests**

Run: `cargo test`
Expected: All existing tests still pass

- [ ] **Step 4: Commit**

```bash
git add crates/compiler/
git commit -m "feat: wire def-parser into compiler pipeline"
```

---

### Task 7: CLI `check-defs` subcommand

**Files:**
- Modify: `crates/cli/Cargo.toml`
- Modify: `crates/cli/src/main.rs`

- [ ] **Step 1: Add def-parser dependency to CLI**

Add to `crates/cli/Cargo.toml`:

```toml
def-parser = { path = "../def-parser" }
```

- [ ] **Step 2: Add check-defs subcommand to CLI**

Add to the `Commands` enum in `main.rs`:

```rust
/// Validate a definitions file
CheckDefs {
    /// Path to the .inkdef.yaml file
    #[arg(value_name = "FILE")]
    file: String,
},
```

Add to the `match` arm:

```rust
Commands::CheckDefs { file } => {
    let source = read_file(&file);
    match def_parser::parse_definitions(&source, &file) {
        Ok(defs) => {
            println!("✓ Valid definitions in {} (v{})", file, defs.version);
            let asset_count = defs.assets.len();
            let char_count = defs.characters.len();
            let scene_count = defs.scenes.len();
            let action_count = defs.actions.len();
            let state_count = defs.state.len();
            let event_count = defs.events.len();
            println!("  {} assets, {} characters, {} scenes, {} actions, {} state vars, {} events",
                asset_count, char_count, scene_count, action_count, state_count, event_count);
        }
        Err(errors) => {
            eprintln!("Validation errors in {}:", file);
            for e in &errors {
                eprintln!("  {}", e);
            }
            std::process::exit(1);
        }
    }
}
```

- [ ] **Step 3: Build and test the CLI**

```bash
echo 'version: 1
assets:
  creak:
    type: audio
    path: creak.ogg' > /tmp/test_defs.inkdef.yaml

cargo run -- check-defs /tmp/test_defs.inkdef.yaml
```

Expected: `✓ Valid definitions in /tmp/test_defs.inkdef.yaml (v1)`

```bash
echo 'version: 1
characters:
  elara:
    display_name: Elara
    portrait: missing_asset' > /tmp/bad_defs.inkdef.yaml

cargo run -- check-defs /tmp/bad_defs.inkdef.yaml
```

Expected: Error about undefined asset 'missing_asset'

- [ ] **Step 4: Commit**

```bash
git add crates/cli/
git commit -m "feat: add check-defs CLI subcommand"
```

---

### Task 8: Full integration test

**Files:**
- Create: `crates/def-parser/tests/integration.rs`

- [ ] **Step 1: Write end-to-end integration test**

```rust
use def_parser::parse_definitions;
use def_parser::types::*;

#[test]
fn test_full_definitions_roundtrip() {
    let yaml = r#"
version: 1
assets:
  creak_door:
    type: audio
    path: audio/creak_door.ogg
    duration: 2.5
  haunted_manor:
    type: image
    path: backgrounds/haunted_manor.png
    width: 1920
    height: 1080
  elara_portrait:
    type: image
    path: portraits/elara.png
  eerie_ambient:
    type: audio
    path: audio/eerie_loop.ogg
    loop: true
characters:
  elara:
    display_name: "Elara"
    portrait: elara_portrait
    color: "#4a9eff"
  narrator:
    display_name: ""
    style: italic
scenes:
  haunted_manor:
    background: haunted_manor
    ambient: eerie_ambient
    characters: [elara]
actions:
  play_sound:
    params:
      - name: asset
        type: audio
        required: true
    returns: void
  start_minigame:
    params:
      - name: name
        type: string
        required: true
      - name: difficulty
        type: int
        required: false
        default: 1
    returns: bool
  shake_screen:
    params:
      - name: intensity
        type: float
        required: false
        default: 0.5
    returns: void
state:
  suspicion_level:
    type: int
    default: 0
    min: 0
    max: 100
  current_companion:
    type: string
    default: ""
  has_map:
    type: bool
    default: false
events:
  on_enter_scene:
    params:
      - name: scene
        type: scene
        required: true
  on_choice_made:
    params:
      - name: choice_text
        type: string
        required: true
"#;

    let defs = parse_definitions(yaml, "game.inkdef.yaml")
        .expect("Should parse and validate successfully");

    // Verify asset types
    assert_eq!(defs.assets.len(), 4);
    assert_eq!(defs.assets.get("creak_door").unwrap().asset_type, "audio");
    assert_eq!(defs.assets.get("haunted_manor").unwrap().asset_type, "image");
    assert!(defs.assets.get("eerie_ambient").unwrap().r#loop == Some(true));

    // Verify characters
    assert_eq!(defs.characters.len(), 2);
    assert_eq!(defs.characters.get("elara").unwrap().portrait, Some("elara_portrait".to_string()));

    // Verify scenes
    assert_eq!(defs.scenes.len(), 1);
    let scene = defs.scenes.get("haunted_manor").unwrap();
    assert_eq!(scene.background, Some("haunted_manor".to_string()));
    assert_eq!(scene.ambient, Some("eerie_ambient".to_string()));

    // Verify actions
    assert_eq!(defs.actions.len(), 3);
    let play_sound = defs.actions.get("play_sound").unwrap();
    assert_eq!(play_sound.params.len(), 1);
    assert!(play_sound.params[0].required);
    assert_eq!(play_sound.returns, "void");

    // Verify state
    assert_eq!(defs.state.len(), 3);
    let suspicion = defs.state.get("suspicion_level").unwrap();
    assert_eq!(suspicion.var_type, "int");
    assert_eq!(suspicion.max, Some(100));

    // Verify events
    assert_eq!(defs.events.len(), 2);
}

#[test]
fn test_multiple_validation_errors() {
    let yaml = r#"
version: 1
characters:
  elara:
    display_name: Elara
    portrait: missing_portrait
scenes:
  manor:
    background: missing_bg
    ambient: missing_audio
    characters: [missing_char]
"#;
    let result = parse_definitions(yaml, "bad.inkdef.yaml");
    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Should have 4 errors: missing portrait, missing bg, missing audio, missing char
    assert!(errors.len() >= 4, "Expected at least 4 errors, got {}: {:?}", errors.len(), errors);
}

#[test]
fn test_empty_definitions_are_valid() {
    let yaml = "version: 1\n";
    let defs = parse_definitions(yaml, "empty.inkdef.yaml").unwrap();
    assert_eq!(defs.version, 1);
    assert!(defs.assets.is_empty());
    assert!(defs.characters.is_empty());
    assert!(defs.scenes.is_empty());
    assert!(defs.actions.is_empty());
    assert!(defs.state.is_empty());
    assert!(defs.events.is_empty());
}
```

- [ ] **Step 2: Run all tests**

Run: `cargo test -p def-parser`
Expected: All 19 tests pass (8 parse + 8 validation + 3 integration)

- [ ] **Step 3: Run full workspace test suite**

Run: `cargo test`
Expected: All tests pass across all crates

- [ ] **Step 4: Commit**

```bash
git add crates/def-parser/
git commit -m "test: add full integration tests for def-parser"
```

---

### Task 9: Documentation and cleanup

**Files:**
- Modify: `narrative/README.md`
- Modify: `narrative/Cargo.toml` (workspace description)

- [ ] **Step 1: Update README with Phase 2 information**

Add to the project structure section:

```markdown
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
```

Add to CLI commands:

```markdown
# Validate definitions
narrative check-defs story.inkdef.yaml
```

Update the roadmap:

```markdown
| 2 | Definitions parser (`.inkdef.yaml`) | ✅ Complete |
```

- [ ] **Step 2: Final build and test**

Run: `cargo test && cargo build`
Expected: All green

- [ ] **Step 3: Commit**

```bash
git add narrative/README.md
git commit -m "docs: update README with Phase 2 def-parser info"
```
