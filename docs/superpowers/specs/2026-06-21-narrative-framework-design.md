# Narrative Framework Design

A declarative framework for story-driven and text games, built as a Rust rewrite of the ink compiler extended with a structured directive system and multi-output compilation.

## Overview

Ink provides a way to structure a narrative history, but one must still glue that together in a game engine (Godot, Unity) to build the complete game. This framework extends ink's model so that the majority of game scene definition — assets, sounds, actions, events, state — lives in a single master file set that the compiler validates and compiles into engine-consumable artifacts.

The framework is a **collaboration bridge**: programmers define what exists (assets, actions, scenes, state, events) in a YAML definitions file, and writers reference those definitions via structured directives in their `.ink` files. The compiler validates cross-references and produces multiple output artifacts that game engines consume.

## Architecture

### Approach: Layered Pipeline

Three independent modules form a compilation pipeline:

- **`ink-parser`** — Reimplements the ink language parser in Rust. Reads `.ink` source, produces a `ParsedStory` IR that includes both standard ink constructs and `@` directive nodes. Hand-written recursive descent parser matching ink's C# architecture.
- **`def-parser`** — Parses `.inkdef.yaml` files using `serde_yaml`. Produces a `Definitions` struct with typed collections of assets, characters, scenes, actions, state, and events. Includes internal schema validation.
- **`compiler`** — Orchestrates the pipeline: parses both sources, validates cross-references (directives against definitions), then generates three outputs: standard ink JSON, directives manifest, and definitions schema.

Two additional crates round out the workspace:

- **`lsp`** — Language server built on `tower-lsp`. Uses `ink-parser` and `def-parser` directly for diagnostics and completions.
- **`cli`** — Command-line interface that exposes the compiler as a tool (replaces `inklecate`).

```
narrative/
├── crates/
│   ├── ink-parser/
│   ├── def-parser/
│   ├── compiler/
│   ├── lsp/
│   └── cli/
├── schemas/
└── Cargo.toml
```

### Relationship to Existing Ink Codebase

This is a **Rust rewrite** of the ink compiler, not a C# fork. The Rust compiler produces the same ink JSON runtime format (version 21) as the C# compiler, ensuring backward compatibility with existing ink runtimes (inkjs, ink-unity-integration). The directive framework is a purely additive layer on top.

## Definitions File Format (`.inkdef.yaml`)

Programmers write this file. It declares the "API" that writers can reference in their `@` directives.

### Top-Level Structure

```yaml
# Schema version for the .inkdef.yaml format
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
    portrait: elara_portrait          # references an asset
    color: "#4a9eff"
  narrator:
    display_name: ""
    style: italic

scenes:
  haunted_manor:
    background: haunted_manor         # references an asset
    ambient: eerie_ambient            # references an asset
    characters: [elara]               # references characters
  village_square:
    background: village_square
    ambient: village_bustle

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
      - name: duration
        type: float
        required: false
        default: 0.3
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
  on_combat_end:
    params:
      - name: victory
        type: bool
        required: true
      - name: damage_taken
        type: int
        required: true
```

### Key Design Decisions

- **Type system is deliberately simple:** `string`, `int`, `float`, `bool`, `audio`, `image`, `scene`, `character`, `asset`. Asset-typed parameters are references to other declarations validated by the compiler.
- **References are by name, not path:** `portrait: elara_portrait` not `portrait: assets/elara_portrait`. Actual file paths live only in asset declarations.
- **Actions have typed, validated parameters:** The compiler checks that action calls reference existing actions with correct parameter types and required params present.
- **State variables are schema, not runtime:** These declare what custom state the narrative can read/write. Actual state lives in the game engine. The compiler validates state references; the definitions schema tells the engine what state to expect.
- **Cross-reference validation within the file:** The `def-parser` catches errors like referencing a nonexistent asset, a character portrait pointing to a non-image asset, or a scene listing an undeclared character.

## Writer Directive Syntax (`@` directives)

Writers type these inside `.ink` files. The `@` prefix makes directives visually distinct from narrative text, tags, and ink logic.

### Syntax Rules

```
@ <directive_type>: <name>(<args>)
```

- `@` must be at the start of a line
- One directive per line
- Directives attach to the **next line of narrative content** that follows them

### Directive Types

1. **Action** — trigger a game-side operation
   ```
   @ action: play_sound(creak_door)
   @ action: start_minigame(lock_picking, 3)
   @ action: shake_screen(0.8, 0.5)
   ```

2. **Scene** — set or change the current scene
   ```
   @ scene: haunted_manor
   @ scene: village_square
   ```

