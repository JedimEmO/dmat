use crate::utils::component_signal::ComponentSignal;
use dominator::{html, Dom, DomBuilder};
use web_sys::HtmlElement;

#[macro_export]
macro_rules! container {
    ($props: expr) => {{
        $crate::components::layouts::container::container($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::layouts::container::container($props, $mixin)
    }};
}

pub fn container<T: Into<ComponentSignal>, F>(child: T, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class("dmat-container")
        .apply(mixin)
        .child_signal(child.into().0)
    })
}
