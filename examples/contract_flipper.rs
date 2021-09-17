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

//! This examples shows how to use the compose_extrinsic macro to create an extrinsic for any (custom)
//! module, whereas the desired module and call are supplied as a string.

use clap::{load_yaml, App};
use keyring::AccountKeyring;
use sp_core::crypto::Pair;

use mushroom::rpc::WsRpcClient;
use mushroom::{compose_extrinsic, Api, UncheckedExtrinsicV4, XtStatus};

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();

    // initialize api and set the signer (sender) that is used to sign the extrinsics
    let from = AccountKeyring::Alice.pair();
    let client = WsRpcClient::new(&url);
    let api = Api::new(client).map(|api| api.set_signer(from)).unwrap();

    // set the recipient
    let to = AccountKeyring::Bob.to_account_id();

    //  Flipper contract address:
    // 5EfiGu5F6o8bzqaoc4V5qYfqWiyRZrNBdMeEs1ddbJeaRyuR
    // 0xe33c0df26324f78febe2a9a881b04bc3f0e03a0fc8490b4fe6f52309ef20619d
    //
    // using subkey inspect:
    // AccountId: 0x732b765cba55b736698d78f49f89b01a675e246a9c97d9bdee387e446ca6a577

    // call Balances::transfer
    // the names are given as strings
    #[allow(clippy::redundant_clone)]
    let xt: UncheckedExtrinsicV4<_> = /*compose_extrinsic!(
        api.clone(),
        "Contracts",
        "call",
        GenericAddress::Id(to),
        "read",
        "get"
    ); */
    compose_extrinsic!(
        api.clone(),
        "Contracts",
        "call",
        //GenericAddress::Raw(hex::decode("e33c0df26324f78febe2a9a881b04bc3f0e03a0fc8490b4fe6f52309ef20619d").unwrap().to_vec()),
        //GenericAddress::Raw(b"5EfiGu5F6o8bzqaoc4V5qYfqWiyRZrNBdMeEs1ddbJeaRyuR".to_vec()),
        //GenericAddress::Raw(hex::decode("732b765cba55b736698d78f49f89b01a675e246a9c97d9bdee387e446ca6a577").unwrap().to_vec()),
        GenericAddress::Raw(b"0x732b765cba55b736698d78f49f89b01a675e246a9c97d9bdee387e446ca6a577".to_vec()),
        Compact(10_u128),
        Compact(10_u128),
        "flip"
    );

    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    // send and watch extrinsic until InBlock
    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);
}

pub fn get_node_url_from_cli() -> String {
    let yml = load_yaml!("./cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9944");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    url
}
