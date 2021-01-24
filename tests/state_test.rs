#[cfg(test)]
use cem::state::handler::{build_state, merge, Block};
use serde_json::json;

#[test]
fn content_merge_test() {
  let original_state_str = r#"{
    "id": "soiadj9087asdbnjk",
    "content": {
      "hash": "2230o363glhhh64",
      "generation": 243,
      "display_name": "Uruk the big barbarian",
      "attack": {
        "axe": "100dmg"
      }
    }
  }"#;
  let new_state = json!(
    r#"{
      "id": "soiadj9087asdbnjk",
      "content": {
        "hash": "2230o363glhhh64",
        "generation": 244,
        "display_name": "Uruk the very big barbarian!",
        "attack": {}
      }
    }"#
  );
  let expected_state = json!(
    r#"{
      "id": "soiadj9087asdbnjk",
      "content": {
        "hash": "2230o363glhhh64",
        "generation": 244,
        "display_name": "Uruk the very big barbarian!",
        "attack": {}
      }
    }"#
  );
  let block = Block::from(original_state_str);
  let result = merge(block.content.clone(), new_state);
  assert_eq!(expected_state, result)
}

#[test]
fn receive_and_store_state() {
  let original_state_str = r#"{
      "id": "soiadj9087asdbnjk",
      "content": {
        "hash": "2230o363glhhh64",
        "generation": 243,
        "display_name": "Uruk the big barbarian" 
      }
    }"#;
  let mut state = build_state();
  let block = Block::from(original_state_str);
  assert!(block.is_valid());
  assert_eq!(state.insert(block.id.clone(), block.content.clone()), None);
  let expected_id: String = String::from("soiadj9087asdbnjk");
  assert_eq!(block.id, expected_id);
}

#[test]
fn invalid_state() {
  let state1_str = r#"{
      "id": "soiadj9087asdbnjk",
      "contento": {
        "hash": "2230o363glhhh64",
        "generation": 243,
        "display_name": "Uruk the big barbarian" 
      }
    }"#;
  let state2_str = r#"{
      "content": {
        "hash": "2230o363glhhh64",
        "generation": 243,
        "display_name": "Uruk the big barbarian" 
      }
    }"#;
  let state3_str = r#"{
      "id": "",
      "content": {
        "hash": "2230o363glhhh64",
        "generation": 243,
        "display_name": "Uruk the big barbarian" 
      }
    }"#;
  let block1 = Block::from(state1_str);
  let block2 = Block::from(state2_str);
  let block3 = Block::from(state3_str);
  assert_eq!(block1.is_valid(), false);
  assert_eq!(block2.is_valid(), false);
  assert_eq!(block3.is_valid(), false);
}
