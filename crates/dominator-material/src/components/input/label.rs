use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::Mutable;

#[inline]
pub fn label_element(
    input: Dom,
    value: &Mutable<String>,
    has_focus: &Mutable<bool>,
    label: &str,
) -> Dom {
    html!("span", {
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
            html!("div", {.class("dmat-notch-middle").child(crate::text!(label, |dom_builder| dom_builder.class("dmat-input-label-text")))}),
            html!("div", {.class("dmat-notch-right")}),
        ])
        .class("dmat-floating-label")

    })
}
