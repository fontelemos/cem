#[cfg(test)]
use cem::helpers::{apply_processing_rules, broadcast_to_peers, generate_state_snapshot, PeerMap, StateLock};
use cem::state::handler::{build_state, Block};
use futures::channel::mpsc::unbounded;
use serde_json::Value;
use std::sync::{Mutex, Arc, RwLock};
use std::{collections::HashMap, net::SocketAddr};
use tungstenite::protocol::Message;

fn build_generic_block(id: &str, time: u64) -> String {
    format!(
        r#"{{
        "id": "{}",
        "content": {{
            "time": {},
            "text": "hello world!" 
        }}
      }}"#,
      id, time
    )
}

#[test]
fn processing_rules_should_return_same_if_none_stored() {
    let received_block: Block = Block::convert(&build_generic_block("mykey", 3000)).unwrap();
    let stored_content: Option<&Value> = None;
    let result_content = apply_processing_rules(stored_content, &received_block.content).unwrap();
    assert_eq!(result_content, received_block.content);
}

#[test]
fn processing_rules_should_return_none_if_received_is_older() {
    let received_block: Block = Block::convert(&build_generic_block("mykey", 1000)).unwrap();

    let expected_block = Block::convert(&build_generic_block("mykey", 3000)).unwrap();
    let stored_content: Option<&Value> = Some(&expected_block.content);

    let result_content = apply_processing_rules(stored_content, &received_block.content);
    assert_eq!(result_content.is_none(), true);
}

#[test]
fn processing_rules_should_return_received_if_received_is_newer() {
    let received_block: Block = Block::convert(&build_generic_block("mykey", 7000)).unwrap();

    let expected_block = Block::convert(&build_generic_block("mykey", 3000)).unwrap();
    let stored_content: Option<&Value> = Some(&expected_block.content);

    let result_content = apply_processing_rules(stored_content, &received_block.content).unwrap();
    assert_eq!(result_content, received_block.content);
}

#[tokio::test]
async fn broadcast_to_peers_is_broadcasting() {
    let dummy_msg = "dummy!!!!";
    let error_msg = "I did not receive the broadcast :(";
    let block: Block = Block::convert(&build_generic_block("mykey", 3000)).unwrap();
    let addr1: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let addr2: SocketAddr = "127.0.0.1:7777".parse().unwrap();

    let peer_map: PeerMap = PeerMap::new(Mutex::new(HashMap::new()));
    let mut peer_map_lock = peer_map.lock().unwrap();

    let (tx1, rx1) = unbounded();
    let (tx2, rx2) = unbounded();
    peer_map_lock.insert(addr1, tx1.clone());
    peer_map_lock.insert(addr2, tx2.clone());

    drop(peer_map_lock);

    broadcast_to_peers(&peer_map.clone(), addr1, &block);
    tx1.unbounded_send(Message::from(dummy_msg)).unwrap(); // send dummy message to broadcaster
    tx2.unbounded_send(Message::from(error_msg)).unwrap(); // send error message to receiver to avoid blocking this thread

    let received_by_addr1 = futures::executor::block_on_stream(rx1).next().unwrap();
    let received_by_addr2 = futures::executor::block_on_stream(rx2).next().unwrap();

    assert_eq!(received_by_addr1.to_string(), dummy_msg.to_string());
    assert_eq!(received_by_addr2.to_string(), block.to_json_string());
}


#[test]
fn generate_state_snapshot_with_1_block_test() {
    let state_lock: StateLock = Arc::new(RwLock::new(build_state()));
    let block1 = Block::convert(&build_generic_block("myid", 12)).unwrap();
    
    let expected_snapshot = serde_json::to_string(&vec![block1.clone()]).unwrap();
    
    let mut state = state_lock.write().unwrap();
    state.insert(block1.id, block1.content);
    drop(state);

    let snapshot = generate_state_snapshot(&state_lock).unwrap();
    assert_eq!(snapshot, expected_snapshot);
}

#[test]
fn generate_state_snapshot_multiple_blocks_test() {
    let state_lock: StateLock = Arc::new(RwLock::new(build_state()));
    let block1 = Block::convert(&build_generic_block("aid", 1)).unwrap();
    let block2 = Block::convert(&build_generic_block("baid", 2)).unwrap();
    let block3 = Block::convert(&build_generic_block("caid", 3)).unwrap();
    
    let mut expected_snapshot: Vec<Block>= vec![block1.clone(), block2.clone(), block3.clone()];
    
    let mut state = state_lock.write().unwrap();
    state.insert(block1.id, block1.content);
    state.insert(block2.id, block2.content);
    state.insert(block3.id, block3.content);
    drop(state);

    let mut snapshot: Vec<Block> = serde_json::from_str(&generate_state_snapshot(&state_lock).unwrap()).unwrap();

    expected_snapshot.sort_by(|b1, b2| {
        let time1: i32 = serde_json::from_value(b1.content["time"].clone()).unwrap();
        let time2: i32 = serde_json::from_value(b2.content["time"].clone()).unwrap();
        time1.cmp(&time2)
    });

    snapshot.sort_by(|b1, b2| {
        let time1: i32 = serde_json::from_value(b1.content["time"].clone()).unwrap();
        let time2: i32 = serde_json::from_value(b2.content["time"].clone()).unwrap();
        time1.cmp(&time2)
    });

    let result: usize = snapshot
        .iter()
        .zip(expected_snapshot.iter())
        .filter(|(block, expected)| block.id != expected.id)
        .collect::<Vec<(&Block, &Block)>>()
        .len();

    assert_eq!(result, 0);
}