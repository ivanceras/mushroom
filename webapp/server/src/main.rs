use futures::future::join;
use sauron::prelude::*;
use serde_json;
use std::net::SocketAddr;
use std::thread;
use warp::{http::Response, Filter};
use webapp_client::App;

mod event_service;
mod node_events;
mod page;

pub(crate) fn client_bg_wasm_content() -> Vec<u8> {
    include_bytes!("../../client/pkg/webapp_client_bg.wasm").to_vec()
}

#[tokio::main]
async fn main() {
    tokio::spawn(event_service::start_websocket_server());
    let root = warp::path::end().map(move || {
        let index_page = page::index().render_to_string();
        Response::builder().body(index_page)
    });

    let wasm_file = warp::path!("pkg" / "webapp_client_bg.wasm").map(move || {
        Response::builder()
            .header("Content-Type", "application/wasm")
            .body(client_bg_wasm_content())
    });

    let routes = warp::get().and(root.or(wasm_file));

    let socket: SocketAddr = ([127, 0, 0, 1], 3131).into();
    println!("serve at http://{}:{}", socket.ip(), socket.port());
    warp::serve(routes).run(socket).await;
}
