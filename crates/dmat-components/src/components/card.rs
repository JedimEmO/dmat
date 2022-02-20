use dominator::{html, Dom, DomBuilder};
use futures_signals::signal::Signal;
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
#[macro_export]
macro_rules! dynamic_card {
    ($props: expr) => {{
        $crate::components::card::dynamic_card($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::card::dynamic_card($props, $mixin)
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

#[inline]
pub fn dynamic_card<F, TContentSignal>(content_signal: TContentSignal, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    TContentSignal: Signal<Item = Option<Dom>> + 'static,
{
    html!("div", {
        .class("dmat-card")
        .apply(mixin)
        .child_signal(content_signal)
    })
}
