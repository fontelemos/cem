#[cfg(test)]
use log::{info, trace};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use cem::helpers::init_log;
use cem::state::handler::{build_state, is_older_than, merge, Block};

fn build_expected_content<'a>() -> &'a str {
  r#"{
    "id": "soiadj9087asdbnjk",
    "content": {
      "time": 20,
      "hash": "2230o363glhhh64",
      "generation": 243,
      "display_name": "Uruk the very VERY BIIIGGGGG barbarian!!!!!",
      "attack": {
        "axe": "9001dmg"
      }
    }
  }"#
}

fn build_response_array_mock<'a>() -> Vec<&'a str> {
  let responses1: Vec<&str> = vec![
    r#"{
        "id": "soiadj9087asdbnjk",
        "content": {
          "time": 2,
          "hash": "2230o363glhhh64",
          "generation": 243,
          "display_name": "Uruk the very big barbarian!!!!!",
          "attack": {
            "axe": "100dmg"
          }
        }
      }"#,
    r#"{
            "id": "soiadj9087asdbnjk",
            "content": {
              "time": 0, 
              "hash": "2230o363glhhh64",
              "generation": 243,
              "display_name": "Uruk the big barbarian",
              "attack": {
                "axe": "100dmg"
              }
            }
          }"#,
    r#"{
            "id": "dxx6903bkdass",
            "content": {
              "time": 1,
              "hash": "2230",
              "generation": 23,
              "display_name": "Bob the mage"
            }
          }"#,
  ];

  let number_of_response_copies = 100;
  let mut responses_temp = vec![];
  for index in 0..number_of_response_copies {
    let mut responses2: Vec<&str> = responses1.clone();
    if index == number_of_response_copies / 2 {
      responses2.push(build_expected_content())
    }
    responses_temp.push(responses2);
  }
  let responses = responses_temp.concat();
  responses
}

#[test]
fn concurrent_insert() {
  init_log();
  let responses: Vec<&str> = build_response_array_mock();
  let n_of_responses = responses.len();
  let rt = tokio::runtime::Runtime::new().unwrap();
  let now = Instant::now();

  rt.block_on(async {
    let state_lock = Arc::new(RwLock::new(build_state()));
    let counter_lock = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for response in responses {
      let current_state_lock = state_lock.clone();
      let current_counter_lock = counter_lock.clone();

      handles.push(tokio::spawn(async move {
        let block = Block::convert(response).unwrap();
        let mut state = current_state_lock.write().unwrap();
        trace!("[{}] Got the lock", block.content["time"].clone());
        match state.get(&block.id.clone()) {
          Some(old_content) => {
            if is_older_than(&old_content, &block.content) {
              let merged_content = merge(&old_content, &block.content);
              state.insert(block.id.clone(), merged_content);
            }
          }
          None => {
            state.insert(block.id.clone(), block.content);
          }
        }
        let mut counter = current_counter_lock.lock().unwrap();
        *counter += 1;
      }));
    }
    futures::future::join_all(handles).await;

    let current_state = state_lock.write().unwrap();
    let counter = counter_lock.lock().unwrap();

    let content = current_state.get("soiadj9087asdbnjk").unwrap();
    let expected_block = Block::convert(build_expected_content()).unwrap();

    info!("tokio processing took: {} ms", now.elapsed().as_millis());
    info!("{} items processed", *counter);
    assert_eq!(content, &expected_block.content);
    assert_eq!(*counter, n_of_responses);
  });
}
