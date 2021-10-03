use sauron::html;
use sauron::prelude::*;

pub fn index() -> Node<()> {
    html(
        [],
        [html::body(
            [],
            [script(
                [r#type("module")],
                [text!(
                    "{}\ninit(new URL('./pkg/webapp_client_bg.wasm', import.meta.url)).catch(console.err);",
                    include_str!("../../client/pkg/webapp_client.js")
                )],
            )],
        )],
    )
}
