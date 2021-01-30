use std::{io::Error, collections::HashMap, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use std::sync::{RwLock, Arc, Mutex};
use futures_util::{StreamExt, stream::TryStreamExt, pin_mut};
use futures_channel::mpsc::{unbounded, UnboundedSender};
use tungstenite::protocol::Message;
use serde_json::{Value};
use log::{info, debug};


use cem::helpers::{ init_log };
use cem::state::handler::{build_state, is_older_than, merge, Block};


type StateLock = Arc<RwLock<HashMap<String, Value>>>;
type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_log();
    info!("Starting CEM :D");
    let addr = "127.0.0.1:9001".to_string();
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    let state_lock: StateLock = Arc::new(RwLock::new(build_state()));
    let peer_map: PeerMap = PeerMap::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, state_lock.clone(), peer_map.clone()));
    }
    Ok(())
}


async fn handle_connection(stream: TcpStream, state_lock: StateLock, peer_map: PeerMap) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let write_and_broadcast = incoming.try_for_each(|msg| {
        let response = msg.to_text().unwrap();
        println!("Received a message from {}: {}", addr, response);

        let block = Block::from(response);
        if block.is_valid() {
            debug!("block is valid!");
            let mut state = state_lock.write().unwrap();
            debug!("Got the lock");

            let mut updated_content: Value = Value::default();
            let updated_id: String = block.id;

            match state.get(&updated_id) {
                Some(old_content) => {
                    if is_older_than(&old_content, &block.content) {
                        updated_content = merge(&old_content, &block.content);
                        debug!("updated state");
                    }
                }
                None => {
                    updated_content = block.content;
                    debug!("new state registered");
                }
            }
            state.insert(updated_id.clone(), updated_content.clone());
            drop(state);
            debug!("state lock RELEASED");

            // broadcast state change
            let peers = peer_map.lock().unwrap();
            let broadcast_recipients = peers
                .iter()
                .filter(|(peer_addr, _)| peer_addr != &&addr)
                .map(|(_, ws_sink)| ws_sink);
            
            let broadcast_msg: String = Block::default()
                .update(updated_id, updated_content)
                .to_json_string();

            for recp in broadcast_recipients {
                recp.unbounded_send(Message::from(broadcast_msg.clone())).unwrap();
            }
            debug!("DONE broadcasting");
        }
        futures::future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(write_and_broadcast, receive_from_others);
    futures::future::select(write_and_broadcast, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}