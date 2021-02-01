use crate::state::handler::{is_older_than, merge, Block};
use env_logger::Env;
use futures::channel::mpsc::UnboundedSender;
use log::debug;
use serde_json::Value;
use std::sync::{Arc, Mutex, RwLock};
use std::{collections::HashMap, net::SocketAddr};
use tungstenite::protocol::Message;

pub type StateLock = Arc<RwLock<HashMap<String, Value>>>;
type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

pub fn init_log() {
    let log_env = Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(log_env);
}

pub fn generate_state_snapshot(state_lock: &StateLock) -> Option<String> {
    let state_snapshot_vec = state_lock
        .read()
        .unwrap()
        .iter()
        .map(|(key, value)| Block {
            id: key.clone(),
            content: value.clone(),
        })
        .fold(vec![], |mut acc, block| {
            acc.push(block);
            acc
        });
    let state_snapshot = serde_json::to_string(&state_snapshot_vec).ok()?;
    debug!("snapshot:{}", state_snapshot);
    Some(state_snapshot)
}

pub fn apply_processing_rules(stored: Option<&Value>, received: &Value) -> Option<Value> {
    let mut updated_content = None;
    match stored {
        Some(old_content) => {
            if is_older_than(&old_content, received) {
                let merged_content = merge(&old_content, received);
                updated_content = Some(merged_content)
            }
        }
        None => {
            updated_content = Some(received.clone());
        }
    }
    debug!("DONE processing");
    updated_content
}

pub fn broadcast_to_peers(peer_map: &PeerMap, broadcaster_addr: SocketAddr, updated_block: &Block) {
    let peers = peer_map.lock().unwrap();
    let broadcast_recipients = peers
        .iter()
        .filter(|(peer_addr, _)| peer_addr != &&broadcaster_addr)
        .map(|(_, ws_sink)| ws_sink);
    let broadcast_msg: String = updated_block.to_json_string();

    for recp in broadcast_recipients {
        recp.unbounded_send(Message::from(broadcast_msg.clone()))
            .unwrap();
    }
    debug!("DONE broadcasting");
}

pub fn update_state_and_broadcast(
    response: &str,
    state_lock: &StateLock,
    peer_map: &PeerMap,
    addr: SocketAddr,
) -> Option<Block> {
    let block = Block::convert(response)?;
    let target_id: String = block.id.clone();

    let mut state = state_lock.write().unwrap();
    let stored_content_option = state.get(&target_id);

    let updated_content = apply_processing_rules(stored_content_option, &block.content)?;

    state.insert(target_id, updated_content.clone());

    let updated_block = block.create_copy_with(updated_content);
    broadcast_to_peers(&peer_map, addr, &updated_block);

    Some(updated_block)
}
