use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::Mutable;

#[inline]
pub fn label_element(value: &Mutable<String>, has_focus: &Mutable<bool>, label: &str) -> Dom {
    html!("span", {
        .class_signal(
            "above",
            clone!(value => map_ref!(
                let focus = has_focus.signal_cloned(),
                let current_value = value.signal_cloned() => move {
                    let has_value = current_value.len() > 0;

                    *focus || has_value
                })))
        .child(crate::text!(label))
        .class("dmat-input-label-text")
    })
}
