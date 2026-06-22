//! Placeholder — types defined in Task 2.
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Definitions {
    pub version: i64,
    #[serde(default)]
    pub assets: HashMap<String, ()>,
}