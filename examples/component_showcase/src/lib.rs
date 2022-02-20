#[macro_use]
extern crate dmat_components;

use crate::main_view::main_view;
use wasm_bindgen::prelude::*;

pub mod components;
pub mod demo_views;
pub mod main_view;
pub mod route;
pub mod utils;

#[wasm_bindgen(start)]
pub async fn main_js() {
    dominator::append_dom(&dominator::body(), main_view());
}
