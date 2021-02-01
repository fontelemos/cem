#[cfg(test)]
use cem::helpers::{apply_processing_rules, broadcast_to_peers, PeerMap};
use cem::state::handler::Block;
use futures::channel::mpsc::unbounded;
use serde_json::Value;
use std::sync::Mutex;
use std::{collections::HashMap, net::SocketAddr};
use tungstenite::protocol::Message;

fn build_generic_block(time: u64) -> String {
    format!(
        r#"{{
        "id": "abcd1234",
        "content": {{
            "time": {},
            "text": "hello world!" 
        }}
      }}"#,
        time
    )
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

#[tokio::test]
async fn broadcast_to_peers_is_broadcasting() {
    let dummy_msg = "dummy!!!!";
    let error_msg = "I did not receive the broadcast :(";
    let block: Block = Block::convert(&build_generic_block(3000)).unwrap();
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
