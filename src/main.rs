use std::{io::Error, collections::HashMap};
use tokio::net::{TcpListener, TcpStream};
use std::sync::{RwLock, Arc};
use futures_util::{StreamExt, stream::TryStreamExt};
use serde_json::{Value};
use log::{info, debug};
use cem::helpers::{ init_log };
use cem::state::handler::{build_state, is_older_than, merge, Block};

type StateLock = Arc<RwLock<HashMap<String, Value>>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_log();
    info!("Starting CEM :D");
    let addr = "127.0.0.1:9001".to_string();
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    let state_lock: StateLock = Arc::new(RwLock::new(build_state()));

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, state_lock.clone()));
    }
    Ok(())
}


async fn handle_connection(stream: TcpStream, state_lock: StateLock) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (_, read) = ws_stream.split();

    let write_on_state = read.try_for_each(|msg| {
        let response = msg.to_text().unwrap();
        println!("Received a message from {}: {}", addr, response);

        let block = Block::from(response);
        if block.is_valid() {
            debug!("block is valid!");
            let mut state = state_lock.write().unwrap();
            debug!("Got the lock");
            match state.get(&block.id.clone()) {
                Some(old_content) => {
                if is_older_than(old_content.clone(), block.content.clone()) {
                    let merged_content = merge(old_content.clone(), block.content);
                    state.insert(block.id.clone(), merged_content);
                    debug!("updated state")
                }
                }
                None => {
                    state.insert(block.id.clone(), block.content);
                    debug!("new state registered")
                }
            }
        }
        debug!("lock RELEASED");
        futures::future::ok(())
    });

    write_on_state.await.unwrap();
    println!("{} disconnected", &addr);
}