use dominator::{html, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};

pub fn assistive_text<TAssistiveTextSignal>(
    assistive_text_signal: TAssistiveTextSignal,
    has_assistive: &Mutable<bool>,
) -> Dom
where
    TAssistiveTextSignal: Signal<Item = Option<Dom>> + 'static,
{
    let has_assistive = has_assistive.clone();

    let assistive_element_signal = assistive_text_signal.map(move |assistive_text| {
        let ass = has_assistive.clone();

        if let Some(assistive_child) = assistive_text {
            ass.set(true);
            Some(html!("div", {
                .child(assistive_child)
                .class("dmat-assistive-text")
            }))
        } else {
            ass.set(false);
            None
        }
    });

    html!("span", {
        .child_signal(assistive_element_signal)
    })
}
