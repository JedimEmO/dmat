use crate::elements::new_html::new_html;
use dominator::DomBuilder;
use futures_signals::signal::{Signal, SignalExt};
use web_sys::Element;

#[inline]
pub fn text<T: Into<String>>(value: T) -> DomBuilder<Element> {
    new_html("span").text(value.into().as_str())
}

pub fn dynamic_text<T: Into<String>, TSig: Signal<Item = T> + 'static>(
    value_signal: TSig,
) -> DomBuilder<Element> {
    new_html("span").text_signal(value_signal.map(|v| v.into()))
}
