use futures::future::join;
use sauron::prelude::*;
use serde_json;
use std::net::SocketAddr;
use std::thread;
use warp::{http::Response, Filter};
use webapp_client::App;

mod event_service;
mod node_events;

#[tokio::main]
async fn main() {
    tokio::spawn(event_service::start_websocket_server());
    let root = warp::path::end().map(move || "Hello from server");

    let routes = warp::get().and(root);

    let socket: SocketAddr = ([127, 0, 0, 1], 3131).into();
    println!("serve at http://{}:{}", socket.ip(), socket.port());
    warp::serve(routes).run(socket).await;
}
