use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};

pub fn assistive_text<TAssistiveTextSignal>(
    assistive_text_signal: TAssistiveTextSignal,
    has_assistive: &Mutable<bool>,
) -> Dom
where
    TAssistiveTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
{
    let has_assistive = has_assistive.clone();

    let assistive_element_signal = map_ref!(
        let assistive_text = assistive_text_signal => move {
            let ass = has_assistive.clone();

            if let Some(str) = assistive_text {
                ass.set(true);
                Some(crate::text!(str, |d| d.class("dmat-assistive-text")))
            } else {
                ass.set(false);
                None
            }
        }
    );

    html!("span", {
        .child_signal(assistive_element_signal)
    })
}
