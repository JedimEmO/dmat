use dmat_components::components::layouts::*;
use dmat_components::components::*;
use dominator::{html, Dom};

pub fn carousel_demo() -> Dom {
    card!({.child(carousel!({
        .item_render_fn(render_carousel_item)
        .apply(|d| d.class("demo-carousel"))
    }))})
}

fn render_carousel_item(_idx: i32) -> Dom {
    container!({
        .apply(|d| d.child(html!("img", {
            .attr("src", "images/shapes.svg")
            .attr("width", "60%")
            .attr("height", "100%")
            .attr("alt", "shapes!")
        })).class("demo-carousel-item"))
    })
}
