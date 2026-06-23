use narrative_compiler::compile_full;

#[test]
fn test_full_compilation_pipeline() {
    let ink = r#"@ scene: haunted_manor
@ character: elara
@ action: play_sound(creak_door)
The door creaked open.
-> END
"#;
    let defs = r#"version: 1
assets:
  creak_door:
    type: audio
    path: audio/creak_door.ogg
  haunted_manor:
    type: image
    path: bg/haunted_manor.png
characters:
  elara:
    display_name: "Elara"
scenes:
  haunted_manor:
    background: haunted_manor
actions:
  play_sound:
    params:
      - name: asset
        type: audio
        required: true
    returns: void
"#;
    let output = compile_full(ink, "test.ink", defs, "test.inkdef.yaml").unwrap();

    // Ink JSON should exist and be valid
    let ink_json: serde_json::Value = serde_json::from_str(&output.ink_json).unwrap();
    assert_eq!(ink_json["inkVersion"], 21);

    // Directives manifest should exist and be valid
    let manifest: serde_json::Value = serde_json::from_str(&output.directives_manifest).unwrap();
    assert_eq!(manifest["version"], 1);

    // Definitions schema should exist and be valid
    let schema: serde_json::Value = serde_json::from_str(&output.definitions_schema).unwrap();
    assert_eq!(schema["version"], 1);
}

#[test]
fn test_directive_validation_catches_bad_reference() {
    let ink = "@ action: nonexistent_action\nHello.\n-> END";
    let defs = "version: 1\n";
    let result = compile_full(ink, "test.ink", defs, "test.inkdef.yaml");
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.contains("Undefined action")));
}