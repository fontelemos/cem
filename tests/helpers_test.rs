#[cfg(test)]
use cem::helpers::{ apply_processing_rules };
use cem::state::handler::{ Block };
use serde_json::{Value};


fn build_generic_block(time: u64) -> String {
    format!(r#"{{
        "id": "abcd1234",
        "content": {{
            "time": {},
            "text": "hello world!" 
        }}
      }}"#, time)
}

#[test]
fn processing_rules_should_return_same_if_none_stored() {
    let received_block: Block = Block::convert(&build_generic_block(3000)).unwrap();
    let stored_content: Option<&Value> = None;
    let result_content = apply_processing_rules(stored_content, &received_block.content).unwrap();
    assert_eq!(result_content, received_block.content);
}

#[test]
fn processing_rules_should_return_none_if_received_is_older() {
    let received_block: Block = Block::convert(&build_generic_block(1000)).unwrap();

    let expected_block = Block::convert(&build_generic_block(3000)).unwrap();
    let stored_content: Option<&Value> = Some(&expected_block.content);

    let result_content = apply_processing_rules(stored_content, &received_block.content);
    assert_eq!(result_content.is_none(), true);
}

#[test]
fn processing_rules_should_return_received_if_received_is_newer() {
    let received_block: Block = Block::convert(&build_generic_block(7000)).unwrap();

    let expected_block = Block::convert(&build_generic_block(3000)).unwrap();
    let stored_content: Option<&Value> = Some(&expected_block.content);

    let result_content = apply_processing_rules(stored_content, &received_block.content).unwrap();
    assert_eq!(result_content, received_block.content);
}