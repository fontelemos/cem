use cem::state::handler::{convert, merge};
#[cfg(test)]
use serde_json::json;

#[test]
fn state_from_method_test() {
  let original_state_str = r#"{
      "name": {
        "hash": "2230o363glhhh64",
        "generation": 243,
        "display_name": "Uruk the big barbarian" 
      },
    }"#;
  let new_state = json!(
    r#"{
      "name": {
        "hash": "2230o363glhhh64",
        "generation": 244,
        "display_name": "Uruk the very big barbarian!" 
      },
    }"#
  );
  let expected_state = json!(
    r#"{
      "name": {
        "hash": "2230o363glhhh64",
        "generation": 244,
        "display_name": "Uruk the very big barbarian!" 
      },
    }"#
  );
  let state = convert(original_state_str);
  let result = merge(state, new_state);
  assert_eq!(expected_state, result)
}
