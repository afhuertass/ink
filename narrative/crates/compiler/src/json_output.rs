//! JSON serialization for ink runtime objects.
//!
//! The ink JSON format uses a specific structure:
//! - Containers are JSON arrays. The LAST element is either:
//!   - `null` if the container has no named elements
//!   - a dict with named sub-containers and/or "#f", "#n" fields
//! - String values use "^" prefix: "Hello" → "^Hello"  
//! - Numbers use standard JSON: 5, 5.5
//! - Booleans are integers: true → 1, false → 0
//! - Control commands are strings: "ev", "out", etc.
//! - Special objects use dict notation: {"->": "path"}, {"VAR=": "name"}

use crate::runtime_types::*;
use serde_json::{self, Map, Number, Value};

/// Serialize a Story to a JSON Value.
pub fn serialize_story(story: &Story) -> Value {
    let mut root_dict = Map::new();
    root_dict.insert("inkVersion".to_string(), Value::Number(Number::from(story.ink_version)));

    // Serialize root container
    let root_value = serialize_container(&story.root);
    root_dict.insert("root".to_string(), root_value);

    // Global variable declarations (in root)
    if !story.variables.is_empty() {
        let vars_array: Vec<Value> = story.variables
            .iter()
            .map(|v| serialize_variable_declaration(v))
            .collect();
        root_dict.insert("globalDeclaration".to_string(), Value::Array(vars_array));
    }

    // List declarations
    if !story.lists.is_empty() {
        let lists_array: Vec<Value> = story.lists
            .iter()
            .map(|l| serialize_list_definition(l))
            .collect();
        let mut lists_dict = Map::new();
        lists_dict.insert("lists".to_string(), Value::Array(lists_array));
        root_dict.insert("listDefs".to_string(), Value::Object(lists_dict));
    }

    Value::Object(root_dict)
}

/// Serialize a Story to a JSON string.
pub fn story_to_json(story: &Story) -> String {
    let value = serialize_story(story);
    serde_json::to_string_pretty(&value).unwrap()
}

/// Serialize a Container to a JSON Value.
/// Containers are: [content..., null]
/// or: [content..., {named: ..., #f: n, #n: name}]
fn serialize_container(container: &Container) -> Value {
    let mut content_values: Vec<Value> = container.content
        .iter()
        .map(serialize_object)
        .collect();

    // The final element carries named containers and flags
    let has_named = !container.named.is_empty();
    let has_flags = !container.flags.is_empty();
    let has_name = container.name.is_some();
    let has_extra = has_named || has_flags || has_name || container.visits.is_some() || container.turn_index.is_some();

    let final_element: Value = if has_extra {
        let mut dict = Map::new();

        // Named sub-containers
        for (name, sub_container) in &container.named {
            dict.insert(name.clone(), serialize_container(sub_container));
        }

        // Flags
        if has_flags {
            dict.insert("#f".to_string(), Value::Number(Number::from(container.flags.value())));
        }

        // Name
        if let Some(ref name) = container.name {
            if !container.named.contains_key(name) {
                dict.insert("#n".to_string(), Value::String(name.clone()));
            }
        }

        // Visits
        if let Some(v) = container.visits {
            dict.insert("#v".to_string(), Value::Number(Number::from(v)));
        }

        // Turn index
        if let Some(t) = container.turn_index {
            dict.insert("#t".to_string(), Value::Number(Number::from(t)));
        }

        Value::Object(dict)
    } else {
        Value::Null
    };

    content_values.push(final_element);
    Value::Array(content_values)
}

/// Serialize any InkObject to a JSON Value.
fn serialize_object(obj: &InkObject) -> Value {
    match obj {
        InkObject::Container(c) => serialize_container(c),
        InkObject::ControlCommand(cmd) => Value::String(cmd.as_str().to_string()),
        InkObject::NativeFuncCall(f) => Value::String(f.as_str().to_string()),
        InkObject::Divert(d) => serialize_divert(d),
        InkObject::VariableAssignment(a) => serialize_variable_assignment(a),
        InkObject::VariableReference(r) => serialize_variable_reference(r),
        InkObject::ChoicePoint(c) => serialize_choice_point(c),
        InkObject::Tag(t) => serialize_tag(t),
        InkObject::Void => Value::String("void".to_string()),
        InkObject::String(s) => Value::String(format!("^{}", s)),
        InkObject::DivertTarget(path) => {
            let mut m = Map::new();
            m.insert("^->".to_string(), Value::String(path.clone()));
            Value::Object(m)
        }
        InkObject::VariablePointer { varname, context_index } => {
            let mut m = Map::new();
            m.insert("^var".to_string(), Value::String(varname.clone()));
            m.insert("ci".to_string(), Value::Number(Number::from(*context_index)));
            Value::Object(m)
        }
        InkObject::Int(n) => Value::Number(Number::from(*n)),
        InkObject::Float(f) => {
            // Handle NaN and Infinity specially
            if f.is_nan() {
                Value::String("NaN".to_string())
            } else if f.is_infinite() {
                if f.is_sign_positive() {
                    Value::String("Infinity".to_string())
                } else {
                    Value::String("-Infinity".to_string())
                }
            } else {
                // Use a number value
                serde_json::Number::from_f64(*f)
                    .map(Value::Number)
                    .unwrap_or_else(|| Value::String(f.to_string()))
            }
        }
    }
}

