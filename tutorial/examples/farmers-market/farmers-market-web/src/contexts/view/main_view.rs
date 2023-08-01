use crate::contexts::domain::product::ProductService;
use crate::contexts::view::product_list::product_list;
use dmat_components::components::layouts::*;
use dominator::{html, Dom};

pub fn main_view(product_service: &'static ProductService) -> Dom {
    app_bar!({
        .header(Some(html!("h1", {
            .text("Farmers Market Demo")
        })))
        .main(Some(container!({
            .apply(|d| d.child(product_list(product_service.products.signal_vec_cloned())))
        })))
    })
}
