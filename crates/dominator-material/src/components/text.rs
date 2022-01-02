use crate::elements::new_html::new_html;
use dominator::traits::AsStr;
use dominator::DomBuilder;
use futures_signals::signal::Signal;
use web_sys::Element;

pub fn text<T: AsStr>(value: T) -> DomBuilder<Element> {
    new_html("span").text(value.as_str())
}

pub fn dynamic_text<T: AsStr, TSig: Signal<Item = T> + 'static>(
    value_signal: TSig,
) -> DomBuilder<Element> {
    new_html("span").text_signal(value_signal)
}