fn serialize_divert(d: &Divert) -> Value {
    let mut m = Map::new();

    if d.is_external {
        // External function call: {"x()": "name", "exArgs": n}
        if let Some(ref target) = d.target {
            m.insert("x()".to_string(), Value::String(target.clone()));
        }
        if let Some(args) = d.external_args {
            m.insert("exArgs".to_string(), Value::Number(Number::from(args)));
        }
    } else if d.is_function_call && d.is_tunnel {
        // Function call with tunnel semantics
        if let Some(ref target) = d.target {
            m.insert("f()".to_string(), Value::String(target.clone()));
        }
    } else if d.is_function_call {
        // Function call: {"f()": "path"}
        if let Some(ref target) = d.target {
            m.insert("f()".to_string(), Value::String(target.clone()));
        }
    } else if d.is_tunnel {
        // Tunnel: {"->t->": "path"}
        if let Some(ref target) = d.target {
            m.insert("->t->".to_string(), Value::String(target.clone()));
        } else {
            m.insert("->t->".to_string(), Value::Null);
        }
    } else if d.is_variable {
        // Variable divert: {"->": "varname", "var": true}
        if let Some(ref target) = d.target {
            m.insert("->".to_string(), Value::String(target.clone()));
        }
        m.insert("var".to_string(), Value::Bool(true));
    } else {
        // Standard divert: {"->": "path"}
        if let Some(ref target) = d.target {
            m.insert("->".to_string(), Value::String(target.clone()));
        } else {
            m.insert("->".to_string(), Value::Null);
        }
    }

    if d.is_conditional {
        m.insert("c".to_string(), Value::Bool(true));
    }

    Value::Object(m)
}

fn serialize_variable_assignment(a: &VariableAssignment) -> Value {
    let mut m = Map::new();
    let key = if a.is_global { "VAR=" } else { "temp=" };
    m.insert(key.to_string(), Value::String(a.varname.clone()));

    if a.is_global {
        m.insert("re".to_string(), Value::Bool(true));
    }

    Value::Object(m)
}

fn serialize_variable_reference(r: &VariableReference) -> Value {
    let mut m = Map::new();
    if r.is_read_count {
        m.insert("CNT?".to_string(), Value::String(r.varname.clone()));
    } else {
        m.insert("VAR?".to_string(), Value::String(r.varname.clone()));
    }
    if r.context_index >= 0 {
        m.insert("ci".to_string(), Value::Number(Number::from(r.context_index)));
    }
    Value::Object(m)
}

fn serialize_choice_point(c: &ChoicePoint) -> Value {
    let mut m = Map::new();
    m.insert("*".to_string(), Value::String(c.target_path.clone()));
    m.insert("flg".to_string(), Value::Number(Number::from(c.flags.value())));
    Value::Object(m)
}

fn serialize_tag(t: &Tag) -> Value {
    let mut m = Map::new();
    m.insert("#".to_string(), Value::String(t.text.clone()));
    Value::Object(m)
}

fn serialize_variable_declaration(v: &VariableAssignment) -> Value {
    let mut m = Map::new();
    m.insert("VAR=".to_string(), Value::String(v.varname.clone()));
    m.insert("initialValue".to_string(), Value::Null);
    m.insert("isGlobal".to_string(), Value::Bool(true));
    Value::Object(m)
}

fn serialize_list_definition(l: &ListDefinition) -> Value {
    let mut m = Map::new();
    m.insert("name".to_string(), Value::String(l.name.clone()));
    
    let items: Vec<Value> = l.items.iter().map(|item| {
        let mut item_map = Map::new();
        item_map.insert("name".to_string(), Value::String(item.name.clone()));
        item_map.insert("value".to_string(), Value::Number(Number::from(item.value)));
        Value::Object(item_map)
    }).collect();
    
    m.insert("items".to_string(), Value::Array(items));
    Value::Object(m)
}

// -------------------------------------------------------------------------------------------------
// Tests
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_serialization() {
        let story = Story::new();
        let json = story_to_json(&story);
        assert!(json.contains("\"inkVersion\": 21"));
    }

    #[test]
    fn test_knot_in_output() {
        let mut story = Story::new();
        let mut knot = Container::new();
        knot.push(InkObject::String("Hello world".to_string()));
        knot.push(InkObject::ControlCommand(ControlCommand::End));
        story.add_knot("greeting", knot);

        let json = story_to_json(&story);
        assert!(json.contains("greeting"));
    }

    #[test]
    fn test_divert_serialization() {
        let obj = InkObject::Divert(Divert::new("somewhere"));
        let value = serialize_object(&obj);
        let s = serde_json::to_string(&value).unwrap();
        assert!(s.contains("\"->\":\"somewhere\""));
    }

    #[test]
    fn test_conditional_divert() {
        let mut d = Divert::new("target");
        d.is_conditional = true;
        let obj = InkObject::Divert(d);
        let value = serialize_object(&obj);
        let s = serde_json::to_string(&value).unwrap();
        assert!(s.contains("\"c\":true"));
    }

    #[test]
    fn test_function_divert() {
        let obj = InkObject::Divert(Divert::function("my_func"));
        let value = serialize_object(&obj);
        let s = serde_json::to_string(&value).unwrap();
        assert!(s.contains("\"f()\":\"my_func\""));
    }

    #[test]
    fn test_choice_point() {
        let cp = ChoicePoint::new(".^.c").has_start_content().has_choice_only_content();
        let obj = InkObject::ChoicePoint(cp);
        let value = serialize_object(&obj);
        let s = serde_json::to_string(&value).unwrap();
        assert!(s.contains("\"*\":\".^.c\""));
        assert!(s.contains("\"flg\":22")); // start + choice-only + once-only
    }
}