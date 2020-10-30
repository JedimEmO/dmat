use crate::main_view::MainView;
use wasm_bindgen::prelude::*;

pub mod components;
pub mod main_view;

#[wasm_bindgen(start)]
pub async fn main_js() {
    dominator::append_dom(&dominator::body(), MainView::new().render());
}
