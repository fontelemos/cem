use crate::state::handler::{is_older_than, merge, Block};
use env_logger::Env;
use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::{pin_mut, sink::SinkExt, stream::TryStreamExt, StreamExt};
use log::{debug, error, info, trace};
use serde_json::Value;
use std::sync::{Arc, Mutex, RwLock};
use std::{collections::HashMap, net::SocketAddr};
use tokio::net::TcpStream;
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

pub async fn handle_connection(stream: TcpStream, state_lock: StateLock, peer_map: PeerMap) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);
    if let Some(state_snapshot) = generate_state_snapshot(&state_lock) {
        let (mut outgoing, incoming) = ws_stream.split();
        outgoing.send(Message::from(state_snapshot)).await.unwrap();
        trace!("snapshot sent to {}", addr);

        let (tx, rx) = unbounded();
        peer_map.lock().unwrap().insert(addr, tx.clone());

        let write_and_broadcast = incoming.try_for_each(|msg| {
            let response = msg.to_text().unwrap();
            println!("Received a message from {}: {}", addr, response);
            update_state(response, &state_lock).and_then(|updated_block| {
                broadcast_to_peers(&peer_map, addr, &updated_block);
                Some(())
            });
            futures::future::ok(())
        });

        let receive_from_others = rx.map(Ok).forward(outgoing);
        pin_mut!(write_and_broadcast, receive_from_others);
        futures::future::select(write_and_broadcast, receive_from_others).await;

        peer_map.lock().unwrap().remove(&addr);
    } else {
        error!("couldn't build state snapshot! State is corrupted!");
        error!("corrupted state: {:?}", state_lock.read().unwrap());
    }

    println!("{} disconnected", &addr);
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

pub fn broadcast_to_peers(peer_map: &PeerMap, broadcaster_addr: SocketAddr, block: &Block) -> bool {
    let peers = peer_map.lock().unwrap();
    let broadcast_recipients = peers
        .iter()
        .filter(|(peer_addr, _)| peer_addr != &&broadcaster_addr)
        .map(|(_, ws_sink)| ws_sink);

    let broadcast_msg_content: String = block.to_json_string();
    let broadcast_msg = Message::from(broadcast_msg_content.clone());

    let broadcast_status = broadcast_recipients
        .map(|recp| recp.unbounded_send(broadcast_msg.clone()))
        .fold(true, |acc, status| {
            if status.is_err() {
                error!("unable to broadcast: {:?}", status);
            }
            acc && status.is_ok()
        });

    debug!("DONE broadcasting");
    broadcast_status
}

pub fn update_state(response: &str, state_lock: &StateLock) -> Option<Block> {
    let block = Block::convert(response)?;
    let target_id: String = block.id.clone();

    let mut state = state_lock.write().unwrap();
    let stored_content_option = state.get(&target_id);

    let updated_content = apply_processing_rules(stored_content_option, &block.content)?;

    state.insert(target_id, updated_content.clone());

    let updated_block = block.create_copy_with(updated_content);

    Some(updated_block)
}
