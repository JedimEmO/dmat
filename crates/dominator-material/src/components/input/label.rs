use crate::futures_signals::signal::SignalExt;
use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};

#[inline]
pub fn label_element<TLabelSignal: Signal<Item = Option<String>> + Unpin + 'static>(
    input: Dom,
    value: &Mutable<String>,
    has_focus: &Mutable<bool>,
    label: TLabelSignal,
) -> Dom {
    html!("label", {
        .class_signal(
            "above",
            clone!(value => map_ref!(
                let focus = has_focus.signal_cloned(),
                let current_value = value.signal_cloned() => move {
                    let has_value = current_value.len() > 0;

                    *focus || has_value
                })))
        .children(&mut [
            input,
            html!("div", {.class("dmat-notch-left")}),
            html!("div", {
                .class("dmat-notch-middle")
                .apply(|dom_builder| {
                    dom_builder.child_signal(label.map(|label_content| {
                        if let Some(label_text) = label_content {
                            Some(crate::text!(label_text, |dom_builder| dom_builder.class("dmat-input-label-text")))
                        } else {
                            None
                        }
                    }))
                })
            }),
            html!("div", {.class("dmat-notch-right")}),
        ])
        .class("dmat-floating-label")

    })
}
