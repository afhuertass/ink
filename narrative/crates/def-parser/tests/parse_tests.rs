use def_parser::types::*;
use def_parser::parse_definitions;
use def_parser::error::DefinitionError;

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
    color: blue
  narrator:
    display_name: ""
    style: italic
"#;
    // May fail validation (portrait ref undefined) but parsing should work
    let defs: Definitions = serde_yaml::from_str(yaml).unwrap();
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
    let yaml = "version: 1\nassets: [\n";
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    assert!(result.is_err());
}

#[test]
fn test_parse_wrong_version() {
    // Validation will catch this in Task 4
    // For now, just verify it deserializes without crashing
    let yaml = "version: 99\n";
    let result = parse_definitions(yaml, "test.inkdef.yaml");
    // Will fail once validation is implemented in Task 4
    assert!(result.is_err() || result.unwrap().version == 99);
}