3. **Character** — declare who is speaking/active
   ```
   @ character: elara
   @ character: elara(expression: angry)
   ```
   Parenthetical modifiers on any directive type pass named key-value pairs to the engine. The compiler validates modifier keys against the definitions schema (if the character definition declares supported modifiers). Unrecognized modifiers produce warnings, not errors.

4. **State** — read or modify custom game state
   ```
   @ state: suspicion_level = suspicion_level + 10
   @ state: has_map = true
   @ state: current_companion = "elara"
   ```
   State directives always use assignment (`=`). The right-hand side can reference the same variable (for increments/decrements) or be a literal value. This avoids ambiguity with ink's `~` assignment syntax and keeps state directives as explicit signals to the engine, not computed expressions.

5. **Event** — emit a named event
   ```
   @ event: on_enter_scene(haunted_manor)
   @ event: on_combat_end(victory: true, damage_taken: 15)
   ```

6. **Asset** — preload or explicitly reference an asset
   ```
   @ asset: creak_door
   @ asset: [creak_door, haunted_manor]
   ```

### Stacking Directives

Multiple directives can stack before a single narrative line — they all attach to the same ink path:

```
@ scene: haunted_manor
@ character: elara
@ action: play_sound(creak_door)
@ event: on_enter_scene(haunted_manor)
The door creaked open, and Elara stepped inside.
```

Directives can also appear within choices and conditional blocks.

### Directives vs. Tags

| | Tags (`#`) | Directives (`@`) |
|---|---|---|
| **Purpose** | Arbitrary metadata | Structured, validated game commands |
| **Validation** | None — freeform strings | Checked against definitions schema |
| **Autocomplete** | No | Yes — names, params, types |
| **Audience** | Either domain | Writer domain only |

Tags remain for unstructured metadata. Directives are the structured, validated writer→engine channel. They coexist.

## Engine SDK Interface

The engine integration has two layers: the narrative runtime (existing, solved by inkjs/ink-unity/etc.) and the directive runtime (new).

### The Contract: Three Files

A game engine receives three compiled files:

```
story.ink.json          # Standard ink runtime format
story.directives.json   # Directive manifest keyed by ink paths
story.schema.json       # Definitions schema (validated, normalized)
```

### Directive Runner Interface

The platform-agnostic API each engine SDK implements:

```
DirectiveRunner:
  load(directives_json, schema_json) -> Result
  on_advance(current_ink_path: Path) -> Vec<Directive>
  on_action(name: String, handler: Fn(ActionArgs))
  on_scene(name: String, handler: Fn(SceneDef))
  on_event(name: String, handler: Fn(EventArgs))
  on_state_change(variable: String, handler: Fn(OldValue, NewValue))
  get_character(name: String) -> CharacterDef
  get_asset(name: String) -> AssetDef
  get_scene(name: String) -> SceneDef
```

### Godot Integration

A Godot SDK would be a GDExtension or C# plugin that:

1. Wraps an ink runtime
2. Implements the `DirectiveRunner` interface
3. Exposes signals that map to directive handlers:

```gdscript
func _ready():
    var narrative = NarrativeSDK.new()
    narrative.load_story("res://story.ink.json",
                         "res://story.directives.json",
                         "res://story.schema.json")
    narrative.connect("action", self, "_on_action")
    narrative.connect("scene_change", self, "_on_scene")
    narrative.connect("event", self, "_on_event")

func _on_action(action_name: String, args: Dictionary):
    match action_name:
        "play_sound": $AudioPlayer.play(args.asset)
        "start_minigame": start_minigame(args.name, args.difficulty)

func _on_scene(scene_name: String, scene_def: Dictionary):
    $Background.texture = load(scene_def.background)
    $AmbientAudio.stream = load(scene_def.ambient_audio)
```

### Key Principle

The SDK is a thin bridge, not an engine. It translates narrative events into engine-native callbacks. The programmer retains full control of how directives map to their game's architecture.

## Compiler Pipeline

```
.ink source ──┐
              ├─▶ Stage 1: Parse ──▶ Stage 2: Validate ──▶ Stage 3: Generate ──▶ Output Artifacts
.inkdef.yaml ─┘
```

### Stage 1: Parse

Two independent parsers run in parallel:

**`ink-parser`** processes the `.ink` file:
- Parses all standard ink constructs plus `@` directives as first-class nodes
- Produces a `ParsedStory` with source location metadata on every node
- Directive nodes embedded at their exact position, attached to the next content node
- Does not validate directives against definitions — just captures what the writer wrote

