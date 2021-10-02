/*
    Copyright 2019 Supercomputing Systems AG
    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

///! Very simple example that shows how to subscribe to events.
///! This also demonstrate to expose this events on a websocket.
use std::sync::mpsc::channel;

use clap::{load_yaml, App};
use codec::Decode;
use log::{debug, error};
use sp_core::sr25519;
use sp_core::H256 as Hash;

// This module depends on node_runtime.
// To avoid dependency collisions, node_runtime has been removed from the substrate-api-client library.
// Replace this crate by your own if you run a custom substrate node to get your custom events.
use node_template_runtime::Event;

use mushroom::rpc::WsRpcClient;
use mushroom::utils::FromHexString;
use mushroom::Api;

use std::net::TcpListener;
use tungstenite::accept;

pub async fn start_websocket_server() {
    let url = "127.0.0.1:9002";
    println!("spawning websocket at: {}", url);
    let server = TcpListener::bind("127.0.0.1:9002").unwrap();

    for stream in server.incoming() {
        let url = "ws://127.0.0.1:9944";
        let mut websocket = accept(stream.unwrap()).unwrap();

        let client = WsRpcClient::new(&url);
        let api = Api::<sr25519::Pair, _>::new(client).unwrap();

        println!("Subscribe to events");
        let (events_in, events_out) = channel();
        api.subscribe_events(events_in).unwrap();
        websocket
            .write_message("I'm just gonna keep sending you messages".into())
            .unwrap();
        loop {
            let event_str = events_out.recv().unwrap();

            let unhex = Vec::from_hex(event_str).unwrap();
            let mut er_enc = unhex.as_slice();
            let events = Vec::<system::EventRecord<Event, Hash>>::decode(&mut er_enc);
            match events {
                Ok(evts) => {
                    for evr in &evts {
                        println!("decoded: {:?} {:?}", evr.phase, evr.event);
                        let message = format!("decoded: {:?} {:?}", evr.phase, evr.event);
                        websocket.write_message(message.into()).unwrap();

                        match &evr.event {
                            Event::Balances(be) => {
                                println!(">>>>>>>>>> balances event: {:?}", be);
                                match &be {
                                    balances::Event::Transfer(transactor, dest, value) => {
                                        println!("Transactor: {:?}", transactor);
                                        println!("Destination: {:?}", dest);
                                        println!("Value: {:?}", value);
                                        return;
                                    }
                                    _ => {
                                        debug!("ignoring unsupported balances event");
                                    }
                                }
                            }
                            _ => debug!("ignoring unsupported module event: {:?}", evr.event),
                        }
                    }
                }
                Err(_) => error!("couldn't decode event record list"),
            }
        }
    }
}
