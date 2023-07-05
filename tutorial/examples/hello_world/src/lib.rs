use dominator::{append_dom, body, html};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() {
    append_dom(
        &body(),
        html!("h1", {
            .text("Hello, world!")
        }),
    );
}