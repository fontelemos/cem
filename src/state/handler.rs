use json_patch::merge as json_merge;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub fn merge(content: &Value, patch: &Value) -> Value {
    let mut original_copy = content.clone();
    json_merge(&mut original_copy, patch);
    original_copy
}

pub fn build_state() -> HashMap<String, Value> {
    HashMap::new()
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Block {
    pub id: String,
    pub content: Value,
}

impl Block {
    pub fn convert(block_string: &str) -> Option<Block> {
        serde_json::from_str(block_string).ok()
    }

    pub fn create_copy_with(&self, new_content: Value) -> Block {
        Block {
            id: self.id.clone(),
            content: new_content,
        }
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or("".to_string())
    }
}

#[derive(Serialize, Deserialize, Default)]
struct TimeJson {
    time: u64,
}

pub fn is_older_than(old: &Value, new: &Value) -> bool {
    let old_time: TimeJson = serde_json::from_value(old.clone()).unwrap_or(TimeJson::default());
    let new_time: TimeJson = serde_json::from_value(new.clone()).unwrap_or(TimeJson::default());
    new_time.time >= old_time.time
}
