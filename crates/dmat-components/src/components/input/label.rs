use crate::futures_signals::signal::SignalExt;
use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::Signal;

#[inline]
pub fn label_element(
    has_value_signal: impl Signal<Item = bool> + 'static,
    has_focus_signal: impl Signal<Item = bool> + 'static,
    label: impl Signal<Item = Option<Dom>> + 'static,
) -> Dom {
    html!("label", {
        .class_signal(
            "above",
            map_ref!(
                let has_focus = has_focus_signal,
                let has_value = has_value_signal => move {
                    *has_focus || *has_value
                }))
        .children(&mut [
            html!("div", {.class("dmat-notch-left")}),
            html!("div", {
                .class("dmat-notch-middle")
                .apply(|dom_builder| {
                    dom_builder.child_signal(label.map(|label_content| {
                        label_content.map(|label_content| {
                            html!("span", {
                                .child(label_content)
                                .class("dmat-input-label-text")
                            })
                        })
                    }))
                })
            }),
            html!("div", {.class("dmat-notch-right")}),
        ])
        .class("dmat-floating-label")
    })
}