**`def-parser`** processes the `.inkdef.yaml` file:
- Deserializes YAML into a `Definitions` struct using `serde_yaml`
- Runs internal validation (referential integrity within the file)
- Produces a typed, validated `Definitions` struct

Both parsers continue on error (best-effort recovery) to report as many issues as possible.

### Stage 2: Validate (Cross-Reference)

The validator walks the `ParsedStory` and checks every `@` directive against the `Definitions`:

- Action exists in definitions? Correct parameter types? Required params present?
- Scene exists? Character exists? Event exists with correct params?
- State variable exists and type matches the operation?
- Asset references point to existing assets of the correct type?

Each directive is validated independently — one bad directive doesn't stop the rest from being checked.

**Error vs. Warning:**
- **Error:** References a nonexistent definition, wrong parameter type, missing required param
- **Warning:** References an unused definition, action return value never used, state variable never read

### Stage 3: Generate

Three output generators:

**1. Ink JSON Generator**
- Walks `ParsedStory`, produces standard ink runtime JSON (version 21)
- `@` directives are stripped — they don't appear in the ink JSON
- Output is semantically identical to the C# compiler's output for the same ink without directives

**2. Directives Manifest Generator**
- Collects all `@` directive nodes from `ParsedStory`
- Resolves each directive's position to an ink runtime path
- Produces the directives manifest:

```json
{
  "version": 1,
  "source": "story.ink",
  "directives": {
    "haunted_house.door.3": [
      {
        "type": "scene",
        "name": "haunted_manor",
        "source": { "file": "story.ink", "line": 42 }
      },
      {
        "type": "character",
        "name": "elara",
        "source": { "file": "story.ink", "line": 43 }
      },
      {
        "type": "action",
        "name": "play_sound",
        "args": { "asset": "creak_door" },
        "source": { "file": "story.ink", "line": 44 }
      }
    ],
    "haunted_house.door.5": [
      {
        "type": "event",
        "name": "on_enter_scene",
        "args": { "scene": "haunted_manor" },
        "source": { "file": "story.ink", "line": 45 }
      }
    ]
  }
}
```

**3. Definitions Schema Generator**
- Serializes the validated `Definitions` into normalized JSON for engine consumption
- Resolves references (character definition includes resolved portrait path, not just asset name)

```json
{
  "version": 1,
  "assets": {
    "creak_door": {
      "type": "audio",
      "path": "audio/creak_door.ogg",
      "duration": 2.5
    }
  },
  "characters": {
    "elara": {
      "display_name": "Elara",
      "portrait": { "path": "portraits/elara.png", "type": "image" },
      "color": "#4a9eff"
    }
  },
  "actions": {
    "play_sound": {
      "params": [{ "name": "asset", "type": "audio", "required": true }],
      "returns": "void"
    }
  },
  "state": {
    "suspicion_level": { "type": "int", "default": 0, "min": 0, "max": 100 }
  },
  "events": {
    "on_enter_scene": {
      "params": [{ "name": "scene", "type": "scene", "required": true }]
    }
  }
}
```

### Source Maps

The `source` field on each directive entry creates a bidirectional link between ink paths and source locations, powering error messages and the LSP's go-to-definition.

### Definitions File Resolution

The compiler and LSP discover the definitions file for a given `.ink` file using this order:

1. **Explicit declaration** — A `DEF` line at the top of the `.ink` file:
   ```
   DEF definitions.inkdef.yaml
   ```
2. **Convention** — Same base name in the same directory: `story.ink` → `story.inkdef.yaml`
3. **Configuration** — Workspace-level mapping

## LSP Architecture

The language server reuses the parser crates directly — no need to run the full compiler pipeline.

### Document State Management

The LSP maintains an in-memory model of all open documents:

```
DocumentState:
  ink_files: Map<Uri, ParsedStory>
  def_files: Map<Uri, Definitions>
  dirty: Set<Uri>
```

When a file changes, only that file is re-parsed. Cross-reference validation runs after a short debounce (300ms).

### Capabilities by File Type

**For `.ink` files (writer experience):**

| Feature | What it does |
|---|---|
| Diagnostics | Validates `@` directives against definitions |
| Completion — directive type | After `@ `, offers: `action:`, `scene:`, `character:`, `state:`, `event:`, `asset:` |
| Completion — names | After `@ action: `, offers all declared action names |
| Completion — params | After `@ action: play_sound(`, offers parameter names and types |
| Completion — references | Inside params, offers contextually correct names (audio assets for `audio` type, etc.) |
| Go-to-definition | From a directive name, jumps to its YAML declaration |
| Hover | Shows action signature, scene summary, etc. |

