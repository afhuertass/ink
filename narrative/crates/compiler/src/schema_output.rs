//! Definitions schema generator.
//! Produces the definitions JSON (normalized, with resolved references)
//! for engine consumption.

use def_parser::types::*;
use serde_json::{Map, Value};

/// Generate the definitions schema JSON from a Definitions struct.
pub fn generate_schema(defs: &Definitions) -> Value {
    let mut root = Map::new();
    root.insert("version".to_string(), Value::Number(defs.version.into()));

    // Assets
    if !defs.assets.is_empty() {
        let assets: Map<String, Value> = defs.assets.iter().map(|(name, asset)| {
            let mut a = Map::new();
            a.insert("type".to_string(), Value::String(asset.asset_type.clone()));
            a.insert("path".to_string(), Value::String(asset.path.clone()));
            if let Some(d) = asset.duration {
                a.insert("duration".to_string(), serde_json::Number::from_f64(d)
                    .map(Value::Number).unwrap_or(Value::Null));
            }
            if let Some(w) = asset.width {
                a.insert("width".to_string(), Value::Number(w.into()));
            }
            if let Some(h) = asset.height {
                a.insert("height".to_string(), Value::Number(h.into()));
            }
            if let Some(l) = asset.r#loop {
                a.insert("loop".to_string(), Value::Bool(l));
            }
            (name.clone(), Value::Object(a))
        }).collect();
        root.insert("assets".to_string(), Value::Object(assets));
    }

    // Characters (with resolved portrait)
    if !defs.characters.is_empty() {
        let chars: Map<String, Value> = defs.characters.iter().map(|(name, char)| {
            let mut c = Map::new();
            c.insert("display_name".to_string(), Value::String(char.display_name.clone()));
            if let Some(ref portrait) = char.portrait {
                // Resolve portrait reference to actual asset
                if let Some(asset) = defs.assets.get(portrait) {
                    let mut p = Map::new();
                    p.insert("path".to_string(), Value::String(asset.path.clone()));
                    p.insert("type".to_string(), Value::String(asset.asset_type.clone()));
                    c.insert("portrait".to_string(), Value::Object(p));
                } else {
                    c.insert("portrait".to_string(), Value::String(portrait.clone()));
                }
            }
            if let Some(ref color) = char.color {
                c.insert("color".to_string(), Value::String(color.clone()));
            }
            if let Some(ref style) = char.style {
                c.insert("style".to_string(), Value::String(style.clone()));
            }
            (name.clone(), Value::Object(c))
        }).collect();
        root.insert("characters".to_string(), Value::Object(chars));
    }

    // Scenes
    if !defs.scenes.is_empty() {
        let scenes: Map<String, Value> = defs.scenes.iter().map(|(name, scene)| {
            let mut s = Map::new();
            if let Some(ref bg) = scene.background {
                if let Some(asset) = defs.assets.get(bg) {
                    let mut b = Map::new();
                    b.insert("path".to_string(), Value::String(asset.path.clone()));
                    b.insert("type".to_string(), Value::String(asset.asset_type.clone()));
                    s.insert("background".to_string(), Value::Object(b));
                } else {
                    s.insert("background".to_string(), Value::String(bg.clone()));
                }
            }
            if let Some(ref amb) = scene.ambient {
                if let Some(asset) = defs.assets.get(amb) {
                    let mut a = Map::new();
                    a.insert("path".to_string(), Value::String(asset.path.clone()));
                    a.insert("type".to_string(), Value::String(asset.asset_type.clone()));
                    s.insert("ambient".to_string(), Value::Object(a));
                } else {
                    s.insert("ambient".to_string(), Value::String(amb.clone()));
                }
            }
            if !scene.characters.is_empty() {
                let chars: Vec<Value> = scene.characters.iter().map(|c| Value::String(c.clone())).collect();
                s.insert("characters".to_string(), Value::Array(chars));
            }
            (name.clone(), Value::Object(s))
        }).collect();
        root.insert("scenes".to_string(), Value::Object(scenes));
    }

    // Actions
    if !defs.actions.is_empty() {
        let actions: Map<String, Value> = defs.actions.iter().map(|(name, action)| {
            let mut a = Map::new();
            if !action.params.is_empty() {
                let params: Vec<Value> = action.params.iter().map(|p| {
                    let mut pm = Map::new();
                    pm.insert("name".to_string(), Value::String(p.name.clone()));
                    pm.insert("type".to_string(), Value::String(p.param_type.clone()));
                    pm.insert("required".to_string(), Value::Bool(p.required));
                    if let Some(ref def) = p.default {
                        pm.insert("default".to_string(), value_to_json(def));
                    }
                    Value::Object(pm)
                }).collect();
                a.insert("params".to_string(), Value::Array(params));
            }
            a.insert("returns".to_string(), Value::String(action.returns.clone()));
            (name.clone(), Value::Object(a))
        }).collect();
        root.insert("actions".to_string(), Value::Object(actions));
    }

    // State
    if !defs.state.is_empty() {
        let state: Map<String, Value> = defs.state.iter().map(|(name, sv)| {
            let mut s = Map::new();
            s.insert("type".to_string(), Value::String(sv.var_type.clone()));
            if let Some(ref def) = sv.default {
                s.insert("default".to_string(), value_to_json(def));
            }
            if let Some(min) = sv.min {
                s.insert("min".to_string(), Value::Number(min.into()));
            }
            if let Some(max) = sv.max {
                s.insert("max".to_string(), Value::Number(max.into()));
            }
            (name.clone(), Value::Object(s))
        }).collect();
        root.insert("state".to_string(), Value::Object(state));
    }

    // Events
    if !defs.events.is_empty() {
        let events: Map<String, Value> = defs.events.iter().map(|(name, event)| {
            let mut e = Map::new();
            if !event.params.is_empty() {
                let params: Vec<Value> = event.params.iter().map(|p| {
                    let mut pm = Map::new();
                    pm.insert("name".to_string(), Value::String(p.name.clone()));
                    pm.insert("type".to_string(), Value::String(p.param_type.clone()));
                    pm.insert("required".to_string(), Value::Bool(p.required));
                    Value::Object(pm)
                }).collect();
                e.insert("params".to_string(), Value::Array(params));
            }
            (name.clone(), Value::Object(e))
        }).collect();
        root.insert("events".to_string(), Value::Object(events));
    }

    Value::Object(root)
}

fn value_to_json(val: &serde_yaml::Value) -> Value {
    match val {
        serde_yaml::Value::Null => Value::Null,
        serde_yaml::Value::Bool(b) => Value::Bool(*b),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Number(i.into())
            } else if let Some(f) = n.as_f64() {
                serde_json::Number::from_f64(f).map(Value::Number).unwrap_or(Value::Null)
            } else {
                Value::Null
            }
        }
        serde_yaml::Value::String(s) => Value::String(s.clone()),
        _ => Value::Null,
    }
}