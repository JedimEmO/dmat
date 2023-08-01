#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate dmat_components;

mod contexts;

use crate::contexts::view::main_view::main_view;
use dominator::append_dom;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
fn main() {
    let host = web_sys::window().unwrap().location().host().unwrap();

    let product_repository =
        contexts::data_access_impl::product_repo_impl::ProductRepositoryImpl::new(host);

    let (product_run_fut, product_service) =
        contexts::domain::product::run_product_service(product_repository);

    spawn_local(product_run_fut);
    append_dom(&dominator::body(), main_view(product_service));
}
