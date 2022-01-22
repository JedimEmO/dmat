use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};

pub fn error_text<TValidSig, TSig>(
    error_text_signal: Option<TSig>,
    is_valid: Option<TValidSig>,
    has_error: &Mutable<bool>,
) -> Option<Dom>
where
    TValidSig: Signal<Item = bool> + Unpin + 'static,
    TSig: Signal<Item = Option<String>> + Unpin + 'static,
{
    is_valid.map(move |valid_sig| {
            error_text_signal.map(|error| {
                let has_error = has_error.clone();
                let error_text_signal = map_ref!(
                let valid = valid_sig,
                let error_text = error => move {
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
        }).or(Some(html!("span"))).unwrap()
    })
}
