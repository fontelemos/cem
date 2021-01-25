#[cfg(test)]
use std::sync::RwLock;
use std::sync::{Arc, Barrier};
use log::info;

use cem::state::handler::{build_state, is_older_than, merge, Block};
use cem::threading::threadpool::ThreadPool;
use cem::helpers::init_log;

#[test]
fn concurrent_insert() {
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

  let responses2: Vec<&str> = responses1.iter().copied().rev().collect();
  let responses3: Vec<&str> = responses1.clone();
  let responses: Vec<&str> = [responses1, responses2, responses3].concat();

  let expected_block = Block::from(
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
  );

  init_log();
  let n_jobs = 9;
  let state_lock = Arc::new(RwLock::new(build_state()));
  let pool = ThreadPool::new(n_jobs);

  let barrier = Arc::new(Barrier::new(n_jobs + 1));

  for response in responses {
    let current_state_lock = state_lock.clone();
    let barrier = barrier.clone();

    pool.execute(move || {
      let block = Block::from(response);
      let mut state = current_state_lock.write().unwrap();
      info!("[{}] Got the lock", block.content["time"].clone());
      match state.get(&block.id.clone()) {
        Some(old_content) => {
          if is_older_than(old_content.clone(), block.content.clone()) {
            let merged_content = merge(old_content.clone(), block.content);
            state.insert(block.id.clone(), merged_content);
          }
        }
        None => {
          state.insert(block.id.clone(), block.content);
        }
      }
      drop(state);
      barrier.wait(); // wait other threads to finish
    })
  }
  barrier.wait();
  let current_state_lock = state_lock.write().unwrap();
  let content = current_state_lock.get("soiadj9087asdbnjk").unwrap();
  assert_eq!(content, &expected_block.content);
}
