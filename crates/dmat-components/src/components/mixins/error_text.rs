use dominator::{html, Dom};
use futures_signals::signal::{Mutable, Signal};

use futures_signals::map_mut;

pub fn error_text<TValidSig, TErrorTextSignal>(
    error_text_signal: TErrorTextSignal,
    is_valid: TValidSig,
    has_error: &Mutable<bool>,
) -> Dom
where
    TValidSig: Signal<Item = bool> + 'static,
    TErrorTextSignal: Signal<Item = Option<Dom>> + 'static,
{
    let has_error = has_error.clone();

    let error_text_signal = map_mut!(
        let valid = is_valid,
        let error = error_text_signal => move {
            if let Some(error_child) = error {
                if !*valid {
                    has_error.set(true);
                    Some(html!("div", {
                            .class("dmat-assistive-text")
                            .class("dmat-error-text")
                            .child(error_child)
                        }))
                } else {
                    None
                }
            } else {
                has_error.set(false);
                None
            }
        }
    );

    html!("span", {
        .child_signal(error_text_signal)
    })
}
