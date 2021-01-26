use std::net::TcpListener;
use std::sync::Arc;
use std::sync::RwLock;
// use tungstenite::server::accept;
// use tungstenite::Message;
use log::info;

use cem::helpers::{ init_log };
//use cem::threading::threadpool::ThreadPool;

fn main() {
    println!("Hello, world!");
    init_log();
    // let game_state_lock = Arc::new(RwLock::new(game_state));
    // let pool = ThreadPool::new(3);

    // info!("Hello, world! Starting websocket seeeeeeerver on 9001");
    // let server = TcpListener::bind("127.0.0.1:9001").unwrap();

    // for stream in server.incoming() {
    //     // let current_state_lock = game_state_lock.clone();

    //     pool.execute(move || {
    //         let mut websocket = accept(stream.unwrap()).unwrap();
    //         while let Ok(payload) = websocket.read_message() {
    //             if payload.is_binary() || payload.is_text() {
    //                 info!("======= Start =======");
    //                 info!("message: {}", payload);
    //                 websocket.write_message(Message::Text(String::from("received"))).unwrap();
    //             }
    //         }
    //     })
    // }
}
