use json_patch::merge as json_merge;
use serde_json::{json, Value};

pub fn convert(content: &str) -> Value {
    json!(content)
}

pub fn merge(content: Value, patch: Value) -> Value {
    let mut original_copy = content.clone();
    json_merge(&mut original_copy, &patch);
    original_copy
}
