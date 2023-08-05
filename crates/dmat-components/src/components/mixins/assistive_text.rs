use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};

pub fn assistive_text(
    assistive_text_signal: impl Signal<Item = Option<Dom>> + 'static,
) -> impl Signal<Item = Option<Dom>> {
    assistive_text_signal.map(|assistive_child| {
        assistive_child.map(|child| {
            html!("div", {
                .child(child)
                .class("-assistive-text")
            })
        })
    })
}
