use ink_parser::parse_story;

#[test]
fn test_action_directive() {
    let source = "@ action: play_sound(creak_door)\nHello\n-> END";
    let story = parse_story(source, "test.ink");
    assert!(!story.has_errors(), "Errors: {:?}", story.errors);
}

#[test]
fn test_scene_directive() {
    let source = "@ scene: haunted_manor\nThe door creaked.\n-> END";
    let story = parse_story(source, "test.ink");
    assert!(!story.has_errors(), "Errors: {:?}", story.errors);
}

#[test]
fn test_character_directive() {
    let source = "@ character: elara\nHello there.\n-> END";
    let story = parse_story(source, "test.ink");
    assert!(!story.has_errors(), "Errors: {:?}", story.errors);
}

#[test]
fn test_state_directive() {
    let source = "@ state: suspicion_level = suspicion_level + 10\nYou look around.\n-> END";
    let story = parse_story(source, "test.ink");
    assert!(!story.has_errors(), "Errors: {:?}", story.errors);
}

#[test]
fn test_event_directive() {
    let source = "@ event: on_enter_scene(haunted_manor)\nYou enter.\n-> END";
    let story = parse_story(source, "test.ink");
    assert!(!story.has_errors(), "Errors: {:?}", story.errors);
}

#[test]
fn test_asset_directive() {
    let source = "@ asset: creak_door\nA sound plays.\n-> END";
    let story = parse_story(source, "test.ink");
    assert!(!story.has_errors(), "Errors: {:?}", story.errors);
}

#[test]
fn test_stacked_directives() {
    let source = "@ scene: haunted_manor\n@ character: elara\n@ action: play_sound(creak_door)\nThe door creaked open.\n-> END";
    let story = parse_story(source, "test.ink");
    assert!(!story.has_errors(), "Errors: {:?}", story.errors);
}

#[test]
fn test_directive_with_modifier() {
    let source = "@ character: elara(expression: angry)\nNo!\n-> END";
    let story = parse_story(source, "test.ink");
    assert!(!story.has_errors(), "Errors: {:?}", story.errors);
}