**For `.inkdef.yaml` files (programmer experience):**

| Feature | What it does |
|---|---|
| Diagnostics | Internal validation: references, type mismatches, missing fields |
| Completion — keys | Autocompletes YAML structure and field names |
| Completion — references | Offers asset names for reference fields |
| Go-to-definition | From a reference, jumps to the target declaration |
| Hover | Shows full definition of referenced entity |

### Performance

- Parsing is fast (Rust, no disk I/O for open documents)
- Cross-reference validation is a single walk of the directive list
- Debounced diagnostics (300ms after last keystroke)
- Definitions file changes trigger re-validation of all open ink files

## Phasing & Milestones

### Phase 1: Rust Ink Compiler — "Foundation"

**Goal:** A Rust compiler that produces the same ink JSON as the C# compiler. Drop-in replacement for `inklecate`.

**Deliverables:**
- `ink-parser` crate — full ink language parser
- `compiler` crate — generates standard ink JSON (version 21)
- `cli` crate — basic command-line interface (compile + play modes)
- Test suite ported from C# `tests/Tests.cs`

**Validation gate:** Semantic equivalence with the C# compiler's JSON output across the full test suite.

**Estimated scope:** ~60-70% of total project effort. The ink language is deceptively complex.

### Phase 2: Definitions Parser — "The Contract"

**Goal:** Parse and validate `.inkdef.yaml` files.

**Deliverables:**
- `def-parser` crate — reads YAML, produces `Definitions` struct
- Internal validation with clear error messages
- CLI: `narrative check-defs story.inkdef.yaml`

**Validation gate:** Parses valid files without error, rejects invalid ones with precise diagnostics.

**Estimated scope:** Medium. `serde_yaml` does the heavy lifting.

### Phase 3: Directives + Validation + Multi-Output — "The Framework"

**Goal:** Minimum viable framework. Writers can use `@` directives, compiler validates them, produces all three outputs.

**Deliverables:**
- Extend `ink-parser` to parse `@` directive syntax
- Cross-reference validation in `compiler`
- Three output generators: ink JSON, directives manifest, definitions schema
- CLI: `narrative compile story.ink`
- `DEF` line support

**Validation gate:** Given a `.ink` + `.inkdef.yaml` pair, compiler produces all three outputs. Invalid directives produce precise errors.

**Estimated scope:** Medium. Parser extension is straightforward. Validation logic is the core work.

### Phase 4: LSP — "The Writer Experience"

**Goal:** Real-time validation and autocompletion in the editor.

**Deliverables:**
- `lsp` crate — full language server
- Diagnostics for `.ink` and `.inkdef.yaml` files
- Autocompletion for directive types, names, parameters, typed references
- Go-to-definition (ink → yaml, yaml → yaml)
- Hover documentation
- VS Code extension

**Validation gate:** Open a `.ink` file in VS Code, see autocomplete from definitions file, see error squiggles on invalid references, Cmd+click jumps to YAML declaration.

**Estimated scope:** Medium. `tower-lsp` handles protocol; completion and diagnostics reuse parser crates.

### Phase 5: Reference SDK (Godot) — "End-to-End Proof"

**Goal:** Working Godot integration proving the contract is real.

**Deliverables:**
- Godot GDExtension or C# plugin implementing `DirectiveRunner`
- Loads all three compiled artifacts
- Dispatches directives as Godot signals
- Demo game: branching narrative with scene changes, sound triggers, state tracking, mini-game hook

**Validation gate:** Play the demo game in Godot. Scene changes, sounds, state, mini-game all work.

**Estimated scope:** Medium. SDK layer is thin by design.

### Phase Dependencies

```
Phase 1: Rust Ink Compiler
    │
    ├── Phase 2: Definitions Parser (can parallel)
    │       │
    │   Phase 3: Directives + Validation + Multi-Output (needs 1 + 2)
    │           │
    │       Phase 4: LSP (needs 1 + 2 + 3)
    │               │
    │           Phase 5: Godot SDK (needs 3)
```

Phases 1 and 2 can be developed in parallel. Phase 3 is the integration point. Phases 4 and 5 depend on Phase 3 but are independent of each other.

### Future Horizons (Not in initial scope)

- More SDKs (Unity, web/TypeScript, Python)
- Rust runtime compiled to WASM for web or static library for engine embedding
- Enhanced LSP (rename refactoring, unused definition warnings, directive usage search)
- Visual editor (Inky successor rendering narrative with directives)
- Hot reload (recompile and push changes to running game engine)
- Testing framework (playthrough automation, assertions on directives fired)
