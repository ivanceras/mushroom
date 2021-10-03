use sauron::js_sys::TypeError;
use sauron::prelude::*;
use sauron::wasm_bindgen::JsCast;
use sauron::web_sys::Response;
use serde::{Deserialize, Serialize};
use web_sys::{MessageEvent, WebSocket};

#[macro_use]
extern crate log;

pub enum Msg {
    WebSocketOpen,
    WebSocketMessage(String),
}

pub struct App {
    websocket: Option<WebSocket>,
}

impl Default for App {
    fn default() -> Self {
        Self { websocket: None }
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Cmd::new(move |program| {
            let websocket = WebSocket::new("ws://127.0.0.1:9002").expect("must open a websocket");
            let program2 = program.clone();
            let open_cb: Closure<dyn Fn(web_sys::Event)> = Closure::wrap(Box::new(move |event| {
                program.dispatch(Msg::WebSocketOpen);
            }));

            websocket
                .add_event_listener_with_callback("open", open_cb.as_ref().unchecked_ref())
                .expect("Unable to start interval");
            open_cb.forget();

            let message_cb: Closure<dyn Fn(web_sys::Event)> =
                Closure::wrap(Box::new(move |event| {
                    let message_event: MessageEvent =
                        event.dyn_into().expect("must cast to MessageEvent");
                    let message = message_event
                        .data()
                        .as_string()
                        .expect("must be a string message");
                    program2.dispatch(Msg::WebSocketMessage(message));
                }));

            websocket
                .add_event_listener_with_callback("message", message_cb.as_ref().unchecked_ref())
                .expect("Unable to start interval");
            message_cb.forget();
        })
    }
    fn view(&self) -> Node<Msg> {
        text("Hello world!")
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::WebSocketOpen => {
                log::info!("Web socket has opened");
            }
            Msg::WebSocketMessage(message) => {
                log::info!("Got a message from websocket: {:?}", message);
            }
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::default());
}
