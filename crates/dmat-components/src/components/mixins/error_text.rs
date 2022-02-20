use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};

pub fn error_text<TValidSig, TErrorTextSignal>(
    error_text_signal: TErrorTextSignal,
    is_valid: TValidSig,
    has_error: &Mutable<bool>,
) -> Dom
where
    TValidSig: Signal<Item = bool> + Unpin + 'static,
    TErrorTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
{
    let has_error = has_error.clone();

    let error_text_signal = map_ref!(
        let valid = is_valid,
        let error_text = error_text_signal => move {
            if let Some(str) = error_text {
                if !*valid {
                    has_error.set(true);
                    return Some(crate::text!(str, |d| d.class("dmat-assistive-text").class("dmat-error-text")));
                }
            }

            has_error.set(false);

            None
        }
    );

    html!("span", {
        .child_signal(error_text_signal)
    })
}
