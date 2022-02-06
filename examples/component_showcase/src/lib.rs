#![feature(async_closure)]

#[macro_use]
extern crate dominator_material;

use crate::main_view::main_view;
use wasm_bindgen::prelude::*;

pub mod components;
pub mod main_view;
pub mod route;

#[wasm_bindgen(start)]
pub async fn main_js() {
    dominator::append_dom(&dominator::body(), main_view());
}
