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

    assert_eq!(defs.assets.len(), 4);
    assert_eq!(defs.assets.get("creak_door").unwrap().asset_type, "audio");
    assert_eq!(defs.assets.get("haunted_manor").unwrap().asset_type, "image");
    assert!(defs.assets.get("eerie_ambient").unwrap().r#loop == Some(true));

    assert_eq!(defs.characters.len(), 2);
    assert_eq!(defs.characters.get("elara").unwrap().portrait, Some("elara_portrait".to_string()));

    assert_eq!(defs.scenes.len(), 1);
    let scene = defs.scenes.get("haunted_manor").unwrap();
    assert_eq!(scene.background, Some("haunted_manor".to_string()));
    assert_eq!(scene.ambient, Some("eerie_ambient".to_string()));

    assert_eq!(defs.actions.len(), 3);
    let play_sound = defs.actions.get("play_sound").unwrap();
    assert_eq!(play_sound.params.len(), 1);
    assert!(play_sound.params[0].required);
    assert_eq!(play_sound.returns, "void");

    assert_eq!(defs.state.len(), 3);
    let suspicion = defs.state.get("suspicion_level").unwrap();
    assert_eq!(suspicion.var_type, "int");
    assert_eq!(suspicion.max, Some(100));

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