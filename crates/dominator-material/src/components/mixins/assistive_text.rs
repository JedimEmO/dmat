use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};

pub fn assistive_text<TSig>(
    assistive_text_signal: Option<TSig>,
    has_assistive: &Mutable<bool>,
) -> Option<Dom>
where
    TSig: Signal<Item = Option<String>> + Unpin + 'static,
{
    assistive_text_signal.map(|assistive| {
        let has_assistive = has_assistive.clone();

        let assistive_element_signal = map_ref!(
            let assistive_text = assistive => move {
                let ass = has_assistive.clone();

                if let Some(str) = assistive_text {
                    ass.set(true);
                    return Some(crate::text!(str, |d| d.class("dmat-assistive-text")))
                }

                ass.set(false);
                None
            }
        );

        html!("span", {
            .child_signal(assistive_element_signal)
        })
    })
}
