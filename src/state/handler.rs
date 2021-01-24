use json_patch::merge as json_merge;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

fn convert(content: &str) -> Value {
    json!(content)
}

pub fn merge(content: Value, patch: Value) -> Value {
    let mut original_copy = content.clone();
    json_merge(&mut original_copy, &patch);
    original_copy
}

pub fn build_state() -> HashMap<String, Value> {
    HashMap::new()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub id: String,
    pub content: Value
}

impl From<&str> for Block {
    fn from(block_string: &str) -> Block {
        let empty_block = Block { 
            id: "" .to_string(),
            content: convert("{}")
        };
        let block: Block = serde_json::from_str(block_string)
            .unwrap_or(empty_block);
        block
    }
}

impl Block {
    pub fn is_valid(&self) -> bool {
        let invalid_id = String::from("");
        self.id != invalid_id
    }
}