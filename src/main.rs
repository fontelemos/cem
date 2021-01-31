use std::{io::Error, collections::HashMap};
use tokio::net::{TcpListener, TcpStream};
use std::sync::{RwLock, Arc, Mutex};
use futures::{StreamExt, stream::TryStreamExt, pin_mut};
use futures::channel::mpsc::{unbounded};
use log::{info};
use cem::helpers::{ init_log, update_state_and_broadcast, PeerMap, StateLock };
use cem::state::handler::{build_state};


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

        update_state_and_broadcast(response, &state_lock, &peer_map, addr);

        futures::future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(write_and_broadcast, receive_from_others);
    futures::future::select(write_and_broadcast, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}