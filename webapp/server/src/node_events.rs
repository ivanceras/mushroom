use clap::{load_yaml, App};
use codec::Decode;
use log::{debug, error};
use mycelium::AccountInfo;
use sp_core::sr25519;
use sp_core::H256 as Hash;
use sp_keyring::AccountKeyring;
use std::sync::mpsc::channel;
// This module depends on node_runtime.
// To avoid dependency collisions, node_runtime has been removed from the substrate-api-client library.
// Replace this crate by your own if you run a custom substrate node to get your custom events.
use mycelium::rpc::WsRpcClient;
use mycelium::utils::FromHexString;
use mycelium::Api;
use mycelium::Metadata;
use node_template_runtime::Event;
use std::convert::TryFrom;

pub fn get_storage() {
    env_logger::init();
    let url = get_node_url_from_cli();

    let client = WsRpcClient::new(&url);
    let mut api = Api::new(client).unwrap();

    // get some plain storage value
    let result: u128 = api
        .get_storage_value("Balances", "TotalIssuance", None)
        .unwrap()
        .unwrap();
    println!("[+] TotalIssuance is {}", result);

    let proof = api
        .get_storage_value_proof("Balances", "TotalIssuance", None)
        .unwrap();
    println!("[+] StorageValueProof: {:?}", proof);

    // get StorageMap
    let account = AccountKeyring::Alice.public();
    let result: AccountInfo = api
        .get_storage_map("System", "Account", account, None)
        .unwrap()
        .or_else(|| Some(AccountInfo::default()))
        .unwrap();
    println!("[+] AccountInfo for Alice is {:?}", result);

    // get StorageMap key prefix
    let result = api.get_storage_map_key_prefix("System", "Account").unwrap();
    println!("[+] key prefix for System Account map is {:?}", result);

    // get Alice's AccountNonce with api.get_nonce()
    let signer = AccountKeyring::Alice.pair();
    api.signer = Some(signer);
    println!("[+] Alice's Account Nonce is {}", api.get_nonce().unwrap());
}

pub fn get_node_url_from_cli() -> String {
    let yml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9944");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    url
}

pub fn listen_to_events() {
    let url = get_node_url_from_cli();
    println!("url: {}", url);

    let client = WsRpcClient::new(&url);
    let api = Api::<sr25519::Pair, _>::new(client).unwrap();

    println!("Subscribe to events");
    let (events_in, events_out) = channel();
    api.subscribe_events(events_in).unwrap();

    loop {
        let event_str = events_out.recv().unwrap();

        let _unhex = Vec::from_hex(event_str).unwrap();
        let mut _er_enc = _unhex.as_slice();
        let _events = Vec::<system::EventRecord<Event, Hash>>::decode(&mut _er_enc);
        match _events {
            Ok(evts) => {
                for evr in &evts {
                    println!("decoded: {:?}, {:?}", evr.phase, evr.event);
                    match &evr.event {
                        Event::Balances(be) => {
                            println!(">>>>>>>>>> balances event: {:?}", be);
                            match &be {
                                balances::Event::Transfer(transactor, dest, value) => {
                                    println!("Transactor: {:?}", transactor);
                                    println!("Destination: {:?}", dest);
                                    println!("Value: {:?}", value);
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

pub fn get_metadata() -> String {
    let url = get_node_url_from_cli();
    let client = WsRpcClient::new(&url);
    let api = Api::<sr25519::Pair, _>::new(client).unwrap();

    Metadata::pretty_format(&api.get_metadata().unwrap())
        .unwrap_or_else(|| "pretty format failed".to_string())
}

pub fn show_metadata() {
    let url = get_node_url_from_cli();
    let client = WsRpcClient::new(&url);
    let api = Api::<sr25519::Pair, _>::new(client).unwrap();

    let meta = Metadata::try_from(api.get_metadata().unwrap()).unwrap();

    /*
    meta.print_overview();
    meta.print_modules_with_calls();
    meta.print_modules_with_events();
    */

    // print full substrate metadata json formatted
    println!(
        "{}",
        Metadata::pretty_format(&api.get_metadata().unwrap())
            .unwrap_or_else(|| "pretty format failed".to_string())
    )
}
