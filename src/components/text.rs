use dominator::traits::AsStr;
use dominator::{html, Dom};
use futures_signals::signal::Signal;

pub fn text<T: AsStr>(value: T) -> Dom {
    html!("span", {
        .text(value.as_str())
    })
}

pub fn dynamic_text<T: AsStr, TSig: Signal<Item = T> + 'static>(value_signal: TSig) -> Dom {
    html!("span", {
        .text_signal(value_signal)
    })
}
