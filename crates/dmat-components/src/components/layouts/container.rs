use dominator::{html, Dom, DomBuilder};
use web_sys::HtmlElement;

#[macro_export]
macro_rules! container {
    ($mixin: expr) => {{
        $crate::components::layouts::container::container($mixin)
    }};
}

pub fn container<F>(mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class("dmat-container")
        .apply(mixin)
    })
}
