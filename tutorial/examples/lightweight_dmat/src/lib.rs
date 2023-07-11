#[macro_use]
extern crate dmat_components;

use dominator::{append_dom, body, html};
use wasm_bindgen::prelude::*;

mod tasks;

#[wasm_bindgen(start)]
fn main_js() {
    append_dom(
        &body(),
        card!({
            .child(html!("div", {
                .child(tasks::counter::counter())
            }))
        }),
    );
}
