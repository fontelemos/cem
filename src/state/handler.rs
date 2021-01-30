use json_patch::merge as json_merge;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::collections::HashMap;


pub fn merge(content: &Value, patch: &Value) -> Value {
    let mut original_copy = content.clone();
    json_merge(&mut original_copy, patch);
    original_copy
}

pub fn build_state() -> HashMap<String, Value> {
    HashMap::new()
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Block {
    pub id: String,
    pub content: Value
}

impl From<&str> for Block {
    fn from(block_string: &str) -> Block {
        serde_json::from_str(block_string)
            .unwrap_or(Block::default())
    }
}

impl Block {
    pub fn is_valid(&self) -> bool {
        let invalid_id = String::from("");
        self.id != invalid_id
    }

    pub fn update(&mut self, block_id: String, block_content: Value) -> &Block {
        self.id = block_id;
        self.content = block_content;
        self
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or("".to_string())
    }
}

#[derive(Serialize, Deserialize, Default)]
struct TimeJson {
    time: u64
}

pub fn is_older_than(old: &Value, new: &Value) -> bool {
    let old_time: TimeJson = serde_json::from_value(old.clone())
        .unwrap_or(TimeJson::default());
    let new_time: TimeJson = serde_json::from_value(new.clone())
        .unwrap_or(TimeJson::default());
    new_time.time >= old_time.time
}