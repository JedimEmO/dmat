use dominator::{html, Dom, DomBuilder};
use futures_signals::signal::{Signal, SignalExt};
use web_sys::HtmlElement;

#[inline]
pub fn text<T: Into<String>, F>(value: T, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("span", {
        .apply(mixin)
        .text(value.into().as_str())
    })
}

#[inline]
pub fn dynamic_text<T: Into<String>, TSig: Signal<Item = T> + 'static, F>(
    value_signal: TSig,
    mixin: F,
) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("span", {
        .apply(mixin)
        .text_signal(value_signal.map(|v| v.into()))
    })
}
