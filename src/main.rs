use cem::helpers::{handle_connection, init_log, PeerMap, StateLock};
use cem::state::handler::build_state;
use log::info;
use std::sync::{Arc, Mutex, RwLock};
use std::{collections::HashMap, io::Error};
use tokio::net::TcpListener;

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
        tokio::spawn(handle_connection(
            stream,
            state_lock.clone(),
            peer_map.clone(),
        ));
    }
    Ok(())
}
