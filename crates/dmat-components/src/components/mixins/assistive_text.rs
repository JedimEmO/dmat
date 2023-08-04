use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};

pub fn assistive_text<TAssistiveTextSignal>(
    assistive_text_signal: TAssistiveTextSignal,
) -> impl Signal<Item = Option<Dom>>
where
    TAssistiveTextSignal: Signal<Item = Option<Dom>> + 'static,
{
    assistive_text_signal.map(move |assistive_text| {
        assistive_text.map(|assistive_child| {
            html!("div", {
                .child(assistive_child)
                .class("-assistive-text")
            })
        })
    })
}
