use env_logger::Env;

use std::{collections::HashMap, net::SocketAddr};
use std::sync::{RwLock, Arc, Mutex};
use futures::channel::mpsc::{UnboundedSender};
use tungstenite::protocol::Message;
use serde_json::{Value};
use log::{debug};
use crate::state::handler::{is_older_than, merge, Block};

type StateLock = Arc<RwLock<HashMap<String, Value>>>;
type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;


pub fn init_log() {
    let log_env = Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(log_env);
}

fn apply_processing_rules(stored_block_option: Option<&Value>, received_block: Block) -> Option<Block> {
    let mut updated_block = None;
    match stored_block_option {
        Some(old_content) => {
            if is_older_than(&old_content, &received_block.content) {
                let merged_content = merge(&old_content, &received_block.content);
                updated_block = Some(received_block.create_copy_with(merged_content));
            }
        }
        None => {
            updated_block = Some(received_block.clone());
        }
    }
    debug!("DONE processing");
    updated_block
}

fn start_broadcast(peer_map: &PeerMap, addr: SocketAddr, updated_block: &Block) {
    let peers = peer_map.lock().unwrap();
    let broadcast_recipients = peers
        .iter()
        .filter(|(peer_addr, _)| peer_addr != &&addr)
        .map(|(_, ws_sink)| ws_sink);
    
    let broadcast_msg: String = updated_block.to_json_string();

    for recp in broadcast_recipients {
        recp.unbounded_send(Message::from(broadcast_msg.clone())).unwrap();
    }
    debug!("DONE broadcasting");
}

pub fn update_state_and_broadcast(response: &str, state_lock: &StateLock, peer_map: &PeerMap, addr: SocketAddr) {
    if let Some(block) = Block::convert(response) {
        let target_id: String = block.id.clone();

        let mut state = state_lock.write().unwrap();
        let stored_block_option = state.get(&target_id);

        if let Some(updated_block) = apply_processing_rules(stored_block_option, block) {
            state.insert(target_id, updated_block.content.clone());
            start_broadcast(&peer_map, addr, &updated_block);
        }
    }
}