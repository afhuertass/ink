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