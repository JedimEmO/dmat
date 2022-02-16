use dominator::{html, Dom, DomBuilder};
use web_sys::HtmlElement;

#[macro_export]
macro_rules! card {
    ($props: expr) => {{
        $crate::components::card::card($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::card::card($props, $mixin)
    }};
}

#[inline]
pub fn card<F>(content: Dom, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class("dmat-card")
        .apply(mixin)
        .child(content)
    })
}
