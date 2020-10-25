
use wasm_bindgen::prelude::*;
use crate::main_view::MainView;

pub mod main_view;

#[wasm_bindgen(start)]
pub async fn main_js() {
    dominator::append_dom(&dominator::body(), MainView::build().dom());
}
