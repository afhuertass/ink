use crate::types::*;
use crate::error::DefinitionError;

/// Known parameter types in the definitions type system.
const VALID_PARAM_TYPES: &[&str] = &[
    "string", "int", "float", "bool",
    "audio", "image", "scene", "character", "asset",
    "void",
];

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
        return errors;
    }

    // Validate character portraits reference existing image assets
    for (char_name, character) in &defs.characters {
        if let Some(ref portrait) = character.portrait {
            match defs.assets.get(portrait) {
                Some(asset) => {
                    if asset.asset_type != "image" {
                        errors.push(DefinitionError::TypeMismatch {
                            expected: "image".to_string(),
                            actual: asset.asset_type.clone(),
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
                            actual: asset.asset_type.clone(),
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
                            actual: asset.asset_type.clone(),
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

    // Validate action param types
    for (action_name, action) in &defs.actions {
        for param in &action.params {
            validate_param_type(
                &param.param_type,
                &format!("action '{}'.param '{}'", action_name, param.name),
                filename,
                &mut errors,
            );
        }
    }

    // Validate event param types
    for (event_name, event) in &defs.events {
        for param in &event.params {
            validate_param_type(
                &param.param_type,
                &format!("event '{}'.param '{}'", event_name, param.name),
                filename,
                &mut errors,
            );
        }
    }

    errors
}

fn validate_param_type(
    param_type: &str,
    context: &str,
    filename: &str,
    errors: &mut Vec<DefinitionError>,
) {
    if !VALID_PARAM_TYPES.contains(&param_type) {
        errors.push(DefinitionError::TypeMismatch {
            expected: "valid type".to_string(),
            actual: param_type.to_string(),
            name: param_type.to_string(),
            referenced_by: context.to_string(),
            filename: filename.to_string(),
        });
    }
}