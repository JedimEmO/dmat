use dominator::DomBuilder;
use futures_signals::map_ref;
use futures_signals::signal::Signal;
use web_sys::Element;

/// Creates a DomBuilder<A> -> DomBuilder<A> lambda which will apply the
/// provided bool signal to the disabled signal of the builder
/// if the disabled_signal parameter is some
#[inline]
pub fn with_disabled_signal<TSig, A: AsRef<Element>>(
    disabled_signal: Option<TSig>,
) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A>
where
    TSig: Signal<Item = bool> + Unpin + 'static,
{
    move |d| {
        d.apply_if(disabled_signal.is_some(), move |inner_builder| {
            inner_builder.attribute_signal(
                "disabled",
                map_ref!(let is_disabled = disabled_signal.unwrap() => {
                    match is_disabled {
                        true => Some("disabled"),
                        _ => None
                    }
                }),
            )
        })
    }
}
