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
    pub asset_type: String,
    pub path: String,

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

/// Action/event parameter.
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