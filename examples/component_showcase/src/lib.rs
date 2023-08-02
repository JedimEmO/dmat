#[macro_use]
extern crate dmat_components;

#[macro_use]
extern crate dmat_vis;

#[macro_use]
extern crate log;

use crate::main_view::main_view;
use wasm_bindgen::prelude::*;

pub mod components;
pub mod demo_views;
pub mod main_view;
pub mod route;
pub mod utils;
pub mod vis_components;

#[wasm_bindgen(start)]
pub async fn main_js() {
    wasm_logger::init(wasm_logger::Config::default());
    debug!("Starting dmat-components showcase");
    dominator::append_dom(&dominator::body(), main_view());
